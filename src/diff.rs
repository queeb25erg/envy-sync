//! Diff module for comparing local and remote .env file states.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum DiffEntry {
    Added(String),
    Removed(String),
    Modified { local: String, remote: String },
    Unchanged(String),
}

#[derive(Debug, Default)]
pub struct EnvDiff {
    pub entries: HashMap<String, DiffEntry>,
}

impl EnvDiff {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn has_changes(&self) -> bool {
        self.entries.values().any(|e| !matches!(e, DiffEntry::Unchanged(_)))
    }

    pub fn added(&self) -> Vec<&str> {
        self.entries
            .iter()
            .filter_map(|(k, v)| matches!(v, DiffEntry::Added(_)).then_some(k.as_str()))
            .collect()
    }

    pub fn removed(&self) -> Vec<&str> {
        self.entries
            .iter()
            .filter_map(|(k, v)| matches!(v, DiffEntry::Removed(_)).then_some(k.as_str()))
            .collect()
    }

    pub fn modified(&self) -> Vec<&str> {
        self.entries
            .iter()
            .filter_map(|(k, v)| matches!(v, DiffEntry::Modified { .. }).then_some(k.as_str()))
            .collect()
    }
}

/// Compute a diff between local and remote env maps.
pub fn compute_diff(
    local: &HashMap<String, String>,
    remote: &HashMap<String, String>,
) -> EnvDiff {
    let mut diff = EnvDiff::new();

    for (key, local_val) in local {
        match remote.get(key) {
            Some(remote_val) if remote_val == local_val => {
                diff.entries.insert(key.clone(), DiffEntry::Unchanged(local_val.clone()));
            }
            Some(remote_val) => {
                diff.entries.insert(
                    key.clone(),
                    DiffEntry::Modified {
                        local: local_val.clone(),
                        remote: remote_val.clone(),
                    },
                );
            }
            None => {
                diff.entries.insert(key.clone(), DiffEntry::Added(local_val.clone()));
            }
        }
    }

    for (key, remote_val) in remote {
        if !local.contains_key(key) {
            diff.entries.insert(key.clone(), DiffEntry::Removed(remote_val.clone()));
        }
    }

    diff
}
