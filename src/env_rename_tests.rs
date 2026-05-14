#[cfg(test)]
mod tests {
    use super::super::env_rename::*;
    use std::collections::HashMap;

    fn make_env() -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("DATABASE_URL".to_string(), "postgres://localhost/db".to_string());
        m.insert("API_KEY".to_string(), "secret123".to_string());
        m.insert("DEBUG".to_string(), "true".to_string());
        m
    }

    #[test]
    fn test_rename_existing_key() {
        let env = make_env();
        let (updated, result) = rename_key(&env, "API_KEY", "SERVICE_API_KEY", false);
        assert!(result.renamed);
        assert!(updated.contains_key("SERVICE_API_KEY"));
        assert!(!updated.contains_key("API_KEY"));
        assert_eq!(updated["SERVICE_API_KEY"], "secret123");
        assert!(result.reason.is_none());
    }

    #[test]
    fn test_rename_missing_key_returns_error() {
        let env = make_env();
        let (updated, result) = rename_key(&env, "MISSING_KEY", "NEW_KEY", false);
        assert!(!result.renamed);
        assert!(result.reason.is_some());
        assert!(result.reason.unwrap().contains("not found"));
        assert_eq!(updated.len(), env.len());
    }

    #[test]
    fn test_rename_conflicts_without_force() {
        let env = make_env();
        let (updated, result) = rename_key(&env, "API_KEY", "DEBUG", false);
        assert!(!result.renamed);
        assert!(result.reason.unwrap().contains("already exists"));
        assert_eq!(updated["API_KEY"], "secret123");
        assert_eq!(updated["DEBUG"], "true");
    }

    #[test]
    fn test_rename_conflicts_with_force_overwrites() {
        let env = make_env();
        let (updated, result) = rename_key(&env, "API_KEY", "DEBUG", true);
        assert!(result.renamed);
        assert_eq!(updated["DEBUG"], "secret123");
        assert!(!updated.contains_key("API_KEY"));
    }

    #[test]
    fn test_bulk_rename_all_succeed() {
        let env = make_env();
        let renames = vec![
            ("API_KEY".to_string(), "SERVICE_KEY".to_string()),
            ("DEBUG".to_string(), "APP_DEBUG".to_string()),
        ];
        let (updated, results) = bulk_rename(&env, &renames, false);
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r.renamed));
        assert!(updated.contains_key("SERVICE_KEY"));
        assert!(updated.contains_key("APP_DEBUG"));
        assert!(!updated.contains_key("API_KEY"));
        assert!(!updated.contains_key("DEBUG"));
    }

    #[test]
    fn test_bulk_rename_partial_failure() {
        let env = make_env();
        let renames = vec![
            ("API_KEY".to_string(), "SERVICE_KEY".to_string()),
            ("NONEXISTENT".to_string(), "SOMETHING".to_string()),
        ];
        let (updated, results) = bulk_rename(&env, &renames, false);
        assert!(results[0].renamed);
        assert!(!results[1].renamed);
        assert!(updated.contains_key("SERVICE_KEY"));
        assert!(!updated.contains_key("SOMETHING"));
    }
}
