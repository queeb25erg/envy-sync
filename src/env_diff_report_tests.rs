#[cfg(test)]
mod tests {
    use super::super::env_diff_report::{DiffEntry, EnvDiffReport};
    use std::collections::HashMap;

    fn map(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    #[test]
    fn test_added_keys() {
        let old = map(&[]);
        let new = map(&[("FOO", "bar")]);
        let report = EnvDiffReport::generate(&old, &new);
        assert_eq!(report.entries.len(), 1);
        assert!(matches!(&report.entries[0].1, DiffEntry::Added(v) if v == "bar"));
    }

    #[test]
    fn test_removed_keys() {
        let old = map(&[("FOO", "bar")]);
        let new = map(&[]);
        let report = EnvDiffReport::generate(&old, &new);
        assert!(matches!(&report.entries[0].1, DiffEntry::Removed(v) if v == "bar"));
    }

    #[test]
    fn test_modified_keys() {
        let old = map(&[("FOO", "old_val")]);
        let new = map(&[("FOO", "new_val")]);
        let report = EnvDiffReport::generate(&old, &new);
        assert!(matches!(
            &report.entries[0].1,
            DiffEntry::Modified { old, new } if old == "old_val" && new == "new_val"
        ));
    }

    #[test]
    fn test_unchanged_keys() {
        let old = map(&[("FOO", "same")]);
        let new = map(&[("FOO", "same")]);
        let report = EnvDiffReport::generate(&old, &new);
        assert!(matches!(&report.entries[0].1, DiffEntry::Unchanged(_)));
    }

    #[test]
    fn test_has_changes_false_when_identical() {
        let env = map(&[("A", "1"), ("B", "2")]);
        let report = EnvDiffReport::generate(&env, &env);
        assert!(!report.has_changes());
    }

    #[test]
    fn test_has_changes_true_when_different() {
        let old = map(&[("A", "1")]);
        let new = map(&[("A", "2")]);
        let report = EnvDiffReport::generate(&old, &new);
        assert!(report.has_changes());
    }

    #[test]
    fn test_summary_counts() {
        let old = map(&[("REMOVED", "x"), ("SAME", "s"), ("CHANGED", "old")]);
        let new = map(&[("ADDED", "y"), ("SAME", "s"), ("CHANGED", "new")]);
        let report = EnvDiffReport::generate(&old, &new);
        let (added, removed, modified) = report.summary();
        assert_eq!(added, 1);
        assert_eq!(removed, 1);
        assert_eq!(modified, 1);
    }

    #[test]
    fn test_display_output_contains_summary() {
        let old = map(&[("FOO", "1")]);
        let new = map(&[("BAR", "2")]);
        let report = EnvDiffReport::generate(&old, &new);
        let output = format!("{}", report);
        assert!(output.contains("Summary:"));
        assert!(output.contains("+1 added"));
        assert!(output.contains("-1 removed"));
    }
}
