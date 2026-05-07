#[cfg(test)]
mod tests {
    use crate::audit::{AuditAction, AuditEntry, AuditLog};

    #[test]
    fn test_audit_entry_creation() {
        let entry = AuditEntry::new(AuditAction::Push, "pushed 3 keys", true);
        assert_eq!(entry.action, AuditAction::Push);
        assert_eq!(entry.details, "pushed 3 keys");
        assert!(entry.success);
    }

    #[test]
    fn test_audit_entry_display_success() {
        let entry = AuditEntry::new(AuditAction::Pull, "pulled from s3", true);
        let display = entry.to_string();
        assert!(display.contains("PULL"));
        assert!(display.contains("OK"));
        assert!(display.contains("pulled from s3"));
    }

    #[test]
    fn test_audit_entry_display_failure() {
        let entry = AuditEntry::new(AuditAction::BackendConnect, "connection refused", false);
        let display = entry.to_string();
        assert!(display.contains("BACKEND_CONNECT"));
        assert!(display.contains("FAIL"));
    }

    #[test]
    fn test_audit_log_record_and_retrieve() {
        let mut log = AuditLog::new();
        log.record(AuditAction::Push, "pushed env", true);
        log.record(AuditAction::Pull, "pulled env", true);
        assert_eq!(log.entries().len(), 2);
    }

    #[test]
    fn test_audit_log_filter_by_action() {
        let mut log = AuditLog::new();
        log.record(AuditAction::Push, "push 1", true);
        log.record(AuditAction::Pull, "pull 1", true);
        log.record(AuditAction::Push, "push 2", false);

        let pushes = log.filter_by_action(&AuditAction::Push);
        assert_eq!(pushes.len(), 2);

        let pulls = log.filter_by_action(&AuditAction::Pull);
        assert_eq!(pulls.len(), 1);
    }

    #[test]
    fn test_audit_log_failures() {
        let mut log = AuditLog::new();
        log.record(AuditAction::Push, "ok push", true);
        log.record(AuditAction::Decrypt, "bad key", false);
        log.record(AuditAction::MergeConflict, "conflict detected", false);

        let failures = log.failures();
        assert_eq!(failures.len(), 2);
        assert!(!failures[0].success);
    }

    #[test]
    fn test_audit_log_to_log_lines() {
        let mut log = AuditLog::new();
        log.record(AuditAction::Encrypt, "encrypted 5 vars", true);
        log.record(AuditAction::ConfigUpdate, "updated backend url", true);

        let lines = log.to_log_lines();
        assert_eq!(lines.len(), 2);
        assert!(lines[0].contains("ENCRYPT"));
        assert!(lines[1].contains("CONFIG_UPDATE"));
    }

    #[test]
    fn test_empty_log() {
        let log = AuditLog::new();
        assert!(log.entries().is_empty());
        assert!(log.failures().is_empty());
        assert!(log.to_log_lines().is_empty());
    }
}
