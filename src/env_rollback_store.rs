//! env_rollback_store: Persistence layer for rollback operation history.

use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct RollbackRecord {
    pub snapshot_id: String,
    pub performed_at: DateTime<Utc>,
    pub restored_keys: Vec<String>,
    pub removed_keys: Vec<String>,
    pub dry_run: bool,
}

#[derive(Debug, Default)]
pub struct RollbackStore {
    records: Vec<RollbackRecord>,
}

impl RollbackStore {
    pub fn new() -> Self {
        RollbackStore { records: Vec::new() }
    }

    pub fn record(&mut self, entry: RollbackRecord) {
        self.records.push(entry);
    }

    pub fn history(&self) -> &[RollbackRecord] {
        &self.records
    }

    pub fn last(&self) -> Option<&RollbackRecord> {
        self.records.last()
    }

    pub fn find_by_snapshot(&self, snapshot_id: &str) -> Vec<&RollbackRecord> {
        self.records
            .iter()
            .filter(|r| r.snapshot_id == snapshot_id)
            .collect()
    }

    pub fn clear(&mut self) {
        self.records.clear();
    }

    pub fn count(&self) -> usize {
        self.records.len()
    }
}
