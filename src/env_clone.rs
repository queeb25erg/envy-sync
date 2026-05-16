//! env_clone: Clone an environment profile under a new name, optionally filtering keys.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct CloneOptions {
    pub source: String,
    pub destination: String,
    pub key_filter: Option<Vec<String>>,
    pub overwrite: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CloneResult {
    pub source: String,
    pub destination: String,
    pub keys_cloned: Vec<String>,
    pub keys_skipped: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub enum CloneError {
    SourceNotFound(String),
    DestinationExists(String),
    EmptySource,
}

impl std::fmt::Display for CloneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CloneError::SourceNotFound(name) => write!(f, "Source profile '{}' not found", name),
            CloneError::DestinationExists(name) => {
                write!(f, "Destination profile '{}' already exists", name)
            }
            CloneError::EmptySource => write!(f, "Source profile has no keys to clone"),
        }
    }
}

/// Clone environment variables from source to destination profile.
/// Returns a CloneResult describing what was cloned.
pub fn clone_env(
    profiles: &mut HashMap<String, HashMap<String, String>>,
    opts: &CloneOptions,
) -> Result<CloneResult, CloneError> {
    let source_env = profiles
        .get(&opts.source)
        .ok_or_else(|| CloneError::SourceNotFound(opts.source.clone()))
        .map(|m| m.clone())?;

    if source_env.is_empty() {
        return Err(CloneError::EmptySource);
    }

    if !opts.overwrite && profiles.contains_key(&opts.destination) {
        return Err(CloneError::DestinationExists(opts.destination.clone()));
    }

    let mut keys_cloned = Vec::new();
    let mut keys_skipped = Vec::new();
    let mut dest_env: HashMap<String, String> = HashMap::new();

    for (key, value) in &source_env {
        let include = match &opts.key_filter {
            Some(filter) => filter.contains(key),
            None => true,
        };
        if include {
            dest_env.insert(key.clone(), value.clone());
            keys_cloned.push(key.clone());
        } else {
            keys_skipped.push(key.clone());
        }
    }

    keys_cloned.sort();
    keys_skipped.sort();

    profiles.insert(opts.destination.clone(), dest_env);

    Ok(CloneResult {
        source: opts.source.clone(),
        destination: opts.destination.clone(),
        keys_cloned,
        keys_skipped,
    })
}
