//! CLI command handlers for tag management.

use crate::tag::TagStore;

#[derive(Debug)]
pub enum TagCommand {
    Add { env: String, tag: String },
    Remove { env: String, tag: String },
    List { env: String },
    Find { tag: String },
}

/// Execute a tag command against a mutable TagStore.
/// Returns a human-readable result string.
pub fn execute(store: &mut TagStore, cmd: TagCommand) -> String {
    match cmd {
        TagCommand::Add { env, tag } => {
            let added = store.add_tag(&env, &tag);
            if added {
                format!("Tag '{}' added to environment '{}'.", tag, env)
            } else {
                format!("Tag '{}' already exists on environment '{}'.", tag, env)
            }
        }
        TagCommand::Remove { env, tag } => {
            let removed = store.remove_tag(&env, &tag);
            if removed {
                format!("Tag '{}' removed from environment '{}'.", tag, env)
            } else {
                format!("Tag '{}' not found on environment '{}'.", tag, env)
            }
        }
        TagCommand::List { env } => {
            let tags = store.get_tags(&env);
            if tags.is_empty() {
                format!("No tags found for environment '{}'.", env)
            } else {
                format!("Tags for '{}': {}", env, tags.join(", "))
            }
        }
        TagCommand::Find { tag } => {
            let envs = store.find_by_tag(&tag);
            if envs.is_empty() {
                format!("No environments found with tag '{}'.", tag)
            } else {
                format!("Environments tagged '{}': {}", tag, envs.join(", "))
            }
        }
    }
}
