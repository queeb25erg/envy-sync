//! CLI handlers for profile subcommands

use crate::profile::{Profile, ProfileStore};

#[derive(Debug)]
pub enum ProfileCommand {
    Create { name: String, description: Option<String> },
    Delete { name: String },
    List,
    Show { name: String },
    SetVar { profile: String, key: String, value: String },
    RemoveVar { profile: String, key: String },
}

pub fn handle_profile_command(cmd: ProfileCommand, store: &mut ProfileStore) -> Result<String, String> {
    match cmd {
        ProfileCommand::Create { name, description } => {
            if store.contains(&name) {
                return Err(format!("Profile '{}' already exists", name));
            }
            let mut p = Profile::new(&name);
            if let Some(desc) = description {
                p = p.with_description(desc);
            }
            store.add(p);
            Ok(format!("Created profile '{}'.", name))
        }
        ProfileCommand::Delete { name } => {
            store.remove(&name)
                .map(|_| format!("Deleted profile '{}'.", name))
                .ok_or_else(|| format!("Profile '{}' not found", name))
        }
        ProfileCommand::List => {
            let names = store.list_names();
            if names.is_empty() {
                Ok("No profiles found.".to_string())
            } else {
                Ok(names.iter().map(|n| format!("  - {}", n)).collect::<Vec<_>>().join("\n"))
            }
        }
        ProfileCommand::Show { name } => {
            let p = store.get(&name).ok_or_else(|| format!("Profile '{}' not found", name))?;
            let mut out = format!("Profile: {}\n", p.name);
            if let Some(d) = &p.description { out.push_str(&format!("  Description: {}\n", d)); }
            out.push_str(&format!("  Variables ({}):\n", p.var_count()));
            let mut keys: Vec<&String> = p.env_vars.keys().collect();
            keys.sort();
            for k in keys { out.push_str(&format!("    {}={}\n", k, p.env_vars[k])); }
            Ok(out)
        }
        ProfileCommand::SetVar { profile, key, value } => {
            let p = store.get_mut(&profile).ok_or_else(|| format!("Profile '{}' not found", profile))?;
            p.set_var(&key, &value);
            Ok(format!("Set '{}' in profile '{}'.", key, profile))
        }
        ProfileCommand::RemoveVar { profile, key } => {
            let p = store.get_mut(&profile).ok_or_else(|| format!("Profile '{}' not found", profile))?;
            p.remove_var(&key).map(|_| format!("Removed '{}' from profile '{}'.", key, profile))
                .ok_or_else(|| format!("Key '{}' not found in profile '{}'.", key, profile))
        }
    }
}
