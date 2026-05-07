//! Snapshot module: capture and restore point-in-time states of .env files.

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A snapshot of environment variables at a specific point in time.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Snapshot {
    pub id: String,
    pub label: Option<String>,
    pub created_at: DateTime<Utc>,
    pub vars: HashMap<String, String>,
}

impl Snapshot {
    /// Create a new snapshot from a map of environment variables.
    pub fn new(vars: HashMap<String, String>, label: Option<String>) -> Self {
        let id = format!("{}", Utc::now().timestamp_millis());
        Snapshot {
            id,
            label,
            created_at: Utc::now(),
            vars,
        }
    }

    /// Serialize the snapshot to a JSON string.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserialize a snapshot from a JSON string.
    pub fn from_json(data: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(data)
    }

    /// Returns the number of variables in the snapshot.
    pub fn var_count(&self) -> usize {
        self.vars.len()
    }

    /// Returns a human-readable display name for the snapshot.
    pub fn display_name(&self) -> String {
        match &self.label {
            Some(l) => format!("{} ({})", l, self.id),
            None => format!("snapshot-{}", self.id),
        }
    }
}

/// Manages a collection of snapshots.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SnapshotStore {
    pub snapshots: Vec<Snapshot>,
}

impl SnapshotStore {
    pub fn new() -> Self {
        SnapshotStore { snapshots: vec![] }
    }

    pub fn add(&mut self, snapshot: Snapshot) {
        self.snapshots.push(snapshot);
    }

    pub fn find_by_id(&self, id: &str) -> Option<&Snapshot> {
        self.snapshots.iter().find(|s| s.id == id)
    }

    pub fn latest(&self) -> Option<&Snapshot> {
        self.snapshots.iter().max_by_key(|s| s.created_at)
    }

    pub fn remove_by_id(&mut self, id: &str) -> bool {
        let before = self.snapshots.len();
        self.snapshots.retain(|s| s.id != id);
        self.snapshots.len() < before
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(data: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(data)
    }
}
