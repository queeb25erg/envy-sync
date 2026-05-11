//! Persistence layer for saved .env templates.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct TemplateStore {
    dir: PathBuf,
}

impl TemplateStore {
    pub fn new(dir: impl AsRef<Path>) -> Self {
        Self {
            dir: dir.as_ref().to_path_buf(),
        }
    }

    fn template_path(&self, name: &str) -> PathBuf {
        self.dir.join(format!("{}.env.tmpl", name))
    }

    pub fn save(&self, name: &str, content: &str) -> Result<(), String> {
        fs::create_dir_all(&self.dir)
            .map_err(|e| format!("Cannot create template dir: {}", e))?;
        let path = self.template_path(name);
        fs::write(&path, content)
            .map_err(|e| format!("Failed to save template '{}': {}", name, e))
    }

    pub fn load(&self, name: &str) -> Result<String, String> {
        let path = self.template_path(name);
        fs::read_to_string(&path)
            .map_err(|e| format!("Template '{}' not found: {}", name, e))
    }

    pub fn delete(&self, name: &str) -> Result<(), String> {
        let path = self.template_path(name);
        fs::remove_file(&path)
            .map_err(|e| format!("Failed to delete template '{}': {}", name, e))
    }

    pub fn list(&self) -> Result<Vec<String>, String> {
        if !self.dir.exists() {
            return Ok(vec![]);
        }
        let entries = fs::read_dir(&self.dir)
            .map_err(|e| format!("Cannot read template dir: {}", e))?;
        let mut names = Vec::new();
        for entry in entries.flatten() {
            let fname = entry.file_name();
            let fname = fname.to_string_lossy();
            if fname.ends_with(".env.tmpl") {
                let name = fname.trim_end_matches(".env.tmpl").to_string();
                names.push(name);
            }
        }
        names.sort();
        Ok(names)
    }
}
