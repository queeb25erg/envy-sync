use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum SchemaFieldType {
    String,
    Integer,
    Boolean,
    Url,
}

#[derive(Debug, Clone)]
pub struct SchemaField {
    pub field_type: SchemaFieldType,
    pub required: bool,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct EnvSchema {
    pub fields: HashMap<String, SchemaField>,
}

#[derive(Debug, PartialEq)]
pub enum SchemaViolation {
    MissingRequired(String),
    TypeMismatch { key: String, expected: SchemaFieldType, value: String },
    UnknownKey(String),
}

impl EnvSchema {
    pub fn new() -> Self {
        EnvSchema {
            fields: HashMap::new(),
        }
    }

    pub fn add_field(&mut self, key: &str, field: SchemaField) {
        self.fields.insert(key.to_string(), field);
    }

    pub fn validate(&self, env: &HashMap<String, String>) -> Vec<SchemaViolation> {
        let mut violations = Vec::new();

        for (key, field) in &self.fields {
            match env.get(key) {
                None => {
                    if field.required {
                        violations.push(SchemaViolation::MissingRequired(key.clone()));
                    }
                }
                Some(value) => {
                    if !is_valid_type(value, &field.field_type) {
                        violations.push(SchemaViolation::TypeMismatch {
                            key: key.clone(),
                            expected: field.field_type.clone(),
                            value: value.clone(),
                        });
                    }
                }
            }
        }

        for key in env.keys() {
            if !self.fields.contains_key(key) {
                violations.push(SchemaViolation::UnknownKey(key.clone()));
            }
        }

        violations
    }
}

fn is_valid_type(value: &str, field_type: &SchemaFieldType) -> bool {
    match field_type {
        SchemaFieldType::String => true,
        SchemaFieldType::Integer => value.parse::<i64>().is_ok(),
        SchemaFieldType::Boolean => matches!(value.to_lowercase().as_str(), "true" | "false" | "1" | "0"),
        SchemaFieldType::Url => value.starts_with("http://") || value.starts_with("https://"),
    }
}
