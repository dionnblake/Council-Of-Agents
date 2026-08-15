use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use thiserror::Error;

use crate::model::{FailureType, ProviderConfig, ProviderKind, ServingIdentityStatus};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CommandSpec {
    pub program: PathBuf,
    pub args: Vec<String>,
    pub environment: BTreeMap<String, String>,
    pub working_directory: PathBuf,
    pub prompt_via_stdin: bool,
    pub timeout_ms: u64,
    pub kill_fallback: Option<CommandInvocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CommandInvocation {
    pub program: PathBuf,
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderCallRequest {
    pub provider: ProviderKind,
    pub model: String,
    #[serde(default)]
    pub turn_id: Option<String>,
    pub packet_path: PathBuf,
    pub packet_directory: PathBuf,
    pub schema_path: PathBuf,
    pub working_directory: PathBuf,
    pub scratch_directory: PathBuf,
    pub prompt: String,
    pub timeout_ms: u64,
    #[serde(default)]
    pub linux_packet_path: Option<PathBuf>,
    #[serde(default)]
    pub linux_working_directory: Option<PathBuf>,
    #[serde(default)]
    pub linux_schema_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderCallResult {
    pub provider: ProviderKind,
    pub exit_code: Option<i32>,
    pub wall_ms: u128,
    pub stdout: String,
    pub stderr: String,
    pub requested_model: String,
    pub reported_served_model: Option<String>,
    pub serving_identity_status: ServingIdentityStatus,
    pub failure_type: Option<FailureType>,
    pub raw_artifact_id: String,
    pub timed_out: bool,
    pub cancellation_fallback_ran: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RepairPolicy {
    NoAutomaticRepair,
    OneRepairAttempt,
    SchemaRedesignRequired,
}

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("provider is disabled: {0:?}")]
    Disabled(ProviderKind),
    #[error("provider configuration is invalid: {0}")]
    InvalidConfiguration(String),
    #[error("provider safety preflight failed: {0}")]
    SafetyPreflight(String),
    #[error("provider executable was not found: {0}")]
    ExecutableMissing(PathBuf),
    #[error("antigravity credit guard is not false")]
    AntigravityCreditGuard,
    #[error("provider prompt must remain a short packet reference")]
    PromptTooLong,
    #[error("provider process runner failed: {0}")]
    ProcessRunner(String),
}

#[derive(Debug, Clone)]
pub struct ProviderRegistry {
    configs: BTreeMap<ProviderKind, ProviderConfig>,
}

impl ProviderRegistry {
    pub fn defaults() -> Self {
        Self {
            configs: ProviderConfig::defaults()
                .into_iter()
                .map(|config| (config.provider.clone(), config))
                .collect(),
        }
    }

    pub fn from_configs(configs: Vec<ProviderConfig>) -> Self {
        Self {
            configs: configs
                .into_iter()
                .map(|config| (config.provider.clone(), config))
                .collect(),
        }
    }

    pub fn config(&self, provider: &ProviderKind) -> Option<&ProviderConfig> {
        self.configs.get(provider)
    }

    pub fn all(&self) -> impl Iterator<Item = &ProviderConfig> {
        self.configs.values()
    }

    pub fn preflight(&self, provider: &ProviderKind) -> Result<(), ProviderError> {
        let config = self
            .config(provider)
            .ok_or_else(|| ProviderError::InvalidConfiguration(provider.slug().to_string()))?;
        if !config.enabled {
            return Err(ProviderError::Disabled(provider.clone()));
        }
        if matches!(provider, ProviderKind::Antigravity) {
            let allowed = config
                .safety_settings
                .get("useG1Credits")
                .and_then(Value::as_bool);
            if allowed != Some(false) {
                return Err(ProviderError::AntigravityCreditGuard);
            }
        }
        if matches!(provider, ProviderKind::CodexWsl) {
            if config.wsl_distribution.as_deref() != Some("CouncilCodexWSL")
                || config.wsl_user.as_deref() != Some("council")
                || config.wsl_home.as_deref() != Some("/home/council")
                || config.codex_home.as_deref() != Some("/home/council/.codex")
            {
                return Err(ProviderError::SafetyPreflight(
                    "Codex WSL configuration does not match the certified boundary".to_string(),
                ));
            }
        }
        Ok(())
    }

    pub fn build_command(
        &self,
        request: &ProviderCallRequest,
    ) -> Result<CommandSpec, ProviderError> {
        self.preflight(&request.provider)?;
        let config = self.config(&request.provider).ok_or_else(|| {
            ProviderError::InvalidConfiguration(request.provider.slug().to_string())
        })?;
        match request.provider {
            ProviderKind::CodexWsl => build_codex_command(config, request),
            ProviderKind::Claude => build_claude_command(config, request),
            ProviderKind::Antigravity => build_antigravity_command(config, request),
        }
    }
}

pub fn repair_policy_for(provider: &ProviderKind) -> RepairPolicy {
    match provider {
        ProviderKind::Claude | ProviderKind::CodexWsl => RepairPolicy::NoAutomaticRepair,
        ProviderKind::Antigravity => RepairPolicy::OneRepairAttempt,
    }
}

pub const BLOCKED_BILLING_ENVIRONMENT_KEYS: &[&str] = &[
    "OPENAI_API_KEY",
    "OPENAI_BASE_URL",
    "OPENAI_API_BASE",
    "OPENAI_ORG_ID",
    "OPENAI_PROJECT_ID",
    "ANTHROPIC_API_KEY",
    "ANTHROPIC_BASE_URL",
    "GOOGLE_API_KEY",
    "GEMINI_API_KEY",
];

pub fn billing_environment_status() -> BTreeMap<String, bool> {
    BLOCKED_BILLING_ENVIRONMENT_KEYS
        .iter()
        .map(|key| ((*key).to_string(), std::env::var_os(key).is_some()))
        .collect()
}

pub fn ensure_subscription_environment() -> Result<(), ProviderError> {
    if let Some(key) = BLOCKED_BILLING_ENVIRONMENT_KEYS
        .iter()
        .find(|key| std::env::var_os(key).is_some())
    {
        return Err(ProviderError::SafetyPreflight(format!(
            "{key} is present; account-based subscription execution is required"
        )));
    }
    Ok(())
}

fn build_codex_command(
    config: &ProviderConfig,
    request: &ProviderCallRequest,
) -> Result<CommandSpec, ProviderError> {
    let distribution = config.wsl_distribution.as_deref().ok_or_else(|| {
        ProviderError::InvalidConfiguration("missing WSL distribution".to_string())
    })?;
    let user = config
        .wsl_user
        .as_deref()
        .ok_or_else(|| ProviderError::InvalidConfiguration("missing WSL user".to_string()))?;
    let home = config
        .wsl_home
        .as_deref()
        .ok_or_else(|| ProviderError::InvalidConfiguration("missing WSL home".to_string()))?;
    let codex_home = config
        .codex_home
        .as_deref()
        .ok_or_else(|| ProviderError::InvalidConfiguration("missing CODEX_HOME".to_string()))?;
    let linux_working_directory = request.linux_working_directory.as_deref().ok_or_else(|| {
        ProviderError::InvalidConfiguration("missing Linux working directory".to_string())
    })?;
    let linux_packet_path = request.linux_packet_path.as_deref().ok_or_else(|| {
        ProviderError::InvalidConfiguration("missing Linux packet path".to_string())
    })?;
    let linux_schema = request.linux_schema_path.as_deref().ok_or_else(|| {
        ProviderError::InvalidConfiguration("missing Linux schema path".to_string())
    })?;
    if !request.prompt.contains(&linux_path(linux_packet_path)) {
        return Err(ProviderError::InvalidConfiguration(
            "Codex prompt must explicitly reference the Linux packet path".to_string(),
        ));
    }
    let mut args = vec![
        "-d".to_string(),
        distribution.to_string(),
        "--user".to_string(),
        user.to_string(),
        "--".to_string(),
        "env".to_string(),
        "-i".to_string(),
        format!("HOME={home}"),
        format!("CODEX_HOME={codex_home}"),
        "USER=council".to_string(),
        "LOGNAME=council".to_string(),
        "SHELL=/bin/bash".to_string(),
        "LANG=C.UTF-8".to_string(),
        "TERM=dumb".to_string(),
        "PATH=/home/council/.local/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin".to_string(),
        "codex".to_string(),
        "exec".to_string(),
        "--strict-config".to_string(),
        "--json".to_string(),
        "--ephemeral".to_string(),
        "--skip-git-repo-check".to_string(),
        "--sandbox".to_string(),
        "read-only".to_string(),
        "-C".to_string(),
        linux_path(linux_working_directory),
        "-m".to_string(),
        request.model.clone(),
        "--output-schema".to_string(),
        linux_path(linux_schema),
        "-".to_string(),
    ];
    args.extend(config.extra_args.clone());
    Ok(CommandSpec {
        program: config.executable.clone(),
        args,
        environment: BTreeMap::new(),
        working_directory: request.working_directory.clone(),
        prompt_via_stdin: true,
        timeout_ms: request.timeout_ms,
        kill_fallback: Some(CommandInvocation {
            program: PathBuf::from("wsl.exe"),
            args: vec!["--terminate".to_string(), distribution.to_string()],
        }),
    })
}

fn build_claude_command(
    config: &ProviderConfig,
    request: &ProviderCallRequest,
) -> Result<CommandSpec, ProviderError> {
    let config_dir = config.config_dir.as_ref().ok_or_else(|| {
        ProviderError::InvalidConfiguration("missing Claude config directory".to_string())
    })?;
    let mut args = vec![
        "--print".to_string(),
        "--no-session-persistence".to_string(),
        "--setting-sources".to_string(),
        "local".to_string(),
        "--output-format".to_string(),
        "json".to_string(),
        "--json-schema".to_string(),
        request.schema_path.to_string_lossy().to_string(),
        "--model".to_string(),
        request.model.clone(),
    ];
    args.extend(config.extra_args.clone());
    let mut environment = safe_host_environment();
    environment.insert(
        "CLAUDE_CONFIG_DIR".to_string(),
        config_dir.to_string_lossy().to_string(),
    );
    Ok(CommandSpec {
        program: config.executable.clone(),
        args,
        environment,
        working_directory: request.working_directory.clone(),
        prompt_via_stdin: true,
        timeout_ms: request.timeout_ms,
        kill_fallback: None,
    })
}

fn build_antigravity_command(
    config: &ProviderConfig,
    request: &ProviderCallRequest,
) -> Result<CommandSpec, ProviderError> {
    if request.prompt.len() > 2_000 {
        return Err(ProviderError::PromptTooLong);
    }
    let mut args = vec![
        "-p".to_string(),
        request.prompt.clone(),
        "--model".to_string(),
        request.model.clone(),
        "--output-format".to_string(),
        "json".to_string(),
        "--json-schema".to_string(),
        request.schema_path.to_string_lossy().to_string(),
    ];
    args.extend(config.extra_args.clone());
    let mut environment = safe_host_environment();
    environment.extend(
        config.safety_settings.iter().filter_map(|(key, value)| {
            value.as_str().map(|value| (key.clone(), value.to_string()))
        }),
    );
    Ok(CommandSpec {
        program: config.executable.clone(),
        args,
        environment,
        working_directory: request.working_directory.clone(),
        prompt_via_stdin: false,
        timeout_ms: request.timeout_ms,
        kill_fallback: None,
    })
}

fn linux_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn safe_host_environment() -> BTreeMap<String, String> {
    const ALLOWED: &[&str] = &[
        "APPDATA",
        "ComSpec",
        "LOCALAPPDATA",
        "Path",
        "PATHEXT",
        "PROCESSOR_ARCHITECTURE",
        "SystemRoot",
        "TEMP",
        "TMP",
        "USERPROFILE",
        "WINDIR",
    ];
    ALLOWED
        .iter()
        .filter_map(|key| {
            std::env::var(key)
                .ok()
                .map(|value| ((*key).to_string(), value))
        })
        .collect()
}

pub fn classify_failure(stdout: &str, stderr: &str, exit_code: Option<i32>) -> Option<FailureType> {
    let combined = format!("{stdout}\n{stderr}").to_ascii_lowercase();
    if combined.contains("not logged in")
        || combined.contains("login required")
        || combined.contains("authentication required")
        || combined.contains("401")
    {
        return Some(FailureType::AuthRequired);
    }
    if combined.contains("session limit")
        || combined.contains("rate limit")
        || combined.contains("quota")
        || combined.contains("429")
    {
        return Some(FailureType::ProviderLimit);
    }
    if combined.contains("schema") && combined.contains("invalid") {
        return Some(FailureType::SchemaInvalid);
    }
    if exit_code.is_some_and(|code| code != 0) {
        return Some(FailureType::ProcessError);
    }
    None
}

pub fn serving_identity_from_jsonl(
    stdout: &str,
    requested_model: &str,
) -> (Option<String>, ServingIdentityStatus) {
    let mut served = None;
    for line in stdout.lines() {
        let Ok(value) = serde_json::from_str::<Value>(line) else {
            continue;
        };
        find_model_field(&value, &mut served);
    }
    match served {
        Some(model) if model == requested_model => {
            (Some(model), ServingIdentityStatus::VerifiedMatch)
        }
        Some(model) => (Some(model), ServingIdentityStatus::VerifiedMismatch),
        None => (None, ServingIdentityStatus::ProviderDoesNotReport),
    }
}

fn find_model_field(value: &Value, served: &mut Option<String>) {
    match value {
        Value::Object(map) => {
            for (key, child) in map {
                if matches!(
                    key.as_str(),
                    "served_model" | "servedModel" | "model_id" | "modelId"
                ) && child.is_string()
                {
                    *served = child.as_str().map(str::to_string);
                }
                find_model_field(child, served);
            }
        }
        Value::Array(values) => {
            for child in values {
                find_model_field(child, served);
            }
        }
        _ => {}
    }
}

pub fn antigravity_credit_guard_from_json(path: &Path) -> Result<(), ProviderError> {
    let bytes = std::fs::read(path).map_err(|error| {
        ProviderError::SafetyPreflight(format!("cannot read Antigravity settings: {error}"))
    })?;
    let value: Value = serde_json::from_slice(&bytes).map_err(|error| {
        ProviderError::SafetyPreflight(format!("invalid Antigravity settings JSON: {error}"))
    })?;
    if value.get("useG1Credits").and_then(Value::as_bool) == Some(false) {
        Ok(())
    } else {
        Err(ProviderError::AntigravityCreditGuard)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codex_command_has_isolation_flags_and_no_parent_environment() {
        let registry = ProviderRegistry::defaults();
        let request = ProviderCallRequest {
            provider: ProviderKind::CodexWsl,
            model: "gpt-5.6-luna".to_string(),
            turn_id: None,
            packet_path: PathBuf::from("C:\\council\\packet\\one.md"),
            packet_directory: PathBuf::from("C:\\council\\packet"),
            schema_path: PathBuf::from("C:\\council\\schema.json"),
            working_directory: PathBuf::from("C:\\council\\scratch"),
            scratch_directory: PathBuf::from("C:\\council\\scratch"),
            prompt: "Read the immutable packet at /home/council/council/packet/one.md".to_string(),
            timeout_ms: 180_000,
            linux_packet_path: Some(PathBuf::from("/home/council/council/packet/one.md")),
            linux_working_directory: Some(PathBuf::from("/home/council/council/scratch")),
            linux_schema_path: Some(PathBuf::from("/home/council/council/schema.json")),
        };
        let command = registry.build_command(&request).unwrap();
        assert_eq!(command.program, PathBuf::from("wsl.exe"));
        assert!(command.args.contains(&"CouncilCodexWSL".to_string()));
        assert!(command.args.contains(&"read-only".to_string()));
        assert!(command.args.contains(&"--ephemeral".to_string()));
        assert!(!command.environment.contains_key("OPENAI_API_KEY"));
        assert!(command.kill_fallback.is_some());
    }

    #[test]
    fn antigravity_guard_fails_closed() {
        let config = ProviderConfig::defaults()
            .into_iter()
            .find(|config| config.provider == ProviderKind::Antigravity)
            .unwrap();
        assert_eq!(
            config
                .safety_settings
                .get("useG1Credits")
                .and_then(Value::as_bool),
            Some(false)
        );
    }

    #[test]
    fn model_identity_does_not_invent_served_model() {
        let (served, status) = serving_identity_from_jsonl(
            r#"{"type":"turn.completed","usage":{"input_tokens":10}}"#,
            "gpt-5.6-luna",
        );
        assert!(served.is_none());
        assert_eq!(status, ServingIdentityStatus::ProviderDoesNotReport);
    }
}
