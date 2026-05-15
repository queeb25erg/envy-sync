//! env_rollback_cli: CLI interface for the rollback feature.

use crate::env_rollback::{apply_rollback, RollbackOptions};
use crate::snapshot::Snapshot;
use std::collections::HashMap;

#[derive(Debug)]
pub struct RollbackCliArgs {
    pub snapshot_id: String,
    pub dry_run: bool,
    pub keys: Option<Vec<String>>,
    pub verbose: bool,
}

pub fn run_rollback_cli(
    args: &RollbackCliArgs,
    current: &mut HashMap<String, String>,
    snapshot: &Snapshot,
) -> Result<(), String> {
    if snapshot.id != args.snapshot_id {
        return Err(format!(
            "Snapshot ID mismatch: expected '{}', got '{}'",
            args.snapshot_id, snapshot.id
        ));
    }

    let opts = RollbackOptions {
        dry_run: args.dry_run,
        keys: args.keys.clone(),
    };

    let result = apply_rollback(current, snapshot, &opts);

    if args.dry_run {
        println!("[dry-run] Rollback to snapshot '{}'", args.snapshot_id);
    } else {
        println!("Rolled back to snapshot '{}'", args.snapshot_id);
    }

    if args.verbose {
        for key in &result.restored_keys {
            println!("  restored: {}", key);
        }
        for key in &result.removed_keys {
            println!("  removed:  {}", key);
        }
        for key in &result.unchanged_keys {
            println!("  unchanged: {}", key);
        }
    } else {
        println!(
            "  {} restored, {} removed, {} unchanged",
            result.restored_keys.len(),
            result.removed_keys.len(),
            result.unchanged_keys.len()
        );
    }

    Ok(())
}
