//! Environment variable template rendering with placeholder substitution.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct EnvTemplate {
    pub raw: String,
}

#[derive(Debug, PartialEq)]
pub enum TemplateError {
    MissingVariable(String),
    InvalidSyntax(String),
}

impl std::fmt::Display for TemplateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateError::MissingVariable(k) => write!(f, "Missing variable: {}", k),
            TemplateError::InvalidSyntax(s) => write!(f, "Invalid template syntax: {}", s),
        }
    }
}

impl EnvTemplate {
    pub fn new(raw: impl Into<String>) -> Self {
        Self { raw: raw.into() }
    }

    /// Render the template by substituting `{{VAR}}` placeholders.
    pub fn render(&self, vars: &HashMap<String, String>) -> Result<String, TemplateError> {
        let mut result = self.raw.clone();
        let mut search_from = 0;

        while let Some(start) = result[search_from..].find("{{{") {
            return Err(TemplateError::InvalidSyntax(
                "Triple braces are not supported".into(),
            ));
        }

        search_from = 0;
        while let Some(start) = result[search_from..].find("{{") {
            let abs_start = search_from + start;
            let after_open = abs_start + 2;
            if let Some(end_rel) = result[after_open..].find("}}") {
                let abs_end = after_open + end_rel;
                let key = result[after_open..abs_end].trim().to_string();
                if key.is_empty() {
                    return Err(TemplateError::InvalidSyntax("Empty placeholder".into()));
                }
                let value = vars
                    .get(&key)
                    .ok_or_else(|| TemplateError::MissingVariable(key.clone()))?;
                let placeholder = format!("{{{{{}}}}}", result[after_open..abs_end].to_string());
                result = result.replacen(&placeholder, value, 1);
                search_from = abs_start + value.len();
            } else {
                return Err(TemplateError::InvalidSyntax(
                    "Unclosed placeholder '{{'".into(),
                ));
            }
        }
        Ok(result)
    }

    /// Extract all placeholder variable names from the template.
    pub fn variables(&self) -> Vec<String> {
        let mut vars = Vec::new();
        let mut rest = self.raw.as_str();
        while let Some(start) = rest.find("{{") {
            rest = &rest[start + 2..];
            if let Some(end) = rest.find("}}") {
                let key = rest[..end].trim().to_string();
                if !key.is_empty() {
                    vars.push(key);
                }
                rest = &rest[end + 2..];
            } else {
                break;
            }
        }
        vars
    }
}
