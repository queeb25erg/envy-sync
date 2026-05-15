//! Persistent storage for TTL entries, serialized to a simple TOML-like format.

use crate::env_ttl::{TtlEntry, TtlStore};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

const SEPARATOR: &str = "=";

/// Serialize TtlStore to a string (key=expires_at per line).
pub fn serialize(store: &TtlStore) -> String {
    let mut lines: Vec<String> = store
        .all_entries()
        .iter()
        .map(|e| format!("{}{}{}", e.key, SEPARATOR, e.expires_at))
        .collect();
    lines.sort();
    lines.join("\n")
}

/// Deserialize a TtlStore from a string.
pub fn deserialize(data: &str) -> TtlStore {
    let mut store = TtlStore::new();
    for line in data.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, val)) = line.split_once(SEPARATOR) {
            if let Ok(expires_at) = val.trim().parse::<u64>() {
                store.entries_mut().insert(
                    key.trim().to_string(),
                    TtlEntry {
                        key: key.trim().to_string(),
                        expires_at,
                    },
                );
            }
        }
    }
    store
}

pub fn save(store: &TtlStore, path: &Path) -> std::io::Result<()> {
    fs::write(path, serialize(store))
}

pub fn load(path: &Path) -> std::io::Result<TtlStore> {
    if !path.exists() {
        return Ok(TtlStore::new());
    }
    let data = fs::read_to_string(path)?;
    Ok(deserialize(&data))
}
