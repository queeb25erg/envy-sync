//! CLI handler for the `export` subcommand.

use crate::export::{export_vars, ExportError, ExportFormat};
use std::collections::HashMap;
use std::str::FromStr;

impl FromStr for ExportFormat {
    type Err = ExportError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dotenv" | ".env" => Ok(ExportFormat::DotEnv),
            "json" => Ok(ExportFormat::Json),
            "shell" | "sh" => Ok(ExportFormat::Shell),
            other => Err(ExportError::UnsupportedFormat(other.to_string())),
        }
    }
}

#[derive(Debug)]
pub struct ExportArgs {
    pub format: ExportFormat,
    pub output_path: Option<String>,
}

/// Execute the export command given parsed args and resolved env vars.
pub fn run_export(
    args: &ExportArgs,
    vars: &HashMap<String, String>,
) -> Result<String, ExportError> {
    let content = export_vars(vars, &args.format)?;

    if let Some(path) = &args.output_path {
        std::fs::write(path, &content)
            .map_err(|e| ExportError::Serialization(e.to_string()))?;
        Ok(format!("Exported {} vars to {} ({})", vars.len(), path, args.format))
    } else {
        Ok(content)
    }
}
