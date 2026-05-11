//! Scans .env files for accidentally committed secrets or sensitive patterns.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum SecretSeverity {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone)]
pub struct SecretFinding {
    pub key: String,
    pub pattern: String,
    pub severity: SecretSeverity,
    pub line: usize,
}

#[derive(Debug, Default)]
pub struct ScanResult {
    pub findings: Vec<SecretFinding>,
    pub scanned_keys: usize,
}

impl ScanResult {
    pub fn has_high_severity(&self) -> bool {
        self.findings.iter().any(|f| f.severity == SecretSeverity::High)
    }

    pub fn summary(&self) -> String {
        format!(
            "Scanned {} keys, found {} potential secrets ({} high severity)",
            self.scanned_keys,
            self.findings.len(),
            self.findings.iter().filter(|f| f.severity == SecretSeverity::High).count()
        )
    }
}

pub fn default_patterns() -> Vec<(&'static str, &'static str, SecretSeverity)> {
    vec![
        ("private_key", r"PRIVATE_KEY", SecretSeverity::High),
        ("aws_secret", r"AWS_SECRET", SecretSeverity::High),
        ("password", r"PASSWORD", SecretSeverity::Medium),
        ("token", r"TOKEN", SecretSeverity::Medium),
        ("api_key", r"API_KEY", SecretSeverity::Medium),
        ("secret", r"SECRET", SecretSeverity::Low),
        ("credentials", r"CREDENTIALS", SecretSeverity::Low),
    ]
}

pub fn scan_env(entries: &HashMap<String, String>) -> ScanResult {
    let patterns = default_patterns();
    let mut result = ScanResult {
        scanned_keys: entries.len(),
        ..Default::default()
    };

    for (line, (key, _value)) in entries.iter().enumerate() {
        let key_upper = key.to_uppercase();
        for (pattern_name, pattern, severity) in &patterns {
            if key_upper.contains(pattern) {
                result.findings.push(SecretFinding {
                    key: key.clone(),
                    pattern: pattern_name.to_string(),
                    severity: severity.clone(),
                    line: line + 1,
                });
                break;
            }
        }
    }

    result
}

pub fn scan_with_allowlist(entries: &HashMap<String, String>, allowlist: &[String]) -> ScanResult {
    let mut result = scan_env(entries);
    result.findings.retain(|f| !allowlist.contains(&f.key));
    result
}
