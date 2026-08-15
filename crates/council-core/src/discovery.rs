use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::model::{Intake, ProviderKind};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct DiscoveryNomination {
    pub label: String,
    pub justification: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct DiscoveryProposal {
    pub provider: ProviderKind,
    pub turn_id: String,
    pub raw_artifact_id: String,
    pub nominations: Vec<DiscoveryNomination>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiscoveryCandidate {
    pub id: String,
    pub label: String,
    pub source: String,
    pub status_quo: bool,
    #[serde(default)]
    pub justifications: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiscoveryResult {
    pub round: u8,
    pub candidates: Vec<DiscoveryCandidate>,
    pub bounded: bool,
    #[serde(default)]
    pub proposals: Vec<DiscoveryProposal>,
}

pub fn build_r0_candidate_union(intake: &Intake) -> DiscoveryResult {
    build_candidate_union(intake, &[])
}

pub fn merge_discovery_proposals(
    intake: &Intake,
    proposals: Vec<DiscoveryProposal>,
) -> DiscoveryResult {
    build_candidate_union(intake, &proposals)
}

pub fn extract_discovery_nominations(stdout: &str) -> Option<Vec<DiscoveryNomination>> {
    let values = std::iter::once(stdout.to_string()).chain(
        stdout
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(str::to_string),
    );
    for candidate in values {
        let Ok(value) = serde_json::from_str::<Value>(&candidate) else {
            continue;
        };
        if let Some(nominations) = find_nominations(&value) {
            return Some(nominations);
        }
    }
    None
}

fn find_nominations(value: &Value) -> Option<Vec<DiscoveryNomination>> {
    match value {
        Value::Object(map) => {
            if let Some(candidates) = map.get("candidates").and_then(Value::as_array) {
                let mut nominations = Vec::new();
                for candidate in candidates.iter().take(5) {
                    let Ok(nomination) =
                        serde_json::from_value::<DiscoveryNomination>(candidate.clone())
                    else {
                        return None;
                    };
                    if nomination.label.trim().is_empty()
                        || nomination.justification.trim().is_empty()
                    {
                        return None;
                    }
                    nominations.push(DiscoveryNomination {
                        label: nomination.label.trim().to_string(),
                        justification: nomination.justification.trim().to_string(),
                    });
                }
                if !nominations.is_empty() {
                    return Some(nominations);
                }
            }
            map.values().find_map(find_nominations)
        }
        Value::Array(values) => values.iter().find_map(find_nominations),
        _ => None,
    }
}

fn build_candidate_union(intake: &Intake, proposals: &[DiscoveryProposal]) -> DiscoveryResult {
    let mut candidates = Vec::new();
    fn add_candidate(
        candidates: &mut Vec<DiscoveryCandidate>,
        label: String,
        source: String,
        status_quo: bool,
        justification: String,
    ) {
        if label.trim().is_empty() {
            return;
        }
        if let Some(existing) = candidates
            .iter_mut()
            .find(|candidate| candidate.label.trim().eq_ignore_ascii_case(label.trim()))
        {
            if !justification.trim().is_empty()
                && !existing
                    .justifications
                    .iter()
                    .any(|value| value == justification.trim())
            {
                existing
                    .justifications
                    .push(justification.trim().to_string());
            }
            return;
        }
        if candidates.len() >= 6 {
            return;
        }
        let id = format!("R0-{:03}", candidates.len() + 1);
        candidates.push(DiscoveryCandidate {
            id,
            label: label.trim().to_string(),
            source,
            status_quo,
            justifications: if justification.trim().is_empty() {
                Vec::new()
            } else {
                vec![justification.trim().to_string()]
            },
        });
    }

    add_candidate(
        &mut candidates,
        "STATUS QUO / DEFER THE DECISION".to_string(),
        "controller-required baseline".to_string(),
        true,
        "Preserves the current state while evidence is gathered.".to_string(),
    );
    for option in intake.options.iter().take(5) {
        add_candidate(
            &mut candidates,
            option.clone(),
            "owner-supplied option".to_string(),
            false,
            String::new(),
        );
    }
    for proposal in proposals {
        for nomination in proposal.nominations.iter().take(5) {
            add_candidate(
                &mut candidates,
                nomination.label.clone(),
                format!("{} proposal", proposal.provider.display_name()),
                false,
                nomination.justification.clone(),
            );
        }
    }
    if candidates.len() < 6 {
        add_candidate(
            &mut candidates,
            "BORING ESTABLISHED ALTERNATIVE / PROVEN DEFAULT".to_string(),
            "controller-required baseline".to_string(),
            false,
            "Keeps operational novelty low and preserves a well-understood fallback.".to_string(),
        );
    }

    DiscoveryResult {
        round: 0,
        candidates,
        bounded: true,
        proposals: proposals.to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Intake;

    #[test]
    fn r0_always_preserves_status_quo_and_bounds_candidates() {
        let mut intake = Intake::default();
        intake.options = (0..3).map(|index| format!("Option {index}")).collect();
        let result = build_r0_candidate_union(&intake);
        assert_eq!(result.round, 0);
        assert!(result.bounded);
        assert!(
            result
                .candidates
                .iter()
                .any(|candidate| candidate.status_quo)
        );
        assert!(result.candidates.len() <= 6);
        assert!(
            result
                .candidates
                .iter()
                .any(|candidate| candidate.label.contains("BORING"))
        );
    }

    #[test]
    fn proposal_union_deduplicates_and_preserves_justifications() {
        let intake = Intake::default();
        let result = merge_discovery_proposals(
            &intake,
            vec![DiscoveryProposal {
                provider: ProviderKind::Claude,
                turn_id: "turn-r0".to_string(),
                raw_artifact_id: "raw-r0".to_string(),
                nominations: vec![
                    DiscoveryNomination {
                        label: "SQLite".to_string(),
                        justification: "Local-first and reversible for a small desktop tool."
                            .to_string(),
                    },
                    DiscoveryNomination {
                        label: "sqlite".to_string(),
                        justification: "The ecosystem is mature.".to_string(),
                    },
                ],
            }],
        );
        let sqlite = result
            .candidates
            .iter()
            .find(|candidate| candidate.label.eq_ignore_ascii_case("sqlite"))
            .unwrap();
        assert_eq!(sqlite.justifications.len(), 2);
        assert_eq!(result.proposals.len(), 1);
    }

    #[test]
    fn extracts_bounded_discovery_payload_from_jsonl() {
        let value = extract_discovery_nominations(
            "{\"type\":\"result\"}\n{\"candidates\":[{\"label\":\"SQLite\",\"justification\":\"Local-first.\"}]}"
        )
        .unwrap();
        assert_eq!(value[0].label, "SQLite");
        assert_eq!(value.len(), 1);
    }
}
