#[cfg(test)]
mod tests {
    use crate::snapshot::{Snapshot, SnapshotStore};
    use crate::snapshot_cli::{cmd_create, cmd_delete, cmd_list, cmd_restore};
    use std::collections::HashMap;

    fn sample_vars() -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("DB_HOST".to_string(), "localhost".to_string());
        m.insert("DB_PORT".to_string(), "5432".to_string());
        m.insert("API_KEY".to_string(), "secret".to_string());
        m
    }

    #[test]
    fn test_snapshot_creation() {
        let vars = sample_vars();
        let snap = Snapshot::new(vars.clone(), Some("v1".to_string()));
        assert_eq!(snap.var_count(), 3);
        assert_eq!(snap.label, Some("v1".to_string()));
        assert!(snap.display_name().contains("v1"));
    }

    #[test]
    fn test_snapshot_no_label() {
        let snap = Snapshot::new(sample_vars(), None);
        assert!(snap.display_name().starts_with("snapshot-"));
    }

    #[test]
    fn test_snapshot_json_roundtrip() {
        let snap = Snapshot::new(sample_vars(), Some("roundtrip".to_string()));
        let json = snap.to_json().expect("serialization failed");
        let restored = Snapshot::from_json(&json).expect("deserialization failed");
        assert_eq!(snap.vars, restored.vars);
        assert_eq!(snap.label, restored.label);
    }

    #[test]
    fn test_store_add_and_find() {
        let mut store = SnapshotStore::new();
        let snap = Snapshot::new(sample_vars(), None);
        let id = snap.id.clone();
        store.add(snap);
        assert!(store.find_by_id(&id).is_some());
        assert!(store.find_by_id("nonexistent").is_none());
    }

    #[test]
    fn test_store_latest() {
        let mut store = SnapshotStore::new();
        assert!(store.latest().is_none());
        store.add(Snapshot::new(sample_vars(), Some("first".to_string())));
        store.add(Snapshot::new(sample_vars(), Some("second".to_string())));
        let latest = store.latest().expect("should have latest");
        assert_eq!(latest.label, Some("second".to_string()));
    }

    #[test]
    fn test_store_remove() {
        let mut store = SnapshotStore::new();
        let snap = Snapshot::new(sample_vars(), None);
        let id = snap.id.clone();
        store.add(snap);
        assert!(store.remove_by_id(&id));
        assert!(!store.remove_by_id(&id));
        assert!(store.snapshots.is_empty());
    }

    #[test]
    fn test_cmd_create_and_restore() {
        let mut store = SnapshotStore::new();
        let id = cmd_create(&mut store, sample_vars(), Some("cli-test".to_string()));
        let vars = cmd_restore(&store, &id);
        assert!(vars.is_some());
        assert_eq!(vars.unwrap().get("DB_HOST").unwrap(), "localhost");
    }

    #[test]
    fn test_cmd_delete() {
        let mut store = SnapshotStore::new();
        let id = cmd_create(&mut store, sample_vars(), None);
        assert!(cmd_delete(&mut store, &id));
        assert!(!cmd_delete(&mut store, &id));
    }

    #[test]
    fn test_store_json_roundtrip() {
        let mut store = SnapshotStore::new();
        store.add(Snapshot::new(sample_vars(), Some("a".to_string())));
        store.add(Snapshot::new(sample_vars(), Some("b".to_string())));
        let json = store.to_json().expect("serialize store");
        let restored = SnapshotStore::from_json(&json).expect("deserialize store");
        assert_eq!(restored.snapshots.len(), 2);
    }

    #[test]
    fn test_cmd_list_empty()
    {
        let store = SnapshotStore::new();
        cmd_list(&store); // should not panic
    }
}
