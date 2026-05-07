//! CLI handlers for snapshot commands: create, list, restore, delete.

use crate::snapshot::{Snapshot, SnapshotStore};
use std::collections::HashMap;

/// Print a summary table of all snapshots in the store.
pub fn cmd_list(store: &SnapshotStore) {
    if store.snapshots.is_empty() {
        println!("No snapshots found.");
        return;
    }
    println!("{:<20} {:<30} {:<10} {}", "ID", "Created At", "Vars", "Label");
    println!("{}", "-".repeat(75));
    for s in &store.snapshots {
        let label = s.label.clone().unwrap_or_else(|| "-".to_string());
        println!(
            "{:<20} {:<30} {:<10} {}",
            s.id,
            s.created_at.format("%Y-%m-%d %H:%M:%S UTC"),
            s.var_count(),
            label
        );
    }
}

/// Create a new snapshot from the provided vars map and add it to the store.
pub fn cmd_create(
    store: &mut SnapshotStore,
    vars: HashMap<String, String>,
    label: Option<String>,
) -> String {
    let snapshot = Snapshot::new(vars, label);
    let id = snapshot.id.clone();
    let name = snapshot.display_name();
    store.add(snapshot);
    println!("Snapshot created: {}", name);
    id
}

/// Restore variables from a snapshot by ID. Returns the vars map if found.
pub fn cmd_restore(store: &SnapshotStore, id: &str) -> Option<HashMap<String, String>> {
    match store.find_by_id(id) {
        Some(snapshot) => {
            println!("Restoring snapshot: {}", snapshot.display_name());
            Some(snapshot.vars.clone())
        }
        None => {
            eprintln!("Error: snapshot '{}' not found.", id);
            None
        }
    }
}

/// Delete a snapshot by ID from the store.
pub fn cmd_delete(store: &mut SnapshotStore, id: &str) -> bool {
    if store.remove_by_id(id) {
        println!("Snapshot '{}' deleted.", id);
        true
    } else {
        eprintln!("Error: snapshot '{}' not found.", id);
        false
    }
}
