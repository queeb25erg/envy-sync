#[cfg(test)]
mod tests {
    use super::super::env_secret_scan_cli::*;
    use std::collections::HashMap;

    fn make_entries(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    #[test]
    fn test_run_scan_clean_returns_ok() {
        let entries = make_entries(&[("APP_PORT", "3000")]);
        let opts = ScanCliOptions::default();
        let result = run_scan(&entries, &opts);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_scan_high_severity_fails() {
        let entries = make_entries(&[("AWS_SECRET_ACCESS_KEY", "leaked")]);
        let opts = ScanCliOptions {
            fail_on_high: true,
            ..Default::default()
        };
        let result = run_scan(&entries, &opts);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("high-severity"));
    }

    #[test]
    fn test_run_scan_high_severity_no_fail_when_disabled() {
        let entries = make_entries(&[("AWS_SECRET_ACCESS_KEY", "leaked")]);
        let opts = ScanCliOptions {
            fail_on_high: false,
            ..Default::default()
        };
        let result = run_scan(&entries, &opts);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_scan_allowlist_suppresses_finding() {
        let entries = make_entries(&[("AWS_SECRET_ACCESS_KEY", "leaked")]);
        let opts = ScanCliOptions {
            fail_on_high: true,
            allowlist: vec!["AWS_SECRET_ACCESS_KEY".to_string()],
            verbose: false,
        };
        let result = run_scan(&entries, &opts);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_scan_returns_result_with_correct_count() {
        let entries = make_entries(&[
            ("APP_TOKEN", "tok"),
            ("SAFE_VAR", "ok"),
        ]);
        let opts = ScanCliOptions::default();
        let scan = run_scan(&entries, &opts).unwrap();
        assert_eq!(scan.scanned_keys, 2);
    }
}
