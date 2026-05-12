//! Environment variable history tracking: record and retrieve past values for keys.

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HistoryEntry {
    pub key: String,
    pub value: String,
    pub changed_at: DateTime<Utc>,
    pub changed_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnvHistory {
    /// Maps env key -> list of historical entries (oldest first)
    pub entries: HashMap<String, Vec<HistoryEntry>>,
}

impl EnvHistory {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a new value for a key.
    pub fn record(&mut self, key: &str, value: &str, changed_by: &str) {
        let entry = HistoryEntry {
            key: key.to_string(),
            value: value.to_string(),
            changed_at: Utc::now(),
            changed_by: changed_by.to_string(),
        };
        self.entries
            .entry(key.to_string())
            .or_default()
            .push(entry);
    }

    /// Get the full history for a key, oldest first.
    pub fn get(&self, key: &str) -> Vec<&HistoryEntry> {
        self.entries
            .get(key)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    /// Get the most recent entry for a key.
    pub fn latest(&self, key: &str) -> Option<&HistoryEntry> {
        self.entries.get(key)?.last()
    }

    /// Return all keys that have history.
    pub fn tracked_keys(&self) -> Vec<&String> {
        self.entries.keys().collect()
    }

    /// Trim history for a key to at most `max_entries` most recent entries.
    pub fn trim(&mut self, key: &str, max_entries: usize) {
        if let Some(entries) = self.entries.get_mut(key) {
            if entries.len() > max_entries {
                let drain_count = entries.len() - max_entries;
                entries.drain(..drain_count);
            }
        }
    }

    /// Clear all history for a key.
    pub fn clear_key(&mut self, key: &str) {
        self.entries.remove(key);
    }
}
