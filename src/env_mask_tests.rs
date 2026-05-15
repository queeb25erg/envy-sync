#[cfg(test)]
mod tests {
    use super::super::env_mask::*;
    use std::collections::HashMap;

    #[test]
    fn test_is_sensitive_key_detects_secret() {
        assert!(is_sensitive_key("APP_SECRET"));
        assert!(is_sensitive_key("DB_PASSWORD"));
        assert!(is_sensitive_key("GITHUB_TOKEN"));
        assert!(is_sensitive_key("AWS_API_KEY"));
        assert!(is_sensitive_key("PRIVATE_KEY"));
        assert!(is_sensitive_key("AUTH_TOKEN"));
    }

    #[test]
    fn test_is_sensitive_key_allows_non_sensitive() {
        assert!(!is_sensitive_key("APP_ENV"));
        assert!(!is_sensitive_key("DATABASE_HOST"));
        assert!(!is_sensitive_key("PORT"));
        assert!(!is_sensitive_key("LOG_LEVEL"));
    }

    #[test]
    fn test_mask_string_short_value() {
        assert_eq!(mask_string("abc"), "****");
        assert_eq!(mask_string("ab"), "****");
        assert_eq!(mask_string(""), "****");
    }

    #[test]
    fn test_mask_string_long_value() {
        let result = mask_string("supersecretvalue");
        assert!(result.starts_with("su"));
        assert!(result.ends_with("ue"));
        assert!(result.contains("****"));
    }

    #[test]
    fn test_mask_value_sensitive_key() {
        let masked = mask_value("DB_PASSWORD", "mysecretpassword");
        assert_ne!(masked, "mysecretpassword");
        assert!(masked.contains("****"));
    }

    #[test]
    fn test_mask_value_non_sensitive_key() {
        let result = mask_value("APP_ENV", "production");
        assert_eq!(result, "production");
    }

    #[test]
    fn test_mask_env_map() {
        let mut env = HashMap::new();
        env.insert("APP_ENV".to_string(), "production".to_string());
        env.insert("DB_PASSWORD".to_string(), "s3cr3tpass".to_string());
        env.insert("API_KEY".to_string(), "abcdefghij".to_string());

        let masked = mask_env_map(&env);
        assert_eq!(masked["APP_ENV"], "production");
        assert_ne!(masked["DB_PASSWORD"], "s3cr3tpass");
        assert_ne!(masked["API_KEY"], "abcdefghij");
    }

    #[test]
    fn test_mask_env_pairs() {
        let pairs = vec![
            ("LOG_LEVEL".to_string(), "debug".to_string()),
            ("GITHUB_TOKEN".to_string(), "ghp_longtoken12345".to_string()),
        ];
        let masked = mask_env_pairs(&pairs);
        assert_eq!(masked[0].1, "debug");
        assert_ne!(masked[1].1, "ghp_longtoken12345");
        assert!(masked[1].1.contains("****"));
    }
}
