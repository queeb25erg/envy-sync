//! env_expire.rs — Track and enforce expiry dates on environment variables.

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExpiryEntry {
    pub key: String,
    pub expires_at: DateTime<Utc>,
    pub warn_before_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExpiryStore {
    pub entries: HashMap<String, ExpiryEntry>,
}

#[derive(Debug, PartialEq)]
pub enum ExpiryStatus {
    Valid,
    WarningSoon { days_left: i64 },
    Expired { days_ago: i64 },
}

impl ExpiryStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, key: &str, expires_at: DateTime<Utc>, warn_before_days: u32) {
        self.entries.insert(
            key.to_string(),
            ExpiryEntry {
                key: key.to_string(),
                expires_at,
                warn_before_days,
            },
        );
    }

    pub fn remove(&mut self, key: &str) -> bool {
        self.entries.remove(key).is_some()
    }

    pub fn status(&self, key: &str, now: DateTime<Utc>) -> Option<ExpiryStatus> {
        let entry = self.entries.get(key)?;
        let diff = entry.expires_at.signed_duration_since(now);
        let days = diff.num_days();
        if days < 0 {
            Some(ExpiryStatus::Expired { days_ago: -days })
        } else if days < entry.warn_before_days as i64 {
            Some(ExpiryStatus::WarningSoon { days_left: days })
        } else {
            Some(ExpiryStatus::Valid)
        }
    }

    pub fn expired_keys(&self, now: DateTime<Utc>) -> Vec<&str> {
        self.entries
            .values()
            .filter(|e| e.expires_at <= now)
            .map(|e| e.key.as_str())
            .collect()
    }

    pub fn warning_keys(&self, now: DateTime<Utc>) -> Vec<(&str, i64)> {
        self.entries
            .values()
            .filter_map(|e| {
                let diff = e.expires_at.signed_duration_since(now);
                let days = diff.num_days();
                if days >= 0 && days < e.warn_before_days as i64 {
                    Some((e.key.as_str(), days))
                } else {
                    None
                }
            })
            .collect()
    }
}
