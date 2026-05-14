use std::collections::HashMap;

/// Represents a named group of environment variable keys.
#[derive(Debug, Clone, PartialEq)]
pub struct EnvGroup {
    pub name: String,
    pub keys: Vec<String>,
}

impl EnvGroup {
    pub fn new(name: impl Into<String>, keys: Vec<String>) -> Self {
        Self {
            name: name.into(),
            keys,
        }
    }
}

/// Groups environment variables by their prefix (e.g. `DB_`, `AWS_`).
pub fn group_by_prefix(env: &HashMap<String, String>) -> HashMap<String, EnvGroup> {
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();

    for key in env.keys() {
        let prefix = key
            .find('_')
            .map(|i| key[..i].to_string())
            .unwrap_or_else(|| "MISC".to_string());
        groups.entry(prefix).or_default().push(key.clone());
    }

    groups
        .into_iter()
        .map(|(prefix, mut keys)| {
            keys.sort();
            (prefix.clone(), EnvGroup::new(prefix, keys))
        })
        .collect()
}

/// Filters an env map to only keys belonging to a given group.
pub fn filter_by_group<'a>(
    env: &'a HashMap<String, String>,
    group: &EnvGroup,
) -> HashMap<&'a str, &'a str> {
    env.iter()
        .filter(|(k, _)| group.keys.contains(k))
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect()
}

/// Returns all keys in `env` not belonging to any provided group.
pub fn ungrouped_keys<'a>(
    env: &'a HashMap<String, String>,
    groups: &[EnvGroup],
) -> Vec<&'a str> {
    let grouped: std::collections::HashSet<&str> =
        groups.iter().flat_map(|g| g.keys.iter().map(|k| k.as_str())).collect();
    let mut ungrouped: Vec<&str> = env.keys()
        .filter(|k| !grouped.contains(k.as_str()))
        .map(|k| k.as_str())
        .collect();
    ungrouped.sort();
    ungrouped
}
