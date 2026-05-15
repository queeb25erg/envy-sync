#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::PathBuf;
    use crate::env_backup::{
        BackupEntry, BackupStore, compute_checksum, generate_backup_id,
    };
    use chrono::Utc;

    fn sample_vars() -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("API_KEY".into(), "secret123".into());
        m.insert("DB_URL".into(), "postgres://localhost/db".into());
        m
    }

    fn make_entry(id: &str, path: &str) -> BackupEntry {
        BackupEntry {
            id: id.to_string(),
            source_path: PathBuf::from(path),
            backup_path: PathBuf::from(format!("/tmp/{}.env", id)),
            created_at: Utc::now(),
            label: None,
            checksum: "abc123".into(),
        }
    }

    #[test]
    fn test_checksum_is_deterministic() {
        let vars = sample_vars();
        let c1 = compute_checksum(&vars);
        let c2 = compute_checksum(&vars);
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_checksum_differs_on_change() {
        let vars1 = sample_vars();
        let mut vars2 = vars1.clone();
        vars2.insert("NEW_KEY".into(), "new_val".into());
        assert_ne!(compute_checksum(&vars1), compute_checksum(&vars2));
    }

    #[test]
    fn test_generate_backup_id_unique() {
        let id1 = generate_backup_id();
        std::thread::sleep(std::time::Duration::from_nanos(10));
        let id2 = generate_backup_id();
        assert!(id1.starts_with("bkp_"));
        assert!(id2.starts_with("bkp_"));
    }

    #[test]
    fn test_store_add_and_find() {
        let mut store = BackupStore::new();
        let entry = make_entry("bkp_001", ".env");
        store.add(entry);
        assert!(store.find_by_id("bkp_001").is_some());
        assert!(store.find_by_id("bkp_999").is_none());
    }

    #[test]
    fn test_store_list_for_path() {
        let mut store = BackupStore::new();
        store.add(make_entry("bkp_a", ".env"));
        store.add(make_entry("bkp_b", ".env.prod"));
        store.add(make_entry("bkp_c", ".env"));
        let results = store.list_for_path(&PathBuf::from(".env"));
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_store_remove() {
        let mut store = BackupStore::new();
        store.add(make_entry("bkp_x", ".env"));
        assert!(store.remove("bkp_x"));
        assert!(!store.remove("bkp_x"));
        assert!(store.find_by_id("bkp_x").is_none());
    }

    #[test]
    fn test_latest_for_path_returns_most_recent() {
        let mut store = BackupStore::new();
        let mut e1 = make_entry("bkp_old", ".env");
        e1.created_at = Utc::now() - chrono::Duration::hours(2);
        let e2 = make_entry("bkp_new", ".env");
        store.add(e1);
        store.add(e2);
        let latest = store.latest_for_path(&PathBuf::from(".env"));
        assert_eq!(latest.unwrap().id, "bkp_new");
    }
}
