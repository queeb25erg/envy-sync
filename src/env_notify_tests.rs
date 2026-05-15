#[cfg(test)]
mod tests {
    use crate::env_notify::{detect_changes, NotifyConfig, NotifyEvent};
    use std::collections::HashMap;

    fn make_map(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    #[test]
    fn test_no_changes() {
        let env = make_map(&[("KEY", "value")]);
        let result = detect_changes(&env, &env, &NotifyConfig::default());
        assert!(result.is_empty());
        assert_eq!(result.summary(), "No changes detected.");
    }

    #[test]
    fn test_added_key() {
        let before = make_map(&[]);
        let after = make_map(&[("NEW_KEY", "hello")]);
        let result = detect_changes(&before, &after, &NotifyConfig::default());
        assert_eq!(result.events.len(), 1);
        assert_eq!(result.events[0], NotifyEvent::Added("NEW_KEY".to_string()));
    }

    #[test]
    fn test_removed_key() {
        let before = make_map(&[("OLD_KEY", "bye")]);
        let after = make_map(&[]);
        let result = detect_changes(&before, &after, &NotifyConfig::default());
        assert_eq!(result.events.len(), 1);
        assert_eq!(result.events[0], NotifyEvent::Removed("OLD_KEY".to_string()));
    }

    #[test]
    fn test_modified_key() {
        let before = make_map(&[("KEY", "old")]);
        let after = make_map(&[("KEY", "new")]);
        let result = detect_changes(&before, &after, &NotifyConfig::default());
        assert_eq!(result.events.len(), 1);
        assert_eq!(result.events[0], NotifyEvent::Modified("KEY".to_string()));
    }

    #[test]
    fn test_watch_keys_filter() {
        let before = make_map(&[("A", "1"), ("B", "2")]);
        let after = make_map(&[("A", "changed"), ("B", "changed")]);
        let config = NotifyConfig {
            enabled: true,
            watch_keys: vec!["A".to_string()],
            ignore_keys: vec![],
        };
        let result = detect_changes(&before, &after, &config);
        assert_eq!(result.events.len(), 1);
        assert_eq!(result.events[0], NotifyEvent::Modified("A".to_string()));
    }

    #[test]
    fn test_ignore_keys_filter() {
        let before = make_map(&[("SECRET", "old"), ("SAFE", "old")]);
        let after = make_map(&[("SECRET", "new"), ("SAFE", "new")]);
        let config = NotifyConfig {
            enabled: true,
            watch_keys: vec![],
            ignore_keys: vec!["SECRET".to_string()],
        };
        let result = detect_changes(&before, &after, &config);
        assert_eq!(result.events.len(), 1);
        assert_eq!(result.events[0], NotifyEvent::Modified("SAFE".to_string()));
    }

    #[test]
    fn test_summary_multiple_events() {
        let before = make_map(&[("OLD", "v"), ("KEEP", "same")]);
        let after = make_map(&[("NEW", "v"), ("KEEP", "same")]);
        let result = detect_changes(&before, &after, &NotifyConfig::default());
        let summary = result.summary();
        assert!(summary.contains("NEW"));
        assert!(summary.contains("OLD"));
    }
}
