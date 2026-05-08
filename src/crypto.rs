//! Encryption and decryption utilities for .env file contents.
//!
//! Uses AES-256-GCM for authenticated encryption with a key derived
//! from a user passphrase via Argon2id.

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("encryption failed: {0}")]
    Encryption(String),
    #[error("decryption failed: {0}")]
    Decryption(String),
    #[error("key derivation failed: {0}")]
    KeyDerivation(String),
    #[error("invalid encoded data")]
    InvalidEncoding,
}

/// Derives a 32-byte key from a passphrase and salt using Argon2id.
pub fn derive_key(passphrase: &str, salt: &str) -> Result<[u8; 32], CryptoError> {
    let mut key_bytes = [0u8; 32];
    Argon2::default()
        .hash_password_into(
            passphrase.as_bytes(),
            salt.as_bytes(),
            &mut key_bytes,
        )
        .map_err(|e| CryptoError::KeyDerivation(e.to_string()))?;
    Ok(key_bytes)
}

/// Encrypts plaintext using AES-256-GCM.
/// Returns a base64-encoded string containing `nonce || ciphertext`.
pub fn encrypt(plaintext: &[u8], key_bytes: &[u8; 32]) -> Result<String, CryptoError> {
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|e| CryptoError::Encryption(e.to_string()))?;

    let mut combined = nonce.to_vec();
    combined.extend_from_slice(&ciphertext);
    Ok(BASE64.encode(combined))
}

/// Decrypts a base64-encoded `nonce || ciphertext` blob using AES-256-GCM.
pub fn decrypt(encoded: &str, key_bytes: &[u8; 32]) -> Result<Vec<u8>, CryptoError> {
    let combined = BASE64.decode(encoded).map_err(|_| CryptoError::InvalidEncoding)?;

    if combined.len() < 12 {
        return Err(CryptoError::InvalidEncoding);
    }

    let (nonce_bytes, ciphertext) = combined.split_at(12);
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);

    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| CryptoError::Decryption(e.to_string()))
}

/// Encrypts plaintext and returns the result as a UTF-8 string.
///
/// This is a convenience wrapper around [`encrypt`] for callers that work
/// with string values (e.g. storing encrypted data in a config file).
pub fn encrypt_to_string(plaintext: &str, key_bytes: &[u8; 32]) -> Result<String, CryptoError> {
    encrypt(plaintext.as_bytes(), key_bytes)
}

/// Decrypts a base64-encoded blob and interprets the result as a UTF-8 string.
///
/// Returns [`CryptoError::Decryption`] if the decrypted bytes are not valid UTF-8.
pub fn decrypt_to_string(encoded: &str, key_bytes: &[u8; 32]) -> Result<String, CryptoError> {
    let bytes = decrypt(encoded, key_bytes)?;
    String::from_utf8(bytes)
        .map_err(|e| CryptoError::Decryption(format!("invalid UTF-8: {}", e)))
}
