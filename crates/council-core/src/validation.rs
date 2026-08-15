use regex::Regex;
use serde_json::Value;

use crate::model::{Commitment, Intake, Position};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PositionValidation {
    pub schema_valid: bool,
    pub semantic_valid: bool,
    pub errors: Vec<String>,
    pub position: Option<Position>,
}

pub fn validate_intake(intake: &Intake) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    if intake.question.trim().len() < 12 {
        errors.push("QUESTION must be at least 12 characters".to_string());
    }
    if intake.priority.trim().is_empty() {
        errors.push("WHAT MATTERS MOST? is required".to_string());
    }
    if intake.hard_constraints.is_empty() {
        errors.push("HARD CONSTRAINTS must contain NONE or one or more constraints".to_string());
    }
    if matches!(intake.mode, crate::model::DebateMode::Compare)
        && intake
            .options
            .iter()
            .filter(|value| !value.trim().is_empty())
            .count()
            < 2
    {
        errors.push("COMPARE mode requires at least two options".to_string());
    }
    if let Some(leaning) = &intake.current_leaning {
        if leaning.trim().is_empty() {
            errors.push("current_leaning cannot be blank when supplied".to_string());
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub fn validate_position_value(value: &Value, repository_grounded: bool) -> PositionValidation {
    let mut errors = Vec::new();
    if value.get("schema_version").and_then(Value::as_str) != Some(crate::POSITION_SCHEMA_VERSION) {
        errors.push(format!(
            "schema_version must be {}",
            crate::POSITION_SCHEMA_VERSION
        ));
    }
    let position = match serde_json::from_value::<Position>(value.clone()) {
        Ok(position) => Some(position),
        Err(error) => {
            errors.push(format!("schema parse failed: {error}"));
            None
        }
    };

    if let Some(position_ref) = &position {
        validate_semantics(position_ref, repository_grounded, &mut errors);
    }

    PositionValidation {
        schema_valid: position.is_some(),
        semantic_valid: position.is_some() && errors.is_empty(),
        errors,
        position,
    }
}

fn validate_semantics(position: &Position, repository_grounded: bool, errors: &mut Vec<String>) {
    if position.claims.len() > 7 {
        errors.push("claims may contain at most seven load-bearing claims".to_string());
    }
    if position.recommendation.trim().is_empty() {
        errors.push("recommendation must be non-empty".to_string());
    }
    if position.claims.is_empty() {
        errors.push("claims must contain at least one claim".to_string());
    }
    if position.risks.is_empty() || position.risks.iter().any(|risk| risk.trim().is_empty()) {
        errors.push("risks must contain meaningful non-empty values".to_string());
    }
    for (index, claim) in position.claims.iter().enumerate() {
        if claim.id.trim().is_empty() {
            errors.push(format!("claims[{index}].id must be non-empty"));
        }
        if claim.text.trim().is_empty() {
            errors.push(format!("claims[{index}].text must be non-empty"));
        }
        if repository_grounded && claim.evidence.is_empty() {
            errors.push(format!(
                "claims[{index}] requires evidence for repository-grounded debates"
            ));
        }
        for citation in &claim.evidence {
            if !is_citation_shape(citation) {
                errors.push(format!(
                    "claims[{index}].evidence has invalid path:startLine-endLine value"
                ));
            }
        }
    }
    for (index, response) in position.peer_responses.iter().enumerate() {
        if response.peer_claim_reference.trim().is_empty() {
            errors.push(format!(
                "peer_responses[{index}].peer_claim_reference must be non-empty"
            ));
        }
        if response.reason.trim().is_empty() {
            errors.push(format!("peer_responses[{index}].reason must be non-empty"));
        }
        for citation in &response.evidence {
            if !is_citation_shape(citation) {
                errors.push(format!(
                    "peer_responses[{index}].evidence has invalid path:startLine-endLine value"
                ));
            }
        }
    }
    if position.flip_condition.trim().is_empty() {
        errors.push("flip_condition must be non-empty".to_string());
    }
    if position.cost_if_wrong.trim().is_empty() {
        errors.push("cost_if_wrong must be non-empty".to_string());
    }
    if position.reversibility == crate::model::Reversibility::OneWayDoor
        && position.cost_if_wrong.trim().len() < 12
    {
        errors.push("ONE_WAY_DOOR positions require a specific cost_if_wrong".to_string());
    }
    let placeholder =
        Regex::new(r"(?i)^(n/?a|unknown|tbd|none)$").expect("valid placeholder regex");
    for (name, value) in [
        ("recommendation", position.recommendation.as_str()),
        ("flip_condition", position.flip_condition.as_str()),
        ("cost_if_wrong", position.cost_if_wrong.as_str()),
    ] {
        if placeholder.is_match(value.trim()) {
            errors.push(format!("{name} cannot be a placeholder"));
        }
    }
    if position.commitment == Commitment::Conditional && position.flip_condition.trim().len() < 12 {
        errors.push("CONDITIONAL positions require a material flip condition".to_string());
    }
    if position
        .revision_reason
        .as_deref()
        .is_some_and(|value| value.trim().is_empty())
    {
        errors.push("revision_reason cannot be blank when supplied".to_string());
    }
    if position
        .prior_position_hash
        .as_deref()
        .is_some_and(|value| {
            value.len() != 64 || !value.chars().all(|character| character.is_ascii_hexdigit())
        })
    {
        errors.push("prior_position_hash must be a SHA-256 hex value when supplied".to_string());
    }
}

pub fn is_citation_shape(value: &str) -> bool {
    let Some((path, range)) = value.rsplit_once(':') else {
        return false;
    };
    if path.trim().is_empty() {
        return false;
    }
    let Some((start, end)) = range.split_once('-') else {
        return false;
    };
    start.parse::<u64>().is_ok() && end.parse::<u64>().is_ok() && start != "0" && end != "0"
}

pub fn assign_controller_claim_ids(position: &mut Position, provider: &str, round: u8) {
    for (index, claim) in position.claims.iter_mut().enumerate() {
        claim.id = format!("C-{}-R{}-{:03}", provider.to_uppercase(), round, index + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Claim, Commitment, DebateMode, DecisionType, ProductType, Reversibility};

    fn valid_position() -> Position {
        Position {
            schema_version: crate::POSITION_SCHEMA_VERSION.to_string(),
            recommendation: "Choose the simpler option for the first release".to_string(),
            commitment: Commitment::WouldStake,
            claims: vec![Claim {
                id: "model-id".to_string(),
                text: "The simpler option reduces initial operational risk".to_string(),
                evidence: vec!["src/main.rs:1-4".to_string()],
            }],
            risks: vec!["The simpler option may need migration later".to_string()],
            assumptions: Vec::new(),
            alternatives: Vec::new(),
            flip_condition: "Evidence of a hard performance requirement would change this"
                .to_string(),
            cost_if_wrong: "A later migration would consume engineering time".to_string(),
            when_wrongness_becomes_visible: None,
            reversibility: Reversibility::Easy,
            strongest_argument_against_my_recommendation: String::new(),
            what_my_recommendation_is_bad_at: String::new(),
            acceptance_criteria: Vec::new(),
            implementation_constraints: Vec::new(),
            peer_responses: Vec::new(),
            withdrawn_claims: Vec::new(),
            conceded_claims: Vec::new(),
            remaining_disputes: Vec::new(),
            revision_reason: None,
            prior_position_hash: None,
        }
    }

    #[test]
    fn validates_compare_intake() {
        let intake = Intake {
            question: "Which database should this desktop product use?".to_string(),
            mode: DebateMode::Compare,
            options: vec!["SQLite".to_string(), "PostgreSQL".to_string()],
            product_type: ProductType::Desktop,
            decision_type: DecisionType::Database,
            hard_constraints: vec!["Local-first".to_string()],
            priority: "Simplest to maintain".to_string(),
            current_leaning: None,
            current_leaning_reason: None,
            repository: None,
        };
        assert!(validate_intake(&intake).is_ok());
    }

    #[test]
    fn rejects_missing_compare_option() {
        let mut intake = Intake::default();
        intake.question = "Which database should this product use?".to_string();
        intake.mode = DebateMode::Compare;
        intake.options = vec!["SQLite".to_string()];
        assert!(validate_intake(&intake).is_err());
    }

    #[test]
    fn validates_position_and_assigns_controller_ids() {
        let mut position = valid_position();
        let value = serde_json::to_value(&position).unwrap();
        let report = validate_position_value(&value, true);
        assert!(report.schema_valid);
        assert!(report.semantic_valid);
        assign_controller_claim_ids(&mut position, "codex", 1);
        assert_eq!(position.claims[0].id, "C-CODEX-R1-001");
    }
}
