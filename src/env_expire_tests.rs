//! env_expire_tests.rs — Unit tests for ExpiryStore logic.

#[cfg(test)]
mod tests {
    use crate::env_expire::{ExpiryStatus, ExpiryStore};
    use chrono::{Duration, Utc};

    fn store_with_entries() -> ExpiryStore {
        let mut store = ExpiryStore::new();
        let now = Utc::now();
        store.set("DB_PASSWORD", now - Duration::days(1), 7);
        store.set("API_KEY", now + Duration::days(3), 7);
        store.set("TOKEN", now + Duration::days(30), 7);
        store
    }

    #[test]
    fn test_expired_status() {
        let mut store = ExpiryStore::new();
        let past = Utc::now() - Duration::days(5);
        store.set("OLD_KEY", past, 7);
        let status = store.status("OLD_KEY", Utc::now()).unwrap();
        assert!(matches!(status, ExpiryStatus::Expired { .. }));
        if let ExpiryStatus::Expired { days_ago } = status {
            assert!(days_ago >= 4);
        }
    }

    #[test]
    fn test_warning_status() {
        let mut store = ExpiryStore::new();
        let soon = Utc::now() + Duration::days(3);
        store.set("SOON_KEY", soon, 7);
        let status = store.status("SOON_KEY", Utc::now()).unwrap();
        assert!(matches!(status, ExpiryStatus::WarningSoon { .. }));
    }

    #[test]
    fn test_valid_status() {
        let mut store = ExpiryStore::new();
        let future = Utc::now() + Duration::days(60);
        store.set("SAFE_KEY", future, 7);
        let status = store.status("SAFE_KEY", Utc::now()).unwrap();
        assert_eq!(status, ExpiryStatus::Valid);
    }

    #[test]
    fn test_no_entry_returns_none() {
        let store = ExpiryStore::new();
        assert!(store.status("MISSING", Utc::now()).is_none());
    }

    #[test]
    fn test_expired_keys_list() {
        let store = store_with_entries();
        let expired = store.expired_keys(Utc::now());
        assert_eq!(expired.len(), 1);
        assert!(expired.contains(&"DB_PASSWORD"));
    }

    #[test]
    fn test_warning_keys_list() {
        let store = store_with_entries();
        let warnings = store.warning_keys(Utc::now());
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].0, "API_KEY");
    }

    #[test]
    fn test_remove_entry() {
        let mut store = ExpiryStore::new();
        let future = Utc::now() + Duration::days(10);
        store.set("TEMP_KEY", future, 3);
        assert!(store.remove("TEMP_KEY"));
        assert!(!store.remove("TEMP_KEY"));
        assert!(store.status("TEMP_KEY", Utc::now()).is_none());
    }

    #[test]
    fn test_overwrite_entry() {
        let mut store = ExpiryStore::new();
        let past = Utc::now() - Duration::days(1);
        let future = Utc::now() + Duration::days(30);
        store.set("KEY", past, 7);
        store.set("KEY", future, 7);
        assert_eq!(store.status("KEY", Utc::now()), Some(ExpiryStatus::Valid));
    }
}
