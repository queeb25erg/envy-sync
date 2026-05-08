use crate::snapshot::{Snapshot, SnapshotMeta};
use crate::storage::Storage;
use crate::crypto::decrypt;
use std::collections::HashMap;
use anyhow::{Result, anyhow};

/// Restore a .env file from a named snapshot.
pub fn restore_snapshot(
    storage: &dyn Storage,
    snapshot_id: &str,
    key: &[u8],
) -> Result<HashMap<String, String>> {
    let raw = storage
        .load(&format!("snapshots/{}.enc", snapshot_id))
        .map_err(|e| anyhow!("Snapshot '{}' not found: {}", snapshot_id, e))?;

    let plaintext = decrypt(key, &raw)?;
    let snapshot: Snapshot = serde_json::from_slice(&plaintext)
        .map_err(|e| anyhow!("Failed to deserialize snapshot: {}", e))?;

    Ok(snapshot.env_vars)
}

/// List available snapshots from storage metadata.
pub fn list_snapshots(storage: &dyn Storage) -> Result<Vec<SnapshotMeta>> {
    let raw = storage
        .load("snapshots/index.json")
        .map_err(|_| anyhow!("No snapshot index found"))?;

    let metas: Vec<SnapshotMeta> = serde_json::from_slice(&raw)
        .map_err(|e| anyhow!("Failed to parse snapshot index: {}", e))?;

    Ok(metas)
}

/// Write restored env vars to a .env file path.
pub fn write_env_file(
    path: &str,
    env_vars: &HashMap<String, String>,
) -> Result<()> {
    let mut lines: Vec<String> = env_vars
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect();
    lines.sort();
    let content = lines.join("\n") + "\n";
    std::fs::write(path, content)
        .map_err(|e| anyhow!("Failed to write env file '{}': {}", path, e))
}
