//! Resolve environment variable references within .env files.
//! Supports `${VAR}` and `$VAR` style references, with cycle detection.

use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
pub enum ResolveError {
    CyclicReference(String),
    UndefinedVariable(String),
}

impl std::fmt::Display for ResolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolveError::CyclicReference(v) => write!(f, "Cyclic reference detected for variable: {}", v),
            ResolveError::UndefinedVariable(v) => write!(f, "Undefined variable referenced: {}", v),
        }
    }
}

/// Resolve all variable references in a map of env vars.
/// Returns a new map with all values fully resolved.
pub fn resolve_env(env: &HashMap<String, String>) -> Result<HashMap<String, String>, ResolveError> {
    let mut resolved: HashMap<String, String> = HashMap::new();
    for key in env.keys() {
        let mut visiting = HashSet::new();
        resolve_var(key, env, &mut resolved, &mut visiting)?;
    }
    Ok(resolved)
}

fn resolve_var(
    key: &str,
    env: &HashMap<String, String>,
    resolved: &mut HashMap<String, String>,
    visiting: &mut HashSet<String>,
) -> Result<String, ResolveError> {
    if let Some(val) = resolved.get(key) {
        return Ok(val.clone());
    }
    if visiting.contains(key) {
        return Err(ResolveError::CyclicReference(key.to_string()));
    }
    let raw = env
        .get(key)
        .ok_or_else(|| ResolveError::UndefinedVariable(key.to_string()))?;
    visiting.insert(key.to_string());
    let value = interpolate(raw, env, resolved, visiting)?;
    visiting.remove(key);
    resolved.insert(key.to_string(), value.clone());
    Ok(value)
}

fn interpolate(
    template: &str,
    env: &HashMap<String, String>,
    resolved: &mut HashMap<String, String>,
    visiting: &mut HashSet<String>,
) -> Result<String, ResolveError> {
    let mut result = String::new();
    let mut chars = template.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '$' {
            if chars.peek() == Some(&'{') {
                chars.next();
                let var_name: String = chars.by_ref().take_while(|&c| c != '}').collect();
                let val = resolve_var(&var_name, env, resolved, visiting)?;
                result.push_str(&val);
            } else {
                let var_name: String = chars
                    .by_ref()
                    .take_while(|c| c.is_alphanumeric() || *c == '_')
                    .collect();
                if !var_name.is_empty() {
                    let val = resolve_var(&var_name, env, resolved, visiting)?;
                    result.push_str(&val);
                }
            }
        } else {
            result.push(c);
        }
    }
    Ok(result)
}
