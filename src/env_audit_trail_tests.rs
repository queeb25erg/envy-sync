#[cfg(test)]
mod tests {
    use crate::env_audit_trail::{AuditTrail, MutationKind};

    fn make_trail() -> AuditTrail {
        let mut trail = AuditTrail::new();
        trail.record("DB_URL", MutationKind::Set, "alice", Some("prod".into()), None);
        trail.record("API_KEY", MutationKind::Rotate, "bob", Some("prod".into()), Some("scheduled rotation".into()));
        trail.record("DB_URL", MutationKind::Delete, "alice", Some("staging".into()), None);
        trail.record("SECRET", MutationKind::Import, "ci-bot", None, None);
        trail
    }

    #[test]
    fn test_record_and_len() {
        let trail = make_trail();
        assert_eq!(trail.len(), 4);
        assert!(!trail.is_empty());
    }

    #[test]
    fn test_entries_for_key() {
        let trail = make_trail();
        let db_entries = trail.entries_for_key("DB_URL");
        assert_eq!(db_entries.len(), 2);
        assert!(db_entries.iter().any(|e| e.kind == MutationKind::Set));
        assert!(db_entries.iter().any(|e| e.kind == MutationKind::Delete));
    }

    #[test]
    fn test_entries_by_actor() {
        let trail = make_trail();
        let alice_entries = trail.entries_by_actor("alice");
        assert_eq!(alice_entries.len(), 2);
        let bot_entries = trail.entries_by_actor("ci-bot");
        assert_eq!(bot_entries.len(), 1);
    }

    #[test]
    fn test_mutation_counts() {
        let trail = make_trail();
        let counts = trail.mutation_counts();
        assert_eq!(counts.get("DB_URL"), Some(&2));
        assert_eq!(counts.get("API_KEY"), Some(&1));
        assert_eq!(counts.get("SECRET"), Some(&1));
    }

    #[test]
    fn test_note_and_profile_stored() {
        let trail = make_trail();
        let api_entries = trail.entries_for_key("API_KEY");
        assert_eq!(api_entries.len(), 1);
        assert_eq!(api_entries[0].note.as_deref(), Some("scheduled rotation"));
        assert_eq!(api_entries[0].profile.as_deref(), Some("prod"));
    }

    #[test]
    fn test_clear() {
        let mut trail = make_trail();
        trail.clear();
        assert!(trail.is_empty());
        assert_eq!(trail.len(), 0);
    }

    #[test]
    fn test_all_entries_order() {
        let trail = make_trail();
        let all = trail.all_entries();
        assert_eq!(all[0].key, "DB_URL");
        assert_eq!(all[1].key, "API_KEY");
        assert_eq!(all[3].key, "SECRET");
    }

    #[test]
    fn test_empty_trail() {
        let trail = AuditTrail::new();
        assert!(trail.is_empty());
        assert_eq!(trail.mutation_counts().len(), 0);
    }
}
