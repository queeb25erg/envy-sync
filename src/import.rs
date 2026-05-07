//! Import .env files from various formats into envy-sync storage.

use std::collections::HashMap;
use std::path::Path;
use std::fs;

#[derive(Debug, PartialEq)]
pub enum ImportFormat {
    DotEnv,
    Json,
    Yaml,
}

#[derive(Debug)]
pub struct ImportResult {
    pub entries: HashMap<String, String>,
    pub skipped: Vec<String>,
    pub count: usize,
}

pub fn detect_format(path: &Path) -> ImportFormat {
    match path.extension().and_then(|e| e.to_str()) {
        Some("json") => ImportFormat::Json,
        Some("yaml") | Some("yml") => ImportFormat::Yaml,
        _ => ImportFormat::DotEnv,
    }
}

pub fn import_from_file(path: &Path) -> Result<ImportResult, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    let format = detect_format(path);
    import_from_str(&content, format)
}

pub fn import_from_str(content: &str, format: ImportFormat) -> Result<ImportResult, String> {
    match format {
        ImportFormat::DotEnv => parse_dotenv(content),
        ImportFormat::Json => parse_json(content),
        ImportFormat::Yaml => Err("YAML import not yet supported".to_string()),
    }
}

fn parse_dotenv(content: &str) -> Result<ImportResult, String> {
    let mut entries = HashMap::new();
    let mut skipped = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some(pos) = trimmed.find('=') {
            let key = trimmed[..pos].trim().to_string();
            let raw_val = trimmed[pos + 1..].trim();
            let value = strip_quotes(raw_val).to_string();
            if key.is_empty() {
                skipped.push(format!("line {}: empty key", line_num + 1));
            } else {
                entries.insert(key, value);
            }
        } else {
            skipped.push(format!("line {}: no '=' found", line_num + 1));
        }
    }

    let count = entries.len();
    Ok(ImportResult { entries, skipped, count })
}

fn parse_json(content: &str) -> Result<ImportResult, String> {
    // Minimal JSON object parser for flat string maps
    let trimmed = content.trim();
    if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
        return Err("JSON must be a top-level object".to_string());
    }
    let inner = &trimmed[1..trimmed.len() - 1];
    let mut entries = HashMap::new();
    let mut skipped = Vec::new();

    for pair in inner.split(',') {
        let pair = pair.trim();
        if pair.is_empty() { continue; }
        if let Some(colon) = pair.find(':') {
            let key = pair[..colon].trim().trim_matches('"').to_string();
            let val = pair[colon + 1..].trim().trim_matches('"').to_string();
            if key.is_empty() {
                skipped.push(format!("empty key in pair: {}", pair));
            } else {
                entries.insert(key, val);
            }
        } else {
            skipped.push(format!("invalid pair: {}", pair));
        }
    }

    let count = entries.len();
    Ok(ImportResult { entries, skipped, count })
}

fn strip_quotes(s: &str) -> &str {
    if (s.starts_with('"') && s.ends_with('"')) ||
       (s.starts_with('\'') && s.ends_with('\'')) {
        &s[1..s.len() - 1]
    } else {
        s
    }
}
