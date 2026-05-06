#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::diff::{DiffEntry, DiffKind};
    use crate::merge::{merge, MergeStrategy};

    fn map(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    fn added(key: &str) -> DiffEntry {
        DiffEntry { key: key.to_string(), kind: DiffKind::Added }
    }

    fn removed(key: &str) -> DiffEntry {
        DiffEntry { key: key.to_string(), kind: DiffKind::Removed }
    }

    fn modified(key: &str, local: &str, remote: &str) -> DiffEntry {
        DiffEntry {
            key: key.to_string(),
            kind: DiffKind::Modified {
                local_value: local.to_string(),
                remote_value: remote.to_string(),
            },
        }
    }

    #[test]
    fn test_merge_added_key_from_remote() {
        let local = map(&[("A", "1")]);
        let remote = map(&[("A", "1"), ("B", "2")]);
        let diffs = vec![added("B")];
        let result = merge(&local, &remote, &diffs, &MergeStrategy::PreferRemote).unwrap();
        assert_eq!(result.merged.get("B").map(String::as_str), Some("2"));
    }

    #[test]
    fn test_merge_removed_key() {
        let local = map(&[("A", "1"), ("B", "old")]);
        let remote = map(&[("A", "1")]);
        let diffs = vec![removed("B")];
        let result = merge(&local, &remote, &diffs, &MergeStrategy::PreferRemote).unwrap();
        assert!(!result.merged.contains_key("B"));
    }

    #[test]
    fn test_merge_prefer_remote_on_conflict() {
        let local = map(&[("KEY", "local_val")]);
        let remote = map(&[("KEY", "remote_val")]);
        let diffs = vec![modified("KEY", "local_val", "remote_val")];
        let result = merge(&local, &remote, &diffs, &MergeStrategy::PreferRemote).unwrap();
        assert_eq!(result.merged.get("KEY").map(String::as_str), Some("remote_val"));
    }

    #[test]
    fn test_merge_prefer_local_on_conflict() {
        let local = map(&[("KEY", "local_val")]);
        let remote = map(&[("KEY", "remote_val")]);
        let diffs = vec![modified("KEY", "local_val", "remote_val")];
        let result = merge(&local, &remote, &diffs, &MergeStrategy::PreferLocal).unwrap();
        assert_eq!(result.merged.get("KEY").map(String::as_str), Some("local_val"));
    }

    #[test]
    fn test_merge_error_on_conflict() {
        let local = map(&[("KEY", "local_val")]);
        let remote = map(&[("KEY", "remote_val")]);
        let diffs = vec![modified("KEY", "local_val", "remote_val")];
        let err = merge(&local, &remote, &diffs, &MergeStrategy::ErrorOnConflict)
            .unwrap_err();
        assert!(err.contains("KEY"));
    }

    #[test]
    fn test_merge_no_diffs_returns_local() {
        let local = map(&[("A", "1"), ("B", "2")]);
        let remote = map(&[("A", "1"), ("B", "2")]);
        let result = merge(&local, &remote, &[], &MergeStrategy::PreferRemote).unwrap();
        assert_eq!(result.merged, local);
    }
}
