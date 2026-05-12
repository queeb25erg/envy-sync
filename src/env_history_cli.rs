//! CLI commands for env history: `history show <KEY>`, `history clear <KEY>`.

use crate::env_history::EnvHistory;

#[derive(Debug)]
pub enum HistoryCommand {
    Show { key: String, limit: Option<usize> },
    Clear { key: String },
    List,
}

pub fn run_history_command(cmd: HistoryCommand, history: &mut EnvHistory) {
    match cmd {
        HistoryCommand::Show { key, limit } => {
            let entries = history.get(&key);
            if entries.is_empty() {
                println!("No history found for key: {}", key);
                return;
            }
            let display: Vec<_> = match limit {
                Some(n) => entries.into_iter().rev().take(n).collect(),
                None => entries.into_iter().rev().collect(),
            };
            println!("History for '{}':", key);
            for entry in display {
                println!(
                    "  [{}] {} = {} (by {})",
                    entry.changed_at.format("%Y-%m-%d %H:%M:%S"),
                    entry.key,
                    entry.value,
                    entry.changed_by
                );
            }
        }
        HistoryCommand::Clear { key } => {
            history.clear_key(&key);
            println!("Cleared history for key: {}", key);
        }
        HistoryCommand::List => {
            let mut keys = history.tracked_keys();
            keys.sort();
            if keys.is_empty() {
                println!("No history recorded yet.");
            } else {
                println!("Keys with history:");
                for k in keys {
                    println!("  {}", k);
                }
            }
        }
    }
}
