#[cfg(test)]
mod tests {
    use super::super::config::{Config, ConfigError};
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn sample_config() -> Config {
        Config {
            remote_url: String::from("https://storage.example.com/bucket"),
            env_file_path: PathBuf::from(".env"),
            encrypted: true,
            profile: String::from("production"),
            auto_sync: true,
        }
    }

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.profile, "default");
        assert!(config.encrypted);
        assert!(!config.auto_sync);
        assert_eq!(config.env_file_path, PathBuf::from(".env"));
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.toml");
        let original = sample_config();

        original.save(&config_path).expect("save should succeed");
        let loaded = Config::load(&config_path).expect("load should succeed");

        assert_eq!(loaded.remote_url, original.remote_url);
        assert_eq!(loaded.profile, original.profile);
        assert_eq!(loaded.encrypted, original.encrypted);
        assert_eq!(loaded.auto_sync, original.auto_sync);
        assert_eq!(loaded.env_file_path, original.env_file_path);
    }

    #[test]
    fn test_load_nonexistent_file_returns_error() {
        let result = Config::load(&PathBuf::from("/nonexistent/path/config.toml"));
        assert!(matches!(result, Err(ConfigError::NotFound(_))));
    }

    #[test]
    fn test_validate_empty_remote_url_fails() {
        let mut config = Config::default();
        config.remote_url = String::new();
        let result = config.validate();
        assert!(matches!(result, Err(ConfigError::Validation(_))));
    }

    #[test]
    fn test_validate_empty_profile_fails() {
        let mut config = sample_config();
        config.profile = String::new();
        let result = config.validate();
        assert!(matches!(result, Err(ConfigError::Validation(_))));
    }

    #[test]
    fn test_validate_valid_config_passes() {
        let config = sample_config();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_save_creates_parent_directories() {
        let dir = tempdir().unwrap();
        let nested_path = dir.path().join("nested").join("deep").join("config.toml");
        let config = sample_config();
        assert!(config.save(&nested_path).is_ok());
        assert!(nested_path.exists());
    }
}
