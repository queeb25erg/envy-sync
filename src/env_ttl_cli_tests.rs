#[cfg(test)]
mod tests {
    use crate::env_ttl::TtlStore;
    use crate::env_ttl_cli::{parse_ttl_command, run_ttl_command, TtlCommand};
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    fn now_secs() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    #[test]
    fn test_parse_set() {
        let cmd = parse_ttl_command(&["set", "MY_KEY", "300"]).unwrap();
        assert_eq!(cmd, TtlCommand::Set { key: "MY_KEY".to_string(), ttl_seconds: 300 });
    }

    #[test]
    fn test_parse_get() {
        let cmd = parse_ttl_command(&["get", "MY_KEY"]).unwrap();
        assert_eq!(cmd, TtlCommand::Get { key: "MY_KEY".to_string() });
    }

    #[test]
    fn test_parse_invalid_seconds() {
        assert!(parse_ttl_command(&["set", "KEY", "notanumber"]).is_err());
    }

    #[test]
    fn test_parse_unknown_command() {
        assert!(parse_ttl_command(&["fly", "away"]).is_err());
    }

    #[test]
    fn test_run_set_and_get() {
        let mut store = TtlStore::new();
        let out = run_ttl_command(
            TtlCommand::Set { key: "DB_URL".to_string(), ttl_seconds: 500 },
            &mut store,
        );
        assert!(out.contains("DB_URL"));

        let out = run_ttl_command(TtlCommand::Get { key: "DB_URL".to_string() }, &mut store);
        assert!(out.contains("DB_URL"));
        assert!(out.contains("expires"));
    }

    #[test]
    fn test_run_get_missing_key() {
        let mut store = TtlStore::new();
        let out = run_ttl_command(TtlCommand::Get { key: "MISSING".to_string() }, &mut store);
        assert!(out.contains("No TTL"));
    }

    #[test]
    fn test_run_purge_output() {
        let mut store = TtlStore::new();
        store.entries_mut().insert(
            "OLD".to_string(),
            crate::env_ttl::TtlEntry {
                key: "OLD".to_string(),
                expires_at: now_secs() - 5,
            },
        );
        let out = run_ttl_command(TtlCommand::Purge, &mut store);
        assert!(out.contains("1"));
    }

    #[test]
    fn test_run_list_empty() {
        let mut store = TtlStore::new();
        let out = run_ttl_command(TtlCommand::List, &mut store);
        assert!(out.contains("No TTL"));
    }
}
