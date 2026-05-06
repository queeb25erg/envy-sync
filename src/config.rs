use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub remote_url: String,
    pub env_file_path: PathBuf,
    pub encrypted: bool,
    pub profile: String,
    pub auto_sync: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            remote_url: String::new(),
            env_file_path: PathBuf::from(".env"),
            encrypted: true,
            profile: String::from("default"),
            auto_sync: false,
        }
    }
}

impl Config {
    pub fn load(path: &Path) -> Result<Self, ConfigError> {
        if !path.exists() {
            return Err(ConfigError::NotFound(path.to_path_buf()));
        }
        let contents = fs::read_to_string(path)
            .map_err(|e| ConfigError::Io(e.to_string()))?;
        let config: Config = toml::from_str(&contents)
            .map_err(|e| ConfigError::Parse(e.to_string()))?;
        Ok(config)
    }

    pub fn save(&self, path: &Path) -> Result<(), ConfigError> {
        let contents = toml::to_string_pretty(self)
            .map_err(|e| ConfigError::Serialize(e.to_string()))?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| ConfigError::Io(e.to_string()))?;
        }
        fs::write(path, contents)
            .map_err(|e| ConfigError::Io(e.to_string()))?;
        Ok(())
    }

    pub fn default_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("envy-sync")
            .join("config.toml")
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.remote_url.is_empty() {
            return Err(ConfigError::Validation(
                "remote_url must not be empty".to_string(),
            ));
        }
        if self.profile.is_empty() {
            return Err(ConfigError::Validation(
                "profile must not be empty".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("config file not found: {0}")]
    NotFound(PathBuf),
    #[error("io error: {0}")]
    Io(String),
    #[error("parse error: {0}")]
    Parse(String),
    #[error("serialize error: {0}")]
    Serialize(String),
    #[error("validation error: {0}")]
    Validation(String),
}
