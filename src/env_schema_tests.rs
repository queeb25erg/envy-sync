#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::env_schema::{EnvSchema, SchemaField, SchemaFieldType, SchemaViolation};
    use crate::env_schema_cli::{parse_schema_file, run_schema_check, SchemaCheckArgs};

    fn make_field(field_type: SchemaFieldType, required: bool) -> SchemaField {
        SchemaField { field_type, required, description: None }
    }

    #[test]
    fn test_valid_env_passes() {
        let mut schema = EnvSchema::new();
        schema.add_field("PORT", make_field(SchemaFieldType::Integer, true));
        schema.add_field("DEBUG", make_field(SchemaFieldType::Boolean, false));
        let mut env = HashMap::new();
        env.insert("PORT".to_string(), "8080".to_string());
        env.insert("DEBUG".to_string(), "true".to_string());
        let violations = schema.validate(&env);
        let errors: Vec<_> = violations.iter().filter(|v| !matches!(v, SchemaViolation::UnknownKey(_))).collect();
        assert!(errors.is_empty());
    }

    #[test]
    fn test_missing_required_field() {
        let mut schema = EnvSchema::new();
        schema.add_field("API_KEY", make_field(SchemaFieldType::String, true));
        let env: HashMap<String, String> = HashMap::new();
        let violations = schema.validate(&env);
        assert!(violations.contains(&SchemaViolation::MissingRequired("API_KEY".to_string())));
    }

    #[test]
    fn test_type_mismatch_integer() {
        let mut schema = EnvSchema::new();
        schema.add_field("PORT", make_field(SchemaFieldType::Integer, true));
        let mut env = HashMap::new();
        env.insert("PORT".to_string(), "not_a_number".to_string());
        let violations = schema.validate(&env);
        assert!(violations.iter().any(|v| matches!(v, SchemaViolation::TypeMismatch { key, .. } if key == "PORT")));
    }

    #[test]
    fn test_unknown_key_reported() {
        let schema = EnvSchema::new();
        let mut env = HashMap::new();
        env.insert("EXTRA_KEY".to_string(), "value".to_string());
        let violations = schema.validate(&env);
        assert!(violations.contains(&SchemaViolation::UnknownKey("EXTRA_KEY".to_string())));
    }

    #[test]
    fn test_parse_schema_file() {
        let schema_content = "PORT:integer:required\nDEBUG:boolean\nAPI_URL:url:required\n";
        let schema = parse_schema_file(schema_content);
        assert!(schema.fields.contains_key("PORT"));
        assert_eq!(schema.fields["PORT"].required, true);
        assert_eq!(schema.fields["DEBUG"].required, false);
    }

    #[test]
    fn test_run_schema_check_passes() {
        let schema_content = "PORT:integer:required\n";
        let env_content = "PORT=3000\n";
        let args = SchemaCheckArgs { schema_path: "schema.txt".into(), env_path: ".env".into(), strict: false };
        assert!(run_schema_check(&args, schema_content, env_content).is_ok());
    }

    #[test]
    fn test_run_schema_check_fails_on_missing() {
        let schema_content = "SECRET:string:required\n";
        let env_content = "PORT=3000\n";
        let args = SchemaCheckArgs { schema_path: "schema.txt".into(), env_path: ".env".into(), strict: false };
        assert!(run_schema_check(&args, schema_content, env_content).is_err());
    }
}
