#[cfg(test)]
mod tests {
    use super::super::env_validate_cli::*;
    use std::collections::HashMap;

    fn make_entries(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    #[test]
    fn test_run_validate_ok() {
        let args = ValidateArgs {
            entries: make_entries(&[("DB_HOST", "localhost")]),
            strict: false,
        };
        assert_eq!(run_validate(args), ValidateResult::Ok);
    }

    #[test]
    fn test_run_validate_warnings_non_strict() {
        let args = ValidateArgs {
            entries: make_entries(&[("BAD KEY", "value")]),
            strict: false,
        };
        assert!(matches!(run_validate(args), ValidateResult::Warnings(_)));
    }

    #[test]
    fn test_run_validate_errors_strict() {
        let args = ValidateArgs {
            entries: make_entries(&[("BAD KEY", "value")]),
            strict: true,
        };
        assert!(matches!(run_validate(args), ValidateResult::Errors(_)));
    }

    #[test]
    fn test_format_ok() {
        let msg = format_result(&ValidateResult::Ok);
        assert!(msg.contains("valid"));
    }

    #[test]
    fn test_format_warnings() {
        let result = ValidateResult::Warnings(vec!["Key 'X' is bad".into()]);
        let msg = format_result(&result);
        assert!(msg.contains("WARN"));
        assert!(msg.contains("X"));
    }

    #[test]
    fn test_format_errors() {
        let result = ValidateResult::Errors(vec!["Key 'Y' failed".into()]);
        let msg = format_result(&result);
        assert!(msg.contains("ERROR"));
        assert!(msg.contains("Y"));
    }

    #[test]
    fn test_empty_entries_ok() {
        let args = ValidateArgs {
            entries: HashMap::new(),
            strict: true,
        };
        assert_eq!(run_validate(args), ValidateResult::Ok);
    }
}
