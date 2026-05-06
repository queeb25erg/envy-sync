#[cfg(test)]
mod tests {
    use crate::backend::{Backend, MemoryBackend, RemoteEntry};

    fn make_entry(key: &str) -> RemoteEntry {
        RemoteEntry {
            key: key.to_string(),
            ciphertext: vec![1, 2, 3, 4],
            nonce: vec![5, 6, 7, 8],
            updated_at: 1_700_000_000,
        }
    }

    #[tokio::test]
    async fn test_put_and_get() {
        let backend = MemoryBackend::new();
        let entry = make_entry("prod");
        backend.put("prod", &entry).await.unwrap();
        let result = backend.get("prod").await.unwrap();
        assert!(result.is_some());
        let fetched = result.unwrap();
        assert_eq!(fetched.key, "prod");
        assert_eq!(fetched.ciphertext, vec![1, 2, 3, 4]);
    }

    #[tokio::test]
    async fn test_get_missing_returns_none() {
        let backend = MemoryBackend::new();
        let result = backend.get("nonexistent").await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_list_entries() {
        let backend = MemoryBackend::new();
        backend.put("dev", &make_entry("dev")).await.unwrap();
        backend.put("staging", &make_entry("staging")).await.unwrap();
        let mut names = backend.list().await.unwrap();
        names.sort();
        assert_eq!(names, vec!["dev", "staging"]);
    }

    #[tokio::test]
    async fn test_delete_entry() {
        let backend = MemoryBackend::new();
        backend.put("temp", &make_entry("temp")).await.unwrap();
        backend.delete("temp").await.unwrap();
        let result = backend.get("temp").await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_overwrite_entry() {
        let backend = MemoryBackend::new();
        backend.put("env", &make_entry("env")).await.unwrap();
        let updated = RemoteEntry {
            key: "env".to_string(),
            ciphertext: vec![9, 10, 11],
            nonce: vec![12, 13, 14],
            updated_at: 1_700_001_000,
        };
        backend.put("env", &updated).await.unwrap();
        let fetched = backend.get("env").await.unwrap().unwrap();
        assert_eq!(fetched.ciphertext, vec![9, 10, 11]);
        assert_eq!(fetched.updated_at, 1_700_001_000);
    }
}
