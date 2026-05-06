#[cfg(test)]
mod tests {
    use super::super::diff::{compute_diff, DiffEntry};
    use std::collections::HashMap;

    fn map(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    #[test]
    fn test_no_changes_when_identical() {
        let local = map(&[("KEY", "value"), ("FOO", "bar")]);
        let remote = map(&[("KEY", "value"), ("FOO", "bar")]);
        let diff = compute_diff(&local, &remote);
        assert!(!diff.has_changes());
    }

    #[test]
    fn test_detects_added_key() {
        let local = map(&[("KEY", "value"), ("NEW_KEY", "new")]);
        let remote = map(&[("KEY", "value")]);
        let diff = compute_diff(&local, &remote);
        assert!(diff.has_changes());
        assert!(diff.added().contains(&"NEW_KEY"));
    }

    #[test]
    fn test_detects_removed_key() {
        let local = map(&[("KEY", "value")]);
        let remote = map(&[("KEY", "value"), ("OLD_KEY", "old")]);
        let diff = compute_diff(&local, &remote);
        assert!(diff.has_changes());
        assert!(diff.removed().contains(&"OLD_KEY"));
    }

    #[test]
    fn test_detects_modified_key() {
        let local = map(&[("KEY", "new_value")]);
        let remote = map(&[("KEY", "old_value")]);
        let diff = compute_diff(&local, &remote);
        assert!(diff.has_changes());
        assert!(diff.modified().contains(&"KEY"));
        match diff.entries.get("KEY").unwrap() {
            DiffEntry::Modified { local, remote } => {
                assert_eq!(local, "new_value");
                assert_eq!(remote, "old_value");
            }
            _ => panic!("Expected Modified entry"),
        }
    }

    #[test]
    fn test_empty_maps_produce_no_diff() {
        let diff = compute_diff(&HashMap::new(), &HashMap::new());
        assert!(!diff.has_changes());
        assert!(diff.entries.is_empty());
    }

    #[test]
    fn test_mixed_changes() {
        let local = map(&[("KEEP", "same"), ("CHANGE", "new"), ("ADD", "added")]);
        let remote = map(&[("KEEP", "same"), ("CHANGE", "old"), ("REMOVE", "gone")]);
        let diff = compute_diff(&local, &remote);
        assert!(diff.has_changes());
        assert_eq!(diff.added().len(), 1);
        assert_eq!(diff.removed().len(), 1);
        assert_eq!(diff.modified().len(), 1);
    }
}
