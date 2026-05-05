#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::sync::{merge_remote_wins, parse_env, serialize_env};

    #[test]
    fn test_parse_env_basic() {
        let content = "FOO=bar\nBAZ=qux\n";
        let map = parse_env(content);
        assert_eq!(map.get("FOO"), Some(&"bar".to_string()));
        assert_eq!(map.get("BAZ"), Some(&"qux".to_string()));
    }

    #[test]
    fn test_parse_env_ignores_comments_and_blanks() {
        let content = "# comment\n\nKEY=value\n";
        let map = parse_env(content);
        assert_eq!(map.len(), 1);
        assert_eq!(map.get("KEY"), Some(&"value".to_string()));
    }

    #[test]
    fn test_parse_env_trims_whitespace() {
        let content = "  KEY  =  value  \n";
        let map = parse_env(content);
        assert_eq!(map.get("KEY"), Some(&"value".to_string()));
    }

    #[test]
    fn test_serialize_env_roundtrip() {
        let mut map = HashMap::new();
        map.insert("A".to_string(), "1".to_string());
        map.insert("B".to_string(), "2".to_string());
        let serialized = serialize_env(&map);
        let reparsed = parse_env(&serialized);
        assert_eq!(reparsed.get("A"), Some(&"1".to_string()));
        assert_eq!(reparsed.get("B"), Some(&"2".to_string()));
    }

    #[test]
    fn test_merge_remote_wins_prefers_remote() {
        let mut local = HashMap::new();
        local.insert("KEY".to_string(), "local_value".to_string());
        local.insert("LOCAL_ONLY".to_string(), "only_local".to_string());

        let mut remote = HashMap::new();
        remote.insert("KEY".to_string(), "remote_value".to_string());
        remote.insert("REMOTE_ONLY".to_string(), "only_remote".to_string());

        let merged = merge_remote_wins(&local, &remote);
        assert_eq!(merged.get("KEY"), Some(&"remote_value".to_string()));
        assert_eq!(merged.get("LOCAL_ONLY"), Some(&"only_local".to_string()));
        assert_eq!(merged.get("REMOTE_ONLY"), Some(&"only_remote".to_string()));
    }

    #[test]
    fn test_merge_remote_wins_empty_remote() {
        let mut local = HashMap::new();
        local.insert("KEY".to_string(), "value".to_string());
        let remote = HashMap::new();
        let merged = merge_remote_wins(&local, &remote);
        assert_eq!(merged, local);
    }
}
