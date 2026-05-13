//! Compare two .env files or profiles and report differences with severity levels.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum DiffSeverity {
    Added,
    Removed,
    Changed,
    TypeChanged,
}

#[derive(Debug, Clone)]
pub struct EnvCompareDiff {
    pub key: String,
    pub severity: DiffSeverity,
    pub left_value: Option<String>,
    pub right_value: Option<String>,
}

pub fn compare_envs(
    left: &HashMap<String, String>,
    right: &HashMap<String, String>,
) -> Vec<EnvCompareDiff> {
    let mut diffs = Vec::new();

    for (key, left_val) in left {
        match right.get(key) {
            Some(right_val) if right_val != left_val => {
                let severity = if looks_like_different_type(left_val, right_val) {
                    DiffSeverity::TypeChanged
                } else {
                    DiffSeverity::Changed
                };
                diffs.push(EnvCompareDiff {
                    key: key.clone(),
                    severity,
                    left_value: Some(left_val.clone()),
                    right_value: Some(right_val.clone()),
                });
            }
            None => {
                diffs.push(EnvCompareDiff {
                    key: key.clone(),
                    severity: DiffSeverity::Removed,
                    left_value: Some(left_val.clone()),
                    right_value: None,
                });
            }
            _ => {}
        }
    }

    for (key, right_val) in right {
        if !left.contains_key(key) {
            diffs.push(EnvCompareDiff {
                key: key.clone(),
                severity: DiffSeverity::Added,
                left_value: None,
                right_value: Some(right_val.clone()),
            });
        }
    }

    diffs.sort_by(|a, b| a.key.cmp(&b.key));
    diffs
}

fn looks_like_different_type(a: &str, b: &str) -> bool {
    let a_is_bool = matches!(a.to_lowercase().as_str(), "true" | "false");
    let b_is_bool = matches!(b.to_lowercase().as_str(), "true" | "false");
    let a_is_num = a.parse::<f64>().is_ok();
    let b_is_num = b.parse::<f64>().is_ok();
    (a_is_bool != b_is_bool) || (a_is_num != b_is_num)
}

pub fn format_compare_report(diffs: &[EnvCompareDiff], redact: bool) -> String {
    if diffs.is_empty() {
        return "No differences found.".to_string();
    }
    let mut lines = vec![format!("Found {} difference(s):", diffs.len())];
    for d in diffs {
        let left = d.left_value.as_deref().map(|v| if redact { "[redacted]" } else { v }).unwrap_or("(none)");
        let right = d.right_value.as_deref().map(|v| if redact { "[redacted]" } else { v }).unwrap_or("(none)");
        lines.push(format!("  [{:?}] {} : {} -> {}", d.severity, d.key, left, right));
    }
    lines.join("\n")
}
