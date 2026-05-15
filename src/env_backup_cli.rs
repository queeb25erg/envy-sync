//! env_backup_cli.rs — CLI interface for backup management commands.

use std::path::PathBuf;
use anyhow::Result;
use chrono::Utc;
use crate::env_backup::{BackupEntry, BackupStore, compute_checksum, generate_backup_id};
use crate::env_backup_store::BackupStoreManager;
use std::collections::HashMap;

pub fn cmd_backup_create(
    source: &PathBuf,
    label: Option<String>,
    vars: &HashMap<String, String>,
    base_dir: &PathBuf,
) -> Result<String> {
    let manager = BackupStoreManager::new(base_dir);
    let mut store = manager.load()?;

    let id = generate_backup_id();
    let checksum = compute_checksum(vars);
    let content: String = vars
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("\n");

    let backup_path = manager.write_backup_file(&id, &content)?;

    let entry = BackupEntry {
        id: id.clone(),
        source_path: source.clone(),
        backup_path,
        created_at: Utc::now(),
        label,
        checksum,
    };

    store.add(entry);
    manager.save(&store)?;
    Ok(id)
}

pub fn cmd_backup_list(source: &PathBuf, base_dir: &PathBuf) -> Result<Vec<BackupEntry>> {
    let manager = BackupStoreManager::new(base_dir);
    let store = manager.load()?;
    Ok(store.list_for_path(source).into_iter().cloned().collect())
}

pub fn cmd_backup_restore(id: &str, base_dir: &PathBuf) -> Result<HashMap<String, String>> {
    let manager = BackupStoreManager::new(base_dir);
    let content = manager.read_backup_file(id)?;
    let mut vars = HashMap::new();
    for line in content.lines() {
        if let Some((k, v)) = line.split_once('=') {
            vars.insert(k.to_string(), v.to_string());
        }
    }
    Ok(vars)
}

pub fn cmd_backup_delete(id: &str, base_dir: &PathBuf) -> Result<bool> {
    let manager = BackupStoreManager::new(base_dir);
    let mut store = manager.load()?;
    let removed = store.remove(id);
    if removed {
        manager.save(&store)?;
    }
    Ok(removed)
}
