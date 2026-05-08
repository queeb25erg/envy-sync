#[cfg(test)]
mod tests {
    use crate::tag::TagStore;
    use crate::tag_cli::{execute, TagCommand};

    fn base_store() -> TagStore {
        let mut store = TagStore::new();
        store.add_tag("prod", "live");
        store.add_tag("prod", "verified");
        store
    }

    #[test]
    fn test_add_new_tag_message() {
        let mut store = base_store();
        let msg = execute(&mut store, TagCommand::Add {
            env: "prod".into(),
            tag: "critical".into(),
        });
        assert!(msg.contains("added"));
        assert!(store.get_tags("prod").contains(&"critical".to_string()));
    }

    #[test]
    fn test_add_duplicate_tag_message() {
        let mut store = base_store();
        let msg = execute(&mut store, TagCommand::Add {
            env: "prod".into(),
            tag: "live".into(),
        });
        assert!(msg.contains("already exists"));
    }

    #[test]
    fn test_remove_existing_tag_message() {
        let mut store = base_store();
        let msg = execute(&mut store, TagCommand::Remove {
            env: "prod".into(),
            tag: "live".into(),
        });
        assert!(msg.contains("removed"));
    }

    #[test]
    fn test_remove_missing_tag_message() {
        let mut store = base_store();
        let msg = execute(&mut store, TagCommand::Remove {
            env: "prod".into(),
            tag: "ghost".into(),
        });
        assert!(msg.contains("not found"));
    }

    #[test]
    fn test_list_tags_message() {
        let mut store = base_store();
        let msg = execute(&mut store, TagCommand::List { env: "prod".into() });
        assert!(msg.contains("live") || msg.contains("verified"));
    }

    #[test]
    fn test_list_tags_empty_env() {
        let mut store = base_store();
        let msg = execute(&mut store, TagCommand::List { env: "unknown".into() });
        assert!(msg.contains("No tags"));
    }

    #[test]
    fn test_find_by_tag_message() {
        let mut store = base_store();
        store.add_tag("staging", "live");
        let msg = execute(&mut store, TagCommand::Find { tag: "live".into() });
        assert!(msg.contains("prod"));
        assert!(msg.contains("staging"));
    }

    #[test]
    fn test_find_by_tag_no_match() {
        let mut store = base_store();
        let msg = execute(&mut store, TagCommand::Find { tag: "absent".into() });
        assert!(msg.contains("No environments"));
    }
}
