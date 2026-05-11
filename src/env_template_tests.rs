#[cfg(test)]
mod tests {
    use super::super::env_template::{EnvTemplate, TemplateError};
    use std::collections::HashMap;

    fn vars(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    #[test]
    fn test_render_single_var() {
        let t = EnvTemplate::new("HOST={{ HOST }}");
        let result = t.render(&vars(&[("HOST", "localhost")])).unwrap();
        assert_eq!(result, "HOST=localhost");
    }

    #[test]
    fn test_render_multiple_vars() {
        let t = EnvTemplate::new("DB_HOST={{DB_HOST}}\nDB_PORT={{DB_PORT}}");
        let result = t.render(&vars(&[("DB_HOST", "127.0.0.1"), ("DB_PORT", "5432")])).unwrap();
        assert_eq!(result, "DB_HOST=127.0.0.1\nDB_PORT=5432");
    }

    #[test]
    fn test_render_missing_variable() {
        let t = EnvTemplate::new("KEY={{MISSING}}");
        let err = t.render(&vars(&[])).unwrap_err();
        assert_eq!(err, TemplateError::MissingVariable("MISSING".into()));
    }

    #[test]
    fn test_render_unclosed_placeholder() {
        let t = EnvTemplate::new("KEY={{UNCLOSED");
        let err = t.render(&vars(&[])).unwrap_err();
        assert!(matches!(err, TemplateError::InvalidSyntax(_)));
    }

    #[test]
    fn test_render_empty_placeholder() {
        let t = EnvTemplate::new("KEY={{ }}");
        let err = t.render(&vars(&[])).unwrap_err();
        assert!(matches!(err, TemplateError::InvalidSyntax(_)));
    }

    #[test]
    fn test_variables_extraction() {
        let t = EnvTemplate::new("A={{FOO}}\nB={{BAR}}\nC={{FOO}}");
        let vars = t.variables();
        assert_eq!(vars, vec!["FOO", "BAR", "FOO"]);
    }

    #[test]
    fn test_render_no_placeholders() {
        let t = EnvTemplate::new("PLAIN=value");
        let result = t.render(&vars(&[])).unwrap();
        assert_eq!(result, "PLAIN=value");
    }

    #[test]
    fn test_render_preserves_surrounding_text() {
        let t = EnvTemplate::new("prefix_{{NAME}}_suffix");
        let result = t.render(&vars(&[("NAME", "mid")])).unwrap();
        assert_eq!(result, "prefix_mid_suffix");
    }
}
