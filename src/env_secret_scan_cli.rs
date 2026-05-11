//! CLI interface for the secret scanner.

use std::collections::HashMap;
use crate::env_secret_scan::{scan_with_allowlist, ScanResult, SecretSeverity};

#[derive(Debug)]
pub struct ScanCliOptions {
    pub fail_on_high: bool,
    pub allowlist: Vec<String>,
    pub verbose: bool,
}

impl Default for ScanCliOptions {
    fn default() -> Self {
        Self {
            fail_on_high: true,
            allowlist: vec![],
            verbose: false,
        }
    }
}

pub fn run_scan(
    entries: &HashMap<String, String>,
    opts: &ScanCliOptions,
) -> Result<ScanResult, String> {
    let result = scan_with_allowlist(entries, &opts.allowlist);

    if opts.verbose {
        for finding in &result.findings {
            let sev = match finding.severity {
                SecretSeverity::High => "[HIGH]",
                SecretSeverity::Medium => "[MEDIUM]",
                SecretSeverity::Low => "[LOW]",
            };
            println!("  {} key='{}' matched pattern='{}'", sev, finding.key, finding.pattern);
        }
    }

    println!("{}", result.summary());

    if opts.fail_on_high && result.has_high_severity() {
        return Err(format!(
            "Scan failed: {} high-severity secret(s) found",
            result.findings.iter().filter(|f| f.severity == SecretSeverity::High).count()
        ));
    }

    Ok(result)
}
