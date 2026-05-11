#[cfg(test)]
mod tests {
    use super::super::env_secret_scan::*;
    use std::collections::HashMap;

    fn make_entries(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    #[test]
    fn test_scan_detects_high_severity() {
        let entries = make_entries(&[("AWS_SECRET_ACCESS_KEY", "abc123")]);
        let result = scan_env(&entries);
        assert!(result.has_high_severity());
        assert_eq!(result.findings.len(), 1);
        assert_eq!(result.findings[0].severity, SecretSeverity::High);
    }

    #[test]
    fn test_scan_detects_medium_severity() {
        let entries = make_entries(&[("DB_PASSWORD", "supersecret")]);
        let result = scan_env(&entries);
        assert_eq!(result.findings.len(), 1);
        assert_eq!(result.findings[0].severity, SecretSeverity::Medium);
        assert!(!result.has_high_severity());
    }

    #[test]
    fn test_scan_clean_entries() {
        let entries = make_entries(&[("APP_PORT", "8080"), ("APP_ENV", "production")]);
        let result = scan_env(&entries);
        assert!(result.findings.is_empty());
        assert_eq!(result.scanned_keys, 2);
    }

    #[test]
    fn test_scan_multiple_findings() {
        let entries = make_entries(&[
            ("API_KEY", "key123"),
            ("DB_PASSWORD", "pass"),
            ("PRIVATE_KEY", "-----BEGIN RSA"),
        ]);
        let result = scan_env(&entries);
        assert!(result.findings.len() >= 2);
    }

    #[test]
    fn test_allowlist_removes_finding() {
        let entries = make_entries(&[("API_KEY", "key123"), ("DB_PASSWORD", "pass")]);
        let allowlist = vec!["API_KEY".to_string()];
        let result = scan_with_allowlist(&entries, &allowlist);
        assert!(result.findings.iter().all(|f| f.key != "API_KEY"));
    }

    #[test]
    fn test_summary_format() {
        let entries = make_entries(&[("APP_PORT", "8080")]);
        let result = scan_env(&entries);
        let summary = result.summary();
        assert!(summary.contains("Scanned 1 keys"));
        assert!(summary.contains("0 potential secrets"));
    }

    #[test]
    fn test_case_insensitive_key_matching() {
        let entries = make_entries(&[("db_password", "secret")]);
        let result = scan_env(&entries);
        assert!(!result.findings.is_empty());
    }
}
