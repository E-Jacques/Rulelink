//! Configuration use case: what to warn about before a command runs.

use std::path::Path;

use rulelink_config::{CONFIG_FILE, Config, UnknownKey};

/// One sentence for each name of `rulelink.toml` that has no effect.
///
/// Empty if the configuration cannot be loaded: the command that needs it reports why.
#[must_use]
pub fn warnings(root: &Path) -> Vec<String> {
    let unknown = Config::load(root).map(|config| config.unknown_keys);
    unknown
        .unwrap_or_default()
        .iter()
        .map(|key| {
            format!(
                "{CONFIG_FILE}: unknown section or key `{}` is ignored; check its spelling",
                written(key)
            )
        })
        .collect()
}

/// `section.key`, or a top-level name, quoted if its dot could pass for a section.
fn written(key: &UnknownKey) -> String {
    // A quoted TOML name may hold a line break: it must not add lines to the output.
    let name = key.name.escape_debug();
    match key.section {
        Some(section) => format!("{section}.{name}"),
        None if key.name.contains('.') => format!("\"{name}\""),
        None => name.to_string(),
    }
}
