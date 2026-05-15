//! Tracks a persistent audit trail of all env variable mutations
//! (set, delete, rotate, import, export) with timestamps and actor info.

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MutationKind {
    Set,
    Delete,
    Rotate,
    Import,
    Export,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub key: String,
    pub kind: MutationKind,
    pub actor: String,
    pub timestamp: DateTime<Utc>,
    pub profile: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AuditTrail {
    entries: Vec<AuditEntry>,
}

impl AuditTrail {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(
        &mut self,
        key: impl Into<String>,
        kind: MutationKind,
        actor: impl Into<String>,
        profile: Option<String>,
        note: Option<String>,
    ) {
        self.entries.push(AuditEntry {
            key: key.into(),
            kind,
            actor: actor.into(),
            timestamp: Utc::now(),
            profile,
            note,
        });
    }

    pub fn entries_for_key(&self, key: &str) -> Vec<&AuditEntry> {
        self.entries.iter().filter(|e| e.key == key).collect()
    }

    pub fn entries_by_actor(&self, actor: &str) -> Vec<&AuditEntry> {
        self.entries.iter().filter(|e| e.actor == actor).collect()
    }

    pub fn all_entries(&self) -> &[AuditEntry] {
        &self.entries
    }

    pub fn mutation_counts(&self) -> HashMap<String, usize> {
        let mut counts: HashMap<String, usize> = HashMap::new();
        for entry in &self.entries {
            *counts.entry(entry.key.clone()).or_insert(0) += 1;
        }
        counts
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
