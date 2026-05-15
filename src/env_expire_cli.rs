//! env_expire_cli.rs — CLI interface for managing env variable expiry.

use crate::env_expire::{ExpiryStore, ExpiryStatus};
use chrono::{DateTime, Duration, Utc};

pub fn parse_expiry_date(s: &str) -> Result<DateTime<Utc>, String> {
    DateTime::parse_from_rfc3339(s)
        .map(|d| d.with_timezone(&Utc))
        .map_err(|e| format!("Invalid date '{}': {}", s, e))
}

pub fn cmd_set_expiry(
    store: &mut ExpiryStore,
    key: &str,
    expires_at_str: &str,
    warn_days: u32,
) -> Result<String, String> {
    let expires_at = parse_expiry_date(expires_at_str)?;
    store.set(key, expires_at, warn_days);
    Ok(format!(
        "Set expiry for '{}' to {} (warn {} days before)",
        key, expires_at_str, warn_days
    ))
}

pub fn cmd_remove_expiry(store: &mut ExpiryStore, key: &str) -> String {
    if store.remove(key) {
        format!("Removed expiry for '{}'.", key)
    } else {
        format!("No expiry entry found for '{}'.", key)
    }
}

pub fn cmd_check_expiry(store: &ExpiryStore, key: &str) -> String {
    let now = Utc::now();
    match store.status(key, now) {
        None => format!("No expiry set for '{}'.", key),
        Some(ExpiryStatus::Valid) => format!("'{}' is valid.", key),
        Some(ExpiryStatus::WarningSoon { days_left }) => {
            format!("WARNING: '{}' expires in {} day(s).", key, days_left)
        }
        Some(ExpiryStatus::Expired { days_ago }) => {
            format!("EXPIRED: '{}' expired {} day(s) ago.", key, days_ago)
        }
    }
}

pub fn cmd_list_expired(store: &ExpiryStore) -> String {
    let now = Utc::now();
    let expired = store.expired_keys(now);
    if expired.is_empty() {
        "No expired keys.".to_string()
    } else {
        format!("Expired keys: {}", expired.join(", "))
    }
}

pub fn cmd_list_warnings(store: &ExpiryStore) -> String {
    let now = Utc::now();
    let mut warnings = store.warning_keys(now);
    warnings.sort_by_key(|(_, d)| *d);
    if warnings.is_empty() {
        "No keys expiring soon.".to_string()
    } else {
        let lines: Vec<String> = warnings
            .iter()
            .map(|(k, d)| format!("  {} — {} day(s) left", k, d))
            .collect();
        format!("Keys expiring soon:\n{}", lines.join("\n"))
    }
}

#[allow(dead_code)]
fn days_from_now(days: i64) -> DateTime<Utc> {
    Utc::now() + Duration::days(days)
}
