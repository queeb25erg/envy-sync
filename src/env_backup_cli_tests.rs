#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::PathBuf;
    use tempfile::TempDir;
    use crate::env_backup_cli::{
        cmd_backup_create, cmd_backup_delete, cmd_backup_list, cmd_backup_restore,
    };

    fn sample_vars() -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("TOKEN".into(), "abc".into());
        m.insert("HOST".into(), "localhost".into());
        m
    }

    #[test]
    fn test_create_and_list_backup() {
        let dir = TempDir::new().unwrap();
        let source = PathBuf::from(".env");
        let base = dir.path().to_path_buf();
        let vars = sample_vars();

        let id = cmd_backup_create(&source, Some("pre-deploy".into()), &vars, &base).unwrap();
        assert!(id.starts_with("bkp_"));

        let list = cmd_backup_list(&source, &base).unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, id);
        assert_eq!(list[0].label.as_deref(), Some("pre-deploy"));
    }

    #[test]
    fn test_restore_backup_returns_correct_vars() {
        let dir = TempDir::new().unwrap();
        let source = PathBuf::from(".env");
        let base = dir.path().to_path_buf();
        let vars = sample_vars();

        let id = cmd_backup_create(&source, None, &vars, &base).unwrap();
        let restored = cmd_backup_restore(&id, &base).unwrap();

        assert_eq!(restored.get("TOKEN").map(String::as_str), Some("abc"));
        assert_eq!(restored.get("HOST").map(String::as_str), Some("localhost"));
    }

    #[test]
    fn test_delete_backup() {
        let dir = TempDir::new().unwrap();
        let source = PathBuf::from(".env");
        let base = dir.path().to_path_buf();
        let vars = sample_vars();

        let id = cmd_backup_create(&source, None, &vars, &base).unwrap();
        let deleted = cmd_backup_delete(&id, &base).unwrap();
        assert!(deleted);

        let list = cmd_backup_list(&source, &base).unwrap();
        assert!(list.is_empty());
    }

    #[test]
    fn test_delete_nonexistent_returns_false() {
        let dir = TempDir::new().unwrap();
        let base = dir.path().to_path_buf();
        let deleted = cmd_backup_delete("bkp_nonexistent", &base).unwrap();
        assert!(!deleted);
    }

    #[test]
    fn test_multiple_backups_same_source() {
        let dir = TempDir::new().unwrap();
        let source = PathBuf::from(".env");
        let base = dir.path().to_path_buf();
        let vars = sample_vars();

        cmd_backup_create(&source, Some("v1".into()), &vars, &base).unwrap();
        cmd_backup_create(&source, Some("v2".into()), &vars, &base).unwrap();

        let list = cmd_backup_list(&source, &base).unwrap();
        assert_eq!(list.len(), 2);
    }
}
