#[cfg(test)]
mod tests {
    use crate::env_group::{group_by_prefix, filter_by_group, ungrouped_keys, EnvGroup};
    use std::collections::HashMap;

    fn sample_env() -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("DB_HOST".into(), "localhost".into());
        m.insert("DB_PORT".into(), "5432".into());
        m.insert("AWS_KEY".into(), "abc".into());
        m.insert("AWS_SECRET".into(), "xyz".into());
        m.insert("PORT".into(), "8080".into());
        m
    }

    #[test]
    fn test_group_by_prefix_creates_correct_groups() {
        let env = sample_env();
        let groups = group_by_prefix(&env);
        assert!(groups.contains_key("DB"));
        assert!(groups.contains_key("AWS"));
        let db = &groups["DB"];
        assert_eq!(db.keys.len(), 2);
        assert!(db.keys.contains(&"DB_HOST".to_string()));
        assert!(db.keys.contains(&"DB_PORT".to_string()));
    }

    #[test]
    fn test_group_by_prefix_misc_for_no_underscore() {
        let env = sample_env();
        let groups = group_by_prefix(&env);
        // PORT has no underscore, should fall under MISC
        assert!(groups.contains_key("MISC"));
        let misc = &groups["MISC"];
        assert!(misc.keys.contains(&"PORT".to_string()));
    }

    #[test]
    fn test_filter_by_group_returns_correct_subset() {
        let env = sample_env();
        let group = EnvGroup::new("DB", vec!["DB_HOST".into(), "DB_PORT".into()]);
        let filtered = filter_by_group(&env, &group);
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered["DB_HOST"], "localhost");
        assert_eq!(filtered["DB_PORT"], "5432");
    }

    #[test]
    fn test_filter_by_group_excludes_other_keys() {
        let env = sample_env();
        let group = EnvGroup::new("AWS", vec!["AWS_KEY".into()]);
        let filtered = filter_by_group(&env, &group);
        assert!(!filtered.contains_key("DB_HOST"));
        assert!(filtered.contains_key("AWS_KEY"));
    }

    #[test]
    fn test_ungrouped_keys_returns_keys_not_in_any_group() {
        let env = sample_env();
        let groups = vec![
            EnvGroup::new("DB", vec!["DB_HOST".into(), "DB_PORT".into()]),
            EnvGroup::new("AWS", vec!["AWS_KEY".into(), "AWS_SECRET".into()]),
        ];
        let ungrouped = ungrouped_keys(&env, &groups);
        assert_eq!(ungrouped, vec!["PORT"]);
    }

    #[test]
    fn test_ungrouped_keys_empty_when_all_grouped() {
        let env: HashMap<String, String> =
            [("DB_HOST".into(), "localhost".into())].into_iter().collect();
        let groups = vec![EnvGroup::new("DB", vec!["DB_HOST".into()])];
        let ungrouped = ungrouped_keys(&env, &groups);
        assert!(ungrouped.is_empty());
    }
}
