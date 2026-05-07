#[cfg(test)]
mod tests {
    use crate::lock::{try_acquire, LockError, SyncLock};
    use std::thread;
    use std::time::Duration;

    fn make_lock(machine_id: &str, ttl: u64) -> SyncLock {
        SyncLock::new(machine_id.to_string(), ttl)
    }

    #[test]
    fn test_new_lock_is_not_expired() {
        let lock = make_lock("machine-a", 60);
        assert!(!lock.is_expired());
    }

    #[test]
    fn test_lock_held_by_correct_machine() {
        let lock = make_lock("machine-a", 60);
        assert!(lock.is_held_by("machine-a"));
        assert!(!lock.is_held_by("machine-b"));
    }

    #[test]
    fn test_lock_serialization_roundtrip() {
        let lock = make_lock("machine-xyz", 120);
        let bytes = lock.to_bytes().expect("serialization failed");
        let restored = SyncLock::from_bytes(&bytes).expect("deserialization failed");
        assert_eq!(lock, restored);
    }

    #[test]
    fn test_try_acquire_with_no_existing_lock() {
        let result = try_acquire(None, "machine-a", 60);
        assert!(result.is_ok());
        let lock = result.unwrap();
        assert_eq!(lock.machine_id, "machine-a");
    }

    #[test]
    fn test_try_acquire_same_machine_reacquires() {
        let existing = make_lock("machine-a", 60);
        let bytes = existing.to_bytes().unwrap();
        let result = try_acquire(Some(&bytes), "machine-a", 60);
        assert!(result.is_ok());
    }

    #[test]
    fn test_try_acquire_blocked_by_other_machine() {
        let existing = make_lock("machine-b", 300);
        let bytes = existing.to_bytes().unwrap();
        let result = try_acquire(Some(&bytes), "machine-a", 60);
        assert_eq!(result, Err(LockError::HeldByOther("machine-b".to_string())));
    }

    #[test]
    fn test_try_acquire_after_lock_expiry() {
        // Create a lock with 0 TTL so it's immediately expired
        let mut expired = make_lock("machine-b", 0);
        expired.acquired_at = 0; // force epoch time = definitely expired
        let bytes = expired.to_bytes().unwrap();
        let result = try_acquire(Some(&bytes), "machine-a", 60);
        assert!(result.is_ok(), "Should acquire after expiry");
    }

    #[test]
    fn test_try_acquire_with_invalid_bytes() {
        let garbage = b"not valid json at all!!!";
        let result = try_acquire(Some(garbage), "machine-a", 60);
        assert!(matches!(result, Err(LockError::SerializationError(_))));
    }

    #[test]
    fn test_lock_error_display() {
        let err = LockError::HeldByOther("machine-z".to_string());
        assert!(err.to_string().contains("machine-z"));
        let err2 = LockError::SerializationError("bad json".to_string());
        assert!(err2.to_string().contains("bad json"));
    }
}
