use crate::restore::{restore_snapshot, write_env_file};
use crate::snapshot::Snapshot;
use crate::storage::MockStorage;
use crate::crypto::encrypt;
use std::collections::HashMap;

fn make_key() -> Vec<u8> {
    vec![0u8; 32]
}

fn make_snapshot_bytes(vars: HashMap<String, String>, key: &[u8]) -> Vec<u8> {
    let snap = Snapshot {
        id: "snap-001".to_string(),
        created_at: "2024-01-01T00:00:00Z".to_string(),
        label: "test".to_string(),
        env_vars: vars,
    };
    let plain = serde_json::to_vec(&snap).unwrap();
    encrypt(key, &plain).unwrap()
}

#[test]
fn test_restore_snapshot_success() {
    let key = make_key();
    let mut vars = HashMap::new();
    vars.insert("API_KEY".to_string(), "secret123".to_string());
    vars.insert("DB_URL".to_string(), "postgres://localhost/db".to_string());

    let encrypted = make_snapshot_bytes(vars.clone(), &key);
    let mut storage = MockStorage::new();
    storage.put("snapshots/snap-001.enc", encrypted);

    let result = restore_snapshot(&storage, "snap-001", &key).unwrap();
    assert_eq!(result.get("API_KEY").unwrap(), "secret123");
    assert_eq!(result.get("DB_URL").unwrap(), "postgres://localhost/db");
}

#[test]
fn test_restore_snapshot_not_found() {
    let key = make_key();
    let storage = MockStorage::new();
    let result = restore_snapshot(&storage, "missing-snap", &key);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("not found"));
}

#[test]
fn test_write_env_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join(".env").to_string_lossy().to_string();

    let mut vars = HashMap::new();
    vars.insert("FOO".to_string(), "bar".to_string());
    vars.insert("BAZ".to_string(), "qux".to_string());

    write_env_file(&path, &vars).unwrap();

    let content = std::fs::read_to_string(&path).unwrap();
    assert!(content.contains("FOO=bar"));
    assert!(content.contains("BAZ=qux"));
}
