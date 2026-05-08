#[cfg(test)]
mod tests {
    use crate::tag::TagStore;

    fn make_store() -> TagStore {
        let mut store = TagStore::new();
        store.add_tag("production", "stable");
        store.add_tag("production", "reviewed");
        store.add_tag("staging", "wip");
        store.add_tag("staging", "stable");
        store
    }

    #[test]
    fn test_add_tag_returns_true_on_new() {
        let mut store = TagStore::new();
        assert!(store.add_tag("dev", "local"));
    }

    #[test]
    fn test_add_tag_returns_false_on_duplicate() {
        let mut store = make_store();
        assert!(!store.add_tag("production", "stable"));
    }

    #[test]
    fn test_remove_tag_existing() {
        let mut store = make_store();
        assert!(store.remove_tag("staging", "wip"));
        assert!(!store.get_tags("staging").contains(&"wip".to_string()));
    }

    #[test]
    fn test_remove_tag_nonexistent() {
        let mut store = make_store();
        assert!(!store.remove_tag("production", "ghost"));
    }

    #[test]
    fn test_remove_last_tag_clears_env_entry() {
        let mut store = TagStore::new();
        store.add_tag("solo", "only");
        store.remove_tag("solo", "only");
        assert!(!store.tags.contains_key("solo"));
    }

    #[test]
    fn test_get_tags_sorted() {
        let store = make_store();
        let tags = store.get_tags("production");
        assert_eq!(tags, vec!["reviewed", "stable"]);
    }

    #[test]
    fn test_get_tags_empty_env() {
        let store = make_store();
        assert!(store.get_tags("nonexistent").is_empty());
    }

    #[test]
    fn test_find_by_tag() {
        let store = make_store();
        let envs = store.find_by_tag("stable");
        assert_eq!(envs, vec!["production", "staging"]);
    }

    #[test]
    fn test_find_by_tag_no_match() {
        let store = make_store();
        assert!(store.find_by_tag("ghost").is_empty());
    }

    #[test]
    fn test_clear_env() {
        let mut store = make_store();
        store.clear_env("production");
        assert!(store.get_tags("production").is_empty());
        assert!(!store.tags.contains_key("production"));
    }

    #[test]
    fn test_serialization_roundtrip() {
        let store = make_store();
        let bytes = store.to_bytes().expect("serialize");
        let restored = TagStore::from_bytes(&bytes).expect("deserialize");
        assert_eq!(store, restored);
    }
}
