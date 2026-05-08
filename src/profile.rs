//! Profile management: named environment profiles (e.g., dev, staging, prod)

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Profile {
    pub name: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub env_vars: HashMap<String, String>,
}

impl Profile {
    pub fn new(name: impl Into<String>) -> Self {
        Profile {
            name: name.into(),
            description: None,
            tags: vec![],
            env_vars: HashMap::new(),
        }
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn set_var(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.env_vars.insert(key.into(), value.into());
    }

    pub fn get_var(&self, key: &str) -> Option<&String> {
        self.env_vars.get(key)
    }

    pub fn remove_var(&mut self, key: &str) -> Option<String> {
        self.env_vars.remove(key)
    }

    pub fn merge_from(&mut self, other: &Profile) {
        for (k, v) in &other.env_vars {
            self.env_vars.entry(k.clone()).or_insert_with(|| v.clone());
        }
    }

    pub fn var_count(&self) -> usize {
        self.env_vars.len()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProfileStore {
    pub profiles: HashMap<String, Profile>,
}

impl ProfileStore {
    pub fn new() -> Self {
        ProfileStore::default()
    }

    pub fn add(&mut self, profile: Profile) {
        self.profiles.insert(profile.name.clone(), profile);
    }

    pub fn get(&self, name: &str) -> Option<&Profile> {
        self.profiles.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Profile> {
        self.profiles.get_mut(name)
    }

    pub fn remove(&mut self, name: &str) -> Option<Profile> {
        self.profiles.remove(name)
    }

    pub fn list_names(&self) -> Vec<&String> {
        let mut names: Vec<&String> = self.profiles.keys().collect();
        names.sort();
        names
    }

    pub fn contains(&self, name: &str) -> bool {
        self.profiles.contains_key(name)
    }
}
