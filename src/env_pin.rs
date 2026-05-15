//! env_pin: Pin specific environment variable keys to prevent accidental overwrite during sync.

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct PinSet {
    pub keys: HashSet<String>,
}

impl PinSet {
    pub fn new() -> Self {
        Self {
            keys: HashSet::new(),
        }
    }

    pub fn pin(&mut self, key: &str) {
        self.keys.insert(key.to_string());
    }

    pub fn unpin(&mut self, key: &str) -> bool {
        self.keys.remove(key)
    }

    pub fn is_pinned(&self, key: &str) -> bool {
        self.keys.contains(key)
    }

    pub fn list(&self) -> Vec<&str> {
        let mut keys: Vec<&str> = self.keys.iter().map(|s| s.as_str()).collect();
        keys.sort();
        keys
    }

    pub fn filter_protected<'a>(
        &self,
        incoming: &'a [(String, String)],
    ) -> Vec<&'a (String, String)> {
        incoming
            .iter()
            .filter(|(k, _)| !self.is_pinned(k))
            .collect()
    }

    pub fn from_lines(lines: &[&str]) -> Self {
        let mut set = Self::new();
        for line in lines {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                set.pin(trimmed);
            }
        }
        set
    }

    pub fn to_lines(&self) -> String {
        let mut keys = self.list();
        keys.sort();
        keys.join("\n")
    }
}

impl Default for PinSet {
    fn default() -> Self {
        Self::new()
    }
}
