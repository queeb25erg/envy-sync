//! env_expire_cli_tests.rs — Tests for the expiry CLI command handlers.

#[cfg(test)]
mod tests {
    use crate::env_expire::ExpiryStore;
    use crate::env_expire_cli::*;
    use chrono::{Duration, Utc};

    fn future_rfc3339(days: i64) -> String {
        (Utc::now() + Duration::days(days)).to_rfc3339()
    }

    fn past_rfc3339(days: i64) -> String {
        (Utc::now() - Duration::days(days)).to_rfc3339()
    }

    #[test]
    fn test_set_expiry_success() {
        let mut store = ExpiryStore::new();
        let date = future_rfc3339(30);
        let result = cmd_set_expiry(&mut store, "API_KEY", &date, 7);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("API_KEY"));
    }

    #[test]
    fn test_set_expiry_invalid_date() {
        let mut store = ExpiryStore::new();
        let result = cmd_set_expiry(&mut store, "KEY", "not-a-date", 7);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid date"));
    }

    #[test]
    fn test_remove_existing_key() {
        let mut store = ExpiryStore::new();
        let date = future_rfc3339(10);
        cmd_set_expiry(&mut store, "MY_KEY", &date, 3).unwrap();
        let msg = cmd_remove_expiry(&mut store, "MY_KEY");
        assert!(msg.contains("Removed"));
    }

    #[test]
    fn test_remove_missing_key() {
        let mut store = ExpiryStore::new();
        let msg = cmd_remove_expiry(&mut store, "GHOST");
        assert!(msg.contains("No expiry entry found"));
    }

    #[test]
    fn test_check_expiry_no_entry() {
        let store = ExpiryStore::new();
        let msg = cmd_check_expiry(&store, "UNKNOWN");
        assert!(msg.contains("No expiry set"));
    }

    #[test]
    fn test_check_expiry_expired() {
        let mut store = ExpiryStore::new();
        let date = past_rfc3339(3);
        cmd_set_expiry(&mut store, "OLD", &date, 7).unwrap();
        let msg = cmd_check_expiry(&store, "OLD");
        assert!(msg.contains("EXPIRED"));
    }

    #[test]
    fn test_list_expired_empty() {
        let store = ExpiryStore::new();
        let msg = cmd_list_expired(&store);
        assert!(msg.contains("No expired"));
    }

    #[test]
    fn test_list_warnings_output() {
        let mut store = ExpiryStore::new();
        let soon = future_rfc3339(2);
        cmd_set_expiry(&mut store, "WARN_KEY", &soon, 7).unwrap();
        let msg = cmd_list_warnings(&store);
        assert!(msg.contains("WARN_KEY"));
        assert!(msg.contains("day(s) left"));
    }

    #[test]
    fn test_list_warnings_empty() {
        let store = ExpiryStore::new();
        let msg = cmd_list_warnings(&store);
        assert!(msg.contains("No keys expiring soon"));
    }
}
