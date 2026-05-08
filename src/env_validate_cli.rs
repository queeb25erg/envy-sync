//! CLI interface for the env-validate feature.

use crate::env_validate::{validate_entries, ValidationError};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ValidateArgs {
    pub entries: HashMap<String, String>,
    pub strict: bool,
}

#[derive(Debug, PartialEq)]
pub enum ValidateResult {
    Ok,
    Warnings(Vec<String>),
    Errors(Vec<String>),
}

pub fn run_validate(args: ValidateArgs) -> ValidateResult {
    let errors = validate_entries(&args.entries);

    if errors.is_empty() {
        return ValidateResult::Ok;
    }

    let messages: Vec<String> = errors.iter().map(|e| e.to_string()).collect();

    if args.strict {
        ValidateResult::Errors(messages)
    } else {
        ValidateResult::Warnings(messages)
    }
}

pub fn format_result(result: &ValidateResult) -> String {
    match result {
        ValidateResult::Ok => "✓ All entries are valid.".to_string(),
        ValidateResult::Warnings(msgs) => {
            let lines: Vec<String> = msgs.iter().map(|m| format!("  WARN: {}", m)).collect();
            format!("Validation warnings:\n{}", lines.join("\n"))
        }
        ValidateResult::Errors(msgs) => {
            let lines: Vec<String> = msgs.iter().map(|m| format!("  ERROR: {}", m)).collect();
            format!("Validation failed:\n{}", lines.join("\n"))
        }
    }
}
