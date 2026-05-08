//! Serialization and persistence helpers for ProfileStore

use crate::profile::ProfileStore;
use serde_json;

#[derive(Debug)]
pub enum ProfileStoreError {
    SerializationError(String),
    DeserializationError(String),
}

impl std::fmt::Display for ProfileStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProfileStoreError::SerializationError(e) => write!(f, "Serialization error: {}", e),
            ProfileStoreError::DeserializationError(e) => write!(f, "Deserialization error: {}", e),
        }
    }
}

pub fn serialize_store(store: &ProfileStore) -> Result<Vec<u8>, ProfileStoreError> {
    serde_json::to_vec_pretty(store)
        .map_err(|e| ProfileStoreError::SerializationError(e.to_string()))
}

pub fn deserialize_store(data: &[u8]) -> Result<ProfileStore, ProfileStoreError> {
    serde_json::from_slice(data)
        .map_err(|e| ProfileStoreError::DeserializationError(e.to_string()))
}

pub fn store_to_string(store: &ProfileStore) -> Result<String, ProfileStoreError> {
    serde_json::to_string_pretty(store)
        .map_err(|e| ProfileStoreError::SerializationError(e.to_string()))
}

pub fn store_from_str(s: &str) -> Result<ProfileStore, ProfileStoreError> {
    serde_json::from_str(s)
        .map_err(|e| ProfileStoreError::DeserializationError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile::Profile;

    #[test]
    fn roundtrip_empty_store() {
        let store = ProfileStore::new();
        let data = serialize_store(&store).unwrap();
        let restored = deserialize_store(&data).unwrap();
        assert_eq!(restored.list_names().len(), 0);
    }

    #[test]
    fn roundtrip_with_profiles() {
        let mut store = ProfileStore::new();
        let mut p = Profile::new("prod");
        p.set_var("DB_URL", "postgres://prod");
        store.add(p);
        let s = store_to_string(&store).unwrap();
        let restored = store_from_str(&s).unwrap();
        assert!(restored.contains("prod"));
        assert_eq!(restored.get("prod").unwrap().get_var("DB_URL").unwrap(), "postgres://prod");
    }
}
