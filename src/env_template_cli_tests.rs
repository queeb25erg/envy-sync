#[cfg(test)]
mod tests {
    use super::super::env_template_cli::{parse_var_pair, run_template_render, TemplateRenderArgs};
    use std::collections::HashMap;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_parse_var_pair_valid() {
        let (k, v) = parse_var_pair("FOO=bar").unwrap();
        assert_eq!(k, "FOO");
        assert_eq!(v, "bar");
    }

    #[test]
    fn test_parse_var_pair_with_equals_in_value() {
        let (k, v) = parse_var_pair("KEY=a=b").unwrap();
        assert_eq!(k, "KEY");
        assert_eq!(v, "a=b");
    }

    #[test]
    fn test_parse_var_pair_invalid() {
        let err = parse_var_pair("NOEQUALSSIGN").unwrap_err();
        assert!(err.contains("Invalid var pair"));
    }

    #[test]
    fn test_run_template_render_to_file() {
        let dir = tempdir().unwrap();
        let tmpl_path = dir.path().join("test.env.tmpl");
        let out_path = dir.path().join("out.env");
        fs::write(&tmpl_path, "DB={{DB_NAME}}\nPORT={{PORT}}").unwrap();

        let mut vars = HashMap::new();
        vars.insert("DB_NAME".into(), "mydb".into());
        vars.insert("PORT".into(), "5432".into());

        let args = TemplateRenderArgs {
            template_path: tmpl_path.to_string_lossy().into(),
            output_path: Some(out_path.to_string_lossy().into()),
            vars,
            dry_run: false,
        };

        run_template_render(args).unwrap();
        let content = fs::read_to_string(&out_path).unwrap();
        assert_eq!(content, "DB=mydb\nPORT=5432");
    }

    #[test]
    fn test_run_template_render_missing_var_error() {
        let dir = tempdir().unwrap();
        let tmpl_path = dir.path().join("test.env.tmpl");
        fs::write(&tmpl_path, "KEY={{MISSING}}").unwrap();

        let args = TemplateRenderArgs {
            template_path: tmpl_path.to_string_lossy().into(),
            output_path: None,
            vars: HashMap::new(),
            dry_run: false,
        };

        let err = run_template_render(args).unwrap_err();
        assert!(err.contains("Missing variable"));
    }

    #[test]
    fn test_run_template_render_dry_run() {
        let dir = tempdir().unwrap();
        let tmpl_path = dir.path().join("test.env.tmpl");
        fs::write(&tmpl_path, "A={{FOO}}\nB={{BAR}}").unwrap();

        let mut vars = HashMap::new();
        vars.insert("FOO".into(), "x".into());

        let args = TemplateRenderArgs {
            template_path: tmpl_path.to_string_lossy().into(),
            output_path: None,
            vars,
            dry_run: true,
        };

        // dry run should succeed without rendering
        assert!(run_template_render(args).is_ok());
    }
}
