//! Rename environment variable keys across .env files.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct RenameResult {
    pub old_key: String,
    pub new_key: String,
    pub renamed: bool,
    pub reason: Option<String>,
}

/// Rename a key in a map of env vars. Returns the updated map and a result.
pub fn rename_key(
    env: &HashMap<String, String>,
    old_key: &str,
    new_key: &str,
    force: bool,
) -> (HashMap<String, String>, RenameResult) {
    let mut updated = env.clone();

    if !env.contains_key(old_key) {
        return (
            updated,
            RenameResult {
                old_key: old_key.to_string(),
                new_key: new_key.to_string(),
                renamed: false,
                reason: Some(format!("Key '{}' not found", old_key)),
            },
        );
    }

    if env.contains_key(new_key) && !force {
        return (
            updated,
            RenameResult {
                old_key: old_key.to_string(),
                new_key: new_key.to_string(),
                renamed: false,
                reason: Some(format!("Key '{}' already exists; use force to overwrite", new_key)),
            },
        );
    }

    let value = updated.remove(old_key).unwrap();
    updated.insert(new_key.to_string(), value);

    (
        updated,
        RenameResult {
            old_key: old_key.to_string(),
            new_key: new_key.to_string(),
            renamed: true,
            reason: None,
        },
    )
}

/// Bulk rename keys using a list of (old, new) pairs.
pub fn bulk_rename(
    env: &HashMap<String, String>,
    renames: &[(String, String)],
    force: bool,
) -> (HashMap<String, String>, Vec<RenameResult>) {
    let mut current = env.clone();
    let mut results = Vec::new();

    for (old_key, new_key) in renames {
        let (next, result) = rename_key(&current, old_key, new_key, force);
        current = next;
        results.push(result);
    }

    (current, results)
}
