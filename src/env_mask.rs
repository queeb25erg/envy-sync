//! env_mask.rs — Mask sensitive environment variable values for display/logging.

use std::collections::HashMap;

/// Default mask string used to replace sensitive values.
const DEFAULT_MASK: &str = "****";

/// Patterns that indicate a variable is likely sensitive.
const SENSITIVE_PATTERNS: &[&str] = &[
    "SECRET", "PASSWORD", "PASSWD", "TOKEN", "API_KEY", "APIKEY",
    "PRIVATE", "CREDENTIAL", "AUTH", "ACCESS_KEY", "CERT", "PASSPHRASE",
];

/// Determines whether a key name suggests a sensitive value.
pub fn is_sensitive_key(key: &str) -> bool {
    let upper = key.to_uppercase();
    SENSITIVE_PATTERNS.iter().any(|pat| upper.contains(pat))
}

/// Masks the value of a key if it is considered sensitive.
/// Returns the original value if the key is not sensitive.
pub fn mask_value(key: &str, value: &str) -> String {
    if is_sensitive_key(key) {
        mask_string(value)
    } else {
        value.to_string()
    }
}

/// Masks a string, showing only the first 2 and last 2 characters if long enough.
/// Falls back to the default mask for short strings.
pub fn mask_string(value: &str) -> String {
    if value.len() <= 4 {
        DEFAULT_MASK.to_string()
    } else {
        let chars: Vec<char> = value.chars().collect();
        let first: String = chars[..2].iter().collect();
        let last: String = chars[chars.len() - 2..].iter().collect();
        format!("{}{}{}" , first, DEFAULT_MASK, last)
    }
}

/// Applies masking to an entire map of environment variables.
/// Returns a new map with sensitive values masked.
pub fn mask_env_map(env: &HashMap<String, String>) -> HashMap<String, String> {
    env.iter()
        .map(|(k, v)| (k.clone(), mask_value(k, v)))
        .collect()
}

/// Applies masking to a list of (key, value) pairs.
pub fn mask_env_pairs(pairs: &[(String, String)]) -> Vec<(String, String)> {
    pairs
        .iter()
        .map(|(k, v)| (k.clone(), mask_value(k, v)))
        .collect()
}
