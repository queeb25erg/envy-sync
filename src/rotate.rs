//! Key rotation module for re-encrypting stored .env files with a new key.

use crate::crypto::{decrypt, encrypt, CryptoError};
use crate::storage::{Storage, StorageError};

#[derive(Debug, thiserror::Error)]
pub enum RotateError {
    #[error("Crypto error: {0}")]
    Crypto(#[from] CryptoError),
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
    #[error("No entries found to rotate")]
    NoEntries,
}

/// Represents the result of a rotation operation.
#[derive(Debug, Default)]
pub struct RotationResult {
    pub rotated: usize,
    pub skipped: usize,
    pub failed: Vec<String>,
}

/// Rotate all encrypted entries in storage from `old_key` to `new_key`.
pub fn rotate_keys(
    storage: &mut dyn Storage,
    old_key: &[u8],
    new_key: &[u8],
) -> Result<RotationResult, RotateError> {
    let keys = storage.list()?;
    if keys.is_empty() {
        return Err(RotateError::NoEntries);
    }

    let mut result = RotationResult::default();

    for key in &keys {
        match storage.get(key) {
            Ok(Some(ciphertext)) => {
                match decrypt(&ciphertext, old_key) {
                    Ok(plaintext) => match encrypt(&plaintext, new_key) {
                        Ok(new_ciphertext) => {
                            if let Err(e) = storage.put(key, &new_ciphertext) {
                                result.failed.push(format!("{key}: store error: {e}"));
                            } else {
                                result.rotated += 1;
                            }
                        }
                        Err(e) => result.failed.push(format!("{key}: encrypt error: {e}")),
                    },
                    Err(e) => result.failed.push(format!("{key}: decrypt error: {e}")),
                }
            }
            Ok(None) => result.skipped += 1,
            Err(e) => result.failed.push(format!("{key}: read error: {e}")),
        }
    }

    Ok(result)
}

/// Verify that all entries in storage can be decrypted with `key`.
pub fn verify_rotation(storage: &dyn Storage, key: &[u8]) -> Result<Vec<String>, RotateError> {
    let keys = storage.list()?;
    let mut invalid = Vec::new();

    for k in &keys {
        if let Ok(Some(ciphertext)) = storage.get(k) {
            if decrypt(&ciphertext, key).is_err() {
                invalid.push(k.clone());
            }
        }
    }

    Ok(invalid)
}
