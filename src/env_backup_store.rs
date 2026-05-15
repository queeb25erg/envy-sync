//! env_backup_store.rs — Persist and load backup metadata from disk.

use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use crate::env_backup::BackupStore;

const BACKUP_INDEX_FILE: &str = ".envy_backups.json";

pub struct BackupStoreManager {
    index_path: PathBuf,
}

impl BackupStoreManager {
    pub fn new(base_dir: &Path) -> Self {
        Self {
            index_path: base_dir.join(BACKUP_INDEX_FILE),
        }
    }

    pub fn load(&self) -> Result<BackupStore> {
        if !self.index_path.exists() {
            return Ok(BackupStore::new());
        }
        let content = fs::read_to_string(&self.index_path)
            .with_context(|| format!("Failed to read backup index: {:?}", self.index_path))?;
        serde_json::from_str(&content)
            .with_context(|| "Failed to parse backup index JSON")
    }

    pub fn save(&self, store: &BackupStore) -> Result<()> {
        if let Some(parent) = self.index_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| "Failed to create backup directory")?;
        }
        let json = serde_json::to_string_pretty(store)
            .with_context(|| "Failed to serialize backup store")?;
        fs::write(&self.index_path, json)
            .with_context(|| format!("Failed to write backup index: {:?}", self.index_path))
    }

    pub fn backup_dir(&self) -> PathBuf {
        self.index_path
            .parent()
            .unwrap_or(Path::new("."))
            .join(".envy_backup_files")
    }

    pub fn write_backup_file(&self, id: &str, content: &str) -> Result<PathBuf> {
        let dir = self.backup_dir();
        fs::create_dir_all(&dir)
            .with_context(|| "Failed to create backup files directory")?;
        let path = dir.join(format!("{}.env", id));
        fs::write(&path, content)
            .with_context(|| format!("Failed to write backup file: {:?}", path))?;
        Ok(path)
    }

    pub fn read_backup_file(&self, id: &str) -> Result<String> {
        let path = self.backup_dir().join(format!("{}.env", id));
        fs::read_to_string(&path)
            .with_context(|| format!("Backup file not found: {:?}", path))
    }
}
