use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuditAction {
    Push,
    Pull,
    Encrypt,
    Decrypt,
    ConfigUpdate,
    BackendConnect,
    BackendDisconnect,
    MergeConflict,
    MergeResolved,
}

impl fmt::Display for AuditAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuditAction::Push => write!(f, "PUSH"),
            AuditAction::Pull => write!(f, "PULL"),
            AuditAction::Encrypt => write!(f, "ENCRYPT"),
            AuditAction::Decrypt => write!(f, "DECRYPT"),
            AuditAction::ConfigUpdate => write!(f, "CONFIG_UPDATE"),
            AuditAction::BackendConnect => write!(f, "BACKEND_CONNECT"),
            AuditAction::BackendDisconnect => write!(f, "BACKEND_DISCONNECT"),
            AuditAction::MergeConflict => write!(f, "MERGE_CONFLICT"),
            AuditAction::MergeResolved => write!(f, "MERGE_RESOLVED"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub timestamp: DateTime<Utc>,
    pub action: AuditAction,
    pub details: String,
    pub success: bool,
}

impl AuditEntry {
    pub fn new(action: AuditAction, details: impl Into<String>, success: bool) -> Self {
        Self {
            timestamp: Utc::now(),
            action,
            details: details.into(),
            success,
        }
    }
}

impl fmt::Display for AuditEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} | {} | {}",
            self.timestamp.format("%Y-%m-%dT%H:%M:%SZ"),
            self.action,
            if self.success { "OK" } else { "FAIL" },
            self.details
        )
    }
}

#[derive(Debug, Default)]
pub struct AuditLog {
    entries: Vec<AuditEntry>,
}

impl AuditLog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, action: AuditAction, details: impl Into<String>, success: bool) {
        self.entries.push(AuditEntry::new(action, details, success));
    }

    pub fn entries(&self) -> &[AuditEntry] {
        &self.entries
    }

    pub fn filter_by_action(&self, action: &AuditAction) -> Vec<&AuditEntry> {
        self.entries.iter().filter(|e| &e.action == action).collect()
    }

    pub fn failures(&self) -> Vec<&AuditEntry> {
        self.entries.iter().filter(|e| !e.success).collect()
    }

    pub fn to_log_lines(&self) -> Vec<String> {
        self.entries.iter().map(|e| e.to_string()).collect()
    }
}
