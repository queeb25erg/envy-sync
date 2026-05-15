#[cfg(test)]
mod tests {
    use crate::env_pin::PinSet;
    use crate::env_pin_cli::{parse_pin_command, run_pin_command, PinCommand};

    #[test]
    fn test_parse_add() {
        let cmd = parse_pin_command(&["add", "MY_KEY"]).unwrap();
        assert_eq!(cmd, PinCommand::Add("MY_KEY".to_string()));
    }

    #[test]
    fn test_parse_remove() {
        let cmd = parse_pin_command(&["remove", "MY_KEY"]).unwrap();
        assert_eq!(cmd, PinCommand::Remove("MY_KEY".to_string()));
    }

    #[test]
    fn test_parse_list() {
        let cmd = parse_pin_command(&["list"]).unwrap();
        assert_eq!(cmd, PinCommand::List);
    }

    #[test]
    fn test_parse_check() {
        let cmd = parse_pin_command(&["check", "DB_URL"]).unwrap();
        assert_eq!(cmd, PinCommand::Check("DB_URL".to_string()));
    }

    #[test]
    fn test_parse_invalid() {
        assert!(parse_pin_command(&["frobnicate"]).is_err());
    }

    #[test]
    fn test_run_add() {
        let mut ps = PinSet::new();
        let result = run_pin_command(PinCommand::Add("SECRET".to_string()), &mut ps);
        assert!(result.contains("Pinned"));
        assert!(ps.is_pinned("SECRET"));
    }

    #[test]
    fn test_run_remove_existing() {
        let mut ps = PinSet::new();
        ps.pin("SECRET");
        let result = run_pin_command(PinCommand::Remove("SECRET".to_string()), &mut ps);
        assert!(result.contains("Unpinned"));
        assert!(!ps.is_pinned("SECRET"));
    }

    #[test]
    fn test_run_remove_nonexistent() {
        let mut ps = PinSet::new();
        let result = run_pin_command(PinCommand::Remove("GHOST".to_string()), &mut ps);
        assert!(result.contains("not pinned"));
    }

    #[test]
    fn test_run_list_empty() {
        let mut ps = PinSet::new();
        let result = run_pin_command(PinCommand::List, &mut ps);
        assert!(result.contains("No keys"));
    }

    #[test]
    fn test_run_list_with_keys() {
        let mut ps = PinSet::new();
        ps.pin("ALPHA");
        ps.pin("BETA");
        let result = run_pin_command(PinCommand::List, &mut ps);
        assert!(result.contains("ALPHA"));
        assert!(result.contains("BETA"));
    }

    #[test]
    fn test_run_check_pinned() {
        let mut ps = PinSet::new();
        ps.pin("LOCKED");
        let result = run_pin_command(PinCommand::Check("LOCKED".to_string()), &mut ps);
        assert!(result.contains("is pinned"));
    }

    #[test]
    fn test_run_check_not_pinned() {
        let mut ps = PinSet::new();
        let result = run_pin_command(PinCommand::Check("FREE".to_string()), &mut ps);
        assert!(result.contains("not pinned"));
    }
}
