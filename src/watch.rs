//! Watch for local .env file changes and auto-sync to remote backend.

use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};

use crate::config::Config;
use crate::sync::SyncEngine;

/// Represents a detected file change event.
#[derive(Debug, Clone, PartialEq)]
pub enum WatchEvent {
    Modified(PathBuf),
    Created(PathBuf),
    Removed(PathBuf),
}

/// Watches a directory for .env file changes and triggers sync.
pub struct EnvWatcher {
    path: PathBuf,
    debounce_ms: u64,
}

impl EnvWatcher {
    pub fn new(path: impl AsRef<Path>, debounce_ms: u64) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            debounce_ms,
        }
    }

    /// Collect raw notify events and map to WatchEvents for .env files only.
    pub fn collect_events(&self, rx: &Receiver<DebouncedEvent>) -> Vec<WatchEvent> {
        let mut events = Vec::new();
        while let Ok(event) = rx.try_recv() {
            match event {
                DebouncedEvent::Write(p) | DebouncedEvent::Chmod(p) => {
                    if is_env_file(&p) {
                        events.push(WatchEvent::Modified(p));
                    }
                }
                DebouncedEvent::Create(p) => {
                    if is_env_file(&p) {
                        events.push(WatchEvent::Created(p));
                    }
                }
                DebouncedEvent::Remove(p) => {
                    if is_env_file(&p) {
                        events.push(WatchEvent::Removed(p));
                    }
                }
                DebouncedEvent::Rename(src, dst) => {
                    if is_env_file(&src) {
                        events.push(WatchEvent::Removed(src));
                    }
                    if is_env_file(&dst) {
                        events.push(WatchEvent::Created(dst));
                    }
                }
                _ => {}
            }
        }
        events
    }

    /// Start watching and sync on every detected change.
    pub fn start(&self, config: &Config, engine: &SyncEngine) -> notify::Result<()> {
        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_millis(self.debounce_ms))?;
        watcher.watch(&self.path, RecursiveMode::NonRecursive)?;

        println!("[watch] Watching {:?} for .env changes...", self.path);

        loop {
            let events = self.collect_events(&rx);
            for event in events {
                match &event {
                    WatchEvent::Modified(p) | WatchEvent::Created(p) => {
                        println!("[watch] Change detected: {:?}", p);
                        if let Err(e) = engine.push(config) {
                            eprintln!("[watch] Sync push failed: {}", e);
                        } else {
                            println!("[watch] Sync push succeeded.");
                        }
                    }
                    WatchEvent::Removed(p) => {
                        println!("[watch] File removed: {:?} — skipping auto-sync.", p);
                    }
                }
            }
            std::thread::sleep(Duration::from_millis(200));
        }
    }
}

fn is_env_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|n| n.starts_with(".env"))
        .unwrap_or(false)
}
