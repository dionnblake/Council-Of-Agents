use chrono::Utc;
use rusqlite::{Connection, OptionalExtension, params};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::path::Path;
use thiserror::Error;

use crate::evidence::VerifiedEvidence;
use crate::model::{
    Debate, DebateState, DecisionRecord, ExactConfigurationStatus, FailureType, ModelSelection,
    ProviderConfig, ProviderKind, ProviderPosition, ServingIdentityStatus, TurnState, new_id,
};
use crate::packet::WrittenPacket;
use crate::providers::ProviderCallResult;
use crate::snapshot::{
    SnapshotManifest, SnapshotReviewDecision, SnapshotReviewRecord, snapshot_review_id,
};
use crate::state::{DebateEvent, DebateStateMachine};

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("state transition failed: {0}")]
    Transition(#[from] crate::state::StateTransitionError),
    #[error("debate not found: {0}")]
    DebateNotFound(String),
    #[error("immutable record conflict: {0}")]
    Conflict(String),
}

pub struct Database {
    connection: Connection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistedTurnStatus {
    pub round: u8,
    pub provider: ProviderKind,
    pub state: TurnState,
    pub attempts: usize,
    pub failure_type: Option<FailureType>,
    pub requested_model: String,
    pub requested_reasoning_effort: String,
    pub reported_served_model: Option<String>,
    pub serving_identity_status: ServingIdentityStatus,
    pub exact_configuration_status: ExactConfigurationStatus,
    pub exact_configuration_evidence: Option<String>,
    pub certification_boundary: String,
}

impl Database {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, DatabaseError> {
        let connection = Connection::open(path)?;
        let database = Self { connection };
        database.initialize()?;
        Ok(database)
    }

    pub fn in_memory() -> Result<Self, DatabaseError> {
        let connection = Connection::open_in_memory()?;
        let database = Self { connection };
        database.initialize()?;
        Ok(database)
    }

    pub fn initialize(&self) -> Result<(), DatabaseError> {
        self.connection.execute_batch(
            r#"
            PRAGMA foreign_keys = ON;
            PRAGMA journal_mode = WAL;

            CREATE TABLE IF NOT EXISTS debates (
              id TEXT PRIMARY KEY,
              state TEXT NOT NULL,
              intake_json TEXT NOT NULL,
              council_size INTEGER NOT NULL,
              independent_only INTEGER NOT NULL DEFAULT 0,
              discovery_json TEXT,
              provider_models_json TEXT NOT NULL DEFAULT '{}',
              created_at TEXT NOT NULL,
              updated_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS providers (
              slug TEXT PRIMARY KEY,
              config_json TEXT NOT NULL,
              updated_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS app_settings (
              key TEXT PRIMARY KEY,
              value TEXT NOT NULL,
              updated_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS provider_certifications (
              id INTEGER PRIMARY KEY AUTOINCREMENT,
              provider_slug TEXT NOT NULL,
              status TEXT NOT NULL,
              evidence TEXT,
              observed_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS discovery_proposals (
              id TEXT PRIMARY KEY,
              debate_id TEXT NOT NULL REFERENCES debates(id),
              provider_slug TEXT NOT NULL,
              proposal_json TEXT NOT NULL,
              raw_artifact_id TEXT NOT NULL,
              created_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS turns (
              id TEXT PRIMARY KEY,
              debate_id TEXT NOT NULL REFERENCES debates(id),
              round INTEGER NOT NULL,
              provider_slug TEXT NOT NULL,
              state TEXT NOT NULL,
              packet_hash TEXT,
              requested_model TEXT NOT NULL,
              requested_reasoning_effort TEXT NOT NULL DEFAULT '',
              exact_configuration_status TEXT NOT NULL DEFAULT '"UNVERIFIED_CONFIGURATION"',
              exact_configuration_evidence TEXT,
              certification_boundary TEXT NOT NULL DEFAULT 'council-provider-boundary.v1',
              reported_served_model TEXT,
              serving_identity_status TEXT NOT NULL,
              created_at TEXT NOT NULL,
              updated_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS attempts (
              id TEXT PRIMARY KEY,
              turn_id TEXT NOT NULL REFERENCES turns(id),
              attempt_number INTEGER NOT NULL,
              state TEXT NOT NULL,
              failure_type TEXT,
              exit_code INTEGER,
              wall_ms INTEGER,
              created_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS dispatch_intents (
              call_id TEXT PRIMARY KEY,
              debate_id TEXT NOT NULL REFERENCES debates(id),
              turn_id TEXT NOT NULL REFERENCES turns(id),
              round INTEGER NOT NULL,
              provider_slug TEXT NOT NULL,
              attempt_number INTEGER NOT NULL,
              status TEXT NOT NULL,
              result_artifact_id TEXT,
              created_at TEXT NOT NULL,
              updated_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS positions (
              id INTEGER PRIMARY KEY AUTOINCREMENT,
              turn_id TEXT NOT NULL REFERENCES turns(id),
              position_json TEXT NOT NULL,
              accepted INTEGER NOT NULL,
              created_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS claims (
              id TEXT PRIMARY KEY,
              position_id INTEGER NOT NULL REFERENCES positions(id),
              text TEXT NOT NULL,
              evidence_json TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS claim_relations (
              id TEXT PRIMARY KEY,
              position_id INTEGER REFERENCES positions(id),
              source_claim_id TEXT NOT NULL,
              target_claim_id TEXT,
              relation TEXT NOT NULL,
              reason TEXT NOT NULL,
              evidence_json TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS evidence (
              id INTEGER PRIMARY KEY AUTOINCREMENT,
              debate_id TEXT NOT NULL REFERENCES debates(id),
              file TEXT NOT NULL,
              requested_range TEXT NOT NULL,
              resolved_range TEXT,
              content TEXT NOT NULL,
              content_hash TEXT NOT NULL,
              file_hash TEXT,
              verdict TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS snapshots (
              id TEXT PRIMARY KEY,
              debate_id TEXT NOT NULL REFERENCES debates(id),
              root TEXT NOT NULL,
              manifest_json TEXT NOT NULL,
              manifest_hash TEXT NOT NULL,
              created_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS snapshot_files (
              id INTEGER PRIMARY KEY AUTOINCREMENT,
              snapshot_id TEXT NOT NULL REFERENCES snapshots(id),
              relative_path TEXT NOT NULL,
              size INTEGER NOT NULL,
              sha256 TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS snapshot_exclusions (
              id INTEGER PRIMARY KEY AUTOINCREMENT,
              snapshot_id TEXT NOT NULL REFERENCES snapshots(id),
              relative_path TEXT NOT NULL,
              reason TEXT NOT NULL,
              detail TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS snapshot_reviews (
              id TEXT PRIMARY KEY,
              debate_id TEXT NOT NULL REFERENCES debates(id),
              snapshot_id TEXT NOT NULL REFERENCES snapshots(id),
              manifest_hash TEXT NOT NULL,
              exclusion_set_hash TEXT NOT NULL,
              source_tree_hash TEXT NOT NULL,
              secret_exclusion_count INTEGER NOT NULL,
              exclusions_json TEXT NOT NULL,
              decision TEXT NOT NULL,
              rationale TEXT,
              created_at TEXT NOT NULL,
              reviewed_at TEXT
            );
            CREATE TABLE IF NOT EXISTS context_packets (
              packet_id TEXT PRIMARY KEY,
              debate_id TEXT NOT NULL REFERENCES debates(id),
              turn_id TEXT NOT NULL,
              provider_slug TEXT NOT NULL,
              path TEXT NOT NULL,
              sha256 TEXT NOT NULL,
              bytes INTEGER NOT NULL,
              schema_version TEXT NOT NULL,
              skills_json TEXT NOT NULL DEFAULT '[]',
              created_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS raw_artifacts (
              id TEXT PRIMARY KEY,
              turn_id TEXT NOT NULL REFERENCES turns(id),
              stdout TEXT NOT NULL,
              stderr TEXT NOT NULL,
              exit_code INTEGER,
              wall_ms INTEGER,
              requested_model TEXT NOT NULL,
              requested_reasoning_effort TEXT NOT NULL DEFAULT '',
              exact_configuration_status TEXT NOT NULL DEFAULT '"UNVERIFIED_CONFIGURATION"',
              exact_configuration_evidence TEXT,
              certification_boundary TEXT NOT NULL DEFAULT 'council-provider-boundary.v1',
              packet_hash TEXT,
              schema_hash TEXT,
              failure_type TEXT,
              created_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS decisions (
              debate_id TEXT PRIMARY KEY REFERENCES debates(id),
              decision_json TEXT NOT NULL,
              created_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS exports (
              id TEXT PRIMARY KEY,
              debate_id TEXT NOT NULL REFERENCES debates(id),
              kind TEXT NOT NULL,
              path TEXT NOT NULL,
              content_hash TEXT NOT NULL,
              created_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS safety_events (
              id INTEGER PRIMARY KEY AUTOINCREMENT,
              debate_id TEXT,
              event_type TEXT NOT NULL,
              detail TEXT NOT NULL,
              created_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS preflight_results (
              id INTEGER PRIMARY KEY AUTOINCREMENT,
              debate_id TEXT,
              provider_slug TEXT NOT NULL,
              status TEXT NOT NULL,
              detail TEXT NOT NULL,
              created_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS evaluation_metrics (
              id INTEGER PRIMARY KEY AUTOINCREMENT,
              debate_id TEXT NOT NULL REFERENCES debates(id),
              round INTEGER NOT NULL,
              metrics_json TEXT NOT NULL,
              created_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS audit_events (
              sequence INTEGER PRIMARY KEY AUTOINCREMENT,
              debate_id TEXT,
              action TEXT NOT NULL,
              payload_json TEXT NOT NULL,
              previous_hash TEXT,
              event_hash TEXT NOT NULL UNIQUE,
              created_at TEXT NOT NULL
            );
            CREATE TRIGGER IF NOT EXISTS audit_events_no_update
              BEFORE UPDATE ON audit_events
              BEGIN SELECT RAISE(ABORT, 'audit events are append-only'); END;
            CREATE TRIGGER IF NOT EXISTS audit_events_no_delete
              BEFORE DELETE ON audit_events
              BEGIN SELECT RAISE(ABORT, 'audit events are append-only'); END;
            "#,
        )?;
        let _ = self.connection.execute(
            "ALTER TABLE debates ADD COLUMN provider_models_json TEXT NOT NULL DEFAULT '{}'",
            [],
        );
        let _ = self.connection.execute(
            "ALTER TABLE debates ADD COLUMN independent_only INTEGER NOT NULL DEFAULT 0",
            [],
        );
        let _ = self
            .connection
            .execute("ALTER TABLE debates ADD COLUMN discovery_json TEXT", []);
        let _ = self.connection.execute(
            "ALTER TABLE context_packets ADD COLUMN skills_json TEXT NOT NULL DEFAULT '[]'",
            [],
        );
        let _ = self.connection.execute(
            "ALTER TABLE turns ADD COLUMN requested_reasoning_effort TEXT NOT NULL DEFAULT ''",
            [],
        );
        let _ = self.connection.execute(
            "ALTER TABLE turns ADD COLUMN exact_configuration_status TEXT NOT NULL DEFAULT '\"UNVERIFIED_CONFIGURATION\"'",
            [],
        );
        let _ = self.connection.execute(
            "ALTER TABLE turns ADD COLUMN exact_configuration_evidence TEXT",
            [],
        );
        let _ = self.connection.execute(
            "ALTER TABLE turns ADD COLUMN certification_boundary TEXT NOT NULL DEFAULT 'council-provider-boundary.v1'",
            [],
        );
        let _ = self.connection.execute(
            "ALTER TABLE raw_artifacts ADD COLUMN requested_reasoning_effort TEXT NOT NULL DEFAULT ''",
            [],
        );
        let _ = self.connection.execute(
            "ALTER TABLE raw_artifacts ADD COLUMN exact_configuration_status TEXT NOT NULL DEFAULT '\"UNVERIFIED_CONFIGURATION\"'",
            [],
        );
        let _ = self.connection.execute(
            "ALTER TABLE raw_artifacts ADD COLUMN exact_configuration_evidence TEXT",
            [],
        );
        let _ = self.connection.execute(
            "ALTER TABLE raw_artifacts ADD COLUMN certification_boundary TEXT NOT NULL DEFAULT 'council-provider-boundary.v1'",
            [],
        );
        let _ = self.connection.execute(
            "ALTER TABLE claim_relations ADD COLUMN position_id INTEGER REFERENCES positions(id)",
            [],
        );
        let _ = self
            .connection
            .execute("ALTER TABLE evidence ADD COLUMN file_hash TEXT", []);
        Ok(())
    }

    pub fn create_debate(&self, debate: &Debate) -> Result<(), DatabaseError> {
        self.connection.execute(
            "INSERT INTO debates (id, state, intake_json, council_size, independent_only, discovery_json, provider_models_json, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                debate.id,
                serde_json::to_string(&debate.state)?,
                serde_json::to_string(&debate.intake)?,
                debate.council_size,
                debate.independent_only,
                debate
                    .discovery
                    .as_ref()
                    .map(serde_json::to_string)
                    .transpose()?,
                serde_json::to_string(&debate.provider_models)?,
                debate.created_at.to_rfc3339(),
                debate.updated_at.to_rfc3339()
            ],
        )?;
        self.append_audit_event(
            Some(&debate.id),
            "DEBATE_CREATED",
            json!({"state": debate.state, "council_size": debate.council_size}),
        )?;
        Ok(())
    }

    pub fn load_debate(&self, debate_id: &str) -> Result<Debate, DatabaseError> {
        let row = self
            .connection
            .query_row(
                "SELECT id, state, intake_json, council_size, independent_only, discovery_json, provider_models_json, created_at, updated_at FROM debates WHERE id = ?1",
                params![debate_id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, u8>(3)?,
                        row.get::<_, bool>(4)?,
                        row.get::<_, Option<String>>(5)?,
                        row.get::<_, String>(6)?,
                        row.get::<_, String>(7)?,
                        row.get::<_, String>(8)?,
                    ))
                },
            )
            .optional()?
            .ok_or_else(|| DatabaseError::DebateNotFound(debate_id.to_string()))?;
        deserialize_debate_row(row)
    }

    pub fn list_debates(&self, limit: u32) -> Result<Vec<Debate>, DatabaseError> {
        let mut statement = self.connection.prepare(
            "SELECT id, state, intake_json, council_size, independent_only, discovery_json, provider_models_json, created_at, updated_at FROM debates ORDER BY created_at DESC LIMIT ?1",
        )?;
        let rows = statement.query_map(params![limit], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, u8>(3)?,
                row.get::<_, bool>(4)?,
                row.get::<_, Option<String>>(5)?,
                row.get::<_, String>(6)?,
                row.get::<_, String>(7)?,
                row.get::<_, String>(8)?,
            ))
        })?;
        let rows = rows.collect::<Result<Vec<_>, _>>()?;
        rows.into_iter().map(deserialize_debate_row).collect()
    }

    pub fn transition_debate(
        &self,
        debate_id: &str,
        event: DebateEvent,
    ) -> Result<DebateState, DatabaseError> {
        let state_text: String = self
            .connection
            .query_row(
                "SELECT state FROM debates WHERE id = ?1",
                params![debate_id],
                |row| row.get(0),
            )
            .optional()?
            .ok_or_else(|| DatabaseError::DebateNotFound(debate_id.to_string()))?;
        let current: DebateState = serde_json::from_str(&state_text)?;
        let mut machine = DebateStateMachine::new(current);
        let next = machine.transition(event.clone())?;
        self.connection.execute(
            "UPDATE debates SET state = ?1, updated_at = ?2 WHERE id = ?3",
            params![
                serde_json::to_string(&next)?,
                Utc::now().to_rfc3339(),
                debate_id
            ],
        )?;
        self.append_audit_event(
            Some(debate_id),
            "STATE_TRANSITION",
            json!({"event": format!("{event:?}"), "state": next}),
        )?;
        Ok(next)
    }

    pub fn update_turn_state(&self, turn_id: &str, state: TurnState) -> Result<(), DatabaseError> {
        self.connection.execute(
            "UPDATE turns SET state = ?1, updated_at = ?2 WHERE id = ?3",
            params![
                serde_json::to_string(&state)?,
                Utc::now().to_rfc3339(),
                turn_id
            ],
        )?;
        Ok(())
    }

    pub fn latest_turn_statuses(
        &self,
        debate_id: &str,
    ) -> Result<Vec<PersistedTurnStatus>, DatabaseError> {
        let latest_round: Option<u8> = self.connection.query_row(
            "SELECT MAX(round) FROM turns WHERE debate_id = ?1",
            params![debate_id],
            |row| row.get(0),
        )?;
        let Some(latest_round) = latest_round else {
            return Ok(Vec::new());
        };
        let mut statement = self.connection.prepare(
            "SELECT t.round, t.provider_slug, t.state, COUNT(a.id),
                    (SELECT a2.failure_type FROM attempts a2
                     WHERE a2.turn_id = t.id
                     ORDER BY a2.attempt_number DESC, a2.created_at DESC LIMIT 1),
                    t.requested_model, t.requested_reasoning_effort,
                    t.exact_configuration_status, t.exact_configuration_evidence,
                    t.certification_boundary, t.reported_served_model,
                    t.serving_identity_status
             FROM turns t
             LEFT JOIN attempts a ON a.turn_id = t.id
             WHERE t.debate_id = ?1 AND t.round = ?2
             GROUP BY t.id
             ORDER BY t.provider_slug",
        )?;
        let rows = statement.query_map(params![debate_id, latest_round], |row| {
            Ok((
                row.get::<_, u8>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, i64>(3)?,
                row.get::<_, Option<String>>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, String>(6)?,
                row.get::<_, String>(7)?,
                row.get::<_, Option<String>>(8)?,
                row.get::<_, String>(9)?,
                row.get::<_, Option<String>>(10)?,
                row.get::<_, String>(11)?,
            ))
        })?;
        let mut statuses = Vec::new();
        for row in rows {
            let (
                round,
                provider_slug,
                state,
                attempts,
                failure_type,
                requested_model,
                requested_reasoning_effort,
                exact_configuration_status,
                exact_configuration_evidence,
                certification_boundary,
                reported_served_model,
                serving_identity_status,
            ) = row?;
            let Some(provider) = ProviderKind::from_slug(&provider_slug) else {
                continue;
            };
            statuses.push(PersistedTurnStatus {
                round,
                provider,
                state: serde_json::from_str(&state)?,
                attempts: usize::try_from(attempts).map_err(|_| {
                    DatabaseError::Conflict(format!("invalid attempt count for {provider_slug}"))
                })?,
                failure_type: failure_type
                    .as_deref()
                    .map(serde_json::from_str)
                    .transpose()?,
                requested_model,
                requested_reasoning_effort,
                reported_served_model,
                serving_identity_status: serde_json::from_str(&serving_identity_status)?,
                exact_configuration_status: serde_json::from_str(&exact_configuration_status)?,
                exact_configuration_evidence,
                certification_boundary,
            });
        }
        Ok(statuses)
    }

    pub fn save_provider_position(
        &self,
        position: &ProviderPosition,
    ) -> Result<i64, DatabaseError> {
        let transaction = self.connection.unchecked_transaction()?;
        let position_json = serde_json::to_string(&position.position)?;
        transaction.execute(
            "INSERT INTO positions (turn_id, position_json, accepted, created_at) VALUES (?1, ?2, 1, ?3)",
            params![position.turn_id, position_json, Utc::now().to_rfc3339()],
        )?;
        let position_id = transaction.last_insert_rowid();
        for claim in &position.position.claims {
            transaction.execute(
                "INSERT INTO claims (id, position_id, text, evidence_json) VALUES (?1, ?2, ?3, ?4)",
                params![
                    claim.id,
                    position_id,
                    claim.text,
                    serde_json::to_string(&claim.evidence)?
                ],
            )?;
        }
        for (index, response) in position.position.peer_responses.iter().enumerate() {
            let relation_id = new_id("relation");
            let source_claim_id = if response.peer_claim_reference.trim().is_empty() {
                format!(
                    "PEER-REF-{}-{:03}",
                    position.turn_id.to_uppercase(),
                    index + 1
                )
            } else {
                response.peer_claim_reference.clone()
            };
            transaction.execute(
                "INSERT INTO claim_relations (id, position_id, source_claim_id, target_claim_id, relation, reason, evidence_json) VALUES (?1, ?2, ?3, NULL, ?4, ?5, ?6)",
                params![
                    relation_id,
                    position_id,
                    source_claim_id,
                    serde_json::to_string(&response.classification)?,
                    response.reason,
                    serde_json::to_string(&response.evidence)?
                ],
            )?;
        }
        transaction.commit()?;
        Ok(position_id)
    }

    pub fn latest_provider_positions(
        &self,
        debate_id: &str,
    ) -> Result<Vec<ProviderPosition>, DatabaseError> {
        let mut statement = self.connection.prepare(
            "SELECT t.provider_slug, t.round, t.id, t.requested_model,
                    t.requested_reasoning_effort, t.exact_configuration_status,
                    t.exact_configuration_evidence, t.certification_boundary,
                    t.reported_served_model, t.serving_identity_status,
                    p.id, p.position_json,
                    COALESCE((SELECT ra.id FROM raw_artifacts ra
                              WHERE ra.turn_id = t.id
                              ORDER BY ra.created_at DESC LIMIT 1), '')
             FROM positions p
             JOIN turns t ON t.id = p.turn_id
             WHERE t.debate_id = ?1 AND p.accepted = 1
             ORDER BY t.round ASC, p.id ASC",
        )?;
        let rows = statement.query_map(params![debate_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, u8>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, Option<String>>(6)?,
                row.get::<_, String>(7)?,
                row.get::<_, Option<String>>(8)?,
                row.get::<_, String>(9)?,
                row.get::<_, i64>(10)?,
                row.get::<_, String>(11)?,
                row.get::<_, String>(12)?,
            ))
        })?;
        let mut latest = std::collections::BTreeMap::<ProviderKind, ProviderPosition>::new();
        for row in rows {
            let (
                provider_slug,
                round,
                turn_id,
                requested_model,
                requested_reasoning_effort,
                exact_configuration_status,
                exact_configuration_evidence,
                certification_boundary,
                reported_served_model,
                serving_identity_status,
                _position_id,
                position_json,
                raw_artifact_id,
            ) = row?;
            let Some(provider) = ProviderKind::from_slug(&provider_slug) else {
                continue;
            };
            let candidate = ProviderPosition {
                provider: provider.clone(),
                round,
                turn_id,
                position: serde_json::from_str(&position_json)?,
                raw_artifact_id,
                requested_model,
                requested_reasoning_effort,
                reported_served_model,
                serving_identity_status: serde_json::from_str(&serving_identity_status)?,
                exact_configuration_status: serde_json::from_str(&exact_configuration_status)?,
                exact_configuration_evidence,
                certification_boundary,
            };
            latest.insert(provider, candidate);
        }
        Ok(latest.into_values().collect())
    }

    pub fn all_provider_positions(
        &self,
        debate_id: &str,
    ) -> Result<Vec<ProviderPosition>, DatabaseError> {
        let mut statement = self.connection.prepare(
            "SELECT t.provider_slug, t.round, t.id, t.requested_model,
                    t.requested_reasoning_effort, t.exact_configuration_status,
                    t.exact_configuration_evidence, t.certification_boundary,
                    t.reported_served_model, t.serving_identity_status,
                    p.position_json,
                    COALESCE((SELECT ra.id FROM raw_artifacts ra
                              WHERE ra.turn_id = t.id
                              ORDER BY ra.created_at DESC LIMIT 1), '')
             FROM positions p
             JOIN turns t ON t.id = p.turn_id
             WHERE t.debate_id = ?1 AND p.accepted = 1
             ORDER BY t.round ASC, p.id ASC",
        )?;
        let rows = statement.query_map(params![debate_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, u8>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, Option<String>>(6)?,
                row.get::<_, String>(7)?,
                row.get::<_, Option<String>>(8)?,
                row.get::<_, String>(9)?,
                row.get::<_, String>(10)?,
                row.get::<_, String>(11)?,
            ))
        })?;
        let rows = rows.collect::<Result<Vec<_>, _>>()?;
        rows.into_iter()
            .map(
                |(
                    provider_slug,
                    round,
                    turn_id,
                    requested_model,
                    requested_reasoning_effort,
                    exact_configuration_status,
                    exact_configuration_evidence,
                    certification_boundary,
                    reported_served_model,
                    serving_identity_status,
                    position_json,
                    raw_artifact_id,
                )| {
                    let provider = ProviderKind::from_slug(&provider_slug)
                        .ok_or_else(|| DatabaseError::DebateNotFound(provider_slug.clone()))?;
                    Ok(ProviderPosition {
                        provider,
                        round,
                        turn_id,
                        position: serde_json::from_str(&position_json)?,
                        raw_artifact_id,
                        requested_model,
                        requested_reasoning_effort,
                        reported_served_model,
                        serving_identity_status: serde_json::from_str(&serving_identity_status)?,
                        exact_configuration_status: serde_json::from_str(
                            &exact_configuration_status,
                        )?,
                        exact_configuration_evidence,
                        certification_boundary,
                    })
                },
            )
            .collect()
    }

    pub fn save_provider_config(&self, config: &ProviderConfig) -> Result<(), DatabaseError> {
        self.connection.execute(
            "INSERT OR REPLACE INTO providers (slug, config_json, updated_at) VALUES (?1, ?2, ?3)",
            params![
                config.provider.slug(),
                serde_json::to_string(config)?,
                Utc::now().to_rfc3339()
            ],
        )?;
        Ok(())
    }

    pub fn provider_configs(&self) -> Result<Vec<ProviderConfig>, DatabaseError> {
        let mut statement = self
            .connection
            .prepare("SELECT config_json FROM providers ORDER BY slug")?;
        let rows = statement.query_map([], |row| row.get::<_, String>(0))?;
        rows.map(|row| {
            let value = row?;
            serde_json::from_str(&value).map_err(DatabaseError::from)
        })
        .collect()
    }

    pub fn save_app_setting(&self, key: &str, value: &str) -> Result<(), DatabaseError> {
        self.connection.execute(
            "INSERT OR REPLACE INTO app_settings (key, value, updated_at) VALUES (?1, ?2, ?3)",
            params![key, value, Utc::now().to_rfc3339()],
        )?;
        Ok(())
    }

    pub fn load_app_setting(&self, key: &str) -> Result<Option<String>, DatabaseError> {
        self.connection
            .query_row(
                "SELECT value FROM app_settings WHERE key = ?1",
                params![key],
                |row| row.get(0),
            )
            .optional()
            .map_err(DatabaseError::from)
    }

    pub fn save_discovery_result(
        &self,
        debate_id: &str,
        result: &crate::discovery::DiscoveryResult,
    ) -> Result<(), DatabaseError> {
        self.connection.execute(
            "UPDATE debates SET discovery_json = ?1, updated_at = ?2 WHERE id = ?3",
            params![
                serde_json::to_string(result)?,
                Utc::now().to_rfc3339(),
                debate_id
            ],
        )?;
        self.append_audit_event(
            Some(debate_id),
            "R0_CANDIDATE_UNION_CREATED",
            serde_json::to_value(result)?,
        )?;
        for proposal in &result.proposals {
            self.save_discovery_proposal(debate_id, proposal)?;
        }
        Ok(())
    }

    pub fn save_discovery_proposal(
        &self,
        debate_id: &str,
        proposal: &crate::discovery::DiscoveryProposal,
    ) -> Result<(), DatabaseError> {
        self.connection.execute(
            "INSERT OR REPLACE INTO discovery_proposals (id, debate_id, provider_slug, proposal_json, raw_artifact_id, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                proposal.turn_id,
                debate_id,
                proposal.provider.slug(),
                serde_json::to_string(proposal)?,
                proposal.raw_artifact_id,
                Utc::now().to_rfc3339()
            ],
        )?;
        Ok(())
    }

    pub fn discovery_proposals(
        &self,
        debate_id: &str,
    ) -> Result<Vec<crate::discovery::DiscoveryProposal>, DatabaseError> {
        let mut statement = self.connection.prepare(
            "SELECT proposal_json FROM discovery_proposals WHERE debate_id = ?1 ORDER BY created_at, id",
        )?;
        let rows = statement.query_map(params![debate_id], |row| row.get::<_, String>(0))?;
        rows.map(|row| {
            let value = row?;
            serde_json::from_str(&value).map_err(DatabaseError::from)
        })
        .collect()
    }

    pub fn record_preflight(
        &self,
        debate_id: Option<&str>,
        provider: &ProviderConfig,
        status: &str,
        detail: &str,
    ) -> Result<(), DatabaseError> {
        self.connection.execute(
            "INSERT INTO preflight_results (debate_id, provider_slug, status, detail, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                debate_id,
                provider.provider.slug(),
                status,
                detail,
                Utc::now().to_rfc3339()
            ],
        )?;
        Ok(())
    }

    pub fn create_turn(
        &self,
        turn_id: &str,
        debate_id: &str,
        round: u8,
        provider: &ProviderConfig,
        state: TurnState,
        packet_hash: Option<&str>,
    ) -> Result<(), DatabaseError> {
        let selection = ModelSelection::requested_with_effort_for(
            &provider.provider,
            provider.model_default.clone(),
            provider.reasoning_effort_default.clone(),
        );
        self.connection.execute(
            "INSERT INTO turns (id, debate_id, round, provider_slug, state, packet_hash, requested_model, requested_reasoning_effort, exact_configuration_status, exact_configuration_evidence, certification_boundary, reported_served_model, serving_identity_status, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, NULL, ?12, ?13, ?13)",
            params![
                turn_id,
                debate_id,
                round,
                provider.provider.slug(),
                serde_json::to_string(&state)?,
                packet_hash,
                selection.requested_model,
                selection.reasoning_effort.unwrap_or_default(),
                serde_json::to_string(&selection.exact_configuration_status)?,
                selection.exact_configuration_evidence,
                selection.certification_boundary,
                serde_json::to_string(&ServingIdentityStatus::Unknown)?,
                Utc::now().to_rfc3339()
            ],
        )?;
        Ok(())
    }

    pub fn save_attempt(
        &self,
        attempt_id: &str,
        turn_id: &str,
        attempt_number: u8,
        state: TurnState,
        failure_type: Option<&crate::model::FailureType>,
        result: Option<&ProviderCallResult>,
    ) -> Result<(), DatabaseError> {
        self.connection.execute(
            "INSERT INTO attempts (id, turn_id, attempt_number, state, failure_type, exit_code, wall_ms, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                attempt_id,
                turn_id,
                attempt_number,
                serde_json::to_string(&state)?,
                failure_type.map(serde_json::to_string).transpose()?,
                result.and_then(|value| value.exit_code),
                result.map(|value| value.wall_ms as i64),
                Utc::now().to_rfc3339()
            ],
        )?;
        Ok(())
    }

    pub fn create_dispatch_intent(
        &self,
        call_id: &str,
        debate_id: &str,
        turn_id: &str,
        round: u8,
        provider: &ProviderKind,
        attempt_number: u8,
    ) -> Result<(), DatabaseError> {
        let now = Utc::now().to_rfc3339();
        self.connection.execute(
            "INSERT OR IGNORE INTO dispatch_intents (call_id, debate_id, turn_id, round, provider_slug, attempt_number, status, result_artifact_id, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'NOT_DISPATCHED', NULL, ?7, ?7)",
            params![
                call_id,
                debate_id,
                turn_id,
                round,
                provider.slug(),
                attempt_number,
                now
            ],
        )?;
        Ok(())
    }

    pub fn mark_dispatch_running(&self, call_id: &str) -> Result<(), DatabaseError> {
        self.connection.execute(
            "UPDATE dispatch_intents SET status = 'RUNNING', updated_at = ?1 WHERE call_id = ?2 AND status = 'NOT_DISPATCHED'",
            params![Utc::now().to_rfc3339(), call_id],
        )?;
        Ok(())
    }

    pub fn mark_dispatch_complete(
        &self,
        call_id: &str,
        result_artifact_id: Option<&str>,
        status: &str,
    ) -> Result<(), DatabaseError> {
        self.connection.execute(
            "UPDATE dispatch_intents SET status = ?1, result_artifact_id = ?2, updated_at = ?3 WHERE call_id = ?4",
            params![status, result_artifact_id, Utc::now().to_rfc3339(), call_id],
        )?;
        Ok(())
    }

    pub fn recover_inflight_dispatches(&self) -> Result<u64, DatabaseError> {
        let debate_ids = {
            let mut statement = self.connection.prepare(
                "SELECT DISTINCT debate_id FROM dispatch_intents WHERE status = 'RUNNING'",
            )?;
            statement
                .query_map([], |row| row.get::<_, String>(0))?
                .collect::<Result<Vec<_>, _>>()?
        };
        let changed = self.connection.execute(
            "UPDATE dispatch_intents SET status = 'RUNNING_UNKNOWN', updated_at = ?1 WHERE status = 'RUNNING'",
            params![Utc::now().to_rfc3339()],
        )?;
        if changed > 0 {
            for debate_id in &debate_ids {
                let state_text: String = self.connection.query_row(
                    "SELECT state FROM debates WHERE id = ?1",
                    params![debate_id],
                    |row| row.get(0),
                )?;
                let state: DebateState = serde_json::from_str(&state_text)?;
                if matches!(
                    state,
                    DebateState::Draft
                        | DebateState::Preflight
                        | DebateState::Snapshotting
                        | DebateState::Ready
                        | DebateState::Opening
                        | DebateState::CrossExamination
                        | DebateState::FinalPositions
                        | DebateState::AwaitingHumanDecision
                ) {
                    self.transition_debate(debate_id, DebateEvent::Pause)?;
                } else if !matches!(state, DebateState::Paused) {
                    self.record_safety_event(
                        Some(debate_id),
                        "RECOVERY_STATE_UNEXPECTED",
                        &format!(
                            "in-flight dispatch recovered from terminal state {state:?}; human review is required"
                        ),
                    )?;
                }
            }
            self.append_audit_event(
                None,
                "DISPATCH_RECOVERY_REQUIRED",
                json!({"running_unknown": changed, "debates": debate_ids}),
            )?;
        }
        Ok(changed as u64)
    }

    pub fn dispatch_status(&self, call_id: &str) -> Result<Option<String>, DatabaseError> {
        self.connection
            .query_row(
                "SELECT status FROM dispatch_intents WHERE call_id = ?1",
                params![call_id],
                |row| row.get(0),
            )
            .optional()
            .map_err(DatabaseError::from)
    }

    pub fn save_raw_artifact(
        &self,
        turn_id: &str,
        result: &ProviderCallResult,
        packet_hash: Option<&str>,
        schema_hash: Option<&str>,
    ) -> Result<(), DatabaseError> {
        self.connection.execute(
            "INSERT INTO raw_artifacts (id, turn_id, stdout, stderr, exit_code, wall_ms, requested_model, requested_reasoning_effort, exact_configuration_status, exact_configuration_evidence, certification_boundary, packet_hash, schema_hash, failure_type, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            params![
                result.raw_artifact_id,
                turn_id,
                result.stdout,
                result.stderr,
                result.exit_code,
                result.wall_ms as i64,
                result.requested_model,
                result.requested_reasoning_effort,
                serde_json::to_string(&result.exact_configuration_status)?,
                result.exact_configuration_evidence,
                result.certification_boundary,
                packet_hash,
                schema_hash,
                result.failure_type.as_ref().map(serde_json::to_string).transpose()?,
                Utc::now().to_rfc3339()
            ],
        )?;
        self.connection.execute(
            "UPDATE turns SET reported_served_model = ?1, serving_identity_status = ?2, updated_at = ?3 WHERE id = ?4",
            params![
                result.reported_served_model,
                serde_json::to_string(&result.serving_identity_status)?,
                Utc::now().to_rfc3339(),
                turn_id
            ],
        )?;
        Ok(())
    }

    pub fn save_packet(&self, packet: &WrittenPacket) -> Result<(), DatabaseError> {
        self.connection.execute(
            "INSERT INTO context_packets (packet_id, debate_id, turn_id, provider_slug, path, sha256, bytes, schema_version, skills_json, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                packet.metadata.packet_id,
                packet.metadata.debate_id,
                packet.metadata.turn_id,
                packet.metadata.provider.slug(),
                packet.path.to_string_lossy(),
                packet.sha256,
                packet.bytes as i64,
                packet.metadata.schema_version,
                serde_json::to_string(&packet.metadata.skills)?,
                packet.metadata.created_at.to_rfc3339()
            ],
        )?;
        Ok(())
    }

    pub fn save_snapshot(
        &self,
        debate_id: &str,
        root: &Path,
        manifest: &SnapshotManifest,
    ) -> Result<(), DatabaseError> {
        let transaction = self.connection.unchecked_transaction()?;
        transaction.execute(
            "INSERT INTO snapshots (id, debate_id, root, manifest_json, manifest_hash, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                manifest.snapshot_id,
                debate_id,
                root.to_string_lossy(),
                serde_json::to_string(manifest)?,
                manifest.manifest_sha256,
                manifest.created_at.to_rfc3339()
            ],
        )?;
        for file in &manifest.files {
            transaction.execute(
                "INSERT INTO snapshot_files (snapshot_id, relative_path, size, sha256) VALUES (?1, ?2, ?3, ?4)",
                params![manifest.snapshot_id, file.relative_path, file.size as i64, file.sha256],
            )?;
        }
        for exclusion in &manifest.exclusions {
            transaction.execute(
                "INSERT INTO snapshot_exclusions (snapshot_id, relative_path, reason, detail) VALUES (?1, ?2, ?3, ?4)",
                params![
                    manifest.snapshot_id,
                    exclusion.relative_path,
                    serde_json::to_string(&exclusion.reason)?,
                    exclusion.detail
                ],
            )?;
        }
        transaction.commit()?;
        Ok(())
    }

    pub fn save_evidence(
        &self,
        debate_id: &str,
        evidence: &VerifiedEvidence,
    ) -> Result<(), DatabaseError> {
        self.connection.execute(
            "INSERT INTO evidence (debate_id, file, requested_range, resolved_range, content, content_hash, file_hash, verdict) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                debate_id,
                evidence.file,
                evidence.requested_range,
                evidence.resolved_range,
                evidence.content,
                evidence.content_hash,
                evidence.file_hash,
                serde_json::to_string(&evidence.verdict)?
            ],
        )?;
        Ok(())
    }

    pub fn evidence_for_debate(
        &self,
        debate_id: &str,
    ) -> Result<Vec<VerifiedEvidence>, DatabaseError> {
        let mut statement = self.connection.prepare(
            "SELECT file, requested_range, resolved_range, content, content_hash, file_hash, verdict FROM evidence WHERE debate_id = ?1 ORDER BY id",
        )?;
        let rows = statement.query_map(params![debate_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Option<String>>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, Option<String>>(5)?,
                row.get::<_, String>(6)?,
            ))
        })?;
        rows.map(|row| {
            let (file, requested_range, resolved_range, content, content_hash, file_hash, verdict) =
                row?;
            Ok(VerifiedEvidence {
                file,
                requested_range,
                resolved_range,
                content,
                content_hash,
                file_hash,
                verdict: serde_json::from_str(&verdict)?,
            })
        })
        .collect()
    }

    pub fn latest_snapshot(
        &self,
        debate_id: &str,
    ) -> Result<Option<(std::path::PathBuf, SnapshotManifest)>, DatabaseError> {
        let row = self.connection.query_row(
            "SELECT root, manifest_json FROM snapshots WHERE debate_id = ?1 ORDER BY created_at DESC LIMIT 1",
            params![debate_id],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
        ).optional()?;
        row.map(|(root, manifest)| {
            Ok((
                std::path::PathBuf::from(root),
                serde_json::from_str(&manifest)?,
            ))
        })
        .transpose()
    }

    pub fn save_snapshot_review_pending(
        &self,
        record: &SnapshotReviewRecord,
    ) -> Result<(), DatabaseError> {
        if record.decision != SnapshotReviewDecision::Pending {
            return Err(DatabaseError::Conflict(
                "only pending snapshot reviews may be created".to_string(),
            ));
        }
        let existing = self.snapshot_review_by_id(&record.id)?;
        if let Some(existing) = existing {
            if existing.debate_id == record.debate_id
                && existing.snapshot_id == record.snapshot_id
                && existing.manifest_hash == record.manifest_hash
                && existing.exclusion_set_hash == record.exclusion_set_hash
                && existing.source_tree_hash == record.source_tree_hash
                && existing.exclusions == record.exclusions
                && existing.secret_exclusion_count == record.secret_exclusion_count
                && existing.decision == SnapshotReviewDecision::Pending
            {
                return Ok(());
            }
            return Err(DatabaseError::Conflict(format!(
                "snapshot review {} already exists with different evidence or decision",
                record.id
            )));
        }
        self.connection.execute(
            "INSERT INTO snapshot_reviews (id, debate_id, snapshot_id, manifest_hash, exclusion_set_hash, source_tree_hash, secret_exclusion_count, exclusions_json, decision, rationale, created_at, reviewed_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, NULL, ?10, NULL)",
            params![
                record.id,
                record.debate_id,
                record.snapshot_id,
                record.manifest_hash,
                record.exclusion_set_hash,
                record.source_tree_hash,
                record.secret_exclusion_count,
                serde_json::to_string(&record.exclusions)?,
                serde_json::to_string(&record.decision)?,
                record.created_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn latest_snapshot_review(
        &self,
        debate_id: &str,
    ) -> Result<Option<SnapshotReviewRecord>, DatabaseError> {
        let row = self
            .connection
            .query_row(
                "SELECT id, debate_id, snapshot_id, manifest_hash, exclusion_set_hash, source_tree_hash, secret_exclusion_count, exclusions_json, decision, rationale, created_at, reviewed_at FROM snapshot_reviews WHERE debate_id = ?1 ORDER BY created_at DESC, rowid DESC LIMIT 1",
                params![debate_id],
                snapshot_review_row,
            )
            .optional()?;
        row.map(deserialize_snapshot_review_row).transpose()
    }

    pub fn record_snapshot_review_decision(
        &self,
        debate_id: &str,
        snapshot_id: &str,
        manifest_hash: &str,
        exclusion_set_hash: &str,
        decision: SnapshotReviewDecision,
        rationale: &str,
    ) -> Result<SnapshotReviewRecord, DatabaseError> {
        if decision == SnapshotReviewDecision::Pending {
            return Err(DatabaseError::Conflict(
                "a snapshot review decision must be approved or rejected".to_string(),
            ));
        }
        let review_id =
            snapshot_review_id(debate_id, snapshot_id, manifest_hash, exclusion_set_hash);
        let Some(existing) = self.snapshot_review_by_id(&review_id)? else {
            return Err(DatabaseError::Conflict(
                "snapshot review evidence is not persisted".to_string(),
            ));
        };
        if existing.decision != SnapshotReviewDecision::Pending {
            return Err(DatabaseError::Conflict(
                "snapshot review decision is immutable".to_string(),
            ));
        }
        let reviewed_at = Utc::now().to_rfc3339();
        self.connection.execute(
            "UPDATE snapshot_reviews SET decision = ?1, rationale = ?2, reviewed_at = ?3 WHERE id = ?4 AND debate_id = ?5 AND snapshot_id = ?6 AND manifest_hash = ?7 AND exclusion_set_hash = ?8 AND decision = ?9",
            params![
                serde_json::to_string(&decision)?,
                rationale,
                reviewed_at,
                review_id,
                debate_id,
                snapshot_id,
                manifest_hash,
                exclusion_set_hash,
                serde_json::to_string(&SnapshotReviewDecision::Pending)?,
            ],
        )?;
        self.snapshot_review_by_id(&review_id)?.ok_or_else(|| {
            DatabaseError::Conflict("snapshot review decision could not be reloaded".to_string())
        })
    }

    fn snapshot_review_by_id(
        &self,
        review_id: &str,
    ) -> Result<Option<SnapshotReviewRecord>, DatabaseError> {
        let row = self
            .connection
            .query_row(
                "SELECT id, debate_id, snapshot_id, manifest_hash, exclusion_set_hash, source_tree_hash, secret_exclusion_count, exclusions_json, decision, rationale, created_at, reviewed_at FROM snapshot_reviews WHERE id = ?1",
                params![review_id],
                snapshot_review_row,
            )
            .optional()?;
        row.map(deserialize_snapshot_review_row).transpose()
    }

    pub fn update_debate_provider_models(
        &self,
        debate_id: &str,
        provider_models: &std::collections::BTreeMap<ProviderKind, ModelSelection>,
    ) -> Result<(), DatabaseError> {
        self.connection.execute(
            "UPDATE debates SET provider_models_json = ?1, council_size = ?2, updated_at = ?3 WHERE id = ?4",
            params![
                serde_json::to_string(provider_models)?,
                provider_models.len() as u8,
                Utc::now().to_rfc3339(),
                debate_id
            ],
        )?;
        self.append_audit_event(
            Some(debate_id),
            "COUNCIL_SIZE_CHANGED",
            json!({"council_size": provider_models.len(), "providers": provider_models.keys().map(ProviderKind::slug).collect::<Vec<_>>() }),
        )?;
        Ok(())
    }

    pub fn save_export(
        &self,
        export_id: &str,
        debate_id: &str,
        kind: &str,
        path: &Path,
        content_hash: &str,
    ) -> Result<(), DatabaseError> {
        let path_text = path.to_string_lossy().to_string();
        let existing = self
            .connection
            .query_row(
                "SELECT debate_id, kind, path, content_hash FROM exports WHERE id = ?1",
                params![export_id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                    ))
                },
            )
            .optional()?;
        if let Some((existing_debate, existing_kind, existing_path, existing_hash)) = existing {
            if existing_debate == debate_id
                && existing_kind == kind
                && existing_path == path_text
                && existing_hash == content_hash
            {
                return Ok(());
            }
            return Err(DatabaseError::Conflict(format!(
                "export {export_id} already exists with different content"
            )));
        }
        self.connection.execute(
            "INSERT INTO exports (id, debate_id, kind, path, content_hash, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![export_id, debate_id, kind, path_text, content_hash, Utc::now().to_rfc3339()],
        )?;
        Ok(())
    }

    pub fn record_safety_event(
        &self,
        debate_id: Option<&str>,
        event_type: &str,
        detail: &str,
    ) -> Result<(), DatabaseError> {
        self.connection.execute(
            "INSERT INTO safety_events (debate_id, event_type, detail, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![debate_id, event_type, detail, Utc::now().to_rfc3339()],
        )?;
        Ok(())
    }

    pub fn save_decision(&self, record: &DecisionRecord) -> Result<(), DatabaseError> {
        let decision_json = serde_json::to_string(record)?;
        let existing = self
            .connection
            .query_row(
                "SELECT decision_json FROM decisions WHERE debate_id = ?1",
                params![record.debate.id],
                |row| row.get::<_, String>(0),
            )
            .optional()?;
        if let Some(existing) = existing {
            if existing == decision_json {
                return Ok(());
            }
            return Err(DatabaseError::Conflict(format!(
                "decision for debate {} is immutable",
                record.debate.id
            )));
        }
        self.connection.execute(
            "INSERT INTO decisions (debate_id, decision_json, created_at) VALUES (?1, ?2, ?3)",
            params![record.debate.id, decision_json, Utc::now().to_rfc3339()],
        )?;
        self.append_audit_event(
            Some(&record.debate.id),
            "HUMAN_DECISION_RECORDED",
            serde_json::to_value(&record.human_decision)?,
        )?;
        Ok(())
    }

    pub fn load_decision(&self, debate_id: &str) -> Result<Option<DecisionRecord>, DatabaseError> {
        let value = self
            .connection
            .query_row(
                "SELECT decision_json FROM decisions WHERE debate_id = ?1",
                params![debate_id],
                |row| row.get::<_, String>(0),
            )
            .optional()?;
        value
            .map(|value| serde_json::from_str(&value).map_err(DatabaseError::from))
            .transpose()
    }

    pub fn save_evaluation_metrics(
        &self,
        metrics: &crate::model::EvaluationMetrics,
    ) -> Result<(), DatabaseError> {
        self.connection.execute(
            "INSERT INTO evaluation_metrics (debate_id, round, metrics_json, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![
                metrics.debate_id,
                metrics.round,
                serde_json::to_string(metrics)?,
                Utc::now().to_rfc3339()
            ],
        )?;
        Ok(())
    }

    pub fn evaluation_metrics(
        &self,
        debate_id: &str,
    ) -> Result<Vec<crate::model::EvaluationMetrics>, DatabaseError> {
        let mut statement = self.connection.prepare(
            "SELECT metrics_json FROM evaluation_metrics WHERE debate_id = ?1 ORDER BY round, id",
        )?;
        let rows = statement.query_map(params![debate_id], |row| row.get::<_, String>(0))?;
        let rows = rows.collect::<Result<Vec<_>, _>>()?;
        rows.into_iter()
            .map(|value| serde_json::from_str(&value).map_err(DatabaseError::from))
            .collect()
    }

    pub fn append_audit_event(
        &self,
        debate_id: Option<&str>,
        action: &str,
        payload: serde_json::Value,
    ) -> Result<String, DatabaseError> {
        let previous_hash: Option<String> = self
            .connection
            .query_row(
                "SELECT event_hash FROM audit_events ORDER BY sequence DESC LIMIT 1",
                [],
                |row| row.get(0),
            )
            .optional()?;
        let payload_json = serde_json::to_string(&payload)?;
        let created_at = Utc::now().to_rfc3339();
        let mut digest = Sha256::new();
        digest.update(previous_hash.as_deref().unwrap_or_default().as_bytes());
        digest.update(action.as_bytes());
        digest.update(payload_json.as_bytes());
        digest.update(created_at.as_bytes());
        let event_hash = hex::encode(digest.finalize());
        self.connection.execute(
            "INSERT INTO audit_events (debate_id, action, payload_json, previous_hash, event_hash, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![debate_id, action, payload_json, previous_hash, event_hash, created_at],
        )?;
        Ok(event_hash)
    }

    pub fn audit_hashes(&self) -> Result<Vec<String>, DatabaseError> {
        let mut statement = self
            .connection
            .prepare("SELECT event_hash FROM audit_events ORDER BY sequence")?;
        let rows = statement.query_map([], |row| row.get(0))?;
        Ok(rows.collect::<Result<Vec<String>, _>>()?)
    }

    pub fn audit_count(&self) -> Result<i64, DatabaseError> {
        Ok(self
            .connection
            .query_row("SELECT COUNT(*) FROM audit_events", [], |row| row.get(0))?)
    }

    pub fn audit_action_count(&self, debate_id: &str, action: &str) -> Result<i64, DatabaseError> {
        Ok(self.connection.query_row(
            "SELECT COUNT(*) FROM audit_events WHERE debate_id = ?1 AND action = ?2",
            params![debate_id, action],
            |row| row.get(0),
        )?)
    }
}

fn snapshot_review_row(
    row: &rusqlite::Row<'_>,
) -> rusqlite::Result<(
    String,
    String,
    String,
    String,
    String,
    String,
    i64,
    String,
    String,
    Option<String>,
    String,
    Option<String>,
)> {
    Ok((
        row.get(0)?,
        row.get(1)?,
        row.get(2)?,
        row.get(3)?,
        row.get(4)?,
        row.get(5)?,
        row.get(6)?,
        row.get(7)?,
        row.get(8)?,
        row.get(9)?,
        row.get(10)?,
        row.get(11)?,
    ))
}

fn deserialize_snapshot_review_row(
    row: (
        String,
        String,
        String,
        String,
        String,
        String,
        i64,
        String,
        String,
        Option<String>,
        String,
        Option<String>,
    ),
) -> Result<SnapshotReviewRecord, DatabaseError> {
    let created_at = chrono::DateTime::parse_from_rfc3339(&row.10)
        .map_err(|error| {
            DatabaseError::Conflict(format!("invalid snapshot review timestamp: {error}"))
        })?
        .with_timezone(&Utc);
    let reviewed_at = row
        .11
        .as_deref()
        .map(chrono::DateTime::parse_from_rfc3339)
        .transpose()
        .map_err(|error| {
            DatabaseError::Conflict(format!("invalid snapshot review timestamp: {error}"))
        })?
        .map(|value| value.with_timezone(&Utc));
    Ok(SnapshotReviewRecord {
        id: row.0,
        debate_id: row.1,
        snapshot_id: row.2,
        manifest_hash: row.3,
        exclusion_set_hash: row.4,
        source_tree_hash: row.5,
        secret_exclusion_count: u32::try_from(row.6)
            .map_err(|_| DatabaseError::Conflict("invalid secret exclusion count".to_string()))?,
        exclusions: serde_json::from_str(&row.7)?,
        decision: serde_json::from_str(&row.8)?,
        rationale: row.9,
        created_at,
        reviewed_at,
    })
}

fn deserialize_debate_row(
    row: (
        String,
        String,
        String,
        u8,
        bool,
        Option<String>,
        String,
        String,
        String,
    ),
) -> Result<Debate, DatabaseError> {
    Ok(Debate {
        id: row.0,
        state: serde_json::from_str(&row.1)?,
        intake: serde_json::from_str(&row.2)?,
        council_size: row.3,
        independent_only: row.4,
        discovery: row.5.as_deref().map(serde_json::from_str).transpose()?,
        provider_models: serde_json::from_str(&row.6)?,
        created_at: chrono::DateTime::parse_from_rfc3339(&row.7)
            .map_err(|error| {
                rusqlite::Error::FromSqlConversionFailure(
                    0,
                    rusqlite::types::Type::Text,
                    Box::new(error),
                )
            })?
            .with_timezone(&Utc),
        updated_at: chrono::DateTime::parse_from_rfc3339(&row.8)
            .map_err(|error| {
                rusqlite::Error::FromSqlConversionFailure(
                    0,
                    rusqlite::types::Type::Text,
                    Box::new(error),
                )
            })?
            .with_timezone(&Utc),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{
        CERTIFICATION_BOUNDARY_VERSION, Claim, Commitment, Debate, ExactConfigurationStatus,
        FailureType, Intake, ModelSelection, PeerResponse, PeerResponseClassification, Position,
        ProviderConfig, ProviderKind, Reversibility, TurnState,
    };
    use crate::snapshot::{
        SnapshotBuilder, SnapshotRequest, SnapshotReviewDecision,
        snapshot_exclusion_review_identity, snapshot_review_id,
    };
    use std::collections::BTreeMap;
    use std::fs;

    #[test]
    fn audit_log_is_hash_chained_and_append_only() {
        let database = Database::in_memory().unwrap();
        let mut models = BTreeMap::new();
        models.insert(
            ProviderKind::CodexWsl,
            ModelSelection::requested("gpt-5.6-luna"),
        );
        let debate = Debate::new(Intake::default(), models);
        database.create_debate(&debate).unwrap();
        database
            .append_audit_event(Some(&debate.id), "TEST", json!({"ok": true}))
            .unwrap();
        let loaded = database.load_debate(&debate.id).unwrap();
        assert_eq!(loaded.provider_models, debate.provider_models);
        assert_eq!(database.list_debates(10).unwrap().len(), 1);
        assert_eq!(database.audit_count().unwrap(), 2);
        assert_eq!(database.audit_hashes().unwrap().len(), 2);
        let error = database
            .connection
            .execute("DELETE FROM audit_events WHERE sequence = 1", []);
        assert!(error.is_err());
    }

    #[test]
    fn legacy_turn_and_artifact_rows_receive_exact_configuration_defaults() {
        let temp = tempfile::tempdir().unwrap();
        let database_path = temp.path().join("legacy.sqlite3");
        {
            let connection = Connection::open(&database_path).unwrap();
            connection
                .execute_batch(
                    r#"
                    CREATE TABLE turns (
                      id TEXT PRIMARY KEY,
                      debate_id TEXT NOT NULL,
                      round INTEGER NOT NULL,
                      provider_slug TEXT NOT NULL,
                      state TEXT NOT NULL,
                      packet_hash TEXT,
                      requested_model TEXT NOT NULL,
                      reported_served_model TEXT,
                      serving_identity_status TEXT NOT NULL,
                      created_at TEXT NOT NULL,
                      updated_at TEXT NOT NULL
                    );
                    CREATE TABLE raw_artifacts (
                      id TEXT PRIMARY KEY,
                      turn_id TEXT NOT NULL,
                      stdout TEXT NOT NULL,
                      stderr TEXT NOT NULL,
                      exit_code INTEGER,
                      wall_ms INTEGER,
                      requested_model TEXT NOT NULL,
                      packet_hash TEXT,
                      schema_hash TEXT,
                      failure_type TEXT,
                      created_at TEXT NOT NULL
                    );
                    "#,
                )
                .unwrap();
        }

        let database = Database::open(&database_path).unwrap();
        let now = Utc::now().to_rfc3339();
        database
            .connection
            .execute(
                "INSERT INTO debates (id, state, intake_json, council_size, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?5)",
                params!["legacy-debate", "\"DRAFT\"", "{}", 1, now],
            )
            .unwrap();
        database
            .connection
            .execute(
                "INSERT INTO turns (id, debate_id, round, provider_slug, state, requested_model, serving_identity_status, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?8)",
                params![
                    "legacy-turn",
                    "legacy-debate",
                    1,
                    "codex-wsl",
                    "\"PENDING\"",
                    "gpt-5.6-luna",
                    "\"UNKNOWN\"",
                    Utc::now().to_rfc3339()
                ],
            )
            .unwrap();
        database
            .connection
            .execute(
                "INSERT INTO raw_artifacts (id, turn_id, stdout, stderr, requested_model, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    "legacy-raw",
                    "legacy-turn",
                    "{}",
                    "",
                    "gpt-5.6-luna",
                    Utc::now().to_rfc3339()
                ],
            )
            .unwrap();

        let turn_defaults: (String, String, Option<String>, String) = database
            .connection
            .query_row(
                "SELECT requested_reasoning_effort, exact_configuration_status, exact_configuration_evidence, certification_boundary FROM turns WHERE id = 'legacy-turn'",
                [],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
            )
            .unwrap();
        assert_eq!(turn_defaults.0, "");
        assert_eq!(turn_defaults.1, "\"UNVERIFIED_CONFIGURATION\"");
        assert_eq!(turn_defaults.2, None);
        assert_eq!(turn_defaults.3, CERTIFICATION_BOUNDARY_VERSION);

        let artifact_defaults: (String, String, Option<String>, String) = database
            .connection
            .query_row(
                "SELECT requested_reasoning_effort, exact_configuration_status, exact_configuration_evidence, certification_boundary FROM raw_artifacts WHERE id = 'legacy-raw'",
                [],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
            )
            .unwrap();
        assert_eq!(artifact_defaults, turn_defaults);
    }

    #[test]
    fn snapshot_review_persists_across_reopen_and_binds_exact_evidence() {
        let temp = tempfile::tempdir().unwrap();
        let database_path = temp.path().join("council.sqlite3");
        let source = temp.path().join("source");
        let snapshots = temp.path().join("snapshots");
        fs::create_dir_all(&source).unwrap();
        fs::write(source.join("README.md"), b"safe evidence").unwrap();
        fs::write(source.join(".env"), b"API_KEY=not-for-providers-1234567890").unwrap();
        let manifest = SnapshotBuilder::new()
            .build(&SnapshotRequest {
                source_root: source.clone(),
                destination_root: snapshots,
                snapshot_id: "snapshot-review-test".to_string(),
            })
            .unwrap();
        let root = temp.path().join("snapshots").join(&manifest.snapshot_id);
        let builder = SnapshotBuilder::new();
        let source_tree_hash = builder.source_tree_hash(&source).unwrap();
        let (exclusion_set_hash, exclusions) = snapshot_exclusion_review_identity(&manifest);
        let mut models = BTreeMap::new();
        models.insert(
            ProviderKind::Claude,
            ModelSelection::requested("test-model"),
        );
        let debate = Debate::new(Intake::default(), models);

        let database = Database::open(&database_path).unwrap();
        database.create_debate(&debate).unwrap();
        database
            .save_snapshot(&debate.id, &root, &manifest)
            .unwrap();
        let review = SnapshotReviewRecord {
            id: snapshot_review_id(
                &debate.id,
                &manifest.snapshot_id,
                &manifest.manifest_sha256,
                &exclusion_set_hash,
            ),
            debate_id: debate.id.clone(),
            snapshot_id: manifest.snapshot_id.clone(),
            manifest_hash: manifest.manifest_sha256.clone(),
            exclusion_set_hash,
            source_tree_hash,
            secret_exclusion_count: 1,
            exclusions,
            decision: SnapshotReviewDecision::Pending,
            rationale: None,
            created_at: Utc::now(),
            reviewed_at: None,
        };
        database.save_snapshot_review_pending(&review).unwrap();
        drop(database);

        let database = Database::open(&database_path).unwrap();
        let reopened = database
            .latest_snapshot_review(&debate.id)
            .unwrap()
            .unwrap();
        assert_eq!(reopened.decision, SnapshotReviewDecision::Pending);
        assert_eq!(reopened.manifest_hash, manifest.manifest_sha256);
        assert_eq!(reopened.exclusion_set_hash, review.exclusion_set_hash);
        let approved = database
            .record_snapshot_review_decision(
                &debate.id,
                &manifest.snapshot_id,
                &manifest.manifest_sha256,
                &review.exclusion_set_hash,
                SnapshotReviewDecision::Approved,
                "safe review note",
            )
            .unwrap();
        assert_eq!(approved.decision, SnapshotReviewDecision::Approved);
        assert!(approved.reviewed_at.is_some());
        let stored_exclusions: String = database
            .connection
            .query_row(
                "SELECT exclusions_json FROM snapshot_reviews WHERE id = ?1",
                params![review.id],
                |row| row.get(0),
            )
            .unwrap();
        assert!(!stored_exclusions.contains("not-for-providers"));
        assert!(
            database
                .record_snapshot_review_decision(
                    &debate.id,
                    &manifest.snapshot_id,
                    &manifest.manifest_sha256,
                    &review.exclusion_set_hash,
                    SnapshotReviewDecision::Rejected,
                    "should not overwrite approval",
                )
                .is_err()
        );
    }

    #[test]
    fn latest_positions_round_trip_from_turn_storage() {
        let database = Database::in_memory().unwrap();
        let models = BTreeMap::from([(
            ProviderKind::CodexWsl,
            ModelSelection::requested("gpt-5.6-luna"),
        )]);
        let debate = Debate::new(Intake::default(), models);
        database.create_debate(&debate).unwrap();
        let config = ProviderConfig::defaults()
            .into_iter()
            .find(|config| config.provider == ProviderKind::CodexWsl)
            .unwrap();
        let turn_id = "turn-round-trip";
        database
            .create_turn(
                turn_id,
                &debate.id,
                3,
                &config,
                TurnState::Valid,
                Some("packet-hash"),
            )
            .unwrap();
        let position = ProviderPosition {
            provider: ProviderKind::CodexWsl,
            round: 3,
            turn_id: turn_id.to_string(),
            position: Position {
                schema_version: crate::POSITION_SCHEMA_VERSION.to_string(),
                recommendation: "Keep the reversible local path".to_string(),
                commitment: Commitment::WouldStake,
                claims: vec![Claim {
                    id: "claim-1".to_string(),
                    text: "It keeps the first release small".to_string(),
                    evidence: vec!["packet.md:1-2".to_string()],
                }],
                risks: vec!["A later scale trigger may require migration".to_string()],
                assumptions: Vec::new(),
                alternatives: Vec::new(),
                flip_condition: "Measured scale exceeds the local boundary".to_string(),
                cost_if_wrong: "A contained migration consumes one sprint".to_string(),
                when_wrongness_becomes_visible: None,
                reversibility: Reversibility::Easy,
                strongest_argument_against_my_recommendation: String::new(),
                what_my_recommendation_is_bad_at: String::new(),
                acceptance_criteria: Vec::new(),
                implementation_constraints: Vec::new(),
                peer_responses: vec![PeerResponse {
                    peer_claim_reference: "PEER-CLAIM-1-001".to_string(),
                    classification: PeerResponseClassification::Dispute,
                    reason: "The peer's scale assumption is not supported by the packet."
                        .to_string(),
                    evidence: vec!["packet.md:1-2".to_string()],
                }],
                withdrawn_claims: Vec::new(),
                conceded_claims: Vec::new(),
                remaining_disputes: Vec::new(),
                revision_reason: None,
                prior_position_hash: None,
            },
            raw_artifact_id: "raw-1".to_string(),
            requested_model: "gpt-5.6-luna".to_string(),
            requested_reasoning_effort: "max".to_string(),
            reported_served_model: None,
            serving_identity_status: ServingIdentityStatus::ProviderDoesNotReport,
            exact_configuration_status: ExactConfigurationStatus::UnverifiedConfiguration,
            exact_configuration_evidence: Some(
                "test configuration is intentionally unverified".to_string(),
            ),
            certification_boundary: CERTIFICATION_BOUNDARY_VERSION.to_string(),
        };
        database.save_provider_position(&position).unwrap();
        let stored_reference: String = database
            .connection
            .query_row(
                "SELECT source_claim_id FROM claim_relations WHERE position_id = 1",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(stored_reference, "PEER-CLAIM-1-001");
        let loaded = database.latest_provider_positions(&debate.id).unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(
            loaded[0].position.recommendation,
            position.position.recommendation
        );
        assert_eq!(loaded[0].round, 3);
        assert_eq!(loaded[0].requested_model, "gpt-5.6-luna");
        assert_eq!(loaded[0].requested_reasoning_effort, "max");
        assert_eq!(
            loaded[0].exact_configuration_status,
            ExactConfigurationStatus::UnverifiedConfiguration
        );
        assert_eq!(
            loaded[0].certification_boundary,
            CERTIFICATION_BOUNDARY_VERSION
        );
    }

    #[test]
    fn latest_turn_statuses_preserve_failed_round_seats_for_reload() {
        let database = Database::in_memory().unwrap();
        let models = BTreeMap::from([
            (
                ProviderKind::Claude,
                ModelSelection::requested_with_effort_for(
                    &ProviderKind::Claude,
                    "claude-haiku-4-5-20251001",
                    "high",
                ),
            ),
            (
                ProviderKind::Antigravity,
                ModelSelection::requested_with_effort_for(
                    &ProviderKind::Antigravity,
                    "gemini-3.7-flash-low",
                    "low",
                ),
            ),
        ]);
        let debate = Debate::new(Intake::default(), models);
        database.create_debate(&debate).unwrap();
        let configs = ProviderConfig::defaults();
        let claude = configs
            .iter()
            .find(|config| config.provider == ProviderKind::Claude)
            .unwrap();
        let antigravity = configs
            .iter()
            .find(|config| config.provider == ProviderKind::Antigravity)
            .unwrap();
        database
            .create_turn(
                "turn-claude-reload",
                &debate.id,
                1,
                claude,
                TurnState::Valid,
                None,
            )
            .unwrap();
        database
            .create_turn(
                "turn-antigravity-reload",
                &debate.id,
                1,
                antigravity,
                TurnState::Failed,
                None,
            )
            .unwrap();
        let failure = FailureType::ProcessError;
        database
            .save_attempt(
                "attempt-antigravity-reload",
                "turn-antigravity-reload",
                1,
                TurnState::Failed,
                Some(&failure),
                None,
            )
            .unwrap();

        let statuses = database.latest_turn_statuses(&debate.id).unwrap();
        assert_eq!(statuses.len(), 2);
        let antigravity_status = statuses
            .iter()
            .find(|status| status.provider == ProviderKind::Antigravity)
            .unwrap();
        assert_eq!(antigravity_status.round, 1);
        assert_eq!(antigravity_status.state, TurnState::Failed);
        assert_eq!(antigravity_status.attempts, 1);
        assert_eq!(antigravity_status.failure_type, Some(failure));
        assert_eq!(antigravity_status.requested_model, "gemini-3.7-flash-low");
        assert_eq!(antigravity_status.requested_reasoning_effort, "low");
    }

    #[test]
    fn dispatch_intents_are_idempotent_and_recovery_is_fail_closed() {
        let database = Database::in_memory().unwrap();
        let models = BTreeMap::from([(
            ProviderKind::Claude,
            ModelSelection::requested("test-model"),
        )]);
        let debate = Debate::new(Intake::default(), models);
        database.create_debate(&debate).unwrap();
        let config = ProviderConfig::defaults()
            .into_iter()
            .find(|config| config.provider == ProviderKind::Claude)
            .unwrap();
        database
            .create_turn(
                "turn-intent",
                &debate.id,
                1,
                &config,
                TurnState::Pending,
                None,
            )
            .unwrap();
        database
            .create_dispatch_intent(
                "call-intent",
                &debate.id,
                "turn-intent",
                1,
                &ProviderKind::Claude,
                1,
            )
            .unwrap();
        assert_eq!(
            database.dispatch_status("call-intent").unwrap().as_deref(),
            Some("NOT_DISPATCHED")
        );
        database.mark_dispatch_running("call-intent").unwrap();
        assert_eq!(database.recover_inflight_dispatches().unwrap(), 1);
        assert_eq!(
            database.dispatch_status("call-intent").unwrap().as_deref(),
            Some("RUNNING_UNKNOWN")
        );
        assert_eq!(
            database.load_debate(&debate.id).unwrap().state,
            DebateState::Paused
        );
        assert_eq!(
            database
                .transition_debate(&debate.id, DebateEvent::Resume)
                .unwrap(),
            DebateState::Ready
        );
        database
            .mark_dispatch_complete("call-intent", Some("raw-intent"), "COMPLETED")
            .unwrap();
        database
            .create_dispatch_intent(
                "call-intent",
                &debate.id,
                "turn-intent",
                1,
                &ProviderKind::Claude,
                1,
            )
            .unwrap();
        assert_eq!(
            database.dispatch_status("call-intent").unwrap().as_deref(),
            Some("COMPLETED")
        );
    }

    #[test]
    fn settings_and_exports_are_idempotent_but_not_overwritable() {
        let database = Database::in_memory().unwrap();
        database
            .save_app_setting("export_directory", "C:/council/exports")
            .unwrap();
        assert_eq!(
            database
                .load_app_setting("export_directory")
                .unwrap()
                .as_deref(),
            Some("C:/council/exports")
        );
        let models = BTreeMap::from([(
            ProviderKind::Claude,
            ModelSelection::requested("test-model"),
        )]);
        let debate = Debate::new(Intake::default(), models);
        database.create_debate(&debate).unwrap();
        database
            .save_export(
                "export-1",
                &debate.id,
                "MASTER_PROMPT",
                Path::new("C:/council/master.md"),
                "hash-1",
            )
            .unwrap();
        database
            .save_export(
                "export-1",
                &debate.id,
                "MASTER_PROMPT",
                Path::new("C:/council/master.md"),
                "hash-1",
            )
            .unwrap();
        assert!(matches!(
            database.save_export(
                "export-1",
                &debate.id,
                "MASTER_PROMPT",
                Path::new("C:/council/master.md"),
                "different"
            ),
            Err(DatabaseError::Conflict(_))
        ));
    }
}
