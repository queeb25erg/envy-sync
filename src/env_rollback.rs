//! env_rollback: Roll back environment variables to a previous snapshot or version.

use std::collections::HashMap;
use crate::snapshot::{Snapshot, SnapshotId};

#[derive(Debug, Clone, PartialEq)]
pub struct RollbackResult {
    pub restored_keys: Vec<String>,
    pub removed_keys: Vec<String>,
    pub unchanged_keys: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct RollbackOptions {
    pub dry_run: bool,
    pub keys: Option<Vec<String>>,
}

impl Default for RollbackOptions {
    fn default() -> Self {
        RollbackOptions {
            dry_run: false,
            keys: None,
        }
    }
}

pub fn rollback_to_snapshot(
    current: &HashMap<String, String>,
    target: &Snapshot,
    opts: &RollbackOptions,
) -> RollbackResult {
    let mut restored_keys = Vec::new();
    let mut removed_keys = Vec::new();
    let mut unchanged_keys = Vec::new();

    let filter: Option<&Vec<String>> = opts.keys.as_ref();

    for (key, value) in &target.vars {
        if let Some(keys) = filter {
            if !keys.contains(key) {
                continue;
            }
        }
        match current.get(key) {
            Some(current_val) if current_val == value => unchanged_keys.push(key.clone()),
            _ => restored_keys.push(key.clone()),
        }
    }

    for key in current.keys() {
        if let Some(keys) = filter {
            if !keys.contains(key) {
                continue;
            }
        }
        if !target.vars.contains_key(key) {
            removed_keys.push(key.clone());
        }
    }

    RollbackResult {
        restored_keys,
        removed_keys,
        unchanged_keys,
    }
}

pub fn apply_rollback(
    current: &mut HashMap<String, String>,
    target: &Snapshot,
    opts: &RollbackOptions,
) -> RollbackResult {
    let result = rollback_to_snapshot(current, target, opts);
    if !opts.dry_run {
        for key in &result.restored_keys {
            if let Some(val) = target.vars.get(key) {
                current.insert(key.clone(), val.clone());
            }
        }
        for key in &result.removed_keys {
            current.remove(key);
        }
    }
    result
}
