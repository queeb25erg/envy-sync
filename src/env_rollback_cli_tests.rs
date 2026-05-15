//! Tests for env_rollback_cli module.

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::env_rollback_cli::{run_rollback_cli, RollbackCliArgs};
    use crate::snapshot::Snapshot;

    fn make_snapshot(id: &str, vars: Vec<(&str, &str)>) -> Snapshot {
        Snapshot {
            id: id.to_string(),
            vars: vars.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
            created_at: chrono::Utc::now(),
            label: None,
        }
    }

    #[test]
    fn test_cli_rollback_success() {
        let mut current: HashMap<String, String> = [("DB_URL".to_string(), "new".to_string())].into();
        let snap = make_snapshot("snap-001", vec![("DB_URL", "old")]);
        let args = RollbackCliArgs {
            snapshot_id: "snap-001".to_string(),
            dry_run: false,
            keys: None,
            verbose: false,
        };
        let result = run_rollback_cli(&args, &mut current, &snap);
        assert!(result.is_ok());
        assert_eq!(current["DB_URL"], "old");
    }

    #[test]
    fn test_cli_rollback_dry_run_no_change() {
        let mut current: HashMap<String, String> = [("DB_URL".to_string(), "new".to_string())].into();
        let snap = make_snapshot("snap-001", vec![("DB_URL", "old")]);
        let args = RollbackCliArgs {
            snapshot_id: "snap-001".to_string(),
            dry_run: true,
            keys: None,
            verbose: false,
        };
        let result = run_rollback_cli(&args, &mut current, &snap);
        assert!(result.is_ok());
        assert_eq!(current["DB_URL"], "new");
    }

    #[test]
    fn test_cli_rollback_snapshot_id_mismatch() {
        let mut current: HashMap<String, String> = HashMap::new();
        let snap = make_snapshot("snap-999", vec![]);
        let args = RollbackCliArgs {
            snapshot_id: "snap-001".to_string(),
            dry_run: false,
            keys: None,
            verbose: false,
        };
        let result = run_rollback_cli(&args, &mut current, &snap);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("mismatch"));
    }

    #[test]
    fn test_cli_rollback_with_key_filter() {
        let mut current: HashMap<String, String> = [
            ("A".to_string(), "new_a".to_string()),
            ("B".to_string(), "new_b".to_string()),
        ].into();
        let snap = make_snapshot("snap-002", vec![("A", "old_a"), ("B", "old_b")]);
        let args = RollbackCliArgs {
            snapshot_id: "snap-002".to_string(),
            dry_run: false,
            keys: Some(vec!["A".to_string()]),
            verbose: true,
        };
        let result = run_rollback_cli(&args, &mut current, &snap);
        assert!(result.is_ok());
        assert_eq!(current["A"], "old_a");
        assert_eq!(current["B"], "new_b");
    }
}
