//! Notification system for environment variable change events.
//!
//! Supports notifying when variables are added, removed, or modified
//! across sync operations.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum NotifyEvent {
    Added(String),
    Removed(String),
    Modified(String),
    Expired(String),
}

#[derive(Debug, Clone)]
pub struct NotifyConfig {
    pub enabled: bool,
    pub watch_keys: Vec<String>,
    pub ignore_keys: Vec<String>,
}

impl Default for NotifyConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            watch_keys: vec![],
            ignore_keys: vec![],
        }
    }
}

#[derive(Debug, Default)]
pub struct NotifyResult {
    pub events: Vec<NotifyEvent>,
}

impl NotifyResult {
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn summary(&self) -> String {
        if self.events.is_empty() {
            return "No changes detected.".to_string();
        }
        let mut lines = vec![];
        for event in &self.events {
            let line = match event {
                NotifyEvent::Added(k) => format!("+ {k} (added)"),
                NotifyEvent::Removed(k) => format!("- {k} (removed)"),
                NotifyEvent::Modified(k) => format!("~ {k} (modified)"),
                NotifyEvent::Expired(k) => format!("! {k} (expired)"),
            };
            lines.push(line);
        }
        lines.join("\n")
    }
}

pub fn detect_changes(
    before: &HashMap<String, String>,
    after: &HashMap<String, String>,
    config: &NotifyConfig,
) -> NotifyResult {
    let mut events = vec![];

    let should_watch = |key: &str| -> bool {
        if config.ignore_keys.iter().any(|k| k == key) {
            return false;
        }
        if config.watch_keys.is_empty() {
            return true;
        }
        config.watch_keys.iter().any(|k| k == key)
    };

    for (key, after_val) in after {
        if !should_watch(key) {
            continue;
        }
        match before.get(key) {
            None => events.push(NotifyEvent::Added(key.clone())),
            Some(before_val) if before_val != after_val => {
                events.push(NotifyEvent::Modified(key.clone()))
            }
            _ => {}
        }
    }

    for key in before.keys() {
        if !should_watch(key) {
            continue;
        }
        if !after.contains_key(key) {
            events.push(NotifyEvent::Removed(key.clone()));
        }
    }

    events.sort_by(|a, b| {
        let key_a = match a {
            NotifyEvent::Added(k) | NotifyEvent::Removed(k)
            | NotifyEvent::Modified(k) | NotifyEvent::Expired(k) => k,
        };
        let key_b = match b {
            NotifyEvent::Added(k) | NotifyEvent::Removed(k)
            | NotifyEvent::Modified(k) | NotifyEvent::Expired(k) => k,
        };
        key_a.cmp(key_b)
    });

    NotifyResult { events }
}
