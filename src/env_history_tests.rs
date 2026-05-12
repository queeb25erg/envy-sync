#[cfg(test)]
mod tests {
    use crate::env_history::EnvHistory;
    use crate::env_history_cli::{run_history_command, HistoryCommand};

    fn make_history() -> EnvHistory {
        let mut h = EnvHistory::new();
        h.record("DATABASE_URL", "postgres://old", "alice");
        h.record("DATABASE_URL", "postgres://new", "bob");
        h.record("API_KEY", "key-abc", "alice");
        h
    }

    #[test]
    fn test_record_and_get() {
        let h = make_history();
        let entries = h.get("DATABASE_URL");
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].value, "postgres://old");
        assert_eq!(entries[1].value, "postgres://new");
    }

    #[test]
    fn test_latest() {
        let h = make_history();
        let latest = h.latest("DATABASE_URL").unwrap();
        assert_eq!(latest.value, "postgres://new");
        assert_eq!(latest.changed_by, "bob");
    }

    #[test]
    fn test_latest_missing_key() {
        let h = make_history();
        assert!(h.latest("NONEXISTENT").is_none());
    }

    #[test]
    fn test_tracked_keys() {
        let h = make_history();
        let mut keys: Vec<_> = h.tracked_keys().iter().map(|s| s.as_str()).collect();
        keys.sort();
        assert_eq!(keys, vec!["API_KEY", "DATABASE_URL"]);
    }

    #[test]
    fn test_trim() {
        let mut h = make_history();
        h.trim("DATABASE_URL", 1);
        let entries = h.get("DATABASE_URL");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].value, "postgres://new");
    }

    #[test]
    fn test_clear_key() {
        let mut h = make_history();
        h.clear_key("DATABASE_URL");
        assert!(h.get("DATABASE_URL").is_empty());
        assert_eq!(h.get("API_KEY").len(), 1);
    }

    #[test]
    fn test_cli_list_empty() {
        let mut h = EnvHistory::new();
        // Should not panic
        run_history_command(HistoryCommand::List, &mut h);
    }

    #[test]
    fn test_cli_show_with_limit() {
        let mut h = make_history();
        // Should not panic and only show 1 entry
        run_history_command(
            HistoryCommand::Show {
                key: "DATABASE_URL".to_string(),
                limit: Some(1),
            },
            &mut h,
        );
    }

    #[test]
    fn test_cli_clear() {
        let mut h = make_history();
        run_history_command(
            HistoryCommand::Clear {
                key: "API_KEY".to_string(),
            },
            &mut h,
        );
        assert!(h.get("API_KEY").is_empty());
    }
}
