#[cfg(test)]
mod tests {
    use crate::env_ttl::{TtlEntry, TtlStore};
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    fn now_secs() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    #[test]
    fn test_ttl_entry_not_expired() {
        let entry = TtlEntry::new("DB_PASS", 3600);
        assert!(!entry.is_expired());
        assert!(entry.seconds_remaining() > 0);
    }

    #[test]
    fn test_ttl_entry_already_expired() {
        let entry = TtlEntry {
            key: "OLD_KEY".to_string(),
            expires_at: now_secs() - 10,
        };
        assert!(entry.is_expired());
        assert!(entry.seconds_remaining() < 0);
    }

    #[test]
    fn test_store_set_and_get() {
        let mut store = TtlStore::new();
        store.set("API_KEY", 60);
        let entry = store.get("API_KEY").expect("should exist");
        assert_eq!(entry.key, "API_KEY");
        assert!(!entry.is_expired());
    }

    #[test]
    fn test_store_remove() {
        let mut store = TtlStore::new();
        store.set("TOKEN", 100);
        assert!(store.remove("TOKEN"));
        assert!(store.get("TOKEN").is_none());
        assert!(!store.remove("TOKEN"));
    }

    #[test]
    fn test_expired_keys_and_purge() {
        let mut store = TtlStore::new();
        store.set("FRESH", 9999);
        // Insert already-expired entry manually
        store.entries_mut().insert(
            "STALE".to_string(),
            crate::env_ttl::TtlEntry {
                key: "STALE".to_string(),
                expires_at: now_secs() - 1,
            },
        );
        let expired = store.expired_keys();
        assert!(expired.contains(&"STALE".to_string()));
        assert!(!expired.contains(&"FRESH".to_string()));

        let purged = store.purge_expired();
        assert_eq!(purged.len(), 1);
        assert!(store.get("STALE").is_none());
        assert!(store.get("FRESH").is_some());
    }

    #[test]
    fn test_all_entries() {
        let mut store = TtlStore::new();
        store.set("A", 10);
        store.set("B", 20);
        assert_eq!(store.all_entries().len(), 2);
    }
}
