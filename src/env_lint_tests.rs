#[cfg(test)]
mod tests {
    use crate::env_lint::{lint_env, LintSeverity};

    #[test]
    fn test_no_issues_for_clean_env() {
        let content = "DB_HOST=localhost\nDB_PORT=5432\nAPP_SECRET=abc123\n";
        let issues = lint_env(content);
        assert!(issues.is_empty(), "Expected no issues, got: {:?}", issues);
    }

    #[test]
    fn test_detects_duplicate_key() {
        let content = "DB_HOST=localhost\nDB_HOST=remotehost\n";
        let issues = lint_env(content);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].key, "DB_HOST");
        assert_eq!(issues[0].severity, LintSeverity::Error);
        assert!(issues[0].message.contains("Duplicate"));
    }

    #[test]
    fn test_detects_lowercase_key() {
        let content = "db_host=localhost\n";
        let issues = lint_env(content);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].severity, LintSeverity::Warning);
        assert!(issues[0].message.contains("UPPER_SNAKE_CASE"));
    }

    #[test]
    fn test_detects_empty_value() {
        let content = "API_KEY=\n";
        let issues = lint_env(content);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].severity, LintSeverity::Warning);
        assert!(issues[0].message.contains("empty value"));
    }

    #[test]
    fn test_detects_unquoted_spaces_in_value() {
        let content = "APP_NAME=my app\n";
        let issues = lint_env(content);
        assert!(issues.iter().any(|i| i.message.contains("not quoted")));
    }

    #[test]
    fn test_quoted_value_with_spaces_is_ok() {
        let content = "APP_NAME=\"my app\"\n";
        let issues = lint_env(content);
        assert!(
            !issues.iter().any(|i| i.message.contains("not quoted")),
            "Quoted value should not trigger space warning"
        );
    }

    #[test]
    fn test_invalid_line_without_equals() {
        let content = "NOTANASSIGNMENT\n";
        let issues = lint_env(content);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].severity, LintSeverity::Error);
        assert!(issues[0].message.contains("valid KEY=VALUE"));
    }

    #[test]
    fn test_comments_and_blank_lines_ignored() {
        let content = "# This is a comment\n\nDB_PORT=5432\n";
        let issues = lint_env(content);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_multiple_issues_reported() {
        let content = "db_host=\ndb_host=value\n";
        let issues = lint_env(content);
        // lowercase + empty value on first, lowercase + duplicate on second
        assert!(issues.len() >= 3);
    }
}
