#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use council_core::{
    ContextPacket, CouncilOrchestrator, Database, Debate, DebateEvent, DebateState, DecisionRecord,
    FailureType, HumanDecision, HumanDecisionKind, Intake, LiveProviderExecutor, ModelSelection,
    POSITION_SCHEMA_VERSION, ProviderCallRequest, ProviderConfig, ProviderKind, ProviderPosition,
    ProviderRegistry, RoundRequest, ServingIdentityStatus, TurnState, WslBridgeRequest,
    billing_environment_status, build_wsl_bridge_plan, compile_decision_record,
    compile_master_prompt, content_hash, deterministic_call_id, ensure_subscription_environment,
    new_id, validate_intake,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use tauri::Manager;

#[derive(Debug, Clone, Serialize)]
struct ProviderStatus {
    provider: String,
    label: String,
    model: String,
    certification: String,
    state: String,
    detail: String,
    requested: String,
    served: String,
}

#[derive(Debug, Clone, Serialize)]
struct DebateSummary {
    id: String,
    state: DebateState,
    question: String,
    created_at: String,
}

#[derive(Debug, Clone, Serialize)]
struct ExportSummary {
    debate_id: String,
    directory: String,
    master_prompt_hash: String,
    decision_record_hash: String,
}

#[derive(Debug, Clone, Serialize)]
struct TurnSummary {
    provider: ProviderKind,
    state: TurnState,
    attempts: usize,
    failure_type: Option<FailureType>,
    requested_model: String,
    reported_served_model: Option<String>,
    serving_identity_status: ServingIdentityStatus,
}

#[derive(Debug, Clone, Serialize)]
struct RunRoundSummary {
    debate_id: String,
    round: u8,
    state: DebateState,
    packet_hashes: BTreeMap<String, String>,
    turns: Vec<TurnSummary>,
    valid_positions: usize,
    message: String,
}

#[derive(Debug, Clone, Deserialize)]
struct DecisionInput {
    debate_id: String,
    kind: String,
    selected_option: Option<String>,
    modified_decision: Option<String>,
    rationale: String,
}

fn certification_label(config: &ProviderConfig) -> String {
    match config.certification {
        council_core::CertificationStatus::Pass => "PASS".to_string(),
        council_core::CertificationStatus::PassWithDeclaredLimitation => {
            "PASS_WITH_DECLARED_LIMITATION".to_string()
        }
        council_core::CertificationStatus::Blocked => "BLOCKED".to_string(),
        council_core::CertificationStatus::Fail => "FAIL".to_string(),
        council_core::CertificationStatus::Disabled => "DISABLED".to_string(),
    }
}

fn command_exists(program: &str) -> bool {
    #[cfg(windows)]
    let mut command = Command::new("where.exe");
    #[cfg(not(windows))]
    let mut command = Command::new("which");
    command
        .arg(program)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn wsl_council_home_ready(config: &ProviderConfig) -> bool {
    let Some(distribution) = config.wsl_distribution.as_deref() else {
        return false;
    };
    if !command_exists("wsl.exe") {
        return false;
    }
    Command::new("wsl.exe")
        .args([
            "-d",
            distribution,
            "--user",
            config.wsl_user.as_deref().unwrap_or("council"),
            "--",
            "test",
            "-d",
            config.wsl_home.as_deref().unwrap_or("/home/council"),
        ])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn antigravity_guard_ready(config: &ProviderConfig) -> bool {
    let candidates = [
        std::env::var_os("USERPROFILE")
            .map(|root| PathBuf::from(root).join(".gemini").join("settings.json")),
        std::env::var_os("APPDATA").map(|root| {
            PathBuf::from(root)
                .join("antigravity")
                .join("settings.json")
        }),
        std::env::var_os("LOCALAPPDATA").map(|root| {
            PathBuf::from(root)
                .join("antigravity")
                .join("settings.json")
        }),
    ];
    candidates
        .into_iter()
        .flatten()
        .find(|path| path.is_file())
        .map(|path| council_core::providers::antigravity_credit_guard_from_json(&path).is_ok())
        .unwrap_or_else(|| {
            config
                .safety_settings
                .get("useG1Credits")
                .and_then(serde_json::Value::as_bool)
                == Some(false)
                && command_exists("agy.exe")
        })
}

fn provider_status(config: &ProviderConfig, registry: &ProviderRegistry) -> ProviderStatus {
    let blocked_billing_key = billing_environment_status()
        .into_iter()
        .find_map(|(key, present)| present.then_some(key));
    let (state, detail) = match config.provider {
        ProviderKind::Claude => {
            let executable = command_exists("claude.exe");
            let config_dir = config.config_dir.as_ref().is_some_and(|path| path.is_dir());
            if executable && config_dir {
                ("READY", "Executable and isolated config found")
            } else if !executable {
                ("NOT_READY", "claude.exe was not found on PATH")
            } else {
                ("NOT_READY", "Dedicated Claude config directory is missing")
            }
        }
        ProviderKind::Antigravity => {
            let executable = command_exists("agy.exe");
            if executable && antigravity_guard_ready(config) {
                (
                    "LIMITED",
                    "Credit guard verified; served identity is limited",
                )
            } else if !executable {
                ("NOT_READY", "agy.exe was not found on PATH")
            } else {
                (
                    "NOT_READY",
                    "useG1Credits=false was not verified from settings",
                )
            }
        }
        ProviderKind::CodexWsl => {
            if wsl_council_home_ready(config) {
                ("READY", "CouncilCodexWSL boundary is reachable")
            } else {
                (
                    "NOT_READY",
                    "CouncilCodexWSL or /home/council is unavailable",
                )
            }
        }
    };
    let preflight_detail = registry
        .preflight(&config.provider)
        .err()
        .map(|error| error.to_string());
    ProviderStatus {
        provider: config.provider.slug().to_uppercase().replace('-', "_"),
        label: config.provider.display_name().to_string(),
        model: config.model_default.clone(),
        certification: certification_label(config),
        state: if preflight_detail.is_some() || blocked_billing_key.is_some() {
            "NOT_READY".to_string()
        } else {
            state.to_string()
        },
        detail: if let Some(key) = blocked_billing_key {
            format!("{key} is present; subscription auth required")
        } else {
            preflight_detail.unwrap_or_else(|| detail.to_string())
        },
        requested: config.model_default.clone(),
        served: match config.provider {
            ProviderKind::Claude => "VERIFIED_MATCH".to_string(),
            _ => format!("{:?}", ServingIdentityStatus::ProviderDoesNotReport).to_uppercase(),
        },
    }
}

fn database_for(app: &tauri::AppHandle) -> Result<Database, String> {
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| format!("cannot resolve app data directory: {error}"))?;
    std::fs::create_dir_all(&data_dir)
        .map_err(|error| format!("cannot create app data directory: {error}"))?;
    Database::open(data_dir.join("council.sqlite3")).map_err(|error| error.to_string())
}

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

fn safe_component(value: &str) -> Result<(), String> {
    if value.is_empty()
        || !value
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || "-_.".contains(character))
    {
        return Err(format!("unsafe runtime path component: {value}"));
    }
    Ok(())
}

fn write_immutable(path: &std::path::Path, bytes: &[u8]) -> Result<(), String> {
    if path.exists() {
        let existing =
            fs::read(path).map_err(|error| format!("cannot read {}: {error}", path.display()))?;
        if existing != bytes {
            return Err(format!("immutable file differs: {}", path.display()));
        }
        return Ok(());
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("cannot create {}: {error}", parent.display()))?;
    }
    fs::write(path, bytes).map_err(|error| format!("cannot write {}: {error}", path.display()))?;
    let mut permissions = fs::metadata(path)
        .map_err(|error| format!("cannot inspect {}: {error}", path.display()))?
        .permissions();
    permissions.set_readonly(true);
    fs::set_permissions(path, permissions)
        .map_err(|error| format!("cannot seal {}: {error}", path.display()))?;
    let persisted =
        fs::read(path).map_err(|error| format!("cannot reread {}: {error}", path.display()))?;
    if persisted != bytes {
        return Err(format!(
            "immutable file changed after write: {}",
            path.display()
        ));
    }
    Ok(())
}

fn run_wsl_mkdir(distribution: &str, user: &str, path: &str) -> Result<(), String> {
    let status = Command::new("wsl.exe")
        .args([
            "-d",
            distribution,
            "--user",
            user,
            "--",
            "mkdir",
            "-p",
            path,
        ])
        .status()
        .map_err(|error| format!("cannot start WSL mkdir: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("WSL mkdir failed for {path}: {status}"))
    }
}

fn bridge_payload_to_wsl(
    source_directory: &std::path::Path,
    distribution: &str,
    user: &str,
    destination: &str,
) -> Result<(), String> {
    let plan = build_wsl_bridge_plan(&WslBridgeRequest {
        source_snapshot: source_directory.to_path_buf(),
        linux_distribution: distribution.to_string(),
        linux_user: user.to_string(),
        linux_destination: destination.to_string(),
    })
    .map_err(|error| error.to_string())?;
    let mut tar = Command::new(&plan.tar_program)
        .args(&plan.tar_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("cannot start tar bridge: {error}"))?;
    let tar_stdout = tar
        .stdout
        .take()
        .ok_or_else(|| "tar bridge did not expose stdout".to_string())?;
    let wsl = Command::new(&plan.wsl_program)
        .args(&plan.wsl_args)
        .stdin(Stdio::from(tar_stdout))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("cannot start WSL bridge: {error}"))?;
    let wsl_output = wsl
        .wait_with_output()
        .map_err(|error| format!("WSL bridge wait failed: {error}"))?;
    let tar_output = tar
        .wait_with_output()
        .map_err(|error| format!("tar bridge wait failed: {error}"))?;
    if !tar_output.status.success() {
        return Err(format!(
            "tar bridge failed: {}",
            String::from_utf8_lossy(&tar_output.stderr)
        ));
    }
    if !wsl_output.status.success() {
        return Err(format!(
            "WSL bridge failed: {}",
            String::from_utf8_lossy(&wsl_output.stderr)
        ));
    }
    Ok(())
}

fn verify_wsl_payload(
    distribution: &str,
    user: &str,
    destination: &str,
    expected: &BTreeMap<String, String>,
) -> Result<(), String> {
    let commands = expected
        .keys()
        .map(|name| format!("sha256sum {}", shell_quote(name)))
        .collect::<Vec<_>>()
        .join("; ");
    let script = format!("set -eu; cd {}; {}", shell_quote(destination), commands);
    let output = Command::new("wsl.exe")
        .args([
            "-d",
            distribution,
            "--user",
            user,
            "--",
            "bash",
            "-lc",
            &script,
        ])
        .output()
        .map_err(|error| format!("cannot verify WSL payload: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "WSL payload verification failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let mut actual = BTreeMap::new();
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let mut fields = line.split_whitespace();
        let Some(hash) = fields.next() else {
            continue;
        };
        let Some(name) = fields.next() else {
            continue;
        };
        actual.insert(name.trim_start_matches("*").to_string(), hash.to_string());
    }
    if &actual != expected {
        return Err(format!(
            "WSL payload hash mismatch: expected {:?}, actual {:?}",
            expected, actual
        ));
    }
    Ok(())
}

fn linux_path(path: &std::path::Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn selected_skill_names(debate: &Debate) -> Vec<String> {
    let mut names = vec!["protocol.v1".to_string(), "output-position.v1".to_string()];
    match &debate.intake.decision_type {
        council_core::DecisionType::Architecture => names.push("architecture.v1".to_string()),
        council_core::DecisionType::Stack => names.push("stack-selection.v1".to_string()),
        council_core::DecisionType::Design => names.push("design-taste.v1".to_string()),
        _ => {}
    }
    names
}

fn round_packet_body(
    debate: &Debate,
    round: u8,
    prior_positions: &[ProviderPosition],
    provider: &ProviderKind,
) -> Result<String, String> {
    let mut skills = vec![
        serde_json::json!({
            "name": "protocol.v1",
            "version": "1.0.0",
            "instructions": include_str!("../../../skills/protocol.v1/SKILL.md"),
        }),
        serde_json::json!({
            "name": "output-position.v1",
            "version": "1.0.0",
            "instructions": include_str!("../../../skills/output-position.v1/SKILL.md"),
        }),
    ];
    match &debate.intake.decision_type {
        council_core::DecisionType::Architecture => skills.push(serde_json::json!({
            "name": "architecture.v1",
            "version": "1.0.0",
            "instructions": include_str!("../../../skills/architecture.v1/SKILL.md"),
        })),
        council_core::DecisionType::Stack => skills.push(serde_json::json!({
            "name": "stack-selection.v1",
            "version": "1.0.0",
            "instructions": include_str!("../../../skills/stack-selection.v1/SKILL.md"),
        })),
        council_core::DecisionType::Design => skills.push(serde_json::json!({
            "name": "design-taste.v1",
            "version": "1.0.0",
            "instructions": include_str!("../../../skills/design-taste.v1/SKILL.md"),
        })),
        _ => {}
    }
    let visible_positions = prior_positions
        .iter()
        .enumerate()
        .filter(|(_, position)| round != 2 || &position.provider != provider)
        .map(|(index, position)| {
            serde_json::json!({
                "label": if &position.provider == provider {
                    "YOUR PRIOR POSITION".to_string()
                } else {
                    format!("PEER POSITION {}", index + 1)
                },
                "position": position.position.clone(),
            })
        })
        .collect::<Vec<_>>();
    serde_json::to_string_pretty(&serde_json::json!({
        "packet_version": "council.packet.v1",
        "debate_id": debate.id,
        "round": round,
        "question": debate.intake.question,
        "mode": debate.intake.mode,
        "options": debate.intake.options,
        "product_type": debate.intake.product_type,
        "decision_type": debate.intake.decision_type,
        "skills": skills,
        "hard_constraints": debate.intake.hard_constraints,
        "priority": debate.intake.priority,
        "prior_positions": visible_positions,
        "rules": [
            "Reason only. Do not modify files, create branches, commit, push, deploy, or hand off implementation.",
            "Return exactly one JSON object matching output-position.v1.",
            "Use controller-owned claim IDs only after validation; do not invent IDs.",
            "Preserve meaningful dissent and state the flip condition, cost if wrong, and reversibility."
        ]
    }))
    .map_err(|error| format!("cannot serialize round packet: {error}"))
}

fn provider_prompt(
    provider: &ProviderKind,
    packet_path: &str,
    schema_path: &str,
    round: u8,
) -> String {
    let path = if matches!(provider, ProviderKind::CodexWsl) {
        packet_path
    } else {
        packet_path
    };
    format!(
        "Council round {round}. Read the immutable packet at {path}. Return exactly one JSON object matching output-position.v1. The schema file is {schema_path}. Reason only. Do not write files or implement anything. Do not add markdown or commentary."
    )
}

#[tauri::command]
fn provider_statuses() -> Vec<ProviderStatus> {
    let registry = ProviderRegistry::defaults();
    registry
        .all()
        .map(|config| provider_status(config, &registry))
        .collect()
}

#[tauri::command]
fn create_debate(
    app: tauri::AppHandle,
    intake: Intake,
    model_overrides: Option<BTreeMap<String, String>>,
) -> Result<DebateSummary, String> {
    validate_intake(&intake).map_err(|errors| errors.join("; "))?;
    let model_overrides = model_overrides.unwrap_or_default();
    let provider_models = ProviderConfig::defaults()
        .into_iter()
        .filter(|config| config.enabled)
        .map(|config| {
            let key = config.provider.slug().to_string();
            let requested_model = model_overrides
                .get(&key)
                .filter(|model| !model.trim().is_empty())
                .cloned()
                .unwrap_or_else(|| config.model_default.clone());
            (config.provider, ModelSelection::requested(requested_model))
        })
        .collect::<BTreeMap<_, _>>();
    let debate = Debate::new(intake, provider_models);
    let database = database_for(&app)?;
    for config in ProviderConfig::defaults() {
        database
            .save_provider_config(&config)
            .map_err(|error| error.to_string())?;
    }
    database
        .create_debate(&debate)
        .map_err(|error| error.to_string())?;
    Ok(DebateSummary {
        id: debate.id,
        state: debate.state,
        question: debate.intake.question,
        created_at: debate.created_at.to_rfc3339(),
    })
}

#[tauri::command]
fn recent_debates(app: tauri::AppHandle) -> Result<Vec<DebateSummary>, String> {
    database_for(&app)?
        .list_debates(20)
        .map(|debates| {
            debates
                .into_iter()
                .map(|debate| DebateSummary {
                    id: debate.id,
                    state: debate.state,
                    question: debate.intake.question,
                    created_at: debate.created_at.to_rfc3339(),
                })
                .collect()
        })
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn debate_positions(
    app: tauri::AppHandle,
    debate_id: String,
) -> Result<Vec<ProviderPosition>, String> {
    database_for(&app)?
        .latest_provider_positions(&debate_id)
        .map_err(|error| error.to_string())
}

fn parse_event(event: &str) -> Result<DebateEvent, String> {
    match event {
        "PREFLIGHT_PASSED" => Ok(DebateEvent::PreflightPassed),
        "SNAPSHOT_STARTED" => Ok(DebateEvent::SnapshotStarted),
        "SNAPSHOT_READY" => Ok(DebateEvent::SnapshotReady),
        "OPENING_STARTED" => Ok(DebateEvent::OpeningStarted),
        "OPENING_COMPLETE" => Ok(DebateEvent::OpeningComplete),
        "CROSS_EXAMINATION_STARTED" => Ok(DebateEvent::CrossExaminationStarted),
        "CROSS_EXAMINATION_COMPLETE" => Ok(DebateEvent::CrossExaminationComplete),
        "FINAL_POSITIONS_STARTED" => Ok(DebateEvent::FinalPositionsStarted),
        "FINAL_POSITIONS_COMPLETE" => Ok(DebateEvent::FinalPositionsComplete),
        "TARGETED_ROUND_REQUESTED" => Ok(DebateEvent::TargetedRoundRequested),
        "HUMAN_DECISION_RECORDED" => Ok(DebateEvent::HumanDecisionRecorded),
        "COMPILE" => Ok(DebateEvent::Compile),
        "EXPORT" => Ok(DebateEvent::Export),
        "PAUSE" => Ok(DebateEvent::Pause),
        "RESUME" => Ok(DebateEvent::Resume),
        "CANCEL" => Ok(DebateEvent::Cancel),
        "FAIL" => Ok(DebateEvent::Fail),
        "SAFETY_ABORT" => Ok(DebateEvent::SafetyAbort),
        _ => Err(format!("unsupported debate event: {event}")),
    }
}

#[tauri::command]
fn transition_debate(
    app: tauri::AppHandle,
    debate_id: String,
    event: String,
) -> Result<DebateState, String> {
    let event = parse_event(&event)?;
    database_for(&app)?
        .transition_debate(&debate_id, event)
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn run_round(
    app: tauri::AppHandle,
    debate_id: String,
    round: u8,
) -> Result<RunRoundSummary, String> {
    if !(1..=5).contains(&round) {
        return Err("round must be 1 through 5".to_string());
    }
    let database = database_for(&app)?;
    let debate = database
        .load_debate(&debate_id)
        .map_err(|error| error.to_string())?;
    if debate.intake.repository.is_some() {
        return Err(
            "repository-grounded runs require the snapshot bridge; no real repository is read by this command"
                .to_string(),
        );
    }
    if round == 4
        && database
            .audit_action_count(&debate_id, "TARGETED_ROUND_REQUESTED")
            .map_err(|error| error.to_string())?
            > 0
    {
        return Err("the V1 contract permits one targeted round".to_string());
    }

    let registry = ProviderRegistry::defaults();
    let mut configs = ProviderConfig::defaults()
        .into_iter()
        .filter(|config| config.enabled)
        .collect::<Vec<_>>();
    for config in &mut configs {
        if let Some(selection) = debate.provider_models.get(&config.provider) {
            config.model_default = selection.requested_model.clone();
        }
    }
    ensure_subscription_environment().map_err(|error| error.to_string())?;
    for config in &configs {
        match registry.preflight(&config.provider) {
            Ok(()) => database
                .record_preflight(
                    Some(&debate_id),
                    config,
                    "PASS",
                    "Static safety preflight passed",
                )
                .map_err(|error| error.to_string())?,
            Err(error) => {
                database
                    .record_preflight(Some(&debate_id), config, "FAIL", &error.to_string())
                    .map_err(|db_error| db_error.to_string())?;
                database
                    .record_safety_event(Some(&debate_id), "PREFLIGHT_FAILED", &error.to_string())
                    .map_err(|db_error| db_error.to_string())?;
                return Err(error.to_string());
            }
        }
    }

    match round {
        1 => {
            if debate.state != DebateState::Draft {
                return Err(format!(
                    "opening round requires DRAFT, found {:?}",
                    debate.state
                ));
            }
            database
                .transition_debate(&debate_id, DebateEvent::PreflightPassed)
                .map_err(|error| error.to_string())?;
            database
                .transition_debate(&debate_id, DebateEvent::SnapshotStarted)
                .map_err(|error| error.to_string())?;
            database
                .transition_debate(&debate_id, DebateEvent::SnapshotReady)
                .map_err(|error| error.to_string())?;
            database
                .transition_debate(&debate_id, DebateEvent::OpeningStarted)
                .map_err(|error| error.to_string())?;
        }
        2 => {
            if debate.state != DebateState::CrossExamination {
                return Err(format!(
                    "cross-examination requires CROSS_EXAMINATION, found {:?}",
                    debate.state
                ));
            }
            database
                .transition_debate(&debate_id, DebateEvent::CrossExaminationStarted)
                .map_err(|error| error.to_string())?;
        }
        3 | 5 => {
            if debate.state != DebateState::FinalPositions {
                return Err(format!(
                    "final-position round requires FINAL_POSITIONS, found {:?}",
                    debate.state
                ));
            }
            database
                .transition_debate(&debate_id, DebateEvent::FinalPositionsStarted)
                .map_err(|error| error.to_string())?;
        }
        4 => {
            if debate.state != DebateState::AwaitingHumanDecision {
                return Err(format!(
                    "targeted round requires AWAITING_HUMAN_DECISION, found {:?}",
                    debate.state
                ));
            }
            database
                .transition_debate(&debate_id, DebateEvent::TargetedRoundRequested)
                .map_err(|error| error.to_string())?;
            database
                .append_audit_event(
                    Some(&debate_id),
                    "TARGETED_ROUND_REQUESTED",
                    serde_json::json!({"round": round}),
                )
                .map_err(|error| error.to_string())?;
            database
                .transition_debate(&debate_id, DebateEvent::CrossExaminationStarted)
                .map_err(|error| error.to_string())?;
        }
        _ => unreachable!(),
    }

    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| format!("cannot resolve app data directory: {error}"))?;
    let schema_text = include_str!("../../../schemas/position.schema.json");
    let schema_root = data_dir.join("schemas");
    let schema_path = schema_root.join("output-position.v1.json");
    write_immutable(&schema_path, schema_text.as_bytes())?;
    let prior_positions = if round == 1 {
        Vec::new()
    } else {
        database
            .latest_provider_positions(&debate_id)
            .map_err(|error| error.to_string())?
    };
    let packet_root = data_dir
        .join("runtime")
        .join("packets")
        .join(&debate_id)
        .join(format!("round-{round}"));
    let scratch_root = data_dir
        .join("runtime")
        .join("scratch")
        .join(&debate_id)
        .join(format!("round-{round}"));
    fs::create_dir_all(&packet_root)
        .map_err(|error| format!("cannot create packet root: {error}"))?;
    fs::create_dir_all(&scratch_root)
        .map_err(|error| format!("cannot create scratch root: {error}"))?;
    let mut requests = Vec::new();
    let mut packet_hashes = BTreeMap::new();
    let mut provider_configs = BTreeMap::new();

    for config in &configs {
        let turn_id = new_id("turn");
        let provider_directory = packet_root.join(config.provider.slug());
        fs::create_dir_all(&provider_directory)
            .map_err(|error| format!("cannot create provider packet directory: {error}"))?;
        let packet = ContextPacket::new(
            &debate_id,
            &turn_id,
            config.provider.clone(),
            POSITION_SCHEMA_VERSION,
            round_packet_body(&debate, round, &prior_positions, &config.provider)?,
        )
        .with_skills(selected_skill_names(&debate));
        let written = packet
            .write_sealed(&provider_directory)
            .map_err(|error| error.to_string())?;
        let schema_for_provider = provider_directory.join("output-position.v1.json");
        write_immutable(&schema_for_provider, schema_text.as_bytes())?;
        database
            .create_turn(
                &turn_id,
                &debate_id,
                round,
                config,
                TurnState::Pending,
                Some(&written.sha256),
            )
            .map_err(|error| error.to_string())?;
        database
            .save_packet(&written)
            .map_err(|error| error.to_string())?;
        packet_hashes.insert(config.provider.slug().to_string(), written.sha256.clone());

        let mut linux_packet_path = None;
        let mut linux_schema_path = None;
        let mut linux_working_directory = None;
        if let (Some(distribution), Some(user), Some(home)) = (
            config.wsl_distribution.as_deref(),
            config.wsl_user.as_deref(),
            config.wsl_home.as_deref(),
        ) {
            let packet_file_name = written
                .path
                .file_name()
                .and_then(|value| value.to_str())
                .ok_or_else(|| "packet filename is not valid UTF-8".to_string())?;
            safe_component(&debate_id)?;
            safe_component(&packet.metadata.packet_id)?;
            let linux_destination = format!(
                "{home}/council/packet/{debate_id}/{}",
                packet.metadata.packet_id
            );
            bridge_payload_to_wsl(&provider_directory, distribution, user, &linux_destination)?;
            let mut expected = BTreeMap::new();
            expected.insert(packet_file_name.to_string(), written.sha256.clone());
            expected.insert(
                "output-position.v1.json".to_string(),
                content_hash(schema_text),
            );
            verify_wsl_payload(distribution, user, &linux_destination, &expected)?;
            let linux_scratch = format!(
                "{home}/council/scratch/{debate_id}/round-{round}/{}",
                packet.metadata.packet_id
            );
            run_wsl_mkdir(distribution, user, &linux_scratch)?;
            linux_packet_path = Some(PathBuf::from(format!(
                "{linux_destination}/{packet_file_name}"
            )));
            linux_schema_path = Some(PathBuf::from(format!(
                "{linux_destination}/output-position.v1.json"
            )));
            linux_working_directory = Some(PathBuf::from(linux_scratch));
        }
        let windows_packet_path = linux_path(&written.path);
        let windows_schema_path = linux_path(&schema_for_provider);
        let packet_reference = linux_packet_path
            .as_deref()
            .map(linux_path)
            .unwrap_or(windows_packet_path);
        let schema_reference = linux_schema_path
            .as_deref()
            .map(linux_path)
            .unwrap_or(windows_schema_path);
        let request = ProviderCallRequest {
            provider: config.provider.clone(),
            model: config.model_default.clone(),
            turn_id: Some(turn_id),
            packet_path: written.path,
            packet_directory: provider_directory,
            schema_path: schema_for_provider,
            working_directory: scratch_root.clone(),
            scratch_directory: scratch_root.clone(),
            prompt: provider_prompt(
                &config.provider,
                &packet_reference,
                &schema_reference,
                round,
            ),
            timeout_ms: config.timeout_ms,
            linux_packet_path,
            linux_working_directory,
            linux_schema_path,
        };
        provider_configs.insert(config.provider.clone(), config.clone());
        requests.push(request);
    }

    database
        .append_audit_event(
            Some(&debate_id),
            "ROUND_DISPATCHED",
            serde_json::json!({"round": round, "providers": requests.len()}),
        )
        .map_err(|error| error.to_string())?;
    let result =
        CouncilOrchestrator::new(LiveProviderExecutor::new(registry)).run(&[RoundRequest {
            round,
            provider_requests: requests.clone(),
            repository_grounded: false,
        }]);

    for turn in &result.turns {
        database
            .update_turn_state(&turn.turn_id, turn.state.clone())
            .map_err(|error| error.to_string())?;
        for attempt in &turn.attempts {
            database
                .save_attempt(
                    &deterministic_call_id(
                        &debate_id,
                        round,
                        &turn.provider,
                        attempt.attempt_number,
                    ),
                    &turn.turn_id,
                    attempt.attempt_number,
                    attempt.state.clone(),
                    attempt.failure_type.as_ref(),
                    attempt.raw_result.as_ref(),
                )
                .map_err(|error| error.to_string())?;
            if let Some(raw_result) = &attempt.raw_result {
                database
                    .save_raw_artifact(
                        &turn.turn_id,
                        raw_result,
                        packet_hashes.get(turn.provider.slug()).map(String::as_str),
                        Some(&content_hash(schema_text)),
                    )
                    .map_err(|error| error.to_string())?;
            }
        }
        if let Some(position) = &turn.position {
            database
                .save_provider_position(position)
                .map_err(|error| error.to_string())?;
        }
    }

    let positions_complete = result.positions.len() == requests.len();
    let state = if positions_complete {
        match round {
            1 => database
                .transition_debate(&debate_id, DebateEvent::OpeningComplete)
                .map_err(|error| error.to_string())?,
            2 | 4 => database
                .transition_debate(&debate_id, DebateEvent::CrossExaminationComplete)
                .map_err(|error| error.to_string())?,
            3 | 5 => database
                .transition_debate(&debate_id, DebateEvent::FinalPositionsComplete)
                .map_err(|error| error.to_string())?,
            _ => unreachable!(),
        }
    } else {
        database
            .record_safety_event(
                Some(&debate_id),
                "ROUND_QUARANTINED",
                "At least one provider did not produce a usable structured position",
            )
            .map_err(|error| error.to_string())?;
        let provider_unavailable = result.turns.iter().any(|turn| {
            turn.attempts.iter().any(|attempt| {
                matches!(
                    &attempt.failure_type,
                    Some(FailureType::AuthRequired | FailureType::ProviderLimit)
                )
            })
        });
        database
            .transition_debate(
                &debate_id,
                if provider_unavailable {
                    DebateEvent::Pause
                } else {
                    DebateEvent::Fail
                },
            )
            .map_err(|error| error.to_string())?
    };
    let turns = result
        .turns
        .iter()
        .map(|turn| {
            let requested_model = provider_configs
                .get(&turn.provider)
                .map(|config| config.model_default.clone())
                .unwrap_or_default();
            let (reported_served_model, serving_identity_status) = turn
                .attempts
                .iter()
                .rev()
                .find_map(|attempt| attempt.raw_result.as_ref())
                .map(|result| {
                    (
                        result.reported_served_model.clone(),
                        result.serving_identity_status.clone(),
                    )
                })
                .unwrap_or((None, ServingIdentityStatus::Unknown));
            TurnSummary {
                provider: turn.provider.clone(),
                state: turn.state.clone(),
                attempts: turn.attempts.len(),
                failure_type: turn
                    .attempts
                    .iter()
                    .rev()
                    .find_map(|attempt| attempt.failure_type.clone()),
                requested_model,
                reported_served_model,
                serving_identity_status,
            }
        })
        .collect::<Vec<_>>();
    Ok(RunRoundSummary {
        debate_id,
        round,
        state: state.clone(),
        packet_hashes,
        turns,
        valid_positions: result.positions.len(),
        message: if positions_complete {
            format!("Round {round} complete. The next transition is human-visible.")
        } else if state == DebateState::Paused {
            "Round paused because a provider requires human availability or quota action."
                .to_string()
        } else {
            "Round quarantined. No downstream round may proceed from incomplete positions."
                .to_string()
        },
    })
}

fn parse_decision_kind(value: &str) -> Result<HumanDecisionKind, String> {
    match value {
        "APPROVE_OPTION" => Ok(HumanDecisionKind::ApproveOption),
        "APPROVE_MODIFIED_DECISION" => Ok(HumanDecisionKind::ApproveModifiedDecision),
        "CHALLENGE_CONSENSUS" => Ok(HumanDecisionKind::ChallengeConsensus),
        "REJECT_ALL" => Ok(HumanDecisionKind::RejectAll),
        "CONTINUE_TARGETED_DEBATE" => {
            Err("continue targeted debate is a round action, not a final decision".to_string())
        }
        _ => Err(format!("unsupported human decision kind: {value}")),
    }
}

#[tauri::command]
fn record_decision(app: tauri::AppHandle, input: DecisionInput) -> Result<DecisionRecord, String> {
    if input.rationale.trim().is_empty() {
        return Err("human rationale is required".to_string());
    }
    let database = database_for(&app)?;
    let debate = database
        .load_debate(&input.debate_id)
        .map_err(|error| error.to_string())?;
    if debate.state != DebateState::AwaitingHumanDecision {
        return Err(format!(
            "human decision requires AWAITING_HUMAN_DECISION, found {:?}",
            debate.state
        ));
    }
    let final_positions = database
        .latest_provider_positions(&input.debate_id)
        .map_err(|error| error.to_string())?;
    if final_positions.is_empty() {
        return Err("cannot record a decision without final positions".to_string());
    }
    let decision_kind = parse_decision_kind(&input.kind)?;
    let recommendations = final_positions
        .iter()
        .map(|position| position.position.recommendation.clone())
        .collect::<Vec<_>>();
    let disagreements = if recommendations
        .windows(2)
        .any(|window| window[0] != window[1])
    {
        vec![format!(
            "Provider recommendations: {}",
            recommendations.join(" | ")
        )]
    } else {
        Vec::new()
    };
    let minority_positions = final_positions
        .iter()
        .skip(1)
        .map(|position| {
            format!(
                "{}: {}",
                position.provider.display_name(),
                position.position.recommendation
            )
        })
        .collect::<Vec<_>>();
    let risks = final_positions
        .iter()
        .flat_map(|position| position.position.risks.clone())
        .collect::<Vec<_>>();
    let acceptance_criteria = final_positions
        .iter()
        .flat_map(|position| position.position.acceptance_criteria.clone())
        .collect::<Vec<_>>();
    let record = DecisionRecord {
        debate,
        final_positions,
        agreements: Vec::new(),
        disagreements: disagreements.clone(),
        most_decision_relevant_dispute: disagreements.first().cloned(),
        minority_positions,
        verified_evidence: Vec::new(),
        unverified_evidence: Vec::new(),
        risks,
        acceptance_criteria,
        human_decision: HumanDecision {
            kind: decision_kind,
            selected_option: input.selected_option,
            modified_decision: input.modified_decision,
            rationale: input.rationale.trim().to_string(),
            decided_at: chrono::Utc::now(),
        },
    };
    database
        .save_decision(&record)
        .map_err(|error| error.to_string())?;
    database
        .transition_debate(&input.debate_id, DebateEvent::HumanDecisionRecorded)
        .map_err(|error| error.to_string())?;
    Ok(record)
}

#[tauri::command]
fn record_human_decision(app: tauri::AppHandle, record: DecisionRecord) -> Result<(), String> {
    let database = database_for(&app)?;
    let debate = database
        .load_debate(&record.debate.id)
        .map_err(|error| error.to_string())?;
    if debate.state != DebateState::AwaitingHumanDecision {
        return Err(format!(
            "human decision requires AWAITING_HUMAN_DECISION, found {:?}",
            debate.state
        ));
    }
    database
        .save_decision(&record)
        .map_err(|error| error.to_string())?;
    database
        .transition_debate(&record.debate.id, DebateEvent::HumanDecisionRecorded)
        .map_err(|error| error.to_string())?;
    Ok(())
}

#[tauri::command]
fn compile_export(app: tauri::AppHandle, record: DecisionRecord) -> Result<ExportSummary, String> {
    let database = database_for(&app)?;
    let debate = database
        .load_debate(&record.debate.id)
        .map_err(|error| error.to_string())?;
    if debate.state == DebateState::Decided {
        database
            .transition_debate(&record.debate.id, DebateEvent::Compile)
            .map_err(|error| error.to_string())?;
    } else if debate.state != DebateState::Compiled && debate.state != DebateState::Exported {
        return Err(format!(
            "export requires DECIDED, COMPILED, or EXPORTED, found {:?}",
            debate.state
        ));
    }
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| format!("cannot resolve app data directory: {error}"))?;
    let export_dir = data_dir.join("exports").join(&record.debate.id);
    std::fs::create_dir_all(&export_dir)
        .map_err(|error| format!("cannot create export directory: {error}"))?;
    let master_prompt = compile_master_prompt(&record);
    let decision_record = compile_decision_record(&record);
    let master_path = export_dir.join("master-prompt.md");
    let decision_path = export_dir.join("decision-record.md");
    std::fs::write(&master_path, master_prompt.as_bytes())
        .map_err(|error| format!("cannot write master prompt: {error}"))?;
    std::fs::write(&decision_path, decision_record.as_bytes())
        .map_err(|error| format!("cannot write decision record: {error}"))?;
    let master_hash = content_hash(&master_prompt);
    let decision_hash = content_hash(&decision_record);
    database
        .save_export(
            &format!("export-master-{}", record.debate.id),
            &record.debate.id,
            "MASTER_PROMPT",
            &master_path,
            &master_hash,
        )
        .map_err(|error| error.to_string())?;
    database
        .save_export(
            &format!("export-decision-{}", record.debate.id),
            &record.debate.id,
            "DECISION_RECORD",
            &decision_path,
            &decision_hash,
        )
        .map_err(|error| error.to_string())?;
    let latest_state = database
        .load_debate(&record.debate.id)
        .map_err(|error| error.to_string())?
        .state;
    if latest_state == DebateState::Compiled {
        database
            .transition_debate(&record.debate.id, DebateEvent::Export)
            .map_err(|error| error.to_string())?;
    }
    Ok(ExportSummary {
        debate_id: record.debate.id,
        directory: export_dir.to_string_lossy().to_string(),
        master_prompt_hash: master_hash,
        decision_record_hash: decision_hash,
    })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            provider_statuses,
            create_debate,
            recent_debates,
            debate_positions,
            transition_debate,
            run_round,
            record_decision,
            record_human_decision,
            compile_export
        ])
        .run(tauri::generate_context!())
        .expect("error while running Council of Agents");
}
