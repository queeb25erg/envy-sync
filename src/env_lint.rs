//! Lint rules for .env files: detect common issues like duplicate keys,
//! missing values, suspicious patterns, and naming convention violations.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum LintSeverity {
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub struct LintIssue {
    pub line: usize,
    pub key: String,
    pub message: String,
    pub severity: LintSeverity,
}

pub fn lint_env(content: &str) -> Vec<LintIssue> {
    let mut issues = Vec::new();
    let mut seen_keys: HashMap<String, usize> = HashMap::new();

    for (idx, raw_line) in content.lines().enumerate() {
        let line_no = idx + 1;
        let line = raw_line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let Some(eq_pos) = line.find('=') else {
            issues.push(LintIssue {
                line: line_no,
                key: line.to_string(),
                message: "Line is not a valid KEY=VALUE assignment".to_string(),
                severity: LintSeverity::Error,
            });
            continue;
        };

        let key = line[..eq_pos].trim().to_string();
        let value = line[eq_pos + 1..].trim();

        // Duplicate key check
        if let Some(prev_line) = seen_keys.get(&key) {
            issues.push(LintIssue {
                line: line_no,
                key: key.clone(),
                message: format!("Duplicate key (first defined on line {})", prev_line),
                severity: LintSeverity::Error,
            });
        } else {
            seen_keys.insert(key.clone(), line_no);
        }

        // Naming convention: keys should be UPPER_SNAKE_CASE
        if key.chars().any(|c| c.is_lowercase()) {
            issues.push(LintIssue {
                line: line_no,
                key: key.clone(),
                message: "Key should be UPPER_SNAKE_CASE".to_string(),
                severity: LintSeverity::Warning,
            });
        }

        // Empty value warning
        if value.is_empty() {
            issues.push(LintIssue {
                line: line_no,
                key: key.clone(),
                message: "Key has an empty value".to_string(),
                severity: LintSeverity::Warning,
            });
        }

        // Unquoted whitespace in value
        if value.contains(' ') && !value.starts_with('"') && !value.starts_with('\'') {
            issues.push(LintIssue {
                line: line_no,
                key: key.clone(),
                message: "Value contains spaces but is not quoted".to_string(),
                severity: LintSeverity::Warning,
            });
        }
    }

    issues
}
