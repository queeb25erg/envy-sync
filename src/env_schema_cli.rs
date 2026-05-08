use std::collections::HashMap;
use crate::env_schema::{EnvSchema, SchemaField, SchemaFieldType, SchemaViolation};

pub struct SchemaCheckArgs {
    pub schema_path: String,
    pub env_path: String,
    pub strict: bool,
}

pub fn parse_schema_file(content: &str) -> EnvSchema {
    let mut schema = EnvSchema::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.splitn(3, ':').collect();
        if parts.len() < 2 {
            continue;
        }
        let key = parts[0].trim();
        let type_str = parts[1].trim();
        let required = parts.get(2).map(|s| s.trim() == "required").unwrap_or(false);
        let field_type = match type_str {
            "integer" => SchemaFieldType::Integer,
            "boolean" => SchemaFieldType::Boolean,
            "url" => SchemaFieldType::Url,
            _ => SchemaFieldType::String,
        };
        schema.add_field(key, SchemaField { field_type, required, description: None });
    }
    schema
}

pub fn run_schema_check(args: &SchemaCheckArgs, schema_content: &str, env_content: &str) -> Result<(), String> {
    let schema = parse_schema_file(schema_content);
    let mut env_map: HashMap<String, String> = HashMap::new();
    for line in env_content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            env_map.insert(k.trim().to_string(), v.trim().to_string());
        }
    }

    let violations = schema.validate(&env_map);
    let errors: Vec<&SchemaViolation> = violations.iter().filter(|v| {
        !matches!(v, SchemaViolation::UnknownKey(_))
    }).collect();
    let warnings: Vec<&SchemaViolation> = violations.iter().filter(|v| {
        matches!(v, SchemaViolation::UnknownKey(_))
    }).collect();

    for w in &warnings {
        if let SchemaViolation::UnknownKey(k) = w {
            eprintln!("[warn] Unknown key in env: {}", k);
        }
    }
    for e in &errors {
        match e {
            SchemaViolation::MissingRequired(k) => eprintln!("[error] Missing required key: {}", k),
            SchemaViolation::TypeMismatch { key, value, .. } => eprintln!("[error] Type mismatch for '{}': got '{}'", key, value),
            _ => {}
        }
    }

    if !errors.is_empty() || (args.strict && !warnings.is_empty()) {
        Err(format!("Schema check failed for '{}'", args.env_path))
    } else {
        println!("Schema check passed for '{}'", args.env_path);
        Ok(())
    }
}
