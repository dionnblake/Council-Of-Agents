use crate::model::{FailureType, ProviderKind, ProviderPosition, TurnState, new_id};
use crate::providers::{
    CommandSpec, ProviderCallRequest, ProviderCallResult, ProviderError, ProviderRegistry,
    RepairPolicy, classify_failure, ensure_subscription_environment, repair_policy_for,
};
use crate::runner::ProcessRunner;
use crate::validation::{assign_controller_claim_ids, validate_position_value};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

pub trait ProviderExecutor: Send + Sync {
    fn execute(&self, request: &ProviderCallRequest) -> Result<ProviderCallResult, ProviderError>;
}

#[derive(Debug, Clone)]
pub struct LiveProviderExecutor {
    pub registry: ProviderRegistry,
    pub runner: ProcessRunner,
}

impl LiveProviderExecutor {
    pub fn new(registry: ProviderRegistry) -> Self {
        Self {
            registry,
            runner: ProcessRunner::default(),
        }
    }
}

impl ProviderExecutor for LiveProviderExecutor {
    fn execute(&self, request: &ProviderCallRequest) -> Result<ProviderCallResult, ProviderError> {
        ensure_subscription_environment()?;
        let command: CommandSpec = self.registry.build_command(request)?;
        let result = self
            .runner
            .run(&command, &request.prompt)
            .map_err(|error| ProviderError::ProcessRunner(error.to_string()))?;
        let (reported_served_model, serving_identity_status) =
            crate::providers::serving_identity_from_jsonl(&result.stdout, &request.model);
        let failure_type = if result.timed_out {
            Some(FailureType::Timeout)
        } else {
            classify_failure(&result.stdout, &result.stderr, result.exit_code)
        };
        Ok(ProviderCallResult {
            provider: request.provider.clone(),
            exit_code: result.exit_code,
            wall_ms: result.wall_ms,
            stdout: result.stdout,
            stderr: result.stderr,
            requested_model: request.model.clone(),
            reported_served_model,
            serving_identity_status,
            failure_type,
            raw_artifact_id: new_id("raw"),
            timed_out: result.timed_out,
            cancellation_fallback_ran: result.cancellation_fallback_ran,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RoundRequest {
    pub round: u8,
    pub provider_requests: Vec<ProviderCallRequest>,
    pub repository_grounded: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AttemptRecord {
    pub attempt_number: u8,
    pub state: TurnState,
    pub failure_type: Option<FailureType>,
    pub raw_result: Option<ProviderCallResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TurnRecord {
    pub turn_id: String,
    pub provider: ProviderKind,
    pub round: u8,
    pub state: TurnState,
    pub repair_policy: RepairPolicy,
    pub attempts: Vec<AttemptRecord>,
    pub position: Option<ProviderPosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CouncilRunResult {
    pub turns: Vec<TurnRecord>,
    pub positions: Vec<ProviderPosition>,
    pub final_positions: Vec<ProviderPosition>,
}

pub struct CouncilOrchestrator<E> {
    pub executor: E,
}

impl<E: ProviderExecutor> CouncilOrchestrator<E> {
    pub fn new(executor: E) -> Self {
        Self { executor }
    }

    pub fn run(&self, rounds: &[RoundRequest]) -> CouncilRunResult {
        let mut turns = Vec::new();
        let mut positions = Vec::new();
        for round in rounds {
            for request in &round.provider_requests {
                let turn = self.run_turn(request, round.round, round.repository_grounded);
                if let Some(position) = &turn.position {
                    positions.push(position.clone());
                }
                turns.push(turn);
            }
        }
        let mut latest = BTreeMap::<ProviderKind, ProviderPosition>::new();
        for position in &positions {
            latest
                .entry(position.provider.clone())
                .and_modify(|current| {
                    if position.round >= current.round {
                        *current = position.clone();
                    }
                })
                .or_insert_with(|| position.clone());
        }
        CouncilRunResult {
            turns,
            positions,
            final_positions: latest.into_values().collect(),
        }
    }

    fn run_turn(
        &self,
        request: &ProviderCallRequest,
        round: u8,
        repository_grounded: bool,
    ) -> TurnRecord {
        let turn_id = request.turn_id.clone().unwrap_or_else(|| new_id("turn"));
        let policy = repair_policy_for(&request.provider);
        let mut attempts = Vec::new();
        let mut attempt_number = 1_u8;
        let mut current_request = request.clone();
        let mut final_position = None;

        let final_state = loop {
            let execution = self.executor.execute(&current_request);
            let Ok(raw_result) = execution else {
                attempts.push(AttemptRecord {
                    attempt_number,
                    state: TurnState::Failed,
                    failure_type: Some(FailureType::SafetyViolation),
                    raw_result: None,
                });
                break TurnState::Failed;
            };

            if let Some(failure_type) = raw_result.failure_type.clone() {
                attempts.push(AttemptRecord {
                    attempt_number,
                    state: state_for_failure(&failure_type),
                    failure_type: Some(failure_type),
                    raw_result: Some(raw_result),
                });
                break attempts
                    .last()
                    .map(|attempt| attempt.state.clone())
                    .unwrap_or(TurnState::Failed);
            }

            let Some(position_value) = extract_position_value(&raw_result.stdout) else {
                let failure_type = FailureType::NoStructuredOutput;
                let should_repair = should_repair(&policy, attempt_number, &failure_type);
                attempts.push(AttemptRecord {
                    attempt_number,
                    state: if should_repair {
                        TurnState::Repairing
                    } else {
                        TurnState::Quarantined
                    },
                    failure_type: Some(failure_type),
                    raw_result: Some(raw_result),
                });
                if should_repair {
                    current_request.prompt = repair_prompt(&request.prompt);
                    attempt_number += 1;
                    continue;
                }
                break TurnState::Quarantined;
            };

            let validation = validate_position_value(&position_value, repository_grounded);
            if validation.schema_valid && validation.semantic_valid {
                let mut position = validation.position.expect("valid position is present");
                assign_controller_claim_ids(&mut position, request.provider.slug(), round);
                let provider_position = ProviderPosition {
                    provider: request.provider.clone(),
                    round,
                    turn_id: turn_id.clone(),
                    position,
                    raw_artifact_id: raw_result.raw_artifact_id.clone(),
                    requested_model: raw_result.requested_model.clone(),
                    reported_served_model: raw_result.reported_served_model.clone(),
                    serving_identity_status: raw_result.serving_identity_status.clone(),
                };
                attempts.push(AttemptRecord {
                    attempt_number,
                    state: TurnState::Valid,
                    failure_type: None,
                    raw_result: Some(raw_result),
                });
                final_position = Some(provider_position);
                break TurnState::Valid;
            }

            let failure_type = if !validation.schema_valid {
                FailureType::SchemaInvalid
            } else {
                FailureType::SemanticInvalid
            };
            let should_repair = should_repair(&policy, attempt_number, &failure_type);
            attempts.push(AttemptRecord {
                attempt_number,
                state: if should_repair {
                    TurnState::Repairing
                } else {
                    TurnState::Quarantined
                },
                failure_type: Some(failure_type),
                raw_result: Some(raw_result),
            });
            if should_repair {
                current_request.prompt = repair_prompt(&request.prompt);
                attempt_number += 1;
                continue;
            }
            break TurnState::Quarantined;
        };

        TurnRecord {
            turn_id,
            provider: request.provider.clone(),
            round,
            state: final_state,
            repair_policy: policy,
            attempts,
            position: final_position,
        }
    }
}

fn repair_prompt(original: &str) -> String {
    format!(
        "{original}\nReturn only one corrected JSON object matching output-position.v1. Do not add commentary."
    )
}

fn should_repair(policy: &RepairPolicy, attempt_number: u8, failure: &FailureType) -> bool {
    matches!(policy, RepairPolicy::OneRepairAttempt)
        && attempt_number == 1
        && matches!(
            failure,
            FailureType::SchemaInvalid
                | FailureType::SemanticInvalid
                | FailureType::NoStructuredOutput
                | FailureType::Truncated
        )
}

fn state_for_failure(failure: &FailureType) -> TurnState {
    match failure {
        FailureType::AuthRequired => TurnState::AuthRequired,
        FailureType::ProviderLimit => TurnState::ProviderLimit,
        FailureType::Timeout => TurnState::Timeout,
        FailureType::SchemaInvalid
        | FailureType::SemanticInvalid
        | FailureType::NoStructuredOutput
        | FailureType::Truncated
        | FailureType::Refusal => TurnState::Quarantined,
        _ => TurnState::Failed,
    }
}

pub fn extract_position_value(stdout: &str) -> Option<Value> {
    if let Ok(value) = serde_json::from_str::<Value>(stdout) {
        if let Some(position) = find_position_object(&value) {
            return Some(position);
        }
    }
    for line in stdout.lines().filter(|line| !line.trim().is_empty()) {
        if let Ok(value) = serde_json::from_str::<Value>(line) {
            if let Some(position) = find_position_object(&value) {
                return Some(position);
            }
        }
    }
    None
}

fn find_position_object(value: &Value) -> Option<Value> {
    match value {
        Value::Object(map) => {
            if map.contains_key("recommendation")
                && map.contains_key("commitment")
                && map.contains_key("claims")
                && map.contains_key("risks")
            {
                return Some(value.clone());
            }
            map.values().find_map(find_position_object)
        }
        Value::Array(values) => values.iter().find_map(find_position_object),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::ServingIdentityStatus;
    use std::collections::VecDeque;
    use std::path::PathBuf;
    use std::sync::{Arc, Mutex};

    struct FakeExecutor {
        outputs: Arc<Mutex<VecDeque<String>>>,
    }

    impl ProviderExecutor for FakeExecutor {
        fn execute(
            &self,
            request: &ProviderCallRequest,
        ) -> Result<ProviderCallResult, ProviderError> {
            let stdout = self.outputs.lock().unwrap().pop_front().unwrap();
            Ok(ProviderCallResult {
                provider: request.provider.clone(),
                exit_code: Some(0),
                wall_ms: 1,
                stdout,
                stderr: String::new(),
                requested_model: request.model.clone(),
                reported_served_model: Some(request.model.clone()),
                serving_identity_status: ServingIdentityStatus::VerifiedMatch,
                failure_type: None,
                raw_artifact_id: new_id("raw"),
                timed_out: false,
                cancellation_fallback_ran: false,
            })
        }
    }

    fn valid_json() -> String {
        serde_json::json!({
            "schema_version": "output-position.v1",
            "recommendation": "Choose the reversible path",
            "commitment": "WOULD_STAKE",
            "claims": [{"id":"provider","text":"It reduces risk","evidence":["src/main.rs:1-2"]}],
            "risks": ["Migration may be needed later"],
            "flip_condition": "Measured scale exceeds the local boundary",
            "cost_if_wrong": "A contained migration consumes one sprint",
            "reversibility": "EASY"
        })
        .to_string()
    }

    fn request(provider: ProviderKind) -> ProviderCallRequest {
        ProviderCallRequest {
            provider,
            model: "test-model".to_string(),
            turn_id: None,
            packet_path: PathBuf::from("packet.md"),
            packet_directory: PathBuf::from("."),
            schema_path: PathBuf::from("schema.json"),
            working_directory: PathBuf::from("."),
            scratch_directory: PathBuf::from("."),
            prompt: "Read packet".to_string(),
            timeout_ms: 1_000,
            linux_packet_path: Some(PathBuf::from("/packet.md")),
            linux_working_directory: Some(PathBuf::from("/")),
            linux_schema_path: Some(PathBuf::from("/schema.json")),
        }
    }

    #[test]
    fn extracts_nested_structured_position() {
        let stdout = serde_json::json!({"type":"result","structured_output":serde_json::from_str::<Value>(&valid_json()).unwrap()}).to_string();
        assert!(extract_position_value(&stdout).is_some());
    }

    #[test]
    fn antigravity_gets_one_repair_attempt_and_controller_ids() {
        let executor = FakeExecutor {
            outputs: Arc::new(Mutex::new(VecDeque::from([
                r#"{"schema_version":"wrong"}"#.to_string(),
                valid_json(),
            ]))),
        };
        let orchestrator = CouncilOrchestrator::new(executor);
        let result = orchestrator.run(&[RoundRequest {
            round: 1,
            provider_requests: vec![request(ProviderKind::Antigravity)],
            repository_grounded: false,
        }]);
        assert_eq!(result.turns[0].state, TurnState::Valid);
        assert_eq!(result.turns[0].attempts.len(), 2);
        assert_eq!(
            result.positions[0].position.claims[0].id,
            "C-ANTIGRAVITY-R1-001"
        );
    }

    #[test]
    fn claude_does_not_automatically_repair() {
        let executor = FakeExecutor {
            outputs: Arc::new(Mutex::new(VecDeque::from([
                r#"{"schema_version":"wrong"}"#.to_string(),
            ]))),
        };
        let orchestrator = CouncilOrchestrator::new(executor);
        let result = orchestrator.run(&[RoundRequest {
            round: 1,
            provider_requests: vec![request(ProviderKind::Claude)],
            repository_grounded: false,
        }]);
        assert_eq!(result.turns[0].state, TurnState::Quarantined);
        assert_eq!(result.turns[0].attempts.len(), 1);
        assert_eq!(
            result.turns[0].repair_policy,
            RepairPolicy::NoAutomaticRepair
        );
    }
}
