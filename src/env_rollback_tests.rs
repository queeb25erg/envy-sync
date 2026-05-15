//! Tests for env_rollback module.

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::env_rollback::{apply_rollback, rollback_to_snapshot, RollbackOptions};
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
    fn test_rollback_restores_changed_key() {
        let mut current: HashMap<String, String> = [("KEY".to_string(), "new_val".to_string())].into();
        let snap = make_snapshot("snap1", vec![("KEY", "old_val")]);
        let opts = RollbackOptions::default();
        let result = apply_rollback(&mut current, &snap, &opts);
        assert!(result.restored_keys.contains(&"KEY".to_string()));
        assert_eq!(current["KEY"], "old_val");
    }

    #[test]
    fn test_rollback_removes_extra_key() {
        let mut current: HashMap<String, String> = [
            ("KEY".to_string(), "val".to_string()),
            ("EXTRA".to_string(), "extra".to_string()),
        ].into();
        let snap = make_snapshot("snap1", vec![("KEY", "val")]);
        let opts = RollbackOptions::default();
        let result = apply_rollback(&mut current, &snap, &opts);
        assert!(result.removed_keys.contains(&"EXTRA".to_string()));
        assert!(!current.contains_key("EXTRA"));
    }

    #[test]
    fn test_rollback_unchanged_key() {
        let mut current: HashMap<String, String> = [("KEY".to_string(), "same".to_string())].into();
        let snap = make_snapshot("snap1", vec![("KEY", "same")]);
        let opts = RollbackOptions::default();
        let result = apply_rollback(&mut current, &snap, &opts);
        assert!(result.unchanged_keys.contains(&"KEY".to_string()));
        assert!(result.restored_keys.is_empty());
    }

    #[test]
    fn test_dry_run_does_not_modify() {
        let mut current: HashMap<String, String> = [("KEY".to_string(), "new".to_string())].into();
        let snap = make_snapshot("snap1", vec![("KEY", "old")]);
        let opts = RollbackOptions { dry_run: true, keys: None };
        apply_rollback(&mut current, &snap, &opts);
        assert_eq!(current["KEY"], "new");
    }

    #[test]
    fn test_rollback_with_key_filter() {
        let mut current: HashMap<String, String> = [
            ("A".to_string(), "new_a".to_string()),
            ("B".to_string(), "new_b".to_string()),
        ].into();
        let snap = make_snapshot("snap1", vec![("A", "old_a"), ("B", "old_b")]);
        let opts = RollbackOptions { dry_run: false, keys: Some(vec!["A".to_string()]) };
        let result = apply_rollback(&mut current, &snap, &opts);
        assert!(result.restored_keys.contains(&"A".to_string()));
        assert!(!result.restored_keys.contains(&"B".to_string()));
        assert_eq!(current["A"], "old_a");
        assert_eq!(current["B"], "new_b");
    }

    #[test]
    fn test_rollback_to_empty_snapshot() {
        let mut current: HashMap<String, String> = [("KEY".to_string(), "val".to_string())].into();
        let snap = make_snapshot("snap_empty", vec![]);
        let opts = RollbackOptions::default();
        let result = apply_rollback(&mut current, &snap, &opts);
        assert!(result.removed_keys.contains(&"KEY".to_string()));
        assert!(current.is_empty());
    }
}
