//! Tag management for .env files — attach, remove, and query tags on stored environments.

use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TagStore {
    /// Maps environment name -> set of tags
    pub tags: HashMap<String, HashSet<String>>,
}

impl TagStore {
    pub fn new() -> Self {
        TagStore {
            tags: HashMap::new(),
        }
    }

    /// Attach a tag to an environment.
    pub fn add_tag(&mut self, env: &str, tag: &str) -> bool {
        let entry = self.tags.entry(env.to_string()).or_insert_with(HashSet::new);
        entry.insert(tag.to_string())
    }

    /// Remove a tag from an environment. Returns true if the tag existed.
    pub fn remove_tag(&mut self, env: &str, tag: &str) -> bool {
        if let Some(set) = self.tags.get_mut(env) {
            let removed = set.remove(tag);
            if set.is_empty() {
                self.tags.remove(env);
            }
            return removed;
        }
        false
    }

    /// List all tags for a given environment.
    pub fn get_tags(&self, env: &str) -> Vec<String> {
        self.tags
            .get(env)
            .map(|s| {
                let mut v: Vec<String> = s.iter().cloned().collect();
                v.sort();
                v
            })
            .unwrap_or_default()
    }

    /// Find all environments that have a specific tag.
    pub fn find_by_tag(&self, tag: &str) -> Vec<String> {
        let mut result: Vec<String> = self
            .tags
            .iter()
            .filter(|(_, tags)| tags.contains(tag))
            .map(|(env, _)| env.clone())
            .collect();
        result.sort();
        result
    }

    /// Remove all tags for an environment (e.g., when env is deleted).
    pub fn clear_env(&mut self, env: &str) {
        self.tags.remove(env);
    }

    /// Serialize the tag store to JSON bytes.
    pub fn to_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }

    /// Deserialize a tag store from JSON bytes.
    pub fn from_bytes(data: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(data)
    }
}

impl Default for TagStore {
    fn default() -> Self {
        Self::new()
    }
}
