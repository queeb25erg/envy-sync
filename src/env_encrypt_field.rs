//! Field-level encryption for selective .env variable encryption.
//!
//! Allows marking specific env keys as "sensitive" and encrypting
//! only those values while leaving others in plaintext.

use std::collections::HashMap;
use crate::crypto::{encrypt, decrypt, CryptoError};

#[derive(Debug, Clone)]
pub struct FieldEncryptConfig {
    /// Keys that should be encrypted at the field level
    pub sensitive_keys: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct FieldEncryptResult {
    pub key: String,
    pub value: String,
    pub encrypted: bool,
}

#[derive(Debug)]
pub enum FieldEncryptError {
    CryptoError(String),
    InvalidEntry(String),
}

impl From<CryptoError> for FieldEncryptError {
    fn from(e: CryptoError) -> Self {
        FieldEncryptError::CryptoError(format!("{:?}", e))
    }
}

impl FieldEncryptConfig {
    pub fn new(sensitive_keys: Vec<String>) -> Self {
        Self { sensitive_keys }
    }

    pub fn is_sensitive(&self, key: &str) -> bool {
        self.sensitive_keys.iter().any(|k| k == key)
    }
}

/// Encrypt sensitive fields in an env map, returning annotated results.
pub fn encrypt_sensitive_fields(
    env: &HashMap<String, String>,
    config: &FieldEncryptConfig,
    password: &str,
) -> Result<Vec<FieldEncryptResult>, FieldEncryptError> {
    let mut results = Vec::new();
    for (key, value) in env {
        if config.is_sensitive(key) {
            let encrypted_value = encrypt(value.as_bytes(), password)
                .map_err(FieldEncryptError::from)?;
            results.push(FieldEncryptResult {
                key: key.clone(),
                value: encrypted_value,
                encrypted: true,
            });
        } else {
            results.push(FieldEncryptResult {
                key: key.clone(),
                value: value.clone(),
                encrypted: false,
            });
        }
    }
    results.sort_by(|a, b| a.key.cmp(&b.key));
    Ok(results)
}

/// Decrypt fields that were encrypted, returning a plain env map.
pub fn decrypt_sensitive_fields(
    fields: &[FieldEncryptResult],
    password: &str,
) -> Result<HashMap<String, String>, FieldEncryptError> {
    let mut map = HashMap::new();
    for field in fields {
        if field.encrypted {
            let plain = decrypt(&field.value, password)
                .map_err(FieldEncryptError::from)?;
            let value = String::from_utf8(plain)
                .map_err(|e| FieldEncryptError::InvalidEntry(e.to_string()))?;
            map.insert(field.key.clone(), value);
        } else {
            map.insert(field.key.clone(), field.value.clone());
        }
    }
    Ok(map)
}
