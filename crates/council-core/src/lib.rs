pub mod bridge;
pub mod compiler;
pub mod discovery;
pub mod evidence;
pub mod handoff;
pub mod model;
pub mod orchestration;
pub mod packet;
pub mod persistence;
pub mod providers;
pub mod runner;
pub mod snapshot;
pub mod state;
pub mod validation;

pub use bridge::{
    BridgeVerification, WslBridgePlan, WslBridgeRequest, build_wsl_bridge_plan,
    verify_bridge_manifests,
};
pub use compiler::{compile_decision_record, compile_master_prompt, content_hash};
pub use discovery::{
    DiscoveryCandidate, DiscoveryNomination, DiscoveryProposal, DiscoveryResult,
    build_r0_candidate_union, merge_discovery_proposals,
};
pub use evidence::{EvidenceIndex, EvidenceVerdict, VerifiedEvidence};
pub use handoff::{
    ReconstructionCheck, StatelessHandoffPacket, StatelessReconstruction, build_stateless_packet,
    validate_reconstruction,
};
pub use model::*;
pub use orchestration::{
    AttemptRecord, CouncilOrchestrator, CouncilRunResult, DiscoveryRunResult, DiscoveryTurnRecord,
    LiveProviderExecutor, RoundRequest, TurnRecord, extract_position_value,
};
pub use packet::{ContextPacket, PacketMetadata};
pub use persistence::Database;
pub use providers::{
    CommandSpec, ProviderCallRequest, ProviderRegistry, RepairPolicy, billing_environment_status,
    ensure_subscription_environment,
};
pub use runner::{ProcessResult, ProcessRunner};
pub use snapshot::{SnapshotBuilder, SnapshotManifest, SnapshotRequest};
pub use state::{DebateEvent, DebateStateMachine};
pub use validation::{PositionValidation, validate_intake, validate_position_value};

pub const APP_NAME: &str = "Council of Agents";
pub const POSITION_SCHEMA_VERSION: &str = "output-position.v1";
