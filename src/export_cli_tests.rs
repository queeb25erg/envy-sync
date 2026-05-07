#[cfg(test)]
mod tests {
    use crate::export::ExportFormat;
    use crate::export_cli::{run_export, ExportArgs};
    use std::collections::HashMap;
    use std::str::FromStr;

    fn sample_vars() -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("FOO".into(), "bar".into());
        m.insert("BAZ".into(), "qux".into());
        m
    }

    #[test]
    fn test_parse_format_dotenv() {
        let f = ExportFormat::from_str("dotenv").unwrap();
        assert_eq!(f, ExportFormat::DotEnv);
    }

    #[test]
    fn test_parse_format_json() {
        let f = ExportFormat::from_str("json").unwrap();
        assert_eq!(f, ExportFormat::Json);
    }

    #[test]
    fn test_parse_format_shell_alias() {
        let f = ExportFormat::from_str("sh").unwrap();
        assert_eq!(f, ExportFormat::Shell);
    }

    #[test]
    fn test_parse_format_unknown_errors() {
        assert!(ExportFormat::from_str("toml").is_err());
    }

    #[test]
    fn test_run_export_no_path_returns_content() {
        let args = ExportArgs {
            format: ExportFormat::DotEnv,
            output_path: None,
        };
        let result = run_export(&args, &sample_vars()).unwrap();
        assert!(result.contains("FOO=bar"));
        assert!(result.contains("BAZ=qux"));
    }

    #[test]
    fn test_run_export_to_file() {
        let tmp = std::env::temp_dir().join("envy_export_test.env");
        let args = ExportArgs {
            format: ExportFormat::DotEnv,
            output_path: Some(tmp.to_string_lossy().to_string()),
        };
        let result = run_export(&args, &sample_vars()).unwrap();
        assert!(result.contains("Exported 2 vars to"));
        let written = std::fs::read_to_string(&tmp).unwrap();
        assert!(written.contains("FOO=bar"));
        let _ = std::fs::remove_file(tmp);
    }
}
