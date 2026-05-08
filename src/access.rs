//! Access control for env file operations.
//! Tracks which users/machines are allowed to read or write specific keys.

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub enum Permission {
    Read,
    Write,
    Admin,
}

#[derive(Debug, Clone)]
pub struct AccessEntry {
    pub identity: String,
    pub permissions: HashSet<String>, // e.g. "read", "write", "admin"
}

#[derive(Debug, Clone, Default)]
pub struct AccessControl {
    /// Map from key pattern (or "*" for all) to list of access entries
    entries: HashMap<String, Vec<AccessEntry>>,
}

impl AccessControl {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn grant(&mut self, key_pattern: &str, identity: &str, perms: Vec<&str>) {
        let entry = AccessEntry {
            identity: identity.to_string(),
            permissions: perms.iter().map(|s| s.to_string()).collect(),
        };
        self.entries
            .entry(key_pattern.to_string())
            .or_default()
            .push(entry);
    }

    pub fn revoke(&mut self, key_pattern: &str, identity: &str) {
        if let Some(entries) = self.entries.get_mut(key_pattern) {
            entries.retain(|e| e.identity != identity);
        }
    }

    pub fn can(&self, identity: &str, key: &str, perm: &str) -> bool {
        // Check exact key match and wildcard
        for pattern in [key, "*"] {
            if let Some(entries) = self.entries.get(pattern) {
                for entry in entries {
                    if entry.identity == identity && entry.permissions.contains(perm) {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn list_identities(&self, key_pattern: &str) -> Vec<String> {
        self.entries
            .get(key_pattern)
            .map(|entries| entries.iter().map(|e| e.identity.clone()).collect())
            .unwrap_or_default()
    }

    pub fn summary(&self) -> HashMap<String, Vec<String>> {
        self.entries
            .iter()
            .map(|(k, v)| {
                let ids: Vec<String> = v.iter().map(|e| e.identity.clone()).collect();
                (k.clone(), ids)
            })
            .collect()
    }
}
