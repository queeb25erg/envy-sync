//! Distributed lock mechanism to prevent concurrent sync conflicts.
//!
//! Provides a simple file-based lock that can be stored in the remote backend
//! to coordinate access across multiple machines.

use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

/// Represents a distributed lock stored in the remote backend.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SyncLock {
    /// Unique identifier for the machine holding the lock.
    pub machine_id: String,
    /// Unix timestamp when the lock was acquired.
    pub acquired_at: u64,
    /// TTL in seconds before the lock is considered stale.
    pub ttl_seconds: u64,
}

impl SyncLock {
    /// Create a new lock for the given machine.
    pub fn new(machine_id: String, ttl_seconds: u64) -> Self {
        let acquired_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self {
            machine_id,
            acquired_at,
            ttl_seconds,
        }
    }

    /// Returns true if this lock has expired based on current time.
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        now.saturating_sub(self.acquired_at) > self.ttl_seconds
    }

    /// Returns true if the lock is held by the given machine.
    pub fn is_held_by(&self, machine_id: &str) -> bool {
        self.machine_id == machine_id
    }

    /// Serialize the lock to a JSON byte vector.
    pub fn to_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }

    /// Deserialize a lock from a JSON byte slice.
    pub fn from_bytes(data: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(data)
    }
}

/// Errors that can occur during lock operations.
#[derive(Debug, PartialEq)]
pub enum LockError {
    /// Another machine currently holds the lock.
    HeldByOther(String),
    /// Failed to serialize or deserialize lock data.
    SerializationError(String),
}

impl std::fmt::Display for LockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LockError::HeldByOther(id) => write!(f, "Lock is held by machine: {}", id),
            LockError::SerializationError(e) => write!(f, "Lock serialization error: {}", e),
        }
    }
}

/// Attempt to acquire a lock, returning an error if a valid lock exists for another machine.
pub fn try_acquire(existing: Option<&[u8]>, machine_id: &str, ttl_seconds: u64) -> Result<SyncLock, LockError> {
    if let Some(data) = existing {
        let lock = SyncLock::from_bytes(data)
            .map_err(|e| LockError::SerializationError(e.to_string()))?;
        if !lock.is_expired() && !lock.is_held_by(machine_id) {
            return Err(LockError::HeldByOther(lock.machine_id.clone()));
        }
    }
    Ok(SyncLock::new(machine_id.to_string(), ttl_seconds))
}
