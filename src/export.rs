//! Export module: serialize and write env vars to various output formats.

use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ExportFormat {
    DotEnv,
    Json,
    Shell,
}

impl fmt::Display for ExportFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExportFormat::DotEnv => write!(f, "dotenv"),
            ExportFormat::Json => write!(f, "json"),
            ExportFormat::Shell => write!(f, "shell"),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ExportError {
    #[error("serialization failed: {0}")]
    Serialization(String),
    #[error("unsupported format: {0}")]
    UnsupportedFormat(String),
}

/// Render env vars into the requested format string.
pub fn export_vars(
    vars: &HashMap<String, String>,
    format: &ExportFormat,
) -> Result<String, ExportError> {
    match format {
        ExportFormat::DotEnv => Ok(render_dotenv(vars)),
        ExportFormat::Json => render_json(vars),
        ExportFormat::Shell => Ok(render_shell(vars)),
    }
}

fn render_dotenv(vars: &HashMap<String, String>) -> String {
    let mut lines: Vec<String> = vars
        .iter()
        .map(|(k, v)| format!("{}={}", k, quote_value(v)))
        .collect();
    lines.sort();
    lines.join("\n")
}

fn render_json(vars: &HashMap<String, String>) -> Result<String, ExportError> {
    serde_json::to_string_pretty(vars)
        .map_err(|e| ExportError::Serialization(e.to_string()))
}

fn render_shell(vars: &HashMap<String, String>) -> String {
    let mut lines: Vec<String> = vars
        .iter()
        .map(|(k, v)| format!("export {}={}", k, quote_value(v)))
        .collect();
    lines.sort();
    lines.join("\n")
}

fn quote_value(v: &str) -> String {
    if v.contains(' ') || v.contains('"') || v.contains('\n') {
        format!("'{}'", v.replace('\'', "'\\''" ))
    } else {
        v.to_string()
    }
}
