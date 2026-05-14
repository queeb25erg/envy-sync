use crate::env_group::{group_by_prefix, filter_by_group};
use std::collections::HashMap;

/// CLI command: list all groups detected from the given env map.
pub fn cmd_list_groups(env: &HashMap<String, String>) {
    let groups = group_by_prefix(env);
    if groups.is_empty() {
        println!("No environment variables found.");
        return;
    }
    let mut names: Vec<&String> = groups.keys().collect();
    names.sort();
    for name in names {
        let group = &groups[name];
        println!("[{}] ({} keys)", group.name, group.keys.len());
        for key in &group.keys {
            println!("  - {}", key);
        }
    }
}

/// CLI command: show only the env vars belonging to a specific prefix group.
pub fn cmd_show_group(env: &HashMap<String, String>, prefix: &str) {
    let groups = group_by_prefix(env);
    match groups.get(prefix) {
        Some(group) => {
            let filtered = filter_by_group(env, group);
            let mut pairs: Vec<(&&str, &&str)> = filtered.iter().collect();
            pairs.sort_by_key(|(k, _)| **k);
            println!("Group: {}", prefix);
            for (k, v) in pairs {
                println!("  {}={}", k, v);
            }
        }
        None => {
            eprintln!("No group found with prefix '{}'.", prefix);
        }
    }
}
