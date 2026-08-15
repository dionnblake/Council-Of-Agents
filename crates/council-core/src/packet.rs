use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

use crate::model::{ProviderKind, new_id};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PacketMetadata {
    pub packet_id: String,
    pub debate_id: String,
    pub turn_id: String,
    pub provider: ProviderKind,
    pub schema_version: String,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub skills: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContextPacket {
    pub metadata: PacketMetadata,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WrittenPacket {
    pub metadata: PacketMetadata,
    pub path: PathBuf,
    pub sha256: String,
    pub bytes: u64,
}

#[derive(Debug, Error)]
pub enum PacketError {
    #[error("packet directory could not be created: {path}: {source}")]
    Directory {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("packet could not be written: {path}: {source}")]
    Write {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("packet already exists: {0}")]
    Exists(PathBuf),
}

impl ContextPacket {
    pub fn new(
        debate_id: impl Into<String>,
        turn_id: impl Into<String>,
        provider: ProviderKind,
        schema_version: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        Self {
            metadata: PacketMetadata {
                packet_id: new_id("packet"),
                debate_id: debate_id.into(),
                turn_id: turn_id.into(),
                provider,
                schema_version: schema_version.into(),
                created_at: Utc::now(),
                skills: Vec::new(),
            },
            body: body.into(),
        }
    }

    pub fn with_skills(mut self, skills: Vec<String>) -> Self {
        self.metadata.skills = skills;
        self
    }

    pub fn write_sealed(&self, directory: impl AsRef<Path>) -> Result<WrittenPacket, PacketError> {
        let directory = directory.as_ref();
        fs::create_dir_all(directory).map_err(|source| PacketError::Directory {
            path: directory.to_path_buf(),
            source,
        })?;
        let path = directory.join(format!("{}.md", self.metadata.packet_id));
        if path.exists() {
            return Err(PacketError::Exists(path));
        }
        let bytes = self.body.as_bytes();
        fs::write(&path, bytes).map_err(|source| PacketError::Write {
            path: path.clone(),
            source,
        })?;
        let mut permissions = fs::metadata(&path)
            .map_err(|source| PacketError::Write {
                path: path.clone(),
                source,
            })?
            .permissions();
        permissions.set_readonly(true);
        fs::set_permissions(&path, permissions).map_err(|source| PacketError::Write {
            path: path.clone(),
            source,
        })?;
        let persisted = fs::read(&path).map_err(|source| PacketError::Write {
            path: path.clone(),
            source,
        })?;
        if persisted != bytes {
            return Err(PacketError::Write {
                path,
                source: std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "packet bytes changed during write",
                ),
            });
        }
        let mut digest = Sha256::new();
        digest.update(bytes);
        Ok(WrittenPacket {
            metadata: self.metadata.clone(),
            path,
            sha256: hex::encode(digest.finalize()),
            bytes: bytes.len() as u64,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn packet_sizes_preserve_exact_bytes_and_terminal_markers() {
        let directory = tempfile::tempdir().unwrap();
        for (size, marker) in [
            (50_000_usize, "PACKET_END_50KB"),
            (200_000_usize, "PACKET_END_200KB"),
            (500_000_usize, "PACKET_END_500KB"),
        ] {
            let mut body = "x".repeat(size.saturating_sub(marker.len()));
            body.push_str(marker);
            let packet = ContextPacket::new(
                "debate-packet-test",
                format!("turn-{size}"),
                ProviderKind::Claude,
                crate::POSITION_SCHEMA_VERSION,
                body.clone(),
            );
            let written = packet.write_sealed(directory.path()).unwrap();
            assert_eq!(written.bytes as usize, size);
            assert_eq!(fs::read_to_string(written.path).unwrap(), body);
        }
    }
}
