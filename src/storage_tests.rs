#[cfg(test)]
mod tests {
    use crate::storage::{new_record, MemoryBackend, StorageBackend};

    fn make_record(name: &str) -> crate::storage::EnvRecord {
        new_record(name, vec![1, 2, 3], vec![4, 5, 6]).expect("record creation failed")
    }

    #[test]
    fn test_put_and_get() {
        let mut backend = MemoryBackend::new();
        let record = make_record("production");
        backend.put("production", record.clone()).unwrap();

        let fetched = backend.get("production").unwrap();
        assert!(fetched.is_some());
        let fetched = fetched.unwrap();
        assert_eq!(fetched.name, "production");
        assert_eq!(fetched.ciphertext, vec![1, 2, 3]);
        assert_eq!(fetched.nonce, vec![4, 5, 6]);
    }

    #[test]
    fn test_get_missing_key_returns_none() {
        let backend = MemoryBackend::new();
        let result = backend.get("nonexistent").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_list_keys_sorted() {
        let mut backend = MemoryBackend::new();
        backend.put("staging", make_record("staging")).unwrap();
        backend.put("production", make_record("production")).unwrap();
        backend.put("development", make_record("development")).unwrap();

        let keys = backend.list().unwrap();
        assert_eq!(keys, vec!["development", "production", "staging"]);
    }

    #[test]
    fn test_delete_existing_key() {
        let mut backend = MemoryBackend::new();
        backend.put("temp", make_record("temp")).unwrap();

        let deleted = backend.delete("temp").unwrap();
        assert!(deleted);
        assert!(backend.get("temp").unwrap().is_none());
    }

    #[test]
    fn test_delete_missing_key_returns_false() {
        let mut backend = MemoryBackend::new();
        let deleted = backend.delete("ghost").unwrap();
        assert!(!deleted);
    }

    #[test]
    fn test_put_overwrites_existing() {
        let mut backend = MemoryBackend::new();
        backend.put("env", make_record("env")).unwrap();

        let updated = new_record("env", vec![9, 8, 7], vec![6, 5, 4]).unwrap();
        backend.put("env", updated).unwrap();

        let fetched = backend.get("env").unwrap().unwrap();
        assert_eq!(fetched.ciphertext, vec![9, 8, 7]);
    }

    #[test]
    fn test_new_record_timestamps_nonzero() {
        let record = make_record("test");
        assert!(record.created_at > 0);
        assert_eq!(record.created_at, record.updated_at);
    }
}
