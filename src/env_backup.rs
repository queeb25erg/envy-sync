//! env_backup.rs — Create and manage local backups of .env files before sync operations.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupEntry {
    pub id: String,
    pub source_path: PathBuf,
    pub backup_path: PathBuf,
    pub created_at: DateTime<Utc>,
    pub label: Option<String>,
    pub checksum: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BackupStore {
    pub entries: Vec<BackupEntry>,
}

impl BackupStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, entry: BackupEntry) {
        self.entries.push(entry);
    }

    pub fn list_for_path(&self, path: &Path) -> Vec<&BackupEntry> {
        self.entries
            .iter()
            .filter(|e| e.source_path == path)
            .collect()
    }

    pub fn find_by_id(&self, id: &str) -> Option<&BackupEntry> {
        self.entries.iter().find(|e| e.id == id)
    }

    pub fn remove(&mut self, id: &str) -> bool {
        let before = self.entries.len();
        self.entries.retain(|e| e.id != id);
        self.entries.len() < before
    }

    pub fn latest_for_path(&self, path: &Path) -> Option<&BackupEntry> {
        self.list_for_path(path)
            .into_iter()
            .max_by_key(|e| e.created_at)
    }
}

pub fn compute_checksum(vars: &HashMap<String, String>) -> String {
    let mut keys: Vec<&String> = vars.keys().collect();
    keys.sort();
    let raw: String = keys
        .iter()
        .map(|k| format!("{}={}", k, vars[*k]))
        .collect::<Vec<_>>()
        .join("\n");
    format!("{:x}", md5_simple(&raw))
}

fn md5_simple(input: &str) -> u64 {
    // Lightweight non-cryptographic checksum for change detection
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in input.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

pub fn generate_backup_id() -> String {
    let ts = Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("bkp_{:x}", ts as u64)
}
