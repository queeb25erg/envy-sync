//! CLI interface for env-notify: display change notifications between env snapshots.

use crate::env_notify::{detect_changes, NotifyConfig};
use std::collections::HashMap;

#[derive(Debug)]
pub struct NotifyCliArgs {
    pub before_path: String,
    pub after_path: String,
    pub watch_keys: Vec<String>,
    pub ignore_keys: Vec<String>,
    pub quiet: bool,
}

fn parse_env_file(content: &str) -> HashMap<String, String> {
    content
        .lines()
        .filter(|l| !l.trim_start().starts_with('#') && l.contains('='))
        .filter_map(|l| {
            let mut parts = l.splitn(2, '=');
            let key = parts.next()?.trim().to_string();
            let val = parts.next().unwrap_or("").trim().to_string();
            if key.is_empty() {
                None
            } else {
                Some((key, val))
            }
        })
        .collect()
}

pub fn run_notify_cli(args: &NotifyCliArgs, before_content: &str, after_content: &str) -> String {
    let before = parse_env_file(before_content);
    let after = parse_env_file(after_content);

    let config = NotifyConfig {
        enabled: true,
        watch_keys: args.watch_keys.clone(),
        ignore_keys: args.ignore_keys.clone(),
    };

    let result = detect_changes(&before, &after, &config);

    if args.quiet {
        if result.is_empty() {
            return String::new();
        }
        return format!("{} change(s) detected.", result.events.len());
    }

    result.summary()
}
