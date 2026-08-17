use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use thiserror::Error;

use crate::model::{
    CERTIFICATION_BOUNDARY_VERSION, ExactConfigurationStatus, FailureType, ProviderConfig,
    ProviderKind, ServingIdentityStatus, supported_reasoning_efforts_for_model,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CommandSpec {
    pub program: PathBuf,
    pub args: Vec<String>,
    pub environment: BTreeMap<String, String>,
    pub working_directory: PathBuf,
    pub prompt_via_stdin: bool,
    pub windows_job_containment: bool,
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
    pub reasoning_effort: String,
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
    #[serde(default)]
    pub snapshot_path: Option<PathBuf>,
    #[serde(default)]
    pub linux_snapshot_path: Option<PathBuf>,
    #[serde(default)]
    pub snapshot_manifest_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderCallResult {
    pub provider: ProviderKind,
    pub exit_code: Option<i32>,
    pub wall_ms: u128,
    pub stdout: String,
    pub stderr: String,
    pub requested_model: String,
    #[serde(default)]
    pub requested_reasoning_effort: String,
    pub reported_served_model: Option<String>,
    pub serving_identity_status: ServingIdentityStatus,
    #[serde(default)]
    pub exact_configuration_status: ExactConfigurationStatus,
    #[serde(default)]
    pub exact_configuration_evidence: Option<String>,
    #[serde(default = "default_certification_boundary")]
    pub certification_boundary: String,
    pub failure_type: Option<FailureType>,
    pub raw_artifact_id: String,
    pub timed_out: bool,
    pub cancellation_fallback_ran: bool,
}

fn default_certification_boundary() -> String {
    CERTIFICATION_BOUNDARY_VERSION.to_string()
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
                .map(|mut config| {
                    config.normalize_defaults();
                    (config.provider.clone(), config)
                })
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
            if let Some(path) = config.safety_config_path.as_deref() {
                antigravity_credit_guard_from_json(path)?;
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

    pub fn validate_subscription_routing(
        &self,
        provider: &ProviderKind,
    ) -> Result<(), ProviderError> {
        self.preflight(provider)?;
        let config = self
            .config(provider)
            .ok_or_else(|| ProviderError::InvalidConfiguration(provider.slug().to_string()))?;
        validate_subscription_configuration(config)
    }

    pub fn validate_all_subscription_routing(&self) -> Result<(), ProviderError> {
        for config in self.all() {
            self.validate_subscription_routing(&config.provider)?;
        }
        Ok(())
    }

    pub fn build_command(
        &self,
        request: &ProviderCallRequest,
    ) -> Result<CommandSpec, ProviderError> {
        self.validate_subscription_routing(&request.provider)?;
        validate_reasoning_effort(&request.provider, &request.model, &request.reasoning_effort)?;
        let config = self.config(&request.provider).ok_or_else(|| {
            ProviderError::InvalidConfiguration(request.provider.slug().to_string())
        })?;
        let command = match request.provider {
            ProviderKind::CodexWsl => build_codex_command(config, request),
            ProviderKind::Claude => build_claude_command(config, request),
            ProviderKind::Antigravity => build_antigravity_command(config, request),
        }?;
        validate_provider_environment(&command.environment)?;
        Ok(command)
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
    "ANTHROPIC_API_BASE",
    "GOOGLE_API_KEY",
    "GEMINI_API_KEY",
    "GOOGLE_APPLICATION_CREDENTIALS",
    "GOOGLE_CLOUD_PROJECT",
    "GOOGLE_GENAI_USE_VERTEXAI",
    "VERTEXAI_PROJECT",
    "VERTEXAI_LOCATION",
    "AZURE_OPENAI_API_KEY",
    "AZURE_OPENAI_ENDPOINT",
    "AZURE_OPENAI_API_VERSION",
    "API_KEY",
    "BASE_URL",
    "API_BASE_URL",
    "CUSTOM_BASE_URL",
];

const BLOCKED_PROVIDER_ROUTING_ARGUMENTS: &[&str] = &[
    "--api-key",
    "--api_key",
    "--base-url",
    "--base_url",
    "--api-base",
    "--api_base",
    "--endpoint",
];

pub fn billing_environment_status() -> BTreeMap<String, bool> {
    BLOCKED_BILLING_ENVIRONMENT_KEYS
        .iter()
        .map(|key| ((*key).to_string(), std::env::var_os(key).is_some()))
        .collect()
}

pub fn validate_provider_environment(
    environment: &BTreeMap<String, String>,
) -> Result<(), ProviderError> {
    if let Some(key) = environment.keys().find(|key| {
        BLOCKED_BILLING_ENVIRONMENT_KEYS
            .iter()
            .any(|blocked| key.eq_ignore_ascii_case(blocked))
    }) {
        return Err(ProviderError::SafetyPreflight(format!(
            "prohibited provider environment variable {key} was injected; subscription routing requires an explicit environment allowlist"
        )));
    }
    Ok(())
}

pub fn validate_provider_extra_args(extra_args: &[String]) -> Result<(), ProviderError> {
    for argument in extra_args {
        let normalized = argument.to_ascii_lowercase();
        if BLOCKED_PROVIDER_ROUTING_ARGUMENTS
            .iter()
            .any(|flag| normalized == *flag || normalized.starts_with(&format!("{flag}=")))
            || normalized.contains("api_key=")
            || normalized.contains("api-key=")
            || normalized.contains("base_url=")
            || normalized.contains("base-url=")
        {
            return Err(ProviderError::SafetyPreflight(
                "provider configuration contains a prohibited API-key or custom-routing argument"
                    .to_string(),
            ));
        }
    }
    Ok(())
}

pub fn validate_reasoning_effort(
    provider: &ProviderKind,
    model: &str,
    reasoning_effort: &str,
) -> Result<(), ProviderError> {
    if supported_reasoning_efforts_for_model(provider, model).contains(&reasoning_effort) {
        Ok(())
    } else {
        Err(ProviderError::InvalidConfiguration(format!(
            "{} model {} does not support reasoning level {:?}; supported levels: {}",
            provider.slug(),
            model,
            reasoning_effort,
            supported_reasoning_efforts_for_model(provider, model).join(", ")
        )))
    }
}

pub fn validate_subscription_configuration(config: &ProviderConfig) -> Result<(), ProviderError> {
    validate_provider_extra_args(&config.extra_args)?;
    validate_provider_environment(&effective_provider_environment(config))
}

pub fn ensure_subscription_environment() -> Result<(), ProviderError> {
    validate_provider_environment(&safe_host_environment())
}

fn effective_provider_environment(config: &ProviderConfig) -> BTreeMap<String, String> {
    let mut environment = safe_host_environment();
    match config.provider {
        ProviderKind::Claude => {
            if let Some(config_dir) = config.config_dir.as_deref() {
                environment.insert(
                    "CLAUDE_CONFIG_DIR".to_string(),
                    config_dir.to_string_lossy().to_string(),
                );
            }
        }
        ProviderKind::Antigravity => {
            environment.extend(config.safety_settings.iter().filter_map(|(key, value)| {
                value.as_str().map(|value| (key.clone(), value.to_string()))
            }));
        }
        ProviderKind::CodexWsl => {}
    }
    environment
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
    if let Some(snapshot_path) = request.linux_snapshot_path.as_deref()
        && !request.prompt.contains(&linux_path(snapshot_path))
    {
        return Err(ProviderError::InvalidConfiguration(
            "Codex prompt must explicitly reference the Linux snapshot path".to_string(),
        ));
    }
    let mut wsl_args = vec![
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
        "-c".to_string(),
        format!("model_reasoning_effort=\"{}\"", request.reasoning_effort),
        "--output-schema".to_string(),
        linux_path(linux_schema),
        "-".to_string(),
    ];
    wsl_args.extend(config.extra_args.clone());
    let command_line = std::iter::once("wsl.exe".to_string())
        .chain(
            wsl_args
                .iter()
                .map(|argument| cmd_token(argument))
                .collect::<Result<Vec<_>, _>>()?,
        )
        .collect::<Vec<_>>()
        .join(" ");
    Ok(CommandSpec {
        program: PathBuf::from("cmd.exe"),
        args: vec![
            "/d".to_string(),
            "/s".to_string(),
            "/c".to_string(),
            command_line,
        ],
        environment: effective_provider_environment(config),
        working_directory: request.working_directory.clone(),
        prompt_via_stdin: true,
        windows_job_containment: false,
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
    if config.config_dir.is_none() {
        return Err(ProviderError::InvalidConfiguration(
            "missing Claude config directory".to_string(),
        ));
    }
    let schema_text = std::fs::read_to_string(&request.schema_path).map_err(|error| {
        ProviderError::InvalidConfiguration(format!(
            "cannot read Claude output schema {}: {error}",
            request.schema_path.display()
        ))
    })?;
    let mut schema_value: Value = serde_json::from_str(&schema_text).map_err(|error| {
        ProviderError::InvalidConfiguration(format!(
            "invalid Claude output schema {}: {error}",
            request.schema_path.display()
        ))
    })?;
    if let Some(object) = schema_value.as_object_mut() {
        object.remove("$schema");
    }
    let schema_text = serde_json::to_string(&schema_value).map_err(|error| {
        ProviderError::InvalidConfiguration(format!(
            "cannot serialize Claude output schema {}: {error}",
            request.schema_path.display()
        ))
    })?;
    let mut args = vec![
        "--print".to_string(),
        "--no-session-persistence".to_string(),
        "--setting-sources".to_string(),
        "local".to_string(),
        "--permission-mode".to_string(),
        "plan".to_string(),
        "--output-format".to_string(),
        "json".to_string(),
        "--json-schema".to_string(),
        schema_text,
        "--model".to_string(),
        request.model.clone(),
        "--effort".to_string(),
        request.reasoning_effort.clone(),
        "--add-dir".to_string(),
        request.packet_directory.to_string_lossy().to_string(),
    ];
    if let Some(snapshot_path) = request.snapshot_path.as_deref() {
        args.push("--add-dir".to_string());
        args.push(snapshot_path.to_string_lossy().to_string());
    }
    args.extend(config.extra_args.clone());
    let environment = effective_provider_environment(config);
    Ok(CommandSpec {
        program: config.executable.clone(),
        args,
        environment,
        working_directory: request.working_directory.clone(),
        prompt_via_stdin: true,
        windows_job_containment: true,
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
        "--effort".to_string(),
        request.reasoning_effort.clone(),
        "--output-format".to_string(),
        "json".to_string(),
        "--json-schema".to_string(),
        request.schema_path.to_string_lossy().to_string(),
        "--add-dir".to_string(),
        request.packet_directory.to_string_lossy().to_string(),
    ];
    if let Some(snapshot_path) = request.snapshot_path.as_deref() {
        args.push("--add-dir".to_string());
        args.push(snapshot_path.to_string_lossy().to_string());
    }
    args.extend(config.extra_args.clone());
    let environment = effective_provider_environment(config);
    Ok(CommandSpec {
        program: config.executable.clone(),
        args,
        environment,
        working_directory: request.working_directory.clone(),
        prompt_via_stdin: false,
        windows_job_containment: true,
        timeout_ms: request.timeout_ms,
        kill_fallback: None,
    })
}

fn linux_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn cmd_token(value: &str) -> Result<String, ProviderError> {
    if value.chars().any(|character| {
        character.is_whitespace() || matches!(character, '&' | '|' | '<' | '>' | '^' | '%' | '!')
    }) {
        return Err(ProviderError::InvalidConfiguration(
            "Codex command argument contains unsafe cmd.exe syntax".to_string(),
        ));
    }
    Ok(value.to_string())
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
    let ambient = std::env::vars().collect::<BTreeMap<_, _>>();
    safe_host_environment_from(ALLOWED, &ambient)
}

fn safe_host_environment_from(
    allowed: &[&str],
    ambient: &BTreeMap<String, String>,
) -> BTreeMap<String, String> {
    allowed
        .iter()
        .filter(|key| {
            !BLOCKED_BILLING_ENVIRONMENT_KEYS
                .iter()
                .any(|blocked| key.eq_ignore_ascii_case(blocked))
        })
        .filter_map(|key| {
            ambient
                .get(*key)
                .or_else(|| {
                    ambient
                        .iter()
                        .find(|(actual, _)| actual.eq_ignore_ascii_case(key))
                        .map(|(_, value)| value)
                })
                .map(|value| ((*key).to_string(), value.clone()))
        })
        .collect()
}

pub fn classify_failure(stdout: &str, stderr: &str, exit_code: Option<i32>) -> Option<FailureType> {
    let mut diagnostics = stderr.to_string();
    for line in stdout.lines() {
        if let Ok(value) = serde_json::from_str::<Value>(line) {
            collect_structured_error(&value, &mut diagnostics);
        }
    }
    let combined = diagnostics.to_ascii_lowercase();
    if combined.contains("not logged in")
        || combined.contains("login required")
        || combined.contains("authentication required")
        || contains_standalone_401(&combined)
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

fn collect_structured_error(value: &Value, output: &mut String) {
    match value {
        Value::Object(map) => {
            let is_error = map
                .get("type")
                .and_then(Value::as_str)
                .is_some_and(|kind| kind.eq_ignore_ascii_case("error"))
                || map.get("is_error").and_then(Value::as_bool) == Some(true)
                || map.get("error").is_some_and(|error| !error.is_null());
            if is_error {
                output.push_str(&value.to_string());
                output.push('\n');
            }
            for child in map.values() {
                collect_structured_error(child, output);
            }
        }
        Value::Array(values) => {
            for child in values {
                collect_structured_error(child, output);
            }
        }
        _ => {}
    }
}

fn contains_standalone_401(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.windows(3).enumerate().any(|(index, window)| {
        if window != b"401" {
            return false;
        }
        let before = index
            .checked_sub(1)
            .and_then(|position| bytes.get(position))
            .copied()
            .unwrap_or(b' ');
        let after = bytes.get(index + 3).copied().unwrap_or(b' ');
        !before.is_ascii_hexdigit() && !after.is_ascii_hexdigit()
    })
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
            reasoning_effort: "max".to_string(),
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
            snapshot_path: None,
            linux_snapshot_path: None,
            snapshot_manifest_hash: None,
        };
        let command = registry.build_command(&request).unwrap();
        assert_eq!(command.program, PathBuf::from("cmd.exe"));
        assert_eq!(command.args[0], "/d");
        assert!(command.args[3].contains("wsl.exe"));
        assert!(command.args[3].contains("CouncilCodexWSL"));
        assert!(command.args[3].contains("read-only"));
        assert!(command.args[3].contains("--ephemeral"));
        assert!(command.args[3].contains("model_reasoning_effort=\"max\""));
        assert!(command.args[3].ends_with(" -"));
        assert!(command.prompt_via_stdin);
        assert!(!command.windows_job_containment);
        for key in BLOCKED_BILLING_ENVIRONMENT_KEYS {
            assert!(!command.environment.contains_key(*key), "{key}");
        }
        assert!(command.kill_fallback.is_some());
    }

    #[test]
    fn ambient_provider_credentials_are_filtered_before_command_creation() {
        let ambient = BTreeMap::from([
            ("OPENAI_API_KEY".to_string(), "host-openai".to_string()),
            (
                "ANTHROPIC_API_KEY".to_string(),
                "host-anthropic".to_string(),
            ),
            ("GEMINI_API_KEY".to_string(), "host-gemini".to_string()),
            (
                "OPENAI_BASE_URL".to_string(),
                "https://custom.invalid".to_string(),
            ),
            ("Path".to_string(), "C:\\Windows\\System32".to_string()),
        ]);
        let effective = safe_host_environment_from(
            &[
                "Path",
                "OPENAI_API_KEY",
                "ANTHROPIC_API_KEY",
                "GEMINI_API_KEY",
            ],
            &ambient,
        );
        assert_eq!(
            effective.get("Path"),
            Some(&"C:\\Windows\\System32".to_string())
        );
        for key in [
            "OPENAI_API_KEY",
            "ANTHROPIC_API_KEY",
            "GEMINI_API_KEY",
            "OPENAI_BASE_URL",
        ] {
            assert!(!effective.contains_key(key), "{key}");
        }

        let registry = ProviderRegistry::defaults();
        for config in registry.all() {
            validate_provider_environment(&effective_provider_environment(config)).unwrap();
        }
    }

    #[test]
    fn every_provider_command_spec_excludes_credentials_and_custom_routes() {
        let tempdir = tempfile::tempdir().unwrap();
        let schema_path = tempdir.path().join("position.schema.json");
        std::fs::write(&schema_path, "{}").unwrap();
        let base_request = ProviderCallRequest {
            provider: ProviderKind::Claude,
            model: "test-model".to_string(),
            reasoning_effort: "high".to_string(),
            turn_id: None,
            packet_path: PathBuf::from("C:\\council\\packet\\one.md"),
            packet_directory: PathBuf::from("C:\\council\\packet"),
            schema_path,
            working_directory: PathBuf::from("C:\\council\\scratch"),
            scratch_directory: PathBuf::from("C:\\council\\scratch"),
            prompt: "Read the immutable packet at /home/council/council/packet/one.md".to_string(),
            timeout_ms: 180_000,
            linux_packet_path: Some(PathBuf::from("/home/council/council/packet/one.md")),
            linux_working_directory: Some(PathBuf::from("/home/council/council/scratch")),
            linux_schema_path: Some(PathBuf::from("/home/council/council/schema.json")),
            snapshot_path: None,
            linux_snapshot_path: None,
            snapshot_manifest_hash: None,
        };
        let registry = ProviderRegistry::defaults();
        for provider in [
            ProviderKind::Claude,
            ProviderKind::Antigravity,
            ProviderKind::CodexWsl,
        ] {
            let mut request = base_request.clone();
            request.provider = provider.clone();
            let command = registry.build_command(&request).unwrap();
            match provider {
                ProviderKind::Claude | ProviderKind::Antigravity => {
                    assert!(
                        command
                            .args
                            .windows(2)
                            .any(|pair| { pair[0] == "--effort" && pair[1] == "high" })
                    );
                }
                ProviderKind::CodexWsl => {
                    assert!(command.args[3].contains("model_reasoning_effort=\"high\""));
                }
            }
            for key in BLOCKED_BILLING_ENVIRONMENT_KEYS {
                assert!(!command.environment.contains_key(*key), "{key}");
            }
            validate_provider_environment(&command.environment).unwrap();
        }
    }

    #[test]
    fn reasoning_effort_validation_is_provider_specific() {
        validate_reasoning_effort(&ProviderKind::Claude, "sonnet", "max").unwrap();
        validate_reasoning_effort(&ProviderKind::CodexWsl, "gpt-5.6-luna", "xhigh").unwrap();
        validate_reasoning_effort(&ProviderKind::CodexWsl, "gpt-5.6-sol", "ultra").unwrap();
        validate_reasoning_effort(&ProviderKind::Antigravity, "gemini-3.7-flash-low", "high")
            .unwrap();
        assert!(
            validate_reasoning_effort(&ProviderKind::CodexWsl, "gpt-5.6-luna", "ultra").is_err()
        );
        assert!(
            validate_reasoning_effort(&ProviderKind::Antigravity, "gemini-3.7-flash-low", "max")
                .is_err()
        );
        assert!(validate_reasoning_effort(&ProviderKind::Claude, "sonnet", "unlimited").is_err());
    }

    #[test]
    fn prohibited_provider_environment_injection_fails_closed() {
        for key in [
            "OPENAI_API_KEY",
            "ANTHROPIC_BASE_URL",
            "GEMINI_API_KEY",
            "CUSTOM_BASE_URL",
        ] {
            let environment = BTreeMap::from([(key.to_string(), "redacted".to_string())]);
            assert!(
                validate_provider_environment(&environment).is_err(),
                "{key}"
            );
        }
    }

    #[test]
    fn prohibited_custom_routing_arguments_fail_closed() {
        for argument in [
            "--base-url=https://custom.invalid".to_string(),
            "--api-key".to_string(),
            "api_key=redacted".to_string(),
        ] {
            assert!(validate_provider_extra_args(&[argument]).is_err());
        }
        assert!(validate_provider_extra_args(&["--strict-config".to_string()]).is_ok());
    }

    #[test]
    fn default_subscription_routing_is_safe_for_all_seats() {
        ProviderRegistry::defaults()
            .validate_all_subscription_routing()
            .unwrap();
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

    #[test]
    fn failure_classifier_ignores_model_text_and_hash_substrings() {
        let output = r#"{"is_error":false,"result":"Schema evolution may invalidate a hypothesis","hash":"a401e9"}"#;
        assert_eq!(classify_failure(output, "", Some(0)), None);
        assert_eq!(
            classify_failure(
                r#"{"type":"error","message":"authentication required"}"#,
                "",
                Some(0)
            ),
            Some(FailureType::AuthRequired)
        );
        assert_eq!(
            classify_failure(r#"{"type":"error","message":"HTTP 401"}"#, "", Some(0)),
            Some(FailureType::AuthRequired)
        );
    }
}
