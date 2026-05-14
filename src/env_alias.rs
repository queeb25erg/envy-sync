//! env_alias.rs — Map environment variable aliases to canonical names.
//!
//! Allows users to define aliases so that e.g. `DB_URL` is treated as
//! equivalent to `DATABASE_URL` during sync, diff, and validation.

use std::collections::HashMap;

/// A registry of alias → canonical name mappings.
#[derive(Debug, Clone, Default)]
pub struct AliasRegistry {
    /// Maps alias name → canonical name.
    aliases: HashMap<String, String>,
}

impl AliasRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register an alias for a canonical key.
    /// Returns an error if the alias is the same as the canonical name.
    pub fn register(&mut self, alias: &str, canonical: &str) -> Result<(), String> {
        if alias == canonical {
            return Err(format!("Alias '{}' cannot equal canonical name", alias));
        }
        self.aliases.insert(alias.to_string(), canonical.to_string());
        Ok(())
    }

    /// Resolve a key to its canonical form, or return the key unchanged.
    pub fn resolve<'a>(&'a self, key: &'a str) -> &'a str {
        self.aliases.get(key).map(|s| s.as_str()).unwrap_or(key)
    }

    /// Return all aliases pointing to a given canonical name.
    pub fn aliases_for(&self, canonical: &str) -> Vec<&str> {
        self.aliases
            .iter()
            .filter(|(_, v)| v.as_str() == canonical)
            .map(|(k, _)| k.as_str())
            .collect()
    }

    /// Apply alias resolution to an entire env map, merging aliased keys
    /// into their canonical equivalents (canonical wins on conflict).
    pub fn normalize_env(&self, env: &HashMap<String, String>) -> HashMap<String, String> {
        let mut result: HashMap<String, String> = HashMap::new();
        for (key, value) in env {
            let canonical = self.resolve(key).to_string();
            // Canonical key takes priority over alias.
            result.entry(canonical).or_insert_with(|| value.clone());
        }
        // Now insert canonical keys explicitly so they overwrite alias values.
        for (key, value) in env {
            if self.aliases.get(key).is_none() {
                result.insert(key.clone(), value.clone());
            }
        }
        result
    }

    /// Number of registered aliases.
    pub fn len(&self) -> usize {
        self.aliases.len()
    }

    /// Whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.aliases.is_empty()
    }
}
