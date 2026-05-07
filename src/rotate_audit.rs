//! Audit log integration for key rotation events.

use crate::audit::{AuditEvent, AuditLog};
use crate::rotate::RotationResult;
use std::time::{SystemTime, UNIX_EPOCH};

/// Severity level for rotation audit events.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RotationStatus {
    Success,
    PartialFailure,
    Failure,
}

impl std::fmt::Display for RotationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "SUCCESS"),
            Self::PartialFailure => write!(f, "PARTIAL_FAILURE"),
            Self::Failure => write!(f, "FAILURE"),
        }
    }
}

/// Determine rotation status from a result.
pub fn rotation_status(result: &RotationResult) -> RotationStatus {
    if result.rotated == 0 && !result.failed.is_empty() {
        RotationStatus::Failure
    } else if !result.failed.is_empty() {
        RotationStatus::PartialFailure
    } else {
        RotationStatus::Success
    }
}

/// Record a key rotation event into the audit log.
pub fn record_rotation(
    log: &mut AuditLog,
    result: &RotationResult,
    initiated_by: &str,
) {
    let status = rotation_status(result);
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let message = format!(
        "Key rotation by '{}': status={}, rotated={}, skipped={}, failed={}",
        initiated_by,
        status,
        result.rotated,
        result.skipped,
        result.failed.len()
    );

    let event = AuditEvent {
        timestamp,
        action: "key_rotation".to_string(),
        actor: initiated_by.to_string(),
        details: message,
    };

    log.record(event);
}
