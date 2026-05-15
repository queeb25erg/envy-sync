//! Mark environment variables as deprecated with optional replacement hints.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct DeprecationEntry {
    pub key: String,
    pub reason: Option<String>,
    pub replacement: Option<String>,
    pub since: Option<String>,
}

impl DeprecationEntry {
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            reason: None,
            replacement: None,
            since: None,
        }
    }

    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub fn with_replacement(mut self, replacement: impl Into<String>) -> Self {
        self.replacement = Some(replacement.into());
        self
    }

    pub fn with_since(mut self, since: impl Into<String>) -> Self {
        self.since = Some(since.into());
        self
    }

    pub fn format_warning(&self) -> String {
        let mut msg = format!("DEPRECATED: `{}`", self.key);
        if let Some(since) = &self.since {
            msg.push_str(&format!(" (since {})", since));
        }
        if let Some(reason) = &self.reason {
            msg.push_str(&format!(" — {}", reason));
        }
        if let Some(replacement) = &self.replacement {
            msg.push_str(&format!(" Use `{}` instead.", replacement));
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct DeprecationRegistry {
    entries: HashMap<String, DeprecationEntry>,
}

impl DeprecationRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, entry: DeprecationEntry) {
        self.entries.insert(entry.key.clone(), entry);
    }

    pub fn is_deprecated(&self, key: &str) -> bool {
        self.entries.contains_key(key)
    }

    pub fn get(&self, key: &str) -> Option<&DeprecationEntry> {
        self.entries.get(key)
    }

    pub fn check_env(&self, env: &HashMap<String, String>) -> Vec<&DeprecationEntry> {
        env.keys()
            .filter_map(|k| self.entries.get(k.as_str()))
            .collect()
    }

    pub fn all_deprecated_keys(&self) -> Vec<&str> {
        self.entries.keys().map(String::as_str).collect()
    }
}
