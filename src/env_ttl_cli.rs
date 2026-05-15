//! CLI interface for TTL management commands.

use crate::env_ttl::TtlStore;

#[derive(Debug, PartialEq)]
pub enum TtlCommand {
    Set { key: String, ttl_seconds: u64 },
    Get { key: String },
    Remove { key: String },
    ListExpired,
    Purge,
    List,
}

pub fn parse_ttl_command(args: &[&str]) -> Result<TtlCommand, String> {
    match args {
        ["set", key, secs] => {
            let ttl = secs.parse::<u64>().map_err(|_| format!("Invalid TTL seconds: {}", secs))?;
            Ok(TtlCommand::Set { key: key.to_string(), ttl_seconds: ttl })
        }
        ["get", key] => Ok(TtlCommand::Get { key: key.to_string() }),
        ["remove", key] => Ok(TtlCommand::Remove { key: key.to_string() }),
        ["list-expired"] => Ok(TtlCommand::ListExpired),
        ["purge"] => Ok(TtlCommand::Purge),
        ["list"] => Ok(TtlCommand::List),
        _ => Err(format!("Unknown TTL command: {:?}", args)),
    }
}

pub fn run_ttl_command(cmd: TtlCommand, store: &mut TtlStore) -> String {
    match cmd {
        TtlCommand::Set { key, ttl_seconds } => {
            store.set(&key, ttl_seconds);
            format!("TTL set for '{}': {} seconds", key, ttl_seconds)
        }
        TtlCommand::Get { key } => match store.get(&key) {
            Some(entry) => {
                let remaining = entry.seconds_remaining();
                if remaining <= 0 {
                    format!("'{}' is expired", key)
                } else {
                    format!("'{}' expires in {} seconds", key, remaining)
                }
            }
            None => format!("No TTL set for '{}'", key),
        },
        TtlCommand::Remove { key } => {
            if store.remove(&key) {
                format!("TTL removed for '{}'", key)
            } else {
                format!("No TTL found for '{}'", key)
            }
        }
        TtlCommand::ListExpired => {
            let keys = store.expired_keys();
            if keys.is_empty() {
                "No expired keys".to_string()
            } else {
                keys.join("\n")
            }
        }
        TtlCommand::Purge => {
            let purged = store.purge_expired();
            format!("Purged {} expired key(s)", purged.len())
        }
        TtlCommand::List => {
            let entries = store.all_entries();
            if entries.is_empty() {
                return "No TTL entries".to_string();
            }
            let mut lines: Vec<String> = entries
                .iter()
                .map(|e| format!("{}: {} secs remaining", e.key, e.seconds_remaining().max(0)))
                .collect();
            lines.sort();
            lines.join("\n")
        }
    }
}
