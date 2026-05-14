#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::env_encrypt_field::{
        FieldEncryptConfig, FieldEncryptResult,
        encrypt_sensitive_fields, decrypt_sensitive_fields,
    };

    fn sample_env() -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("APP_NAME".to_string(), "envy-sync".to_string());
        map.insert("DB_PASSWORD".to_string(), "supersecret".to_string());
        map.insert("API_KEY".to_string(), "abc123".to_string());
        map.insert("LOG_LEVEL".to_string(), "info".to_string());
        map
    }

    #[test]
    fn test_is_sensitive_match() {
        let config = FieldEncryptConfig::new(vec!["DB_PASSWORD".to_string(), "API_KEY".to_string()]);
        assert!(config.is_sensitive("DB_PASSWORD"));
        assert!(config.is_sensitive("API_KEY"));
        assert!(!config.is_sensitive("APP_NAME"));
        assert!(!config.is_sensitive("LOG_LEVEL"));
    }

    #[test]
    fn test_encrypt_marks_sensitive_fields() {
        let env = sample_env();
        let config = FieldEncryptConfig::new(vec!["DB_PASSWORD".to_string(), "API_KEY".to_string()]);
        let results = encrypt_sensitive_fields(&env, &config, "testpass").unwrap();

        let db_pass = results.iter().find(|r| r.key == "DB_PASSWORD").unwrap();
        let app_name = results.iter().find(|r| r.key == "APP_NAME").unwrap();

        assert!(db_pass.encrypted);
        assert_ne!(db_pass.value, "supersecret");
        assert!(!app_name.encrypted);
        assert_eq!(app_name.value, "envy-sync");
    }

    #[test]
    fn test_encrypt_then_decrypt_roundtrip() {
        let env = sample_env();
        let config = FieldEncryptConfig::new(vec!["DB_PASSWORD".to_string(), "API_KEY".to_string()]);
        let encrypted = encrypt_sensitive_fields(&env, &config, "roundtrip-pass").unwrap();
        let decrypted = decrypt_sensitive_fields(&encrypted, "roundtrip-pass").unwrap();

        assert_eq!(decrypted.get("DB_PASSWORD").unwrap(), "supersecret");
        assert_eq!(decrypted.get("API_KEY").unwrap(), "abc123");
        assert_eq!(decrypted.get("APP_NAME").unwrap(), "envy-sync");
        assert_eq!(decrypted.get("LOG_LEVEL").unwrap(), "info");
    }

    #[test]
    fn test_decrypt_wrong_password_fails() {
        let env = sample_env();
        let config = FieldEncryptConfig::new(vec!["DB_PASSWORD".to_string()]);
        let encrypted = encrypt_sensitive_fields(&env, &config, "correct-pass").unwrap();
        let result = decrypt_sensitive_fields(&encrypted, "wrong-pass");
        assert!(result.is_err());
    }

    #[test]
    fn test_no_sensitive_keys_leaves_all_plain() {
        let env = sample_env();
        let config = FieldEncryptConfig::new(vec![]);
        let results = encrypt_sensitive_fields(&env, &config, "anypass").unwrap();
        for r in &results {
            assert!(!r.encrypted);
        }
    }

    #[test]
    fn test_results_sorted_by_key() {
        let env = sample_env();
        let config = FieldEncryptConfig::new(vec![]);
        let results = encrypt_sensitive_fields(&env, &config, "pass").unwrap();
        let keys: Vec<&str> = results.iter().map(|r| r.key.as_str()).collect();
        let mut sorted = keys.clone();
        sorted.sort();
        assert_eq!(keys, sorted);
    }
}
