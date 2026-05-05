#[cfg(test)]
mod tests {
    use crate::crypto::{decrypt, derive_key, encrypt, CryptoError};

    const TEST_PASSPHRASE: &str = "super-secret-passphrase";
    const TEST_SALT: &str = "envy-sync-salt-v1";

    fn test_key() -> [u8; 32] {
        derive_key(TEST_PASSPHRASE, TEST_SALT).expect("key derivation should succeed")
    }

    #[test]
    fn derive_key_is_deterministic() {
        let key1 = derive_key(TEST_PASSPHRASE, TEST_SALT).unwrap();
        let key2 = derive_key(TEST_PASSPHRASE, TEST_SALT).unwrap();
        assert_eq!(key1, key2);
    }

    #[test]
    fn derive_key_differs_with_different_passphrase() {
        let key1 = derive_key(TEST_PASSPHRASE, TEST_SALT).unwrap();
        let key2 = derive_key("other-passphrase", TEST_SALT).unwrap();
        assert_ne!(key1, key2);
    }

    #[test]
    fn encrypt_then_decrypt_roundtrip() {
        let key = test_key();
        let plaintext = b"DATABASE_URL=postgres://localhost/mydb\nSECRET_KEY=abc123";

        let encoded = encrypt(plaintext, &key).expect("encryption should succeed");
        let decrypted = decrypt(&encoded, &key).expect("decryption should succeed");

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn encrypt_produces_different_ciphertext_each_time() {
        let key = test_key();
        let plaintext = b"API_KEY=my-secret-api-key";

        let enc1 = encrypt(plaintext, &key).unwrap();
        let enc2 = encrypt(plaintext, &key).unwrap();

        // Nonces are random, so ciphertexts must differ
        assert_ne!(enc1, enc2);
    }

    #[test]
    fn decrypt_fails_with_wrong_key() {
        let key = test_key();
        let wrong_key = derive_key("wrong-passphrase", TEST_SALT).unwrap();
        let plaintext = b"SENSITIVE=value";

        let encoded = encrypt(plaintext, &key).unwrap();
        let result = decrypt(&encoded, &wrong_key);

        assert!(matches!(result, Err(CryptoError::Decryption(_))));
    }

    #[test]
    fn decrypt_fails_with_invalid_base64() {
        let key = test_key();
        let result = decrypt("not-valid-base64!!!", &key);
        assert!(matches!(result, Err(CryptoError::InvalidEncoding)));
    }

    #[test]
    fn decrypt_fails_with_truncated_data() {
        let key = test_key();
        // Less than 12 bytes (nonce size)
        let short = base64::engine::general_purpose::STANDARD.encode(b"short");
        let result = decrypt(&short, &key);
        assert!(matches!(result, Err(CryptoError::InvalidEncoding)));
    }
}
