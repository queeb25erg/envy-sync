#[cfg(test)]
mod tests {
    use super::super::env_validate::*;
    use std::collections::HashMap;

    fn make_entries(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    #[test]
    fn test_valid_key() {
        assert!(validate_key("DATABASE_URL").is_ok());
        assert!(validate_key("API_KEY_123").is_ok());
    }

    #[test]
    fn test_empty_key() {
        assert_eq!(validate_key(""), Err(ValidationError::EmptyKey));
    }

    #[test]
    fn test_invalid_key_char() {
        let err = validate_key("BAD-KEY");
        assert!(matches!(err, Err(ValidationError::InvalidKeyChar(_, '-'))));
    }

    #[test]
    fn test_valid_entries() {
        let entries = make_entries(&[("HOST", "localhost"), ("PORT", "5432")]);
        assert!(is_valid(&entries));
    }

    #[test]
    fn test_empty_value_detected() {
        let entries = make_entries(&[("HOST", "  ")]);
        let errors = validate_entries(&entries);
        assert!(errors.iter().any(|e| matches!(e, ValidationError::EmptyValue(_))));
    }

    #[test]
    fn test_multiple_errors() {
        let entries = make_entries(&[("BAD KEY", "  ")]);
        let errors = validate_entries(&entries);
        assert!(errors.len() >= 2);
    }

    #[test]
    fn test_display_empty_key() {
        let msg = format!("{}", ValidationError::EmptyKey);
        assert!(msg.contains("Empty key"));
    }

    #[test]
    fn test_display_duplicate_key() {
        let msg = format!("{}", ValidationError::DuplicateKey("FOO".into()));
        assert!(msg.contains("FOO"));
    }
}
