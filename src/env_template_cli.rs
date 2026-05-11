//! CLI interface for rendering .env templates.

use crate::env_template::{EnvTemplate, TemplateError};
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
pub struct TemplateRenderArgs {
    pub template_path: String,
    pub output_path: Option<String>,
    pub vars: HashMap<String, String>,
    pub dry_run: bool,
}

pub fn run_template_render(args: TemplateRenderArgs) -> Result<(), String> {
    let raw = fs::read_to_string(&args.template_path)
        .map_err(|e| format!("Failed to read template '{}': {}", args.template_path, e))?;

    let template = EnvTemplate::new(raw);

    if args.dry_run {
        let vars = template.variables();
        println!("Template variables found ({}):", vars.len());
        for v in &vars {
            let status = if args.vars.contains_key(v) { "✓" } else { "✗ MISSING" };
            println!("  {} {}", status, v);
        }
        return Ok(());
    }

    let rendered = template.render(&args.vars).map_err(|e| e.to_string())?;

    match &args.output_path {
        Some(path) => {
            fs::write(path, &rendered)
                .map_err(|e| format!("Failed to write output '{}': {}", path, e))?;
            println!("Rendered template written to '{}'", path);
        }
        None => print!("{}", rendered),
    }

    Ok(())
}

pub fn parse_var_pair(s: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = s.splitn(2, '=').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid var pair '{}': expected KEY=VALUE", s));
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}
