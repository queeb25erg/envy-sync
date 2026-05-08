use crate::restore::{list_snapshots, restore_snapshot, write_env_file};
use crate::storage::Storage;
use anyhow::Result;

pub struct RestoreOptions {
    pub snapshot_id: Option<String>,
    pub output_path: String,
    pub list_only: bool,
}

pub fn run_restore(
    storage: &dyn Storage,
    key: &[u8],
    opts: RestoreOptions,
) -> Result<()> {
    if opts.list_only {
        let metas = list_snapshots(storage)?;
        if metas.is_empty() {
            println!("No snapshots available.");
        } else {
            println!("{:<36}  {:<20}  {}", "ID", "Created At", "Label");
            println!("{}", "-".repeat(70));
            for m in &metas {
                println!("{:<36}  {:<20}  {}", m.id, m.created_at, m.label);
            }
        }
        return Ok(());
    }

    let id = opts
        .snapshot_id
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("--snapshot-id is required for restore"))?;

    let env_vars = restore_snapshot(storage, id, key)?;
    write_env_file(&opts.output_path, &env_vars)?;
    println!(
        "Restored {} variable(s) from snapshot '{}' to '{}'",
        env_vars.len(),
        id,
        opts.output_path
    );
    Ok(())
}
