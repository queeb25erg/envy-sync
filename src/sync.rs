use std::collections::HashMap;
use crate::crypto::{encrypt, decrypt};
use crate::storage::{StorageBackend, StorageError};

#[derive(Debug, thiserror::Error)]
pub enum SyncError {
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
    #[error("Crypto error: {0}")]
    Crypto(String),
    #[error("Parse error: {0}")]
    Parse(String),
}

/// Parse a .env file content into a key-value map.
pub fn parse_env(content: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            map.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    map
}

/// Serialize a key-value map back into .env file content.
pub fn serialize_env(map: &HashMap<String, String>) -> String {
    let mut lines: Vec<String> = map
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect();
    lines.sort();
    lines.join("\n")
}

/// Push a local .env file to remote encrypted storage.
pub fn push(
    backend: &dyn StorageBackend,
    key: &[u8],
    remote_path: &str,
    local_content: &str,
) -> Result<(), SyncError> {
    let ciphertext = encrypt(key, local_content.as_bytes())
        .map_err(|e| SyncError::Crypto(e.to_string()))?;
    backend.put(remote_path, &ciphertext)?;
    Ok(())
}

/// Pull remote encrypted .env content and return decrypted string.
pub fn pull(
    backend: &dyn StorageBackend,
    key: &[u8],
    remote_path: &str,
) -> Result<String, SyncError> {
    let ciphertext = backend.get(remote_path)?;
    let plaintext = decrypt(key, &ciphertext)
        .map_err(|e| SyncError::Crypto(e.to_string()))?;
    String::from_utf8(plaintext)
        .map_err(|e| SyncError::Parse(e.to_string()))
}

/// Merge remote values into local map, preferring remote on conflict.
pub fn merge_remote_wins(
    local: &HashMap<String, String>,
    remote: &HashMap<String, String>,
) -> HashMap<String, String> {
    let mut merged = local.clone();
    for (k, v) in remote {
        merged.insert(k.clone(), v.clone());
    }
    merged
}
