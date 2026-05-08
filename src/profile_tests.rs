#[cfg(test)]
mod tests {
    use crate::profile::{Profile, ProfileStore};

    fn make_profile(name: &str) -> Profile {
        let mut p = Profile::new(name);
        p.set_var("KEY_A", "val_a");
        p.set_var("KEY_B", "val_b");
        p
    }

    #[test]
    fn test_profile_new() {
        let p = Profile::new("dev");
        assert_eq!(p.name, "dev");
        assert!(p.description.is_none());
        assert_eq!(p.var_count(), 0);
    }

    #[test]
    fn test_profile_set_get_remove() {
        let mut p = Profile::new("test");
        p.set_var("FOO", "bar");
        assert_eq!(p.get_var("FOO"), Some(&"bar".to_string()));
        assert_eq!(p.var_count(), 1);
        p.remove_var("FOO");
        assert!(p.get_var("FOO").is_none());
    }

    #[test]
    fn test_profile_merge_does_not_overwrite() {
        let mut base = make_profile("base");
        let mut other = Profile::new("other");
        other.set_var("KEY_A", "OVERRIDDEN");
        other.set_var("KEY_C", "new_val");
        base.merge_from(&other);
        assert_eq!(base.get_var("KEY_A").unwrap(), "val_a");
        assert_eq!(base.get_var("KEY_C").unwrap(), "new_val");
    }

    #[test]
    fn test_profile_with_description() {
        let p = Profile::new("staging").with_description("Staging environment");
        assert_eq!(p.description.as_deref(), Some("Staging environment"));
    }

    #[test]
    fn test_store_add_get_remove() {
        let mut store = ProfileStore::new();
        store.add(make_profile("dev"));
        assert!(store.contains("dev"));
        assert_eq!(store.get("dev").unwrap().name, "dev");
        store.remove("dev");
        assert!(!store.contains("dev"));
    }

    #[test]
    fn test_store_list_names_sorted() {
        let mut store = ProfileStore::new();
        store.add(Profile::new("prod"));
        store.add(Profile::new("dev"));
        store.add(Profile::new("staging"));
        let names: Vec<&&String> = store.list_names().iter().collect();
        assert_eq!(names[0].as_str(), "dev");
        assert_eq!(names[1].as_str(), "prod");
        assert_eq!(names[2].as_str(), "staging");
    }

    #[test]
    fn test_store_get_mut() {
        let mut store = ProfileStore::new();
        store.add(Profile::new("dev"));
        let p = store.get_mut("dev").unwrap();
        p.set_var("NEW_KEY", "new_value");
        assert_eq!(store.get("dev").unwrap().get_var("NEW_KEY").unwrap(), "new_value");
    }
}
