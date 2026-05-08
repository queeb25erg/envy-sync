#[cfg(test)]
mod tests {
    use crate::profile::{Profile, ProfileStore};
    use crate::profile_cli::{handle_profile_command, ProfileCommand};

    fn store_with_profiles() -> ProfileStore {
        let mut store = ProfileStore::new();
        let mut dev = Profile::new("dev");
        dev.set_var("API_URL", "http://localhost:3000");
        store.add(dev);
        store
    }

    #[test]
    fn test_create_profile() {
        let mut store = ProfileStore::new();
        let result = handle_profile_command(
            ProfileCommand::Create { name: "prod".into(), description: Some("Production".into()) },
            &mut store,
        );
        assert!(result.is_ok());
        assert!(store.contains("prod"));
    }

    #[test]
    fn test_create_duplicate_profile_fails() {
        let mut store = store_with_profiles();
        let result = handle_profile_command(
            ProfileCommand::Create { name: "dev".into(), description: None },
            &mut store,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_profile() {
        let mut store = store_with_profiles();
        let result = handle_profile_command(ProfileCommand::Delete { name: "dev".into() }, &mut store);
        assert!(result.is_ok());
        assert!(!store.contains("dev"));
    }

    #[test]
    fn test_delete_nonexistent_profile_fails() {
        let mut store = ProfileStore::new();
        let result = handle_profile_command(ProfileCommand::Delete { name: "ghost".into() }, &mut store);
        assert!(result.is_err());
    }

    #[test]
    fn test_list_profiles() {
        let mut store = store_with_profiles();
        let result = handle_profile_command(ProfileCommand::List, &mut store).unwrap();
        assert!(result.contains("dev"));
    }

    #[test]
    fn test_set_and_show_var() {
        let mut store = store_with_profiles();
        handle_profile_command(
            ProfileCommand::SetVar { profile: "dev".into(), key: "SECRET".into(), value: "abc123".into() },
            &mut store,
        ).unwrap();
        let result = handle_profile_command(ProfileCommand::Show { name: "dev".into() }, &mut store).unwrap();
        assert!(result.contains("SECRET"));
        assert!(result.contains("abc123"));
    }

    #[test]
    fn test_remove_var() {
        let mut store = store_with_profiles();
        let result = handle_profile_command(
            ProfileCommand::RemoveVar { profile: "dev".into(), key: "API_URL".into() },
            &mut store,
        );
        assert!(result.is_ok());
        assert!(store.get("dev").unwrap().get_var("API_URL").is_none());
    }
}
