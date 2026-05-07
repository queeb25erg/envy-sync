#[cfg(test)]
mod tests {
    use crate::crypto::{decrypt, encrypt};
    use crate::rotate::{rotate_keys, verify_rotation, RotateError};
    use crate::storage::InMemoryStorage;

    fn make_key(seed: u8) -> Vec<u8> {
        vec![seed; 32]
    }

    fn setup_storage(old_key: &[u8]) -> InMemoryStorage {
        let mut storage = InMemoryStorage::new();
        for name in &["prod", "staging", "dev"] {
            let plaintext = format!("SECRET=value_{name}");
            let ciphertext = encrypt(plaintext.as_bytes(), old_key).unwrap();
            storage.put(name, &ciphertext).unwrap();
        }
        storage
    }

    #[test]
    fn test_rotate_all_entries() {
        let old_key = make_key(1);
        let new_key = make_key(2);
        let mut storage = setup_storage(&old_key);

        let result = rotate_keys(&mut storage, &old_key, &new_key).unwrap();
        assert_eq!(result.rotated, 3);
        assert_eq!(result.skipped, 0);
        assert!(result.failed.is_empty());
    }

    #[test]
    fn test_rotated_data_decryptable_with_new_key() {
        let old_key = make_key(1);
        let new_key = make_key(2);
        let mut storage = setup_storage(&old_key);

        rotate_keys(&mut storage, &old_key, &new_key).unwrap();

        let ciphertext = storage.get("prod").unwrap().unwrap();
        let plaintext = decrypt(&ciphertext, &new_key).unwrap();
        assert_eq!(String::from_utf8(plaintext).unwrap(), "SECRET=value_prod");
    }

    #[test]
    fn test_rotate_fails_with_wrong_old_key() {
        let old_key = make_key(1);
        let wrong_key = make_key(99);
        let new_key = make_key(2);
        let mut storage = setup_storage(&old_key);

        let result = rotate_keys(&mut storage, &wrong_key, &new_key).unwrap();
        assert_eq!(result.rotated, 0);
        assert_eq!(result.failed.len(), 3);
    }

    #[test]
    fn test_rotate_empty_storage_returns_error() {
        let mut storage = InMemoryStorage::new();
        let old_key = make_key(1);
        let new_key = make_key(2);

        let err = rotate_keys(&mut storage, &old_key, &new_key).unwrap_err();
        assert!(matches!(err, RotateError::NoEntries));
    }

    #[test]
    fn test_verify_rotation_passes_with_correct_key() {
        let old_key = make_key(1);
        let new_key = make_key(2);
        let mut storage = setup_storage(&old_key);
        rotate_keys(&mut storage, &old_key, &new_key).unwrap();

        let invalid = verify_rotation(&storage, &new_key).unwrap();
        assert!(invalid.is_empty());
    }

    #[test]
    fn test_verify_rotation_fails_with_old_key_after_rotate() {
        let old_key = make_key(1);
        let new_key = make_key(2);
        let mut storage = setup_storage(&old_key);
        rotate_keys(&mut storage, &old_key, &new_key).unwrap();

        let invalid = verify_rotation(&storage, &old_key).unwrap();
        assert_eq!(invalid.len(), 3);
    }
}
