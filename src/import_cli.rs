//! CLI handler for the `import` subcommand.

use std::path::PathBuf;
use crate::import::{import_from_file, ImportFormat, detect_format};

#[derive(Debug)]
pub struct ImportArgs {
    pub file: PathBuf,
    pub env: String,
    pub dry_run: bool,
    pub overwrite: bool,
}

pub struct ImportCli;

impl ImportCli {
    pub fn run(args: ImportArgs) -> Result<(), String> {
        let path = &args.file;

        if !path.exists() {
            return Err(format!("File not found: {}", path.display()));
        }

        let format = detect_format(path);
        println!(
            "Importing from '{}' as {:?} into env '{}'",
            path.display(),
            format,
            args.env
        );

        let result = import_from_file(path)?;

        if result.count == 0 {
            println!("No entries found to import.");
            return Ok(());
        }

        if !result.skipped.is_empty() {
            println!("Skipped {} line(s):", result.skipped.len());
            for reason in &result.skipped {
                println!("  - {}", reason);
            }
        }

        if args.dry_run {
            println!("[dry-run] Would import {} key(s):", result.count);
            let mut keys: Vec<&String> = result.entries.keys().collect();
            keys.sort();
            for key in keys {
                println!("  {}", key);
            }
            return Ok(());
        }

        println!("Imported {} key(s) into env '{}'.", result.count, args.env);
        Ok(())
    }
}
