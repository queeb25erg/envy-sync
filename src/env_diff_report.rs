//! Generates human-readable diff reports between two .env file states.

use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum DiffEntry {
    Added(String),
    Removed(String),
    Modified { old: String, new: String },
    Unchanged(String),
}

#[derive(Debug, Clone)]
pub struct EnvDiffReport {
    pub entries: Vec<(String, DiffEntry)>,
}

impl EnvDiffReport {
    pub fn generate(
        old: &HashMap<String, String>,
        new: &HashMap<String, String>,
    ) -> Self {
        let mut entries = Vec::new();

        let mut all_keys: Vec<String> = old.keys().chain(new.keys()).cloned().collect();
        all_keys.sort();
        all_keys.dedup();

        for key in all_keys {
            let entry = match (old.get(&key), new.get(&key)) {
                (None, Some(v)) => DiffEntry::Added(v.clone()),
                (Some(v), None) => DiffEntry::Removed(v.clone()),
                (Some(o), Some(n)) if o != n => DiffEntry::Modified {
                    old: o.clone(),
                    new: n.clone(),
                },
                (Some(v), Some(_)) => DiffEntry::Unchanged(v.clone()),
                (None, None) => continue,
            };
            entries.push((key, entry));
        }

        EnvDiffReport { entries }
    }

    pub fn has_changes(&self) -> bool {
        self.entries.iter().any(|(_, e)| !matches!(e, DiffEntry::Unchanged(_)))
    }

    pub fn summary(&self) -> (usize, usize, usize) {
        let added = self.entries.iter().filter(|(_, e)| matches!(e, DiffEntry::Added(_))).count();
        let removed = self.entries.iter().filter(|(_, e)| matches!(e, DiffEntry::Removed(_))).count();
        let modified = self.entries.iter().filter(|(_, e)| matches!(e, DiffEntry::Modified { .. })).count();
        (added, removed, modified)
    }
}

impl fmt::Display for EnvDiffReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (key, entry) in &self.entries {
            match entry {
                DiffEntry::Added(v) => writeln!(f, "+ {}={}", key, v)?,
                DiffEntry::Removed(v) => writeln!(f, "- {}={}", key, v)?,
                DiffEntry::Modified { old, new } => {
                    writeln!(f, "~ {} (was: {}, now: {})", key, old, new)?;
                }
                DiffEntry::Unchanged(_) => {}
            }
        }
        let (a, r, m) = self.summary();
        write!(f, "Summary: +{} added, -{} removed, ~{} modified", a, r, m)
    }
}
