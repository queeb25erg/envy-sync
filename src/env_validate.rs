//! Validation logic for .env file entries.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    EmptyKey,
    InvalidKeyChar(String, char),
    EmptyValue(String),
    DuplicateKey(String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::EmptyKey => write!(f, "Empty key found"),
            ValidationError::InvalidKeyChar(key, ch) => {
                write!(f, "Key '{}' contains invalid character '{}'" , key, ch)
            }
            ValidationError::EmptyValue(key) => write!(f, "Key '{}' has empty value", key),
            ValidationError::DuplicateKey(key) => write!(f, "Duplicate key '{}'", key),
        }
    }
}

/// Validates a single key string.
pub fn validate_key(key: &str) -> Result<(), ValidationError> {
    if key.is_empty() {
        return Err(ValidationError::EmptyKey);
    }
    for ch in key.chars() {
        if !ch.is_alphanumeric() && ch != '_' {
            return Err(ValidationError::InvalidKeyChar(key.to_string(), ch));
        }
    }
    Ok(())
}

/// Validates a map of env entries, returning all errors found.
pub fn validate_entries(
    entries: &HashMap<String, String>,
) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    let mut seen_keys: HashMap<String, usize> = HashMap::new();

    for (key, value) in entries {
        if let Err(e) = validate_key(key) {
            errors.push(e);
        }
        if value.trim().is_empty() {
            errors.push(ValidationError::EmptyValue(key.clone()));
        }
        let count = seen_keys.entry(key.clone()).or_insert(0);
        *count += 1;
        if *count > 1 {
            errors.push(ValidationError::DuplicateKey(key.clone()));
        }
    }
    errors
}

/// Returns true if all entries pass validation.
pub fn is_valid(entries: &HashMap<String, String>) -> bool {
    validate_entries(entries).is_empty()
}
