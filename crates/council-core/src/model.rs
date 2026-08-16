use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::PathBuf;
use uuid::Uuid;

use crate::POSITION_SCHEMA_VERSION;

pub fn new_id(prefix: &str) -> String {
    format!("{prefix}-{}", Uuid::new_v4())
}

pub fn deterministic_call_id(
    debate_id: &str,
    round: u8,
    provider: &ProviderKind,
    attempt: u8,
) -> String {
    let mut digest = Sha256::new();
    digest.update(debate_id.as_bytes());
    digest.update([0]);
    digest.update(round.to_string().as_bytes());
    digest.update([0]);
    digest.update(provider.slug().as_bytes());
    digest.update([0]);
    digest.update(attempt.to_string().as_bytes());
    format!("call-{}", hex::encode(digest.finalize()))
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProviderKind {
    Claude,
    Antigravity,
    CodexWsl,
}

impl ProviderKind {
    pub fn slug(&self) -> &'static str {
        match self {
            Self::Claude => "claude",
            Self::Antigravity => "antigravity",
            Self::CodexWsl => "codex-wsl",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Claude => "Claude Code",
            Self::Antigravity => "Antigravity CLI",
            Self::CodexWsl => "Codex WSL",
        }
    }

    pub fn from_slug(value: &str) -> Option<Self> {
        match value {
            "claude" => Some(Self::Claude),
            "antigravity" => Some(Self::Antigravity),
            "codex-wsl" => Some(Self::CodexWsl),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CertificationStatus {
    Pass,
    PassWithDeclaredLimitation,
    Blocked,
    Fail,
    Disabled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ServingIdentityStatus {
    VerifiedMatch,
    VerifiedMismatch,
    ProviderDoesNotReport,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Commitment {
    WouldStake,
    Conditional,
    WouldNotStake,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Reversibility {
    Easy,
    Costly,
    OneWayDoor,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DebateMode {
    Compare,
    Discovery,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProductType {
    Web,
    Android,
    Windows,
    Game,
    Backend,
    AiSystem,
    Desktop,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DecisionType {
    Architecture,
    Stack,
    Design,
    Security,
    Performance,
    Database,
    Dependency,
    Testing,
    General,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DebateState {
    Draft,
    Preflight,
    Snapshotting,
    SnapshotReviewRequired,
    Ready,
    Opening,
    CrossExamination,
    FinalPositions,
    AwaitingHumanDecision,
    Decided,
    Compiled,
    Exported,
    Paused,
    Cancelled,
    Failed,
    SafetyAbort,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TurnState {
    Pending,
    Dispatched,
    Running,
    RawCaptured,
    Validating,
    Valid,
    Repairing,
    Quarantined,
    Timeout,
    AuthRequired,
    ProviderLimit,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FailureType {
    AuthRequired,
    ProviderLimit,
    Timeout,
    ProcessError,
    SchemaInvalid,
    SemanticInvalid,
    NoStructuredOutput,
    Truncated,
    Refusal,
    PacketUnreadable,
    SafetyViolation,
    ModelMismatch,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModelSelection {
    pub requested_model: String,
    pub reported_served_model: Option<String>,
    pub serving_identity_status: ServingIdentityStatus,
}

impl ModelSelection {
    pub fn requested(model: impl Into<String>) -> Self {
        Self {
            requested_model: model.into(),
            reported_served_model: None,
            serving_identity_status: ServingIdentityStatus::Unknown,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Intake {
    pub question: String,
    pub mode: DebateMode,
    pub options: Vec<String>,
    pub product_type: ProductType,
    pub decision_type: DecisionType,
    pub hard_constraints: Vec<String>,
    pub priority: String,
    pub current_leaning: Option<String>,
    pub current_leaning_reason: Option<String>,
    pub repository: Option<PathBuf>,
}

impl Default for Intake {
    fn default() -> Self {
        Self {
            question: String::new(),
            mode: DebateMode::Discovery,
            options: Vec::new(),
            product_type: ProductType::Other,
            decision_type: DecisionType::General,
            hard_constraints: vec!["NONE".to_string()],
            priority: "Best overall".to_string(),
            current_leaning: None,
            current_leaning_reason: None,
            repository: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderConfig {
    pub provider: ProviderKind,
    pub executable: PathBuf,
    pub model_default: String,
    pub enabled: bool,
    pub timeout_ms: u64,
    pub config_dir: Option<PathBuf>,
    #[serde(default)]
    pub safety_config_path: Option<PathBuf>,
    pub wsl_distribution: Option<String>,
    pub wsl_user: Option<String>,
    pub wsl_home: Option<String>,
    pub codex_home: Option<String>,
    pub extra_args: Vec<String>,
    pub certification: CertificationStatus,
    pub certification_evidence: Option<String>,
    pub safety_settings: BTreeMap<String, serde_json::Value>,
}

impl ProviderConfig {
    pub fn defaults() -> Vec<Self> {
        let local_app_data = std::env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));

        vec![
            Self {
                provider: ProviderKind::Claude,
                executable: PathBuf::from("claude.exe"),
                model_default: "claude-haiku-4-5-20251001".to_string(),
                enabled: true,
                timeout_ms: 180_000,
                config_dir: Some(local_app_data.join("council").join("claude-cfg")),
                safety_config_path: None,
                wsl_distribution: None,
                wsl_user: None,
                wsl_home: None,
                codex_home: None,
                extra_args: Vec::new(),
                certification: CertificationStatus::Pass,
                certification_evidence: Some("M0.8 isolated certification".to_string()),
                safety_settings: BTreeMap::new(),
            },
            Self {
                provider: ProviderKind::Antigravity,
                executable: PathBuf::from("agy.exe"),
                model_default: "gemini-3.7-flash-low".to_string(),
                enabled: true,
                timeout_ms: 180_000,
                config_dir: None,
                safety_config_path: None,
                wsl_distribution: None,
                wsl_user: None,
                wsl_home: None,
                codex_home: None,
                extra_args: Vec::new(),
                certification: CertificationStatus::PassWithDeclaredLimitation,
                certification_evidence: Some(
                    "M0.6 17/20 certification; served model not reported".to_string(),
                ),
                safety_settings: BTreeMap::from([(
                    "useG1Credits".to_string(),
                    serde_json::Value::Bool(false),
                )]),
            },
            Self {
                provider: ProviderKind::CodexWsl,
                executable: PathBuf::from("wsl.exe"),
                model_default: "gpt-5.6-luna".to_string(),
                enabled: true,
                timeout_ms: 180_000,
                config_dir: None,
                safety_config_path: None,
                wsl_distribution: Some("CouncilCodexWSL".to_string()),
                wsl_user: Some("council".to_string()),
                wsl_home: Some("/home/council".to_string()),
                codex_home: Some("/home/council/.codex".to_string()),
                extra_args: Vec::new(),
                certification: CertificationStatus::PassWithDeclaredLimitation,
                certification_evidence: Some("CODEX-WSL-FINAL-CERTIFICATION.md".to_string()),
                safety_settings: BTreeMap::new(),
            },
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Debate {
    pub id: String,
    pub intake: Intake,
    pub state: DebateState,
    pub council_size: u8,
    #[serde(default)]
    pub independent_only: bool,
    #[serde(default)]
    pub discovery: Option<crate::discovery::DiscoveryResult>,
    pub provider_models: BTreeMap<ProviderKind, ModelSelection>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Debate {
    pub fn new(intake: Intake, provider_models: BTreeMap<ProviderKind, ModelSelection>) -> Self {
        let now = Utc::now();
        Self {
            id: new_id("debate"),
            intake,
            state: DebateState::Draft,
            council_size: provider_models.len() as u8,
            independent_only: false,
            discovery: None,
            provider_models,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_independent_only(mut self, independent_only: bool) -> Self {
        self.independent_only = independent_only;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Claim {
    pub id: String,
    pub text: String,
    #[serde(deserialize_with = "deserialize_evidence", default)]
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Position {
    #[serde(default = "default_schema_version")]
    pub schema_version: String,
    pub recommendation: String,
    pub commitment: Commitment,
    pub claims: Vec<Claim>,
    pub risks: Vec<String>,
    #[serde(default)]
    pub assumptions: Vec<String>,
    #[serde(default)]
    pub alternatives: Vec<String>,
    pub flip_condition: String,
    pub cost_if_wrong: String,
    #[serde(default)]
    pub when_wrongness_becomes_visible: Option<String>,
    pub reversibility: Reversibility,
    #[serde(default)]
    pub strongest_argument_against_my_recommendation: String,
    #[serde(default)]
    pub what_my_recommendation_is_bad_at: String,
    #[serde(default)]
    pub acceptance_criteria: Vec<String>,
    #[serde(default)]
    pub implementation_constraints: Vec<String>,
    #[serde(default)]
    pub peer_responses: Vec<PeerResponse>,
    #[serde(default)]
    pub withdrawn_claims: Vec<String>,
    #[serde(default)]
    pub conceded_claims: Vec<String>,
    #[serde(default)]
    pub remaining_disputes: Vec<String>,
    #[serde(default)]
    pub revision_reason: Option<String>,
    #[serde(default)]
    pub prior_position_hash: Option<String>,
}

fn default_schema_version() -> String {
    POSITION_SCHEMA_VERSION.to_string()
}

fn deserialize_evidence<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Evidence {
        One(String),
        Many(Vec<String>),
    }

    let value = Option::<Evidence>::deserialize(deserializer)?;
    Ok(match value {
        Some(Evidence::One(value)) => vec![value],
        Some(Evidence::Many(values)) => values,
        None => Vec::new(),
    })
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ClaimRelation {
    pub id: String,
    pub source_claim_id: String,
    pub target_claim_id: Option<String>,
    pub relation: String,
    pub reason: String,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PeerResponseClassification {
    Concede,
    Dispute,
    NoBasisToJudge,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PeerResponse {
    pub peer_claim_reference: String,
    pub classification: PeerResponseClassification,
    pub reason: String,
    #[serde(default)]
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderPosition {
    pub provider: ProviderKind,
    pub round: u8,
    pub turn_id: String,
    pub position: Position,
    pub raw_artifact_id: String,
    pub requested_model: String,
    pub reported_served_model: Option<String>,
    pub serving_identity_status: ServingIdentityStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HumanDecisionKind {
    ApproveOption,
    ApproveModifiedDecision,
    ContinueTargetedDebate,
    ChallengeConsensus,
    RejectAll,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HumanDecision {
    pub kind: HumanDecisionKind,
    pub selected_option: Option<String>,
    pub modified_decision: Option<String>,
    pub rationale: String,
    pub decided_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DecisionRecord {
    pub debate: Debate,
    pub final_positions: Vec<ProviderPosition>,
    pub agreements: Vec<String>,
    pub disagreements: Vec<String>,
    pub most_decision_relevant_dispute: Option<String>,
    pub minority_positions: Vec<String>,
    pub verified_evidence: Vec<String>,
    pub unverified_evidence: Vec<String>,
    pub risks: Vec<String>,
    pub acceptance_criteria: Vec<String>,
    pub human_decision: HumanDecision,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvaluationMetrics {
    pub debate_id: String,
    pub round: u8,
    pub citation_validity: String,
    pub schema_success_percent: u8,
    pub repair_rate_percent: u8,
    pub wall_time_ms_total: u128,
    pub failure_types: BTreeMap<String, u32>,
    pub peer_response_quality_percent: Option<u8>,
    pub revision_frequency_percent: Option<u8>,
    pub decision_changed: Option<bool>,
    pub new_considerations: u32,
    pub independent_only: bool,
}
