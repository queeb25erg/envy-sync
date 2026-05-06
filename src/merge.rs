use std::collections::HashMap;
use crate::diff::{DiffEntry, DiffKind};

/// Strategy for resolving conflicts between local and remote env values.
#[derive(Debug, Clone, PartialEq)]
pub enum MergeStrategy {
    /// Remote value always wins on conflict.
    PreferRemote,
    /// Local value always wins on conflict.
    PreferLocal,
    /// Return an error on conflict; caller must resolve manually.
    ErrorOnConflict,
}

#[derive(Debug, PartialEq)]
pub struct MergeConflict {
    pub key: String,
    pub local_value: String,
    pub remote_value: String,
}

#[derive(Debug)]
pub struct MergeResult {
    pub merged: HashMap<String, String>,
    pub conflicts: Vec<MergeConflict>,
}

/// Merge local and remote env maps using the given strategy.
/// `diffs` is the set of differences computed by `diff::compute`.
pub fn merge(
    local: &HashMap<String, String>,
    remote: &HashMap<String, String>,
    diffs: &[DiffEntry],
    strategy: &MergeStrategy,
) -> Result<MergeResult, String> {
    let mut merged = local.clone();
    let mut conflicts = Vec::new();

    for entry in diffs {
        match &entry.kind {
            DiffKind::Added => {
                // Key exists only in remote — add it locally.
                if let Some(val) = remote.get(&entry.key) {
                    merged.insert(entry.key.clone(), val.clone());
                }
            }
            DiffKind::Removed => {
                // Key was removed in remote — remove locally.
                merged.remove(&entry.key);
            }
            DiffKind::Modified { remote_value, .. } => {
                let local_val = local
                    .get(&entry.key)
                    .cloned()
                    .unwrap_or_default();

                match strategy {
                    MergeStrategy::PreferRemote => {
                        merged.insert(entry.key.clone(), remote_value.clone());
                    }
                    MergeStrategy::PreferLocal => {
                        // Keep local — nothing to do.
                    }
                    MergeStrategy::ErrorOnConflict => {
                        conflicts.push(MergeConflict {
                            key: entry.key.clone(),
                            local_value: local_val,
                            remote_value: remote_value.clone(),
                        });
                    }
                }
            }
        }
    }

    if !conflicts.is_empty() {
        return Err(format!(
            "Merge conflict(s) on keys: {}",
            conflicts
                .iter()
                .map(|c| c.key.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }

    Ok(MergeResult { merged, conflicts })
}
