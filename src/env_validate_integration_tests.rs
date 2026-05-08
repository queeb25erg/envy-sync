#[cfg(test)]
mod integration_tests {
    use crate::env_validate::{is_valid, validate_entries, ValidationError};
    use crate::env_validate_cli::{format_result, run_validate, ValidateArgs, ValidateResult};
    use std::collections::HashMap;

    fn entries(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    #[test]
    fn test_full_valid_flow() {
        let map = entries(&[("API_KEY", "secret"), ("DB_URL", "postgres://localhost/db")]);
        assert!(is_valid(&map));
        let args = ValidateArgs { entries: map, strict: true };
        let result = run_validate(args);
        assert_eq!(result, ValidateResult::Ok);
        let output = format_result(&result);
        assert!(output.contains("valid"));
    }

    #[test]
    fn test_full_invalid_flow_strict() {
        let map = entries(&[("INVALID KEY", "  ")]);
        assert!(!is_valid(&map));
        let args = ValidateArgs { entries: map, strict: true };
        let result = run_validate(args);
        assert!(matches!(result, ValidateResult::Errors(_)));
        let output = format_result(&result);
        assert!(output.contains("ERROR"));
    }

    #[test]
    fn test_full_invalid_flow_lenient() {
        let map = entries(&[("INVALID KEY", "value")]);
        let args = ValidateArgs { entries: map, strict: false };
        let result = run_validate(args);
        assert!(matches!(result, ValidateResult::Warnings(_)));
        let output = format_result(&result);
        assert!(output.contains("WARN"));
    }

    #[test]
    fn test_error_messages_contain_key_name() {
        let map = entries(&[("BAD-KEY", "val")]);
        let errors = validate_entries(&map);
        let has_key_name = errors.iter().any(|e| e.to_string().contains("BAD-KEY"));
        assert!(has_key_name);
    }

    #[test]
    fn test_empty_value_in_strict_mode_is_error() {
        let map = entries(&[("GOOD_KEY", "")]);
        let args = ValidateArgs { entries: map, strict: true };
        assert!(matches!(run_validate(args), ValidateResult::Errors(_)));
    }
}
