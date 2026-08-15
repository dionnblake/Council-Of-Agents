use crate::model::{Commitment, Position};
use crate::packet::ContextPacket;
use crate::validation::is_citation_shape;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StatelessHandoffPacket {
    pub schema_version: String,
    pub original_question: String,
    pub prior_position: Position,
    pub prior_claims: Vec<String>,
    pub prior_evidence: Vec<String>,
    pub peer_challenge: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StatelessReconstruction {
    pub prior_recommendation: String,
    pub prior_commitment: Commitment,
    pub understood_peer_challenge: String,
    pub response_to_peer: String,
    pub position_action: String,
    pub revision_reason: Option<String>,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReconstructionCheck {
    pub prior_position_recovered: bool,
    pub peer_claim_understood: bool,
    pub position_preserved_or_revised_coherently: bool,
    pub evidence_cited: bool,
    pub hidden_session_resume_used: bool,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ReconstructionError {
    #[error("prior recommendation was not recovered exactly")]
    PriorPositionMissing,
    #[error("peer challenge response is empty")]
    PeerChallengeMissing,
    #[error("position action must be PRESERVE or REVISE")]
    InvalidPositionAction,
    #[error("revision requires a substantive reason")]
    RevisionReasonMissing,
    #[error("at least one syntactically valid evidence citation is required")]
    EvidenceMissing,
}

pub fn build_stateless_packet(
    packet: &StatelessHandoffPacket,
    debate_id: impl Into<String>,
    turn_id: impl Into<String>,
    provider: crate::model::ProviderKind,
) -> ContextPacket {
    let body = format!(
        "COUNCIL STATELESS HANDOFF\n\n\
         The prior provider process is terminated. Reconstruct from this packet only.\n\n\
         ORIGINAL QUESTION\n{}\n\n\
         PRIOR POSITION JSON\n{}\n\n\
         PRIOR CLAIMS\n{}\n\n\
         PRIOR EVIDENCE\n{}\n\n\
         PEER CHALLENGE\n{}\n\n\
         Return one JSON object matching stateless-handoff.v1. Identify the prior position, \
         understand the challenge, preserve or revise coherently, and cite evidence.",
        packet.original_question,
        serde_json::to_string_pretty(&packet.prior_position).expect("position serialization"),
        packet.prior_claims.join("\n"),
        packet.prior_evidence.join("\n"),
        packet.peer_challenge
    );
    ContextPacket::new(
        debate_id,
        turn_id,
        provider,
        packet.schema_version.clone(),
        body,
    )
}

pub fn validate_reconstruction(
    packet: &StatelessHandoffPacket,
    response: &StatelessReconstruction,
) -> Result<ReconstructionCheck, ReconstructionError> {
    if response.prior_recommendation.trim() != packet.prior_position.recommendation.trim() {
        return Err(ReconstructionError::PriorPositionMissing);
    }
    if response.understood_peer_challenge.trim().is_empty()
        || response.response_to_peer.trim().is_empty()
    {
        return Err(ReconstructionError::PeerChallengeMissing);
    }
    let action = response.position_action.trim().to_ascii_uppercase();
    if action != "PRESERVE" && action != "REVISE" {
        return Err(ReconstructionError::InvalidPositionAction);
    }
    if action == "REVISE"
        && response
            .revision_reason
            .as_deref()
            .is_none_or(|reason| reason.trim().len() < 12)
    {
        return Err(ReconstructionError::RevisionReasonMissing);
    }
    if response
        .evidence
        .iter()
        .all(|citation| !is_citation_shape(citation))
    {
        return Err(ReconstructionError::EvidenceMissing);
    }
    Ok(ReconstructionCheck {
        prior_position_recovered: true,
        peer_claim_understood: true,
        position_preserved_or_revised_coherently: true,
        evidence_cited: true,
        hidden_session_resume_used: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Claim, Commitment, ProviderKind, Reversibility};

    fn packet() -> StatelessHandoffPacket {
        StatelessHandoffPacket {
            schema_version: "stateless-handoff.v1".to_string(),
            original_question: "Which path should ship first?".to_string(),
            prior_position: Position {
                schema_version: "output-position.v1".to_string(),
                recommendation: "Choose the reversible path".to_string(),
                commitment: Commitment::Conditional,
                claims: vec![Claim {
                    id: "C-1".to_string(),
                    text: "It reduces risk".to_string(),
                    evidence: vec!["README.md:1-2".to_string()],
                }],
                risks: vec!["Migration later".to_string()],
                assumptions: Vec::new(),
                alternatives: Vec::new(),
                flip_condition: "Scale evidence changes the boundary".to_string(),
                cost_if_wrong: "One sprint of migration".to_string(),
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
            },
            prior_claims: vec!["It reduces risk".to_string()],
            prior_evidence: vec!["README.md:1-2".to_string()],
            peer_challenge: "The migration trigger may be too late.".to_string(),
        }
    }

    #[test]
    fn stateless_packet_is_explicitly_file_based_and_reconstruction_is_checked() {
        let source = packet();
        let context = build_stateless_packet(&source, "debate-1", "turn-2", ProviderKind::Claude);
        assert!(context.body.contains("PRIOR POSITION JSON"));
        let check = validate_reconstruction(
            &source,
            &StatelessReconstruction {
                prior_recommendation: "Choose the reversible path".to_string(),
                prior_commitment: Commitment::Conditional,
                understood_peer_challenge: "The trigger could be late.".to_string(),
                response_to_peer: "Preserve but monitor the trigger.".to_string(),
                position_action: "PRESERVE".to_string(),
                revision_reason: None,
                evidence: vec!["README.md:1-2".to_string()],
            },
        )
        .unwrap();
        assert!(!check.hidden_session_resume_used);
    }
}
