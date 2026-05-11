#[cfg(test)]
mod integration_tests {
    use super::super::env_secret_scan::*;
    use super::super::env_secret_scan_cli::*;
    use std::collections::HashMap;

    fn realistic_env() -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("APP_ENV".into(), "production".into());
        m.insert("APP_PORT".into(), "8080".into());
        m.insert("DATABASE_URL".into(), "postgres://localhost/db".into());
        m.insert("AWS_SECRET_ACCESS_KEY".into(), "wJalrXUtnFEMI/K7MDENG".into());
        m.insert("JWT_TOKEN".into(), "eyJhbGciOiJIUzI1NiJ9".into());
        m.insert("STRIPE_API_KEY".into(), "sk_live_abc123".into());
        m
    }

    #[test]
    fn test_full_scan_realistic_env() {
        let entries = realistic_env();
        let result = scan_env(&entries);
        assert!(result.has_high_severity(), "Should detect AWS secret as high severity");
        assert!(result.findings.len() >= 3);
        assert_eq!(result.scanned_keys, 6);
    }

    #[test]
    fn test_cli_scan_with_partial_allowlist() {
        let entries = realistic_env();
        let opts = ScanCliOptions {
            fail_on_high: true,
            allowlist: vec!["AWS_SECRET_ACCESS_KEY".to_string()],
            verbose: false,
        };
        // After allowlisting the high-severity key, should not fail
        let result = run_scan(&entries, &opts);
        assert!(result.is_ok());
        let scan = result.unwrap();
        assert!(scan.findings.iter().all(|f| f.key != "AWS_SECRET_ACCESS_KEY"));
    }

    #[test]
    fn test_scan_empty_env_is_clean() {
        let entries = HashMap::new();
        let result = scan_env(&entries);
        assert!(result.findings.is_empty());
        assert_eq!(result.scanned_keys, 0);
        assert!(!result.has_high_severity());
    }

    #[test]
    fn test_summary_reflects_findings() {
        let entries = realistic_env();
        let result = scan_env(&entries);
        let summary = result.summary();
        assert!(summary.contains("Scanned 6 keys"));
        assert!(!summary.contains("0 potential secrets"));
    }
}
