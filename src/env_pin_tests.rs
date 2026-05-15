#[cfg(test)]
mod tests {
    use crate::env_pin::PinSet;

    #[test]
    fn test_pin_and_check() {
        let mut ps = PinSet::new();
        ps.pin("DATABASE_URL");
        assert!(ps.is_pinned("DATABASE_URL"));
        assert!(!ps.is_pinned("API_KEY"));
    }

    #[test]
    fn test_unpin_existing() {
        let mut ps = PinSet::new();
        ps.pin("SECRET");
        assert!(ps.unpin("SECRET"));
        assert!(!ps.is_pinned("SECRET"));
    }

    #[test]
    fn test_unpin_nonexistent() {
        let mut ps = PinSet::new();
        assert!(!ps.unpin("GHOST"));
    }

    #[test]
    fn test_list_sorted() {
        let mut ps = PinSet::new();
        ps.pin("ZEBRA");
        ps.pin("ALPHA");
        ps.pin("MIDDLE");
        let list = ps.list();
        assert_eq!(list, vec!["ALPHA", "MIDDLE", "ZEBRA"]);
    }

    #[test]
    fn test_filter_protected_removes_pinned() {
        let mut ps = PinSet::new();
        ps.pin("DB_PASS");
        let incoming = vec![
            ("DB_PASS".to_string(), "new_secret".to_string()),
            ("APP_ENV".to_string(), "production".to_string()),
        ];
        let allowed = ps.filter_protected(&incoming);
        assert_eq!(allowed.len(), 1);
        assert_eq!(allowed[0].0, "APP_ENV");
    }

    #[test]
    fn test_from_lines_skips_comments_and_blanks() {
        let lines = vec!["# comment", "", "DATABASE_URL", "API_KEY"];
        let ps = PinSet::from_lines(&lines);
        assert!(ps.is_pinned("DATABASE_URL"));
        assert!(ps.is_pinned("API_KEY"));
        assert_eq!(ps.keys.len(), 2);
    }

    #[test]
    fn test_to_lines_sorted() {
        let mut ps = PinSet::new();
        ps.pin("Z_KEY");
        ps.pin("A_KEY");
        let output = ps.to_lines();
        assert_eq!(output, "A_KEY\nZ_KEY");
    }

    #[test]
    fn test_empty_pin_set() {
        let ps = PinSet::new();
        assert!(ps.list().is_empty());
        assert_eq!(ps.to_lines(), "");
    }
}
