#[cfg(test)]
mod tests {
    use super::super::env_clone::*;
    use std::collections::HashMap;

    fn make_profiles() -> HashMap<String, HashMap<String, String>> {
        let mut profiles = HashMap::new();
        let mut dev = HashMap::new();
        dev.insert("DB_HOST".to_string(), "localhost".to_string());
        dev.insert("DB_PORT".to_string(), "5432".to_string());
        dev.insert("API_KEY".to_string(), "dev-secret".to_string());
        profiles.insert("dev".to_string(), dev);
        profiles
    }

    #[test]
    fn test_clone_all_keys() {
        let mut profiles = make_profiles();
        let opts = CloneOptions {
            source: "dev".to_string(),
            destination: "staging".to_string(),
            key_filter: None,
            overwrite: false,
        };
        let result = clone_env(&mut profiles, &opts).unwrap();
        assert_eq!(result.keys_cloned.len(), 3);
        assert!(result.keys_skipped.is_empty());
        assert!(profiles.contains_key("staging"));
        assert_eq!(profiles["staging"]["DB_HOST"], "localhost");
    }

    #[test]
    fn test_clone_with_filter() {
        let mut profiles = make_profiles();
        let opts = CloneOptions {
            source: "dev".to_string(),
            destination: "staging".to_string(),
            key_filter: Some(vec!["DB_HOST".to_string(), "DB_PORT".to_string()]),
            overwrite: false,
        };
        let result = clone_env(&mut profiles, &opts).unwrap();
        assert_eq!(result.keys_cloned, vec!["DB_HOST", "DB_PORT"]);
        assert_eq!(result.keys_skipped, vec!["API_KEY"]);
        assert!(!profiles["staging"].contains_key("API_KEY"));
    }

    #[test]
    fn test_clone_source_not_found() {
        let mut profiles = make_profiles();
        let opts = CloneOptions {
            source: "nonexistent".to_string(),
            destination: "staging".to_string(),
            key_filter: None,
            overwrite: false,
        };
        let err = clone_env(&mut profiles, &opts).unwrap_err();
        assert_eq!(err, CloneError::SourceNotFound("nonexistent".to_string()));
    }

    #[test]
    fn test_clone_destination_exists_no_overwrite() {
        let mut profiles = make_profiles();
        profiles.insert("staging".to_string(), HashMap::new());
        let opts = CloneOptions {
            source: "dev".to_string(),
            destination: "staging".to_string(),
            key_filter: None,
            overwrite: false,
        };
        let err = clone_env(&mut profiles, &opts).unwrap_err();
        assert_eq!(err, CloneError::DestinationExists("staging".to_string()));
    }

    #[test]
    fn test_clone_destination_exists_with_overwrite() {
        let mut profiles = make_profiles();
        let mut old_staging = HashMap::new();
        old_staging.insert("OLD_KEY".to_string(), "old_val".to_string());
        profiles.insert("staging".to_string(), old_staging);
        let opts = CloneOptions {
            source: "dev".to_string(),
            destination: "staging".to_string(),
            key_filter: None,
            overwrite: true,
        };
        let result = clone_env(&mut profiles, &opts).unwrap();
        assert_eq!(result.keys_cloned.len(), 3);
        assert!(!profiles["staging"].contains_key("OLD_KEY"));
    }

    #[test]
    fn test_clone_error_display() {
        assert_eq!(
            CloneError::SourceNotFound("x".to_string()).to_string(),
            "Source profile 'x' not found"
        );
        assert_eq!(
            CloneError::DestinationExists("y".to_string()).to_string(),
            "Destination profile 'y' already exists"
        );
        assert_eq!(CloneError::EmptySource.to_string(), "Source profile has no keys to clone");
    }
}
