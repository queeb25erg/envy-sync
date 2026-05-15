//! Tests for env_resolve module.

#[cfg(test)]
mod tests {
    use crate::env_resolve::{resolve_env, ResolveError};
    use std::collections::HashMap;

    fn map(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    #[test]
    fn test_simple_resolution() {
        let env = map(&[("BASE", "/home/user"), ("PATH", "${BASE}/bin")]);
        let resolved = resolve_env(&env).unwrap();
        assert_eq!(resolved["PATH"], "/home/user/bin");
    }

    #[test]
    fn test_no_references() {
        let env = map(&[("FOO", "bar"), ("BAZ", "qux")]);
        let resolved = resolve_env(&env).unwrap();
        assert_eq!(resolved["FOO"], "bar");
        assert_eq!(resolved["BAZ"], "qux");
    }

    #[test]
    fn test_chained_references() {
        let env = map(&[
            ("A", "hello"),
            ("B", "${A}_world"),
            ("C", "${B}!"),
        ]);
        let resolved = resolve_env(&env).unwrap();
        assert_eq!(resolved["C"], "hello_world!");
    }

    #[test]
    fn test_cyclic_reference_detected() {
        let env = map(&[("X", "${Y}"), ("Y", "${X}")]);
        let result = resolve_env(&env);
        assert!(matches!(result, Err(ResolveError::CyclicReference(_))));
    }

    #[test]
    fn test_undefined_variable_error() {
        let env = map(&[("FOO", "${UNDEFINED}")]);
        let result = resolve_env(&env);
        assert!(matches!(result, Err(ResolveError::UndefinedVariable(ref v)) if v == "UNDEFINED"));
    }

    #[test]
    fn test_dollar_without_braces() {
        let env = map(&[("HOST", "localhost"), ("URL", "http://$HOST:8080")]);
        let resolved = resolve_env(&env).unwrap();
        assert_eq!(resolved["URL"], "http://localhost:8080");
    }

    #[test]
    fn test_multiple_references_in_value() {
        let env = map(&[
            ("PROTO", "https"),
            ("HOST", "example.com"),
            ("URL", "${PROTO}://${HOST}/api"),
        ]);
        let resolved = resolve_env(&env).unwrap();
        assert_eq!(resolved["URL"], "https://example.com/api");
    }

    #[test]
    fn test_self_reference_is_cyclic() {
        let env = map(&[("LOOP", "${LOOP}")]);
        let result = resolve_env(&env);
        assert!(matches!(result, Err(ResolveError::CyclicReference(_))));
    }
}
