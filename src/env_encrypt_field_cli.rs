//! CLI interface for field-level encryption commands.

use std::collections::HashMap;
use crate::env_encrypt_field::{FieldEncryptConfig, encrypt_sensitive_fields, decrypt_sensitive_fields};

#[derive(Debug)]
pub struct EncryptFieldArgs {
    pub env_file: String,
    pub sensitive_keys: Vec<String>,
    pub password: String,
    pub output_file: Option<String>,
}

#[derive(Debug)]
pub struct DecryptFieldArgs {
    pub env_file: String,
    pub password: String,
    pub output_file: Option<String>,
}

pub fn run_encrypt_fields(args: EncryptFieldArgs) -> Result<String, String> {
    let content = std::fs::read_to_string(&args.env_file)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    let env = parse_env_content(&content);
    let config = FieldEncryptConfig::new(args.sensitive_keys);

    let results = encrypt_sensitive_fields(&env, &config, &args.password)
        .map_err(|e| format!("Encryption error: {:?}", e))?;

    let mut output = String::new();
    for field in &results {
        if field.encrypted {
            output.push_str(&format!("{}=ENC[{}]\n", field.key, field.value));
        } else {
            output.push_str(&format!("{}={}\n", field.key, field.value));
        }
    }

    if let Some(out_path) = args.output_file {
        std::fs::write(&out_path, &output)
            .map_err(|e| format!("Failed to write output: {}", e))?;
        Ok(format!("Encrypted fields written to {}", out_path))
    } else {
        Ok(output)
    }
}

pub fn run_decrypt_fields(args: DecryptFieldArgs) -> Result<HashMap<String, String>, String> {
    use crate::env_encrypt_field::FieldEncryptResult;
    let content = std::fs::read_to_string(&args.env_file)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    let fields: Vec<FieldEncryptResult> = parse_encrypted_env(&content);

    decrypt_sensitive_fields(&fields, &args.password)
        .map_err(|e| format!("Decryption error: {:?}", e))
}

fn parse_env_content(content: &str) -> HashMap<String, String> {
    content.lines()
        .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
        .filter_map(|l| {
            let mut parts = l.splitn(2, '=');
            let key = parts.next()?.trim().to_string();
            let val = parts.next().unwrap_or("").trim().to_string();
            Some((key, val))
        })
        .collect()
}

fn parse_encrypted_env(content: &str) -> Vec<crate::env_encrypt_field::FieldEncryptResult> {
    content.lines()
        .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
        .filter_map(|l| {
            let mut parts = l.splitn(2, '=');
            let key = parts.next()?.trim().to_string();
            let raw = parts.next().unwrap_or("").trim();
            if let Some(inner) = raw.strip_prefix("ENC[").and_then(|s| s.strip_suffix(']')) {
                Some(crate::env_encrypt_field::FieldEncryptResult {
                    key,
                    value: inner.to_string(),
                    encrypted: true,
                })
            } else {
                Some(crate::env_encrypt_field::FieldEncryptResult {
                    key,
                    value: raw.to_string(),
                    encrypted: false,
                })
            }
        })
        .collect()
}
