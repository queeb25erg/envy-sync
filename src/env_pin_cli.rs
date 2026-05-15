//! CLI interface for env_pin: pin/unpin keys and list pinned keys.

use crate::env_pin::PinSet;

#[derive(Debug, PartialEq)]
pub enum PinCommand {
    Add(String),
    Remove(String),
    List,
    Check(String),
}

pub fn parse_pin_command(args: &[&str]) -> Result<PinCommand, String> {
    match args {
        ["add", key] => Ok(PinCommand::Add(key.to_string())),
        ["remove", key] => Ok(PinCommand::Remove(key.to_string())),
        ["list"] => Ok(PinCommand::List),
        ["check", key] => Ok(PinCommand::Check(key.to_string())),
        _ => Err(format!(
            "Unknown pin command: {:?}. Use add <KEY>, remove <KEY>, list, or check <KEY>.",
            args
        )),
    }
}

pub fn run_pin_command(cmd: PinCommand, pin_set: &mut PinSet) -> String {
    match cmd {
        PinCommand::Add(key) => {
            pin_set.pin(&key);
            format!("Pinned: {}", key)
        }
        PinCommand::Remove(key) => {
            if pin_set.unpin(&key) {
                format!("Unpinned: {}", key)
            } else {
                format!("Key '{}' was not pinned.", key)
            }
        }
        PinCommand::List => {
            let keys = pin_set.list();
            if keys.is_empty() {
                "No keys are currently pinned.".to_string()
            } else {
                format!("Pinned keys:\n{}", keys.join("\n"))
            }
        }
        PinCommand::Check(key) => {
            if pin_set.is_pinned(&key) {
                format!("'{}' is pinned.", key)
            } else {
                format!("'{}' is not pinned.", key)
            }
        }
    }
}
