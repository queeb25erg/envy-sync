use std::collections::HashMap;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Represents an encrypted env file stored in a backend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvRecord {
    pub name: String,
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
    pub created_at: u64,
    pub updated_at: u64,
}

/// Trait that all storage backends must implement.
pub trait StorageBackend: Send + Sync {
    /// Store an encrypted env record under the given key.
    fn put(&mut self, key: &str, record: EnvRecord) -> Result<()>;

    /// Retrieve an encrypted env record by key.
    fn get(&self, key: &str) -> Result<Option<EnvRecord>>;

    /// List all stored keys.
    fn list(&self) -> Result<Vec<String>>;

    /// Delete a record by key.
    fn delete(&mut self, key: &str) -> Result<bool>;
}

/// A simple in-memory storage backend, useful for testing.
#[derive(Default, Debug)]
pub struct MemoryBackend {
    store: HashMap<String, EnvRecord>,
}

impl MemoryBackend {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
}

impl StorageBackend for MemoryBackend {
    fn put(&mut self, key: &str, record: EnvRecord) -> Result<()> {
        self.store.insert(key.to_string(), record);
        Ok(())
    }

    fn get(&self, key: &str) -> Result<Option<EnvRecord>> {
        Ok(self.store.get(key).cloned())
    }

    fn list(&self) -> Result<Vec<String>> {
        let mut keys: Vec<String> = self.store.keys().cloned().collect();
        keys.sort();
        Ok(keys)
    }

    fn delete(&mut self, key: &str) -> Result<bool> {
        Ok(self.store.remove(key).is_some())
    }
}

/// Build an `EnvRecord` with the current Unix timestamp.
pub fn new_record(name: &str, ciphertext: Vec<u8>, nonce: Vec<u8>) -> Result<EnvRecord> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .context("system time before Unix epoch")?;
    let ts = now.as_secs();
    Ok(EnvRecord {
        name: name.to_string(),
        ciphertext,
        nonce,
        created_at: ts,
        updated_at: ts,
    })
}
