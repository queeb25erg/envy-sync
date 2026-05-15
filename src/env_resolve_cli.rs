//! CLI interface for the env-resolve feature.

use crate::env_resolve::{resolve_env, ResolveError};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ResolveArgs {
    pub input: HashMap<String, String>,
    pub strict: bool,
}

pub fn run_resolve(args: ResolveArgs) -> Result<HashMap<String, String>, String> {
    match resolve_env(&args.input) {
        Ok(resolved) => Ok(resolved),
        Err(ResolveError::UndefinedVariable(v)) if !args.strict => {
            eprintln!("[warn] Undefined variable '{}' skipped (non-strict mode)", v);
            // Return input as-is for non-strict mode
            Ok(args.input.clone())
        }
        Err(e) => Err(format!("Resolution failed: {}", e)),
    }
}

pub fn print_resolved(resolved: &HashMap<String, String>) {
    let mut keys: Vec<&String> = resolved.keys().collect();
    keys.sort();
    for key in keys {
        println!("{}={}", key, resolved[key]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_resolve_strict_error() {
        let mut input = HashMap::new();
        input.insert("FOO".to_string(), "${MISSING}".to_string());
        let args = ResolveArgs { input, strict: true };
        let result = run_resolve(args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Resolution failed"));
    }

    #[test]
    fn test_run_resolve_non_strict_passes() {
        let mut input = HashMap::new();
        input.insert("FOO".to_string(), "${MISSING}".to_string());
        let args = ResolveArgs { input: input.clone(), strict: false };
        let result = run_resolve(args);
        assert!(result.is_ok());
    }
}
