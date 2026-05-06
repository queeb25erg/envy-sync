use std::collections::HashMap;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Represents a stored env file entry in the remote backend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteEntry {
    pub key: String,
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
    pub updated_at: u64,
}

/// Trait defining the interface for remote storage backends
#[async_trait]
pub trait Backend: Send + Sync {
    /// Upload an encrypted entry to the backend
    async fn put(&self, name: &str, entry: &RemoteEntry) -> Result<()>;

    /// Retrieve an encrypted entry from the backend
    async fn get(&self, name: &str) -> Result<Option<RemoteEntry>>;

    /// List all available env file names in the backend
    async fn list(&self) -> Result<Vec<String>>;

    /// Delete an entry from the backend
    async fn delete(&self, name: &str) -> Result<()>;
}

/// In-memory backend for testing and local development
#[derive(Debug, Default)]
pub struct MemoryBackend {
    store: tokio::sync::Mutex<HashMap<String, RemoteEntry>>,
}

impl MemoryBackend {
    pub fn new() -> Self {
        Self {
            store: tokio::sync::Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl Backend for MemoryBackend {
    async fn put(&self, name: &str, entry: &RemoteEntry) -> Result<()> {
        let mut store = self.store.lock().await;
        store.insert(name.to_string(), entry.clone());
        Ok(())
    }

    async fn get(&self, name: &str) -> Result<Option<RemoteEntry>> {
        let store = self.store.lock().await;
        Ok(store.get(name).cloned())
    }

    async fn list(&self) -> Result<Vec<String>> {
        let store = self.store.lock().await;
        Ok(store.keys().cloned().collect())
    }

    async fn delete(&self, name: &str) -> Result<()> {
        let mut store = self.store.lock().await;
        store.remove(name);
        Ok(())
    }
}
