#[cfg(test)]
mod tests {
    use super::super::env_deprecate::*;
    use std::collections::HashMap;

    fn sample_registry() -> DeprecationRegistry {
        let mut reg = DeprecationRegistry::new();
        reg.register(
            DeprecationEntry::new("OLD_API_KEY")
                .with_reason("Migrated to new auth system")
                .with_replacement("API_KEY_V2")
                .with_since("v1.4.0"),
        );
        reg.register(
            DeprecationEntry::new("LEGACY_DB_URL")
                .with_reason("Use connection pooling config instead"),
        );
        reg
    }

    #[test]
    fn test_is_deprecated_known_key() {
        let reg = sample_registry();
        assert!(reg.is_deprecated("OLD_API_KEY"));
    }

    #[test]
    fn test_is_deprecated_unknown_key() {
        let reg = sample_registry();
        assert!(!reg.is_deprecated("NEW_KEY"));
    }

    #[test]
    fn test_get_entry() {
        let reg = sample_registry();
        let entry = reg.get("OLD_API_KEY").unwrap();
        assert_eq!(entry.replacement.as_deref(), Some("API_KEY_V2"));
        assert_eq!(entry.since.as_deref(), Some("v1.4.0"));
    }

    #[test]
    fn test_format_warning_full() {
        let entry = DeprecationEntry::new("OLD_API_KEY")
            .with_reason("Migrated to new auth system")
            .with_replacement("API_KEY_V2")
            .with_since("v1.4.0");
        let warning = entry.format_warning();
        assert!(warning.contains("DEPRECATED"));
        assert!(warning.contains("OLD_API_KEY"));
        assert!(warning.contains("v1.4.0"));
        assert!(warning.contains("API_KEY_V2"));
        assert!(warning.contains("Migrated to new auth system"));
    }

    #[test]
    fn test_format_warning_minimal() {
        let entry = DeprecationEntry::new("BARE_KEY");
        let warning = entry.format_warning();
        assert!(warning.contains("DEPRECATED"));
        assert!(warning.contains("BARE_KEY"));
    }

    #[test]
    fn test_check_env_finds_deprecated() {
        let reg = sample_registry();
        let mut env = HashMap::new();
        env.insert("OLD_API_KEY".to_string(), "abc123".to_string());
        env.insert("ACTIVE_KEY".to_string(), "xyz".to_string());
        let hits = reg.check_env(&env);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].key, "OLD_API_KEY");
    }

    #[test]
    fn test_check_env_no_deprecated_present() {
        let reg = sample_registry();
        let mut env = HashMap::new();
        env.insert("SOME_KEY".to_string(), "val".to_string());
        let hits = reg.check_env(&env);
        assert!(hits.is_empty());
    }

    #[test]
    fn test_all_deprecated_keys() {
        let reg = sample_registry();
        let mut keys = reg.all_deprecated_keys();
        keys.sort();
        assert_eq!(keys, vec!["LEGACY_DB_URL", "OLD_API_KEY"]);
    }
}
