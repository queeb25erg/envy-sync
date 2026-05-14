#[cfg(test)]
mod tests {
    use super::super::env_alias::AliasRegistry;
    use std::collections::HashMap;

    fn registry_with_defaults() -> AliasRegistry {
        let mut reg = AliasRegistry::new();
        reg.register("DB_URL", "DATABASE_URL").unwrap();
        reg.register("DB_PASS", "DATABASE_PASSWORD").unwrap();
        reg
    }

    #[test]
    fn test_register_and_resolve() {
        let reg = registry_with_defaults();
        assert_eq!(reg.resolve("DB_URL"), "DATABASE_URL");
        assert_eq!(reg.resolve("DB_PASS"), "DATABASE_PASSWORD");
    }

    #[test]
    fn test_resolve_unknown_key_unchanged() {
        let reg = registry_with_defaults();
        assert_eq!(reg.resolve("UNKNOWN_KEY"), "UNKNOWN_KEY");
    }

    #[test]
    fn test_register_alias_equals_canonical_is_error() {
        let mut reg = AliasRegistry::new();
        let result = reg.register("DATABASE_URL", "DATABASE_URL");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot equal"));
    }

    #[test]
    fn test_aliases_for() {
        let reg = registry_with_defaults();
        let mut aliases = reg.aliases_for("DATABASE_URL");
        aliases.sort();
        assert_eq!(aliases, vec!["DB_URL"]);
    }

    #[test]
    fn test_aliases_for_no_match() {
        let reg = registry_with_defaults();
        assert!(reg.aliases_for("NONEXISTENT").is_empty());
    }

    #[test]
    fn test_normalize_env_replaces_alias() {
        let reg = registry_with_defaults();
        let mut env = HashMap::new();
        env.insert("DB_URL".to_string(), "postgres://localhost/db".to_string());
        env.insert("PORT".to_string(), "8080".to_string());

        let normalized = reg.normalize_env(&env);
        assert!(normalized.contains_key("DATABASE_URL"));
        assert!(!normalized.contains_key("DB_URL"));
        assert_eq!(normalized["PORT"], "8080");
    }

    #[test]
    fn test_normalize_env_canonical_wins_over_alias() {
        let reg = registry_with_defaults();
        let mut env = HashMap::new();
        env.insert("DB_URL".to_string(), "alias-value".to_string());
        env.insert("DATABASE_URL".to_string(), "canonical-value".to_string());

        let normalized = reg.normalize_env(&env);
        assert_eq!(normalized["DATABASE_URL"], "canonical-value");
    }

    #[test]
    fn test_len_and_is_empty() {
        let mut reg = AliasRegistry::new();
        assert!(reg.is_empty());
        reg.register("A", "B").unwrap();
        assert_eq!(reg.len(), 1);
        assert!(!reg.is_empty());
    }
}
