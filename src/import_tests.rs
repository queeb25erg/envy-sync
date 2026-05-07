#[cfg(test)]
mod tests {
    use crate::import::*;
    use std::collections::HashMap;

    #[test]
    fn test_parse_dotenv_basic() {
        let content = "KEY1=value1\nKEY2=value2\n";
        let result = import_from_str(content, ImportFormat::DotEnv).unwrap();
        assert_eq!(result.count, 2);
        assert_eq!(result.entries.get("KEY1"), Some(&"value1".to_string()));
        assert_eq!(result.entries.get("KEY2"), Some(&"value2".to_string()));
        assert!(result.skipped.is_empty());
    }

    #[test]
    fn test_parse_dotenv_strips_quotes() {
        let content = "SECRET=\"my secret\"\nTOKEN='abc123'\n";
        let result = import_from_str(content, ImportFormat::DotEnv).unwrap();
        assert_eq!(result.entries.get("SECRET"), Some(&"my secret".to_string()));
        assert_eq!(result.entries.get("TOKEN"), Some(&"abc123".to_string()));
    }

    #[test]
    fn test_parse_dotenv_skips_comments_and_blanks() {
        let content = "# comment\n\nKEY=val\n";
        let result = import_from_str(content, ImportFormat::DotEnv).unwrap();
        assert_eq!(result.count, 1);
        assert!(result.skipped.is_empty());
    }

    #[test]
    fn test_parse_dotenv_skips_invalid_lines() {
        let content = "INVALID_LINE\nKEY=val\n";
        let result = import_from_str(content, ImportFormat::DotEnv).unwrap();
        assert_eq!(result.count, 1);
        assert_eq!(result.skipped.len(), 1);
        assert!(result.skipped[0].contains("no '=' found"));
    }

    #[test]
    fn test_parse_json_basic() {
        let content = r#"{"DB_HOST": "localhost", "DB_PORT": "5432"}"#;
        let result = import_from_str(content, ImportFormat::Json).unwrap();
        assert_eq!(result.count, 2);
        assert_eq!(result.entries.get("DB_HOST"), Some(&"localhost".to_string()));
    }

    #[test]
    fn test_parse_json_invalid_format() {
        let content = "not json";
        let result = import_from_str(content, ImportFormat::Json);
        assert!(result.is_err());
    }

    #[test]
    fn test_yaml_not_supported() {
        let result = import_from_str("key: value", ImportFormat::Yaml);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not yet supported"));
    }

    #[test]
    fn test_detect_format_json() {
        use std::path::Path;
        assert_eq!(detect_format(Path::new("config.json")), ImportFormat::Json);
    }

    #[test]
    fn test_detect_format_yaml() {
        use std::path::Path;
        assert_eq!(detect_format(Path::new("config.yaml")), ImportFormat::Yaml);
        assert_eq!(detect_format(Path::new("config.yml")), ImportFormat::Yaml);
    }

    #[test]
    fn test_detect_format_dotenv_default() {
        use std::path::Path;
        assert_eq!(detect_format(Path::new(".env")), ImportFormat::DotEnv);
        assert_eq!(detect_format(Path::new("envfile")), ImportFormat::DotEnv);
    }
}
