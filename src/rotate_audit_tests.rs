#[cfg(test)]
mod tests {
    use crate::audit::AuditLog;
    use crate::rotate::RotationResult;
    use crate::rotate_audit::{record_rotation, rotation_status, RotationStatus};

    fn make_result(rotated: usize, skipped: usize, failed: Vec<&str>) -> RotationResult {
        RotationResult {
            rotated,
            skipped,
            failed: failed.into_iter().map(String::from).collect(),
        }
    }

    #[test]
    fn test_status_success() {
        let r = make_result(3, 0, vec![]);
        assert_eq!(rotation_status(&r), RotationStatus::Success);
    }

    #[test]
    fn test_status_partial_failure() {
        let r = make_result(2, 0, vec!["dev: decrypt error"]);
        assert_eq!(rotation_status(&r), RotationStatus::PartialFailure);
    }

    #[test]
    fn test_status_full_failure() {
        let r = make_result(0, 0, vec!["prod: decrypt error", "dev: decrypt error"]);
        assert_eq!(rotation_status(&r), RotationStatus::Failure);
    }

    #[test]
    fn test_record_rotation_adds_event() {
        let mut log = AuditLog::new();
        let r = make_result(3, 1, vec![]);
        record_rotation(&mut log, &r, "alice");

        let events = log.events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].action, "key_rotation");
        assert_eq!(events[0].actor, "alice");
        assert!(events[0].details.contains("rotated=3"));
        assert!(events[0].details.contains("skipped=1"));
        assert!(events[0].details.contains("failed=0"));
        assert!(events[0].details.contains("SUCCESS"));
    }

    #[test]
    fn test_record_rotation_partial_failure_details() {
        let mut log = AuditLog::new();
        let r = make_result(1, 0, vec!["staging: decrypt error"]);
        record_rotation(&mut log, &r, "bob");

        let events = log.events();
        assert_eq!(events.len(), 1);
        assert!(events[0].details.contains("PARTIAL_FAILURE"));
        assert!(events[0].details.contains("bob"));
    }
}
