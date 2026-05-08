#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::sync::mpsc::channel;
    use std::time::Duration;

    use notify::DebouncedEvent;

    use crate::watch::{EnvWatcher, WatchEvent};

    fn watcher() -> EnvWatcher {
        EnvWatcher::new("/tmp", 100)
    }

    fn send_event(tx: &std::sync::mpsc::Sender<DebouncedEvent>, event: DebouncedEvent) {
        tx.send(event).unwrap();
    }

    #[test]
    fn test_modified_env_file_detected() {
        let (tx, rx) = channel();
        let path = PathBuf::from("/tmp/.env");
        send_event(&tx, DebouncedEvent::Write(path.clone()));
        let events = watcher().collect_events(&rx);
        assert_eq!(events, vec![WatchEvent::Modified(path)]);
    }

    #[test]
    fn test_created_env_file_detected() {
        let (tx, rx) = channel();
        let path = PathBuf::from("/tmp/.env.production");
        send_event(&tx, DebouncedEvent::Create(path.clone()));
        let events = watcher().collect_events(&rx);
        assert_eq!(events, vec![WatchEvent::Created(path)]);
    }

    #[test]
    fn test_removed_env_file_detected() {
        let (tx, rx) = channel();
        let path = PathBuf::from("/tmp/.env.local");
        send_event(&tx, DebouncedEvent::Remove(path.clone()));
        let events = watcher().collect_events(&rx);
        assert_eq!(events, vec![WatchEvent::Removed(path)]);
    }

    #[test]
    fn test_non_env_file_ignored() {
        let (tx, rx) = channel();
        let path = PathBuf::from("/tmp/config.toml");
        send_event(&tx, DebouncedEvent::Write(path));
        let events = watcher().collect_events(&rx);
        assert!(events.is_empty());
    }

    #[test]
    fn test_rename_generates_remove_and_create() {
        let (tx, rx) = channel();
        let src = PathBuf::from("/tmp/.env.old");
        let dst = PathBuf::from("/tmp/.env.new");
        send_event(&tx, DebouncedEvent::Rename(src.clone(), dst.clone()));
        let events = watcher().collect_events(&rx);
        assert_eq!(events.len(), 2);
        assert_eq!(events[0], WatchEvent::Removed(src));
        assert_eq!(events[1], WatchEvent::Created(dst));
    }

    #[test]
    fn test_rename_non_env_to_env_only_creates() {
        let (tx, rx) = channel();
        let src = PathBuf::from("/tmp/config.toml");
        let dst = PathBuf::from("/tmp/.env");
        send_event(&tx, DebouncedEvent::Rename(src, dst.clone()));
        let events = watcher().collect_events(&rx);
        assert_eq!(events, vec![WatchEvent::Created(dst)]);
    }

    #[test]
    fn test_empty_channel_returns_no_events() {
        let (_tx, rx) = channel::<DebouncedEvent>();
        let events = watcher().collect_events(&rx);
        assert!(events.is_empty());
    }
}
