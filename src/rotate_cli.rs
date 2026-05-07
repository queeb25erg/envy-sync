//! CLI helpers for the key rotation command.

use crate::audit::AuditLog;
use crate::rotate::{rotate_keys, verify_rotation};
use crate::rotate_audit::record_rotation;
use crate::storage::Storage;

/// Options for the rotate command.
#[derive(Debug)]
pub struct RotateOptions<'a> {
    pub old_key: &'a [u8],
    pub new_key: &'a [u8],
    pub actor: &'a str,
    pub verify_after: bool,
    pub dry_run: bool,
}

/// Output of the rotate command.
#[derive(Debug)]
pub struct RotateOutput {
    pub rotated: usize,
    pub skipped: usize,
    pub failed: Vec<String>,
    pub verification_failed: Vec<String>,
}

/// Execute key rotation with optional verification and audit logging.
pub fn run_rotate(
    storage: &mut dyn Storage,
    audit: &mut AuditLog,
    opts: &RotateOptions,
) -> Result<RotateOutput, String> {
    if opts.dry_run {
        let keys = storage.list().map_err(|e| e.to_string())?;
        return Ok(RotateOutput {
            rotated: 0,
            skipped: keys.len(),
            failed: vec![],
            verification_failed: vec![],
        });
    }

    let result = rotate_keys(storage, opts.old_key, opts.new_key)
        .map_err(|e| e.to_string())?;

    record_rotation(audit, &result, opts.actor);

    let verification_failed = if opts.verify_after {
        verify_rotation(storage, opts.new_key).map_err(|e| e.to_string())?
    } else {
        vec![]
    };

    Ok(RotateOutput {
        rotated: result.rotated,
        skipped: result.skipped,
        failed: result.failed,
        verification_failed,
    })
}
