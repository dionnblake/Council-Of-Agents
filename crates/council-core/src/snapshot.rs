use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Component, Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SnapshotExclusionReason {
    GitMetadata,
    GitIgnored,
    AgentConfig,
    Secret,
    ReparsePoint,
    Symlink,
    Unsupported,
    OutsideSource,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SnapshotExclusion {
    pub relative_path: String,
    pub reason: SnapshotExclusionReason,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SnapshotFile {
    pub relative_path: String,
    pub size: u64,
    pub sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SnapshotManifest {
    pub snapshot_id: String,
    pub source_root: String,
    pub created_at: DateTime<Utc>,
    pub files: Vec<SnapshotFile>,
    pub exclusions: Vec<SnapshotExclusion>,
    pub manifest_sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SnapshotReviewDecision {
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SnapshotReviewExclusion {
    pub relative_path: String,
    pub reason: SnapshotExclusionReason,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SnapshotReviewRecord {
    pub id: String,
    pub debate_id: String,
    pub snapshot_id: String,
    pub manifest_hash: String,
    pub exclusion_set_hash: String,
    pub source_tree_hash: String,
    pub secret_exclusion_count: u32,
    pub exclusions: Vec<SnapshotReviewExclusion>,
    pub decision: SnapshotReviewDecision,
    pub rationale: Option<String>,
    pub created_at: DateTime<Utc>,
    pub reviewed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct SnapshotRequest {
    pub source_root: PathBuf,
    pub destination_root: PathBuf,
    pub snapshot_id: String,
}

#[derive(Debug, Error)]
pub enum SnapshotError {
    #[error("source root does not exist: {0}")]
    MissingSource(PathBuf),
    #[error("destination is inside source root: {0}")]
    DestinationInsideSource(PathBuf),
    #[error("filesystem error at {path}: {source}")]
    Filesystem {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("manifest serialization failed: {0}")]
    ManifestSerialization(#[from] serde_json::Error),
    #[error("snapshot destination already exists: {0}")]
    DestinationExists(PathBuf),
    #[error("native ACL sealing failed at {path} with Windows error {code}")]
    Acl { path: PathBuf, code: u32 },
    #[error("snapshot integrity check failed: {0}")]
    Integrity(String),
}

#[derive(Debug, Default)]
pub struct SnapshotBuilder {
    excluded_names: BTreeSet<String>,
    ignored_patterns: Vec<String>,
    secret_regexes: Vec<Regex>,
}

impl SnapshotBuilder {
    pub fn new() -> Self {
        let mut excluded_names = BTreeSet::new();
        for name in [
            ".git",
            ".claude",
            ".codex",
            ".antigravity",
            ".gemini",
            "AGENTS.md",
            "CLAUDE.md",
            "GEMINI.md",
            ".mcp.json",
            "hooks.json",
        ] {
            excluded_names.insert(name.to_string());
        }
        let secret_regexes = [
            r"(?i)-----BEGIN [A-Z ]*PRIVATE KEY-----",
            r"(?i)\b(sk-[A-Za-z0-9]{20,}|AIza[A-Za-z0-9_-]{20,}|ghp_[A-Za-z0-9]{20,}|xox[baprs]-[A-Za-z0-9-]{20,})\b",
            r#"(?i)\b(api[_-]?key|secret|token|password|client_secret)\b\s*[:=]\s*["']?[A-Za-z0-9_./+=-]{16,}"#,
        ]
        .into_iter()
        .map(|pattern| Regex::new(pattern).expect("valid secret pattern"))
        .collect();
        Self {
            excluded_names,
            ignored_patterns: Vec::new(),
            secret_regexes,
        }
    }

    pub fn build(&self, request: &SnapshotRequest) -> Result<SnapshotManifest, SnapshotError> {
        let source =
            fs::canonicalize(&request.source_root).map_err(|source| SnapshotError::Filesystem {
                path: request.source_root.clone(),
                source,
            })?;
        if !source.is_dir() {
            return Err(SnapshotError::MissingSource(source));
        }
        let destination = request.destination_root.join(&request.snapshot_id);
        if destination.exists() {
            return Err(SnapshotError::DestinationExists(destination));
        }
        fs::create_dir_all(&request.destination_root).map_err(|source| {
            SnapshotError::Filesystem {
                path: request.destination_root.clone(),
                source,
            }
        })?;
        let destination_canonical = fs::canonicalize(&request.destination_root)
            .map_err(|source| SnapshotError::Filesystem {
                path: request.destination_root.clone(),
                source,
            })?
            .join(&request.snapshot_id);
        if destination_canonical.starts_with(&source) {
            return Err(SnapshotError::DestinationInsideSource(
                destination_canonical,
            ));
        }
        fs::create_dir_all(&destination_canonical).map_err(|source| SnapshotError::Filesystem {
            path: destination_canonical.clone(),
            source,
        })?;

        let ignored_patterns = self.load_gitignore(&source);
        let mut manifest = SnapshotManifest {
            snapshot_id: request.snapshot_id.clone(),
            source_root: source.to_string_lossy().to_string(),
            created_at: Utc::now(),
            files: Vec::new(),
            exclusions: Vec::new(),
            manifest_sha256: String::new(),
        };
        self.copy_directory(
            &source,
            &source,
            &destination_canonical,
            &ignored_patterns,
            &mut manifest,
        )?;
        manifest
            .files
            .sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
        manifest
            .exclusions
            .sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
        manifest.manifest_sha256 = snapshot_manifest_hash(&manifest)?;
        let manifest_path = destination_canonical.join("snapshot-manifest.json");
        let manifest_bytes = serde_json::to_vec_pretty(&manifest)?;
        fs::write(&manifest_path, manifest_bytes).map_err(|source| SnapshotError::Filesystem {
            path: manifest_path.clone(),
            source,
        })?;
        seal_tree(&destination_canonical)?;
        Ok(manifest)
    }

    pub fn source_tree_hash(&self, source_root: &Path) -> Result<String, SnapshotError> {
        let source = fs::canonicalize(source_root).map_err(|source| SnapshotError::Filesystem {
            path: source_root.to_path_buf(),
            source,
        })?;
        if !source.is_dir() {
            return Err(SnapshotError::MissingSource(source));
        }
        let ignored_patterns = self.load_gitignore(&source);
        let mut records = Vec::new();
        self.fingerprint_directory(&source, &source, &ignored_patterns, &mut records)?;
        records.sort();
        Ok(sha256(records.join("\n").as_bytes()))
    }

    fn fingerprint_directory(
        &self,
        source_root: &Path,
        current_source: &Path,
        ignored_patterns: &[String],
        records: &mut Vec<String>,
    ) -> Result<(), SnapshotError> {
        let mut entries = fs::read_dir(current_source)
            .map_err(|source| SnapshotError::Filesystem {
                path: current_source.to_path_buf(),
                source,
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|source| SnapshotError::Filesystem {
                path: current_source.to_path_buf(),
                source,
            })?;
        entries.sort_by_key(|entry| entry.file_name());

        for entry in entries {
            let source_path = entry.path();
            let relative = source_path
                .strip_prefix(source_root)
                .expect("source path remains beneath source root")
                .to_string_lossy()
                .replace('\\', "/");
            let name = entry.file_name().to_string_lossy().to_string();
            let metadata =
                fs::symlink_metadata(&source_path).map_err(|source| SnapshotError::Filesystem {
                    path: source_path.clone(),
                    source,
                })?;

            if metadata.file_type().is_symlink() {
                records.push(format!("EXCLUDED|{relative}|SYMLINK"));
                continue;
            }
            if has_reparse_point(&metadata) {
                records.push(format!("EXCLUDED|{relative}|REPARSE_POINT"));
                continue;
            }
            if self.excluded_names.contains(&name) {
                records.push(format!(
                    "EXCLUDED|{relative}|{}",
                    if name == ".git" {
                        "GIT_METADATA"
                    } else {
                        "AGENT_CONFIG"
                    }
                ));
                continue;
            }
            if ignored_patterns
                .iter()
                .any(|pattern| ignored(pattern, &relative, metadata.is_dir()))
            {
                records.push(format!("EXCLUDED|{relative}|GIT_IGNORED"));
                continue;
            }
            if metadata.is_dir() {
                records.push(format!("DIRECTORY|{relative}"));
                self.fingerprint_directory(source_root, &source_path, ignored_patterns, records)?;
                continue;
            }
            if !metadata.is_file() {
                records.push(format!("EXCLUDED|{relative}|UNSUPPORTED"));
                continue;
            }
            let bytes = fs::read(&source_path).map_err(|source| SnapshotError::Filesystem {
                path: source_path.clone(),
                source,
            })?;
            let kind = if is_secret_filename(&name)
                || contains_secret(&source_path, &self.secret_regexes)
            {
                "SECRET"
            } else {
                "FILE"
            };
            records.push(format!(
                "{kind}|{relative}|{}|{}",
                bytes.len(),
                sha256(&bytes)
            ));
        }
        Ok(())
    }

    fn copy_directory(
        &self,
        source_root: &Path,
        current_source: &Path,
        current_destination: &Path,
        ignored_patterns: &[String],
        manifest: &mut SnapshotManifest,
    ) -> Result<(), SnapshotError> {
        let mut entries = fs::read_dir(current_source)
            .map_err(|source| SnapshotError::Filesystem {
                path: current_source.to_path_buf(),
                source,
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|source| SnapshotError::Filesystem {
                path: current_source.to_path_buf(),
                source,
            })?;
        entries.sort_by_key(|entry| entry.file_name());

        for entry in entries {
            let source_path = entry.path();
            let relative = source_path
                .strip_prefix(source_root)
                .expect("source path remains beneath source root")
                .to_string_lossy()
                .replace('\\', "/");
            let name = entry.file_name().to_string_lossy().to_string();
            let metadata =
                fs::symlink_metadata(&source_path).map_err(|source| SnapshotError::Filesystem {
                    path: source_path.clone(),
                    source,
                })?;

            if metadata.file_type().is_symlink() {
                manifest.exclusions.push(SnapshotExclusion {
                    relative_path: relative,
                    reason: SnapshotExclusionReason::Symlink,
                    detail: "symbolic links are never followed".to_string(),
                });
                continue;
            }
            if has_reparse_point(&metadata) {
                manifest.exclusions.push(SnapshotExclusion {
                    relative_path: relative,
                    reason: SnapshotExclusionReason::ReparsePoint,
                    detail: "reparse points and junctions are never followed".to_string(),
                });
                continue;
            }
            if self.excluded_names.contains(&name) {
                manifest.exclusions.push(SnapshotExclusion {
                    relative_path: relative,
                    reason: if name == ".git" {
                        SnapshotExclusionReason::GitMetadata
                    } else {
                        SnapshotExclusionReason::AgentConfig
                    },
                    detail: "provider control/config surface is excluded".to_string(),
                });
                continue;
            }
            if ignored_patterns
                .iter()
                .any(|pattern| ignored(pattern, &relative, metadata.is_dir()))
            {
                manifest.exclusions.push(SnapshotExclusion {
                    relative_path: relative,
                    reason: SnapshotExclusionReason::GitIgnored,
                    detail: "matched a .gitignore pattern".to_string(),
                });
                continue;
            }

            let destination_path = current_destination.join(&name);
            if metadata.is_dir() {
                fs::create_dir_all(&destination_path).map_err(|source| {
                    SnapshotError::Filesystem {
                        path: destination_path.clone(),
                        source,
                    }
                })?;
                self.copy_directory(
                    source_root,
                    &source_path,
                    &destination_path,
                    ignored_patterns,
                    manifest,
                )?;
                continue;
            }
            if !metadata.is_file() {
                manifest.exclusions.push(SnapshotExclusion {
                    relative_path: relative,
                    reason: SnapshotExclusionReason::Unsupported,
                    detail: "not a regular file or directory".to_string(),
                });
                continue;
            }
            if is_secret_filename(&name) || contains_secret(&source_path, &self.secret_regexes) {
                manifest.exclusions.push(SnapshotExclusion {
                    relative_path: relative,
                    reason: SnapshotExclusionReason::Secret,
                    detail: "filename or content matched the practical secret scanner".to_string(),
                });
                continue;
            }
            let bytes = fs::read(&source_path).map_err(|source| SnapshotError::Filesystem {
                path: source_path.clone(),
                source,
            })?;
            fs::write(&destination_path, &bytes).map_err(|source| SnapshotError::Filesystem {
                path: destination_path.clone(),
                source,
            })?;
            manifest.files.push(SnapshotFile {
                relative_path: relative,
                size: bytes.len() as u64,
                sha256: sha256(&bytes),
            });
        }
        Ok(())
    }

    fn load_gitignore(&self, source: &Path) -> Vec<String> {
        let path = source.join(".gitignore");
        let Ok(contents) = fs::read_to_string(path) else {
            return self.ignored_patterns.clone();
        };
        contents
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with('#') && !line.starts_with('!'))
            .map(str::to_string)
            .chain(self.ignored_patterns.iter().cloned())
            .collect()
    }
}

pub fn snapshot_manifest_hash(manifest: &SnapshotManifest) -> Result<String, serde_json::Error> {
    let mut clone = manifest.clone();
    clone.manifest_sha256.clear();
    Ok(sha256(&serde_json::to_vec(&clone)?))
}

pub fn snapshot_exclusion_review_identity(
    manifest: &SnapshotManifest,
) -> (String, Vec<SnapshotReviewExclusion>) {
    let mut exclusions = manifest
        .exclusions
        .iter()
        .map(|exclusion| SnapshotReviewExclusion {
            relative_path: exclusion.relative_path.clone(),
            reason: exclusion.reason.clone(),
        })
        .collect::<Vec<_>>();
    exclusions.sort_by(|left, right| {
        left.relative_path.cmp(&right.relative_path).then_with(|| {
            serde_json::to_string(&left.reason)
                .unwrap_or_default()
                .cmp(&serde_json::to_string(&right.reason).unwrap_or_default())
        })
    });
    let serialized = serde_json::to_vec(&exclusions).expect("snapshot exclusions serialize");
    (sha256(&serialized), exclusions)
}

pub fn snapshot_review_id(
    debate_id: &str,
    snapshot_id: &str,
    manifest_hash: &str,
    exclusion_set_hash: &str,
) -> String {
    let mut digest = Sha256::new();
    digest.update(debate_id.as_bytes());
    digest.update([0]);
    digest.update(snapshot_id.as_bytes());
    digest.update([0]);
    digest.update(manifest_hash.as_bytes());
    digest.update([0]);
    digest.update(exclusion_set_hash.as_bytes());
    format!("snapshot-review-{}", hex::encode(digest.finalize()))
}

pub fn verify_sealed_snapshot(
    root: &Path,
    manifest: &SnapshotManifest,
) -> Result<(), SnapshotError> {
    if !root.is_dir() {
        return Err(SnapshotError::Integrity(format!(
            "snapshot root is not a directory: {}",
            root.display()
        )));
    }
    let expected_manifest_hash = snapshot_manifest_hash(manifest)?;
    if expected_manifest_hash != manifest.manifest_sha256 {
        return Err(SnapshotError::Integrity(
            "persisted manifest hash does not match its contents".to_string(),
        ));
    }
    let manifest_path = root.join("snapshot-manifest.json");
    let on_disk_manifest =
        fs::read(&manifest_path).map_err(|source| SnapshotError::Filesystem {
            path: manifest_path.clone(),
            source,
        })?;
    let parsed: SnapshotManifest = serde_json::from_slice(&on_disk_manifest)?;
    if parsed != *manifest {
        return Err(SnapshotError::Integrity(
            "on-disk manifest differs from the persisted review manifest".to_string(),
        ));
    }

    let expected_files = manifest
        .files
        .iter()
        .map(|file| file.relative_path.clone())
        .collect::<BTreeSet<_>>();
    let mut actual_files = BTreeSet::new();
    collect_files(root, root, &mut actual_files)?;
    actual_files.remove("snapshot-manifest.json");
    if actual_files != expected_files {
        return Err(SnapshotError::Integrity(
            "snapshot contains an unexpected or missing file".to_string(),
        ));
    }

    for file in &manifest.files {
        let path = safe_snapshot_path(root, &file.relative_path)?;
        let bytes = fs::read(&path).map_err(|source| SnapshotError::Filesystem {
            path: path.clone(),
            source,
        })?;
        if bytes.len() as u64 != file.size || sha256(&bytes) != file.sha256 {
            return Err(SnapshotError::Integrity(format!(
                "snapshot file hash mismatch at {}",
                file.relative_path
            )));
        }
    }
    for exclusion in &manifest.exclusions {
        let path = safe_snapshot_path(root, &exclusion.relative_path)?;
        if path.exists() {
            return Err(SnapshotError::Integrity(format!(
                "excluded path is present in the snapshot: {}",
                exclusion.relative_path
            )));
        }
    }
    Ok(())
}

fn collect_files(
    root: &Path,
    current: &Path,
    files: &mut BTreeSet<String>,
) -> Result<(), SnapshotError> {
    let entries = fs::read_dir(current).map_err(|source| SnapshotError::Filesystem {
        path: current.to_path_buf(),
        source,
    })?;
    for entry in entries {
        let entry = entry.map_err(|source| SnapshotError::Filesystem {
            path: current.to_path_buf(),
            source,
        })?;
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path).map_err(|source| SnapshotError::Filesystem {
            path: path.clone(),
            source,
        })?;
        if metadata.file_type().is_symlink() || has_reparse_point(&metadata) {
            return Err(SnapshotError::Integrity(
                "sealed snapshot contains a link or reparse point".to_string(),
            ));
        }
        if metadata.is_dir() {
            collect_files(root, &path, files)?;
        } else if metadata.is_file() {
            let relative = path
                .strip_prefix(root)
                .map_err(|_| SnapshotError::Integrity("snapshot path escaped root".to_string()))?
                .to_string_lossy()
                .replace('\\', "/");
            files.insert(relative);
        }
    }
    Ok(())
}

fn safe_snapshot_path(root: &Path, relative: &str) -> Result<PathBuf, SnapshotError> {
    let relative_path = Path::new(relative);
    if relative_path.is_absolute()
        || relative_path.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return Err(SnapshotError::Integrity(
            "snapshot manifest contains an unsafe relative path".to_string(),
        ));
    }
    Ok(root.join(relative_path))
}

fn is_secret_filename(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower == ".env"
        || lower.starts_with(".env.")
        || lower.ends_with(".pem")
        || lower.ends_with(".key")
        || lower.ends_with(".pfx")
        || lower.ends_with(".p12")
        || lower.starts_with("id_rsa")
        || lower.contains("credentials")
}

fn contains_secret(path: &Path, patterns: &[Regex]) -> bool {
    let Ok(bytes) = fs::read(path) else {
        return false;
    };
    if bytes.len() > 2 * 1024 * 1024 || bytes.contains(&0) {
        return false;
    }
    let text = String::from_utf8_lossy(&bytes);
    patterns.iter().any(|pattern| pattern.is_match(&text))
}

fn ignored(pattern: &str, relative: &str, is_dir: bool) -> bool {
    let clean = pattern.trim_start_matches('/').trim_end_matches('/');
    if clean.is_empty() {
        return false;
    }
    if pattern.ends_with('/') && !is_dir {
        return false;
    }
    if clean.contains('/') {
        wildcard_match(clean, relative)
    } else {
        relative
            .split('/')
            .any(|component| wildcard_match(clean, component))
    }
}

fn wildcard_match(pattern: &str, value: &str) -> bool {
    let mut pattern_index = 0;
    let mut value_index = 0;
    let pattern_bytes = pattern.as_bytes();
    let value_bytes = value.as_bytes();
    let mut star = None;
    let mut star_value = 0;
    while value_index < value_bytes.len() {
        if pattern_index < pattern_bytes.len()
            && (pattern_bytes[pattern_index] == value_bytes[value_index]
                || pattern_bytes[pattern_index] == b'?')
        {
            pattern_index += 1;
            value_index += 1;
        } else if pattern_index < pattern_bytes.len() && pattern_bytes[pattern_index] == b'*' {
            star = Some(pattern_index);
            pattern_index += 1;
            star_value = value_index;
        } else if let Some(star_index) = star {
            pattern_index = star_index + 1;
            star_value += 1;
            value_index = star_value;
        } else {
            return false;
        }
    }
    while pattern_index < pattern_bytes.len() && pattern_bytes[pattern_index] == b'*' {
        pattern_index += 1;
    }
    pattern_index == pattern_bytes.len()
}

fn sha256(bytes: &[u8]) -> String {
    let mut digest = Sha256::new();
    digest.update(bytes);
    hex::encode(digest.finalize())
}

fn set_read_only(path: &Path, _directory: bool) -> Result<(), SnapshotError> {
    #[cfg(windows)]
    if _directory {
        return Ok(());
    }
    let mut permissions = fs::metadata(path)
        .map_err(|source| SnapshotError::Filesystem {
            path: path.to_path_buf(),
            source,
        })?
        .permissions();
    permissions.set_readonly(true);
    fs::set_permissions(path, permissions).map_err(|source| SnapshotError::Filesystem {
        path: path.to_path_buf(),
        source,
    })?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = if _directory { 0o555 } else { 0o444 };
        fs::set_permissions(path, fs::Permissions::from_mode(mode)).map_err(|source| {
            SnapshotError::Filesystem {
                path: path.to_path_buf(),
                source,
            }
        })?;
    }
    Ok(())
}

#[cfg(windows)]
fn apply_native_write_deny(path: &Path, directory: bool) -> Result<(), SnapshotError> {
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::{null, null_mut};
    use windows_sys::Win32::Foundation::{ERROR_SUCCESS, LocalFree};
    use windows_sys::Win32::Security::Authorization::{
        BuildTrusteeWithNameW, DENY_ACCESS, EXPLICIT_ACCESS_W, SE_FILE_OBJECT, SetEntriesInAclW,
        SetNamedSecurityInfoW, TRUSTEE_IS_NAME, TRUSTEE_IS_WELL_KNOWN_GROUP,
    };
    use windows_sys::Win32::Security::{
        ACE_FLAGS, ACL, CONTAINER_INHERIT_ACE, DACL_SECURITY_INFORMATION, NO_INHERITANCE,
        OBJECT_INHERIT_ACE,
    };
    use windows_sys::Win32::Storage::FileSystem::{
        DELETE, FILE_ADD_FILE, FILE_ADD_SUBDIRECTORY, FILE_APPEND_DATA, FILE_DELETE_CHILD,
        FILE_WRITE_ATTRIBUTES, FILE_WRITE_DATA, FILE_WRITE_EA,
    };

    let mut object_name: Vec<u16> = path.as_os_str().encode_wide().chain([0]).collect();
    let mut old_dacl: *mut ACL = null_mut();
    let mut security_descriptor = null_mut();
    let result = unsafe {
        windows_sys::Win32::Security::Authorization::GetNamedSecurityInfoW(
            object_name.as_mut_ptr(),
            SE_FILE_OBJECT,
            DACL_SECURITY_INFORMATION,
            null_mut(),
            null_mut(),
            &mut old_dacl,
            null_mut(),
            &mut security_descriptor,
        )
    };
    if result != ERROR_SUCCESS {
        return Err(SnapshotError::Acl {
            path: path.to_path_buf(),
            code: result,
        });
    }

    let mut trustee = windows_sys::Win32::Security::Authorization::TRUSTEE_W {
        TrusteeForm: TRUSTEE_IS_NAME,
        TrusteeType: TRUSTEE_IS_WELL_KNOWN_GROUP,
        ..Default::default()
    };
    unsafe { BuildTrusteeWithNameW(&mut trustee, windows_sys::core::w!("Everyone")) };
    let inheritance: ACE_FLAGS = if directory {
        OBJECT_INHERIT_ACE | CONTAINER_INHERIT_ACE
    } else {
        NO_INHERITANCE
    };
    let permissions = FILE_WRITE_DATA
        | FILE_APPEND_DATA
        | FILE_WRITE_EA
        | FILE_WRITE_ATTRIBUTES
        | DELETE
        | FILE_DELETE_CHILD
        | FILE_ADD_FILE
        | FILE_ADD_SUBDIRECTORY;
    let entry = EXPLICIT_ACCESS_W {
        grfAccessPermissions: permissions,
        grfAccessMode: DENY_ACCESS,
        grfInheritance: inheritance,
        Trustee: trustee,
    };
    let mut new_dacl: *mut ACL = null_mut();
    let acl_result = unsafe { SetEntriesInAclW(1, &entry, old_dacl, &mut new_dacl) };
    if acl_result != ERROR_SUCCESS {
        unsafe {
            LocalFree(security_descriptor);
        }
        return Err(SnapshotError::Acl {
            path: path.to_path_buf(),
            code: acl_result,
        });
    }
    let set_result = unsafe {
        SetNamedSecurityInfoW(
            object_name.as_mut_ptr(),
            SE_FILE_OBJECT,
            DACL_SECURITY_INFORMATION,
            null_mut(),
            null_mut(),
            new_dacl,
            null(),
        )
    };
    unsafe {
        LocalFree(new_dacl as _);
        LocalFree(security_descriptor);
    }
    if set_result != ERROR_SUCCESS {
        return Err(SnapshotError::Acl {
            path: path.to_path_buf(),
            code: set_result,
        });
    }
    Ok(())
}

#[cfg(not(windows))]
fn apply_native_write_deny(_path: &Path, _directory: bool) -> Result<(), SnapshotError> {
    Ok(())
}

fn seal_tree(root: &Path) -> Result<(), SnapshotError> {
    let mut entries = fs::read_dir(root)
        .map_err(|source| SnapshotError::Filesystem {
            path: root.to_path_buf(),
            source,
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|source| SnapshotError::Filesystem {
            path: root.to_path_buf(),
            source,
        })?;
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let path = entry.path();
        let metadata = fs::metadata(&path).map_err(|source| SnapshotError::Filesystem {
            path: path.clone(),
            source,
        })?;
        if metadata.is_dir() {
            seal_tree(&path)?;
            set_read_only(&path, true)?;
            apply_native_write_deny(&path, true)?;
        } else {
            set_read_only(&path, false)?;
            apply_native_write_deny(&path, false)?;
        }
    }
    set_read_only(root, true)?;
    apply_native_write_deny(root, true)?;
    Ok(())
}

#[cfg(windows)]
fn has_reparse_point(metadata: &fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;
    metadata.file_attributes() & 0x400 != 0
}

#[cfg(not(windows))]
fn has_reparse_point(metadata: &fs::Metadata) -> bool {
    metadata.file_type().is_symlink()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_bytes_and_excludes_secrets_and_configs() {
        let source = tempfile::Builder::new()
            .prefix("council-source-")
            .tempdir()
            .unwrap();
        let destination = tempfile::Builder::new()
            .prefix("council-destination-")
            .tempdir()
            .unwrap();
        fs::create_dir_all(source.path().join("src")).unwrap();
        fs::write(
            source.path().join("src").join("main.ts"),
            b"fixture-user\r\nbeta\n",
        )
        .unwrap();
        fs::write(source.path().join(".env"), b"SECRET=should-not-copy").unwrap();
        fs::write(source.path().join("AGENTS.md"), b"provider instruction").unwrap();
        fs::write(source.path().join(".gitignore"), b"ignored.txt\n").unwrap();
        fs::write(source.path().join("ignored.txt"), b"ignored").unwrap();

        let manifest = SnapshotBuilder::new()
            .build(&SnapshotRequest {
                source_root: source.path().to_path_buf(),
                destination_root: destination.path().to_path_buf(),
                snapshot_id: "test".to_string(),
            })
            .unwrap();

        assert_eq!(manifest.files.len(), 2);
        assert_eq!(
            fs::read(destination.path().join("test").join("src").join("main.ts")).unwrap(),
            b"fixture-user\r\nbeta\n"
        );
        assert!(
            manifest
                .exclusions
                .iter()
                .any(|item| item.reason == SnapshotExclusionReason::Secret)
        );
        assert!(
            manifest
                .exclusions
                .iter()
                .any(|item| item.reason == SnapshotExclusionReason::AgentConfig)
        );
        assert!(
            manifest
                .exclusions
                .iter()
                .any(|item| item.reason == SnapshotExclusionReason::GitIgnored)
        );
    }

    #[test]
    fn review_identity_is_safe_and_source_changes_produce_a_new_fingerprint() {
        let source = tempfile::Builder::new()
            .prefix("council-review-source-")
            .tempdir()
            .unwrap();
        let destination = tempfile::Builder::new()
            .prefix("council-review-destination-")
            .tempdir()
            .unwrap();
        fs::write(source.path().join("README.md"), b"reviewable evidence").unwrap();
        fs::write(
            source.path().join(".env"),
            b"TOKEN=review-secret-value-1234567890",
        )
        .unwrap();

        let builder = SnapshotBuilder::new();
        let before = builder.source_tree_hash(source.path()).unwrap();
        let manifest = builder
            .build(&SnapshotRequest {
                source_root: source.path().to_path_buf(),
                destination_root: destination.path().to_path_buf(),
                snapshot_id: "reviewed".to_string(),
            })
            .unwrap();
        let (exclusion_hash, exclusions) = snapshot_exclusion_review_identity(&manifest);
        let serialized = serde_json::to_string(&exclusions).unwrap();
        assert!(!serialized.contains("review-secret-value"));
        assert!(!exclusion_hash.is_empty());
        verify_sealed_snapshot(&destination.path().join("reviewed"), &manifest).unwrap();

        fs::write(
            source.path().join(".env"),
            b"TOKEN=changed-review-secret-value-1234567890",
        )
        .unwrap();
        let after = builder.source_tree_hash(source.path()).unwrap();
        assert_ne!(before, after);
    }
}
