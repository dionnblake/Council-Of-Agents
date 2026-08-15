use crate::snapshot::SnapshotManifest;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WslBridgeRequest {
    pub source_snapshot: PathBuf,
    pub linux_distribution: String,
    pub linux_user: String,
    pub linux_destination: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WslBridgePlan {
    pub tar_program: PathBuf,
    pub tar_args: Vec<String>,
    pub wsl_program: PathBuf,
    pub wsl_args: Vec<String>,
    pub linux_destination: String,
    pub uses_windows_mount: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LinuxManifestFile {
    pub relative_path: String,
    pub size: u64,
    pub sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BridgeMismatch {
    pub relative_path: String,
    pub expected: Option<LinuxManifestFile>,
    pub actual: Option<LinuxManifestFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BridgeVerification {
    pub source_manifest_hash: String,
    pub linux_manifest_hash: String,
    pub file_count: usize,
    pub mismatches: Vec<BridgeMismatch>,
    pub byte_preserved: bool,
}

#[derive(Debug, Error)]
pub enum BridgeError {
    #[error("Windows snapshot root does not exist: {0}")]
    MissingSource(PathBuf),
    #[error(
        "Linux destination must be under /home/council/council/snap or /home/council/council/packet: {0}"
    )]
    InvalidLinuxDestination(String),
    #[error("Linux destination must not contain traversal segments")]
    Traversal,
}

pub fn build_wsl_bridge_plan(request: &WslBridgeRequest) -> Result<WslBridgePlan, BridgeError> {
    let source = std::fs::canonicalize(&request.source_snapshot)
        .map_err(|_| BridgeError::MissingSource(request.source_snapshot.clone()))?;
    if !source.is_dir() {
        return Err(BridgeError::MissingSource(source));
    }
    validate_linux_destination(&request.linux_destination)?;
    let destination = shell_quote(&request.linux_destination);
    let extraction = format!(
        "set -eu; test ! -e {destination}; mkdir -p {destination}; /usr/bin/tar -xf - -C {destination}"
    );
    Ok(WslBridgePlan {
        tar_program: PathBuf::from("tar.exe"),
        tar_args: vec![
            "-C".to_string(),
            source.to_string_lossy().to_string(),
            "-cf".to_string(),
            "-".to_string(),
            ".".to_string(),
        ],
        wsl_program: PathBuf::from("wsl.exe"),
        wsl_args: vec![
            "-d".to_string(),
            request.linux_distribution.clone(),
            "--user".to_string(),
            request.linux_user.clone(),
            "--".to_string(),
            "bash".to_string(),
            "-lc".to_string(),
            extraction,
        ],
        linux_destination: request.linux_destination.clone(),
        uses_windows_mount: false,
    })
}

pub fn verify_bridge_manifests(
    source: &SnapshotManifest,
    linux_files: &[LinuxManifestFile],
) -> BridgeVerification {
    let expected = source
        .files
        .iter()
        .map(|file| {
            (
                file.relative_path.clone(),
                LinuxManifestFile {
                    relative_path: file.relative_path.clone(),
                    size: file.size,
                    sha256: file.sha256.clone(),
                },
            )
        })
        .collect::<BTreeMap<_, _>>();
    let actual = linux_files
        .iter()
        .cloned()
        .map(|file| (file.relative_path.clone(), file))
        .collect::<BTreeMap<_, _>>();
    let mut mismatches = Vec::new();
    let paths = expected
        .keys()
        .chain(actual.keys())
        .cloned()
        .collect::<BTreeSet<_>>();
    for path in paths {
        let expected_file = expected.get(&path).cloned();
        let actual_file = actual.get(&path).cloned();
        if expected_file != actual_file {
            mismatches.push(BridgeMismatch {
                relative_path: path,
                expected: expected_file,
                actual: actual_file,
            });
        }
    }
    mismatches.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    let source_manifest_hash = manifest_file_hash(&expected);
    let linux_manifest_hash = manifest_file_hash(&actual);
    BridgeVerification {
        source_manifest_hash,
        linux_manifest_hash,
        file_count: expected.len(),
        byte_preserved: mismatches.is_empty(),
        mismatches,
    }
}

fn validate_linux_destination(destination: &str) -> Result<(), BridgeError> {
    if !destination.starts_with("/home/council/council/snap/")
        && !destination.starts_with("/home/council/council/packet/")
    {
        return Err(BridgeError::InvalidLinuxDestination(
            destination.to_string(),
        ));
    }
    if destination.split('/').any(|component| component == "..") {
        return Err(BridgeError::Traversal);
    }
    Ok(())
}

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

fn manifest_file_hash(files: &BTreeMap<String, LinuxManifestFile>) -> String {
    let mut digest = Sha256::new();
    for (path, file) in files {
        digest.update(path.as_bytes());
        digest.update([0]);
        digest.update(file.size.to_string().as_bytes());
        digest.update([0]);
        digest.update(file.sha256.as_bytes());
        digest.update([0]);
    }
    hex::encode(digest.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::snapshot::{SnapshotFile, SnapshotManifest};
    use chrono::Utc;

    fn source_manifest() -> SnapshotManifest {
        SnapshotManifest {
            snapshot_id: "snap-1".to_string(),
            source_root: "C:/source".to_string(),
            created_at: Utc::now(),
            files: vec![SnapshotFile {
                relative_path: "src/main.rs".to_string(),
                size: 3,
                sha256: "abc".to_string(),
            }],
            exclusions: Vec::new(),
            manifest_sha256: "manifest".to_string(),
        }
    }

    #[test]
    fn bridge_plan_streams_tar_without_mounting_windows_filesystem() {
        let directory = tempfile::tempdir().unwrap();
        let plan = build_wsl_bridge_plan(&WslBridgeRequest {
            source_snapshot: directory.path().to_path_buf(),
            linux_distribution: "CouncilCodexWSL".to_string(),
            linux_user: "council".to_string(),
            linux_destination: "/home/council/council/snap/s-1".to_string(),
        })
        .unwrap();
        assert!(!plan.uses_windows_mount);
        assert!(!plan.wsl_args.iter().any(|arg| arg.contains("/mnt/c")));
        assert!(plan.tar_args.contains(&"-".to_string()));
    }

    #[test]
    fn bridge_verification_is_fail_closed_on_hash_mismatch() {
        let verification = verify_bridge_manifests(
            &source_manifest(),
            &[LinuxManifestFile {
                relative_path: "src/main.rs".to_string(),
                size: 4,
                sha256: "changed".to_string(),
            }],
        );
        assert!(!verification.byte_preserved);
        assert_eq!(verification.mismatches.len(), 1);
    }
}
