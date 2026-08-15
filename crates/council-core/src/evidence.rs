use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::ops::Range;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EvidenceVerdict {
    VerifiedExact,
    VerifiedContentFoundElsewhere,
    Unverified,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VerifiedEvidence {
    pub file: String,
    pub requested_range: String,
    pub resolved_range: Option<String>,
    pub content: String,
    pub content_hash: String,
    pub file_hash: Option<String>,
    pub verdict: EvidenceVerdict,
}

#[derive(Debug, Error)]
pub enum EvidenceError {
    #[error("evidence root is not a directory: {0}")]
    InvalidRoot(PathBuf),
    #[error("failed to read evidence file {path}: {source}")]
    Read {
        path: PathBuf,
        source: std::io::Error,
    },
}

#[derive(Debug, Clone)]
struct IndexedFile {
    bytes: Vec<u8>,
    line_ranges: Vec<Range<usize>>,
    file_hash: String,
}

#[derive(Debug, Clone)]
pub struct EvidenceIndex {
    root: PathBuf,
    files: BTreeMap<String, IndexedFile>,
}

impl EvidenceIndex {
    pub fn build(root: impl AsRef<Path>) -> Result<Self, EvidenceError> {
        let root = root.as_ref().to_path_buf();
        if !root.is_dir() {
            return Err(EvidenceError::InvalidRoot(root));
        }
        let mut files = BTreeMap::new();
        collect_files(&root, &root, &mut files)?;
        Ok(Self { root, files })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn verify(&self, citation: &str, expected_content: Option<&str>) -> VerifiedEvidence {
        let Some(parsed) = parse_citation(citation) else {
            return unverified(citation);
        };
        let Some(relative_path) = normalize_relative_path(&self.root, &parsed.path) else {
            return unverified(citation);
        };
        let parsed = ParsedCitation {
            path: relative_path,
            start: parsed.start,
            end: parsed.end,
        };
        let Some(file) = self.files.get(&parsed.path) else {
            return unverified(citation);
        };
        if parsed.start == 0 || parsed.end < parsed.start || parsed.end > file.line_ranges.len() {
            return VerifiedEvidence {
                file: parsed.path,
                requested_range: citation.to_string(),
                resolved_range: None,
                content: String::new(),
                content_hash: String::new(),
                file_hash: Some(file.file_hash.clone()),
                verdict: EvidenceVerdict::Unverified,
            };
        }

        let selected = selected_bytes(file, parsed.start, parsed.end);
        let selected_text = String::from_utf8_lossy(selected).to_string();
        let selected_hash = sha256(selected);

        if let Some(expected) = expected_content {
            if selected_text.contains(expected) {
                let path = parsed.path.clone();
                return VerifiedEvidence {
                    file: path.clone(),
                    requested_range: citation.to_string(),
                    resolved_range: Some(format!("{}:{}-{}", path, parsed.start, parsed.end)),
                    content: selected_text,
                    content_hash: selected_hash,
                    file_hash: Some(file.file_hash.clone()),
                    verdict: EvidenceVerdict::VerifiedExact,
                };
            }
            if let Some((start, end)) = find_content(file, expected) {
                return VerifiedEvidence {
                    file: parsed.path.clone(),
                    requested_range: citation.to_string(),
                    resolved_range: Some(format!("{}:{}-{}", parsed.path, start, end)),
                    content: String::from_utf8_lossy(selected).to_string(),
                    content_hash: selected_hash,
                    file_hash: Some(file.file_hash.clone()),
                    verdict: EvidenceVerdict::VerifiedContentFoundElsewhere,
                };
            }
            return VerifiedEvidence {
                file: parsed.path,
                requested_range: citation.to_string(),
                resolved_range: None,
                content: selected_text,
                content_hash: selected_hash,
                file_hash: Some(file.file_hash.clone()),
                verdict: EvidenceVerdict::Unverified,
            };
        }

        VerifiedEvidence {
            file: parsed.path.clone(),
            requested_range: citation.to_string(),
            resolved_range: Some(format!("{}:{}-{}", parsed.path, parsed.start, parsed.end)),
            content: selected_text,
            content_hash: selected_hash,
            file_hash: Some(file.file_hash.clone()),
            verdict: EvidenceVerdict::VerifiedExact,
        }
    }

    pub fn contains(&self, relative: &str) -> bool {
        self.files.contains_key(relative)
    }
}

#[derive(Debug, Clone)]
struct ParsedCitation {
    path: String,
    start: usize,
    end: usize,
}

fn parse_citation(value: &str) -> Option<ParsedCitation> {
    let (path, range) = value.rsplit_once(':')?;
    let (start, end) = range.split_once('-')?;
    Some(ParsedCitation {
        path: path.trim().trim_matches('"').replace('\\', "/"),
        start: start.parse().ok()?,
        end: end.parse().ok()?,
    })
}

fn normalize_relative_path(root: &Path, path: &str) -> Option<String> {
    let root = root.to_string_lossy().replace('\\', "/");
    let mut path = path.replace('\\', "/");
    while path.starts_with("./") {
        path = path.trim_start_matches("./").to_string();
    }
    let normalized_root = root.trim_end_matches('/');
    if path == normalized_root {
        return None;
    }
    if let Some(relative) = path.strip_prefix(&format!("{normalized_root}/")) {
        return Some(relative.to_string());
    }
    if path.starts_with('/') {
        let marker = "/snap/";
        if let Some(index) = path.find(marker) {
            let after_snapshot = &path[index + marker.len()..];
            if let Some(separator) = after_snapshot.find('/') {
                return Some(after_snapshot[separator + 1..].to_string());
            }
        }
        return Some(path.trim_start_matches('/').to_string());
    }
    if path.len() >= 2 && path.as_bytes()[1] == b':' {
        return None;
    }
    Some(path)
}

fn collect_files(
    root: &Path,
    current: &Path,
    files: &mut BTreeMap<String, IndexedFile>,
) -> Result<(), EvidenceError> {
    let entries = fs::read_dir(current)
        .map_err(|source| EvidenceError::Read {
            path: current.to_path_buf(),
            source,
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|source| EvidenceError::Read {
            path: current.to_path_buf(),
            source,
        })?;
    for entry in entries {
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path).map_err(|source| EvidenceError::Read {
            path: path.clone(),
            source,
        })?;
        if metadata.file_type().is_symlink() {
            continue;
        }
        if metadata.is_dir() {
            collect_files(root, &path, files)?;
        } else if metadata.is_file() {
            let bytes = fs::read(&path).map_err(|source| EvidenceError::Read {
                path: path.clone(),
                source,
            })?;
            let relative = path
                .strip_prefix(root)
                .expect("path collected beneath root")
                .to_string_lossy()
                .replace('\\', "/");
            files.insert(
                relative,
                IndexedFile {
                    line_ranges: line_ranges(&bytes),
                    file_hash: sha256(&bytes),
                    bytes,
                },
            );
        }
    }
    Ok(())
}

fn line_ranges(bytes: &[u8]) -> Vec<Range<usize>> {
    if bytes.is_empty() {
        return Vec::new();
    }
    let mut ranges = Vec::new();
    let mut start = 0;
    for (index, byte) in bytes.iter().enumerate() {
        if *byte == b'\n' {
            ranges.push(start..index + 1);
            start = index + 1;
        }
    }
    if start < bytes.len() {
        ranges.push(start..bytes.len());
    }
    ranges
}

fn selected_bytes<'a>(file: &'a IndexedFile, start: usize, end: usize) -> &'a [u8] {
    let first = file.line_ranges[start - 1].start;
    let last = file.line_ranges[end - 1].end;
    &file.bytes[first..last]
}

fn find_content(file: &IndexedFile, expected: &str) -> Option<(usize, usize)> {
    for (index, range) in file.line_ranges.iter().enumerate() {
        let text = String::from_utf8_lossy(&file.bytes[range.clone()]);
        if text.contains(expected) {
            return Some((index + 1, index + 1));
        }
    }
    None
}

fn sha256(bytes: &[u8]) -> String {
    let mut digest = Sha256::new();
    digest.update(bytes);
    hex::encode(digest.finalize())
}

fn unverified(citation: &str) -> VerifiedEvidence {
    VerifiedEvidence {
        file: citation.split(':').next().unwrap_or_default().to_string(),
        requested_range: citation.to_string(),
        resolved_range: None,
        content: String::new(),
        content_hash: String::new(),
        file_hash: None,
        verdict: EvidenceVerdict::Unverified,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn verifies_exact_and_shifted_controls() {
        let directory = tempdir().unwrap();
        let path = directory.path().join("fixture.txt");
        let mut file = fs::File::create(&path).unwrap();
        writeln!(file, "fixture-user").unwrap();
        writeln!(file, "target").unwrap();
        writeln!(file, "nearby").unwrap();
        let index = EvidenceIndex::build(directory.path()).unwrap();

        assert_eq!(
            index.verify("fixture.txt:2-2", Some("target")).verdict,
            EvidenceVerdict::VerifiedExact
        );
        assert_eq!(
            index.verify("fixture.txt:3-3", Some("target")).verdict,
            EvidenceVerdict::VerifiedContentFoundElsewhere
        );
        assert_eq!(
            index.verify("fixture.txt:99-99", Some("target")).verdict,
            EvidenceVerdict::Unverified
        );
        let absolute = format!("{}:2-2", path.to_string_lossy());
        assert_eq!(
            index.verify(&absolute, Some("target")).verdict,
            EvidenceVerdict::VerifiedExact
        );
    }
}
