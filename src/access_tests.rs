#[cfg(test)]
mod tests {
    use crate::access::AccessControl;

    fn setup() -> AccessControl {
        let mut ac = AccessControl::new();
        ac.grant("*", "alice", vec!["read", "write", "admin"]);
        ac.grant("DB_PASSWORD", "bob", vec!["read"]);
        ac.grant("API_KEY", "charlie", vec!["read", "write"]);
        ac
    }

    #[test]
    fn test_admin_can_read_any_key() {
        let ac = setup();
        assert!(ac.can("alice", "DB_PASSWORD", "read"));
        assert!(ac.can("alice", "API_KEY", "write"));
        assert!(ac.can("alice", "SOME_OTHER_KEY", "admin"));
    }

    #[test]
    fn test_restricted_identity_read_allowed_key() {
        let ac = setup();
        assert!(ac.can("bob", "DB_PASSWORD", "read"));
    }

    #[test]
    fn test_restricted_identity_cannot_write() {
        let ac = setup();
        assert!(!ac.can("bob", "DB_PASSWORD", "write"));
    }

    #[test]
    fn test_restricted_identity_cannot_access_other_key() {
        let ac = setup();
        assert!(!ac.can("bob", "API_KEY", "read"));
    }

    #[test]
    fn test_charlie_can_write_api_key() {
        let ac = setup();
        assert!(ac.can("charlie", "API_KEY", "read"));
        assert!(ac.can("charlie", "API_KEY", "write"));
        assert!(!ac.can("charlie", "API_KEY", "admin"));
    }

    #[test]
    fn test_unknown_identity_denied() {
        let ac = setup();
        assert!(!ac.can("mallory", "DB_PASSWORD", "read"));
        assert!(!ac.can("mallory", "*", "admin"));
    }

    #[test]
    fn test_revoke_removes_access() {
        let mut ac = setup();
        assert!(ac.can("bob", "DB_PASSWORD", "read"));
        ac.revoke("DB_PASSWORD", "bob");
        assert!(!ac.can("bob", "DB_PASSWORD", "read"));
    }

    #[test]
    fn test_list_identities() {
        let ac = setup();
        let ids = ac.list_identities("*");
        assert!(ids.contains(&"alice".to_string()));
    }

    #[test]
    fn test_summary_contains_all_patterns() {
        let ac = setup();
        let summary = ac.summary();
        assert!(summary.contains_key("*"));
        assert!(summary.contains_key("DB_PASSWORD"));
        assert!(summary.contains_key("API_KEY"));
    }
}
