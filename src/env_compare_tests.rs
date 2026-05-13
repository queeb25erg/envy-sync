#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::env_compare::{compare_envs, format_compare_report, DiffSeverity};
    use crate::env_compare_cli::{parse_env_file, run_compare, CompareArgs};

    fn make_map(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    #[test]
    fn test_no_differences() {
        let left = make_map(&[("KEY", "val")]);
        let right = make_map(&[("KEY", "val")]);
        let diffs = compare_envs(&left, &right);
        assert!(diffs.is_empty());
    }

    #[test]
    fn test_added_key() {
        let left = make_map(&[]);
        let right = make_map(&[("NEW_KEY", "hello")]);
        let diffs = compare_envs(&left, &right);
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].severity, DiffSeverity::Added);
        assert_eq!(diffs[0].key, "NEW_KEY");
    }

    #[test]
    fn test_removed_key() {
        let left = make_map(&[("OLD_KEY", "bye")]);
        let right = make_map(&[]);
        let diffs = compare_envs(&left, &right);
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].severity, DiffSeverity::Removed);
    }

    #[test]
    fn test_changed_key() {
        let left = make_map(&[("KEY", "old")]);
        let right = make_map(&[("KEY", "new")]);
        let diffs = compare_envs(&left, &right);
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].severity, DiffSeverity::Changed);
    }

    #[test]
    fn test_type_changed() {
        let left = make_map(&[("FLAG", "true")]);
        let right = make_map(&[("FLAG", "yes")]);
        let diffs = compare_envs(&left, &right);
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].severity, DiffSeverity::TypeChanged);
    }

    #[test]
    fn test_format_report_no_diffs() {
        let report = format_compare_report(&[], false);
        assert!(report.contains("No differences"));
    }

    #[test]
    fn test_format_report_redact() {
        let left = make_map(&[("SECRET", "mysecret")]);
        let right = make_map(&[("SECRET", "other")]);
        let diffs = compare_envs(&left, &right);
        let report = format_compare_report(&diffs, true);
        assert!(!report.contains("mysecret"));
        assert!(report.contains("[redacted]"));
    }

    #[test]
    fn test_parse_env_file() {
        let content = "# comment\nKEY=value\nQUOTED=\"hello world\"\n";
        let map = parse_env_file(content);
        assert_eq!(map.get("KEY").map(|s| s.as_str()), Some("value"));
        assert_eq!(map.get("QUOTED").map(|s| s.as_str()), Some("hello world"));
        assert!(!map.contains_key("# comment"));
    }

    #[test]
    fn test_run_compare_missing_file() {
        let args = CompareArgs {
            left_path: "/nonexistent/left.env".to_string(),
            right_path: "/nonexistent/right.env".to_string(),
            redact: false,
            only_severity: None,
        };
        assert!(run_compare(&args).is_err());
    }
}
