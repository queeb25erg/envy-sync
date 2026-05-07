#[cfg(test)]
mod tests {
    use crate::export::{export_vars, ExportFormat};
    use std::collections::HashMap;

    fn sample_vars() -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("APP_ENV".into(), "production".into());
        m.insert("DB_URL".into(), "postgres://localhost/mydb".into());
        m.insert("SECRET".into(), "has space here".into());
        m
    }

    #[test]
    fn test_dotenv_format_sorted() {
        let vars = sample_vars();
        let out = export_vars(&vars, &ExportFormat::DotEnv).unwrap();
        let lines: Vec<&str> = out.lines().collect();
        // Lines should be sorted alphabetically by key
        let mut sorted = lines.clone();
        sorted.sort();
        assert_eq!(lines, sorted);
    }

    #[test]
    fn test_dotenv_quotes_spaces() {
        let vars = sample_vars();
        let out = export_vars(&vars, &ExportFormat::DotEnv).unwrap();
        assert!(out.contains("SECRET='has space here'"));
    }

    #[test]
    fn test_json_format_valid() {
        let vars = sample_vars();
        let out = export_vars(&vars, &ExportFormat::Json).unwrap();
        let parsed: HashMap<String, String> = serde_json::from_str(&out).unwrap();
        assert_eq!(parsed["APP_ENV"], "production");
        assert_eq!(parsed["DB_URL"], "postgres://localhost/mydb");
    }

    #[test]
    fn test_shell_format_export_prefix() {
        let vars = sample_vars();
        let out = export_vars(&vars, &ExportFormat::Shell).unwrap();
        for line in out.lines() {
            assert!(line.starts_with("export "), "Line missing export: {}", line);
        }
    }

    #[test]
    fn test_shell_format_sorted() {
        let vars = sample_vars();
        let out = export_vars(&vars, &ExportFormat::Shell).unwrap();
        let lines: Vec<&str> = out.lines().collect();
        let mut sorted = lines.clone();
        sorted.sort();
        assert_eq!(lines, sorted);
    }

    #[test]
    fn test_empty_vars() {
        let vars = HashMap::new();
        let out = export_vars(&vars, &ExportFormat::DotEnv).unwrap();
        assert_eq!(out, "");
    }
}
