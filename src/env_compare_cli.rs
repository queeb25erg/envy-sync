//! CLI interface for the env-compare feature.

use std::collections::HashMap;
use crate::env_compare::{compare_envs, format_compare_report};

#[derive(Debug)]
pub struct CompareArgs {
    pub left_path: String,
    pub right_path: String,
    pub redact: bool,
    pub only_severity: Option<String>,
}

pub fn parse_env_file(content: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            map.insert(k.trim().to_string(), v.trim().trim_matches('"').to_string());
        }
    }
    map
}

pub fn run_compare(args: &CompareArgs) -> Result<String, String> {
    let left_content = std::fs::read_to_string(&args.left_path)
        .map_err(|e| format!("Failed to read left file '{}': {}", args.left_path, e))?;
    let right_content = std::fs::read_to_string(&args.right_path)
        .map_err(|e| format!("Failed to read right file '{}': {}", args.right_path, e))?;

    let left = parse_env_file(&left_content);
    let right = parse_env_file(&right_content);

    let mut diffs = compare_envs(&left, &right);

    if let Some(ref severity_filter) = args.only_severity {
        let filter = severity_filter.to_lowercase();
        diffs.retain(|d| format!("{:?}", d.severity).to_lowercase() == filter);
    }

    Ok(format_compare_report(&diffs, args.redact))
}
