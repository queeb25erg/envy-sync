//! TTL (time-to-live) management for environment variables.
//! Allows setting expiry durations on individual keys and checking staleness.

use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq)]
pub struct TtlEntry {
    pub key: String,
    pub expires_at: u64, // Unix timestamp
}

impl TtlEntry {
    pub fn new(key: &str, ttl_seconds: u64) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_secs();
        TtlEntry {
            key: key.to_string(),
            expires_at: now + ttl_seconds,
        }
    }

    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_secs();
        now >= self.expires_at
    }

    pub fn seconds_remaining(&self) -> i64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_secs();
        self.expires_at as i64 - now as i64
    }
}

#[derive(Debug, Default)]
pub struct TtlStore {
    entries: HashMap<String, TtlEntry>,
}

impl TtlStore {
    pub fn new() -> Self {
        TtlStore {
            entries: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, ttl_seconds: u64) {
        self.entries.insert(key.to_string(), TtlEntry::new(key, ttl_seconds));
    }

    pub fn get(&self, key: &str) -> Option<&TtlEntry> {
        self.entries.get(key)
    }

    pub fn remove(&mut self, key: &str) -> bool {
        self.entries.remove(key).is_some()
    }

    pub fn expired_keys(&self) -> Vec<String> {
        self.entries
            .values()
            .filter(|e| e.is_expired())
            .map(|e| e.key.clone())
            .collect()
    }

    pub fn purge_expired(&mut self) -> Vec<String> {
        let expired = self.expired_keys();
        for key in &expired {
            self.entries.remove(key);
        }
        expired
    }

    pub fn all_entries(&self) -> Vec<&TtlEntry> {
        self.entries.values().collect()
    }
}
