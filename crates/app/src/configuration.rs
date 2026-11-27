//! Configuration use case: what to warn about before a command runs.

use std::path::Path;

use rulelink_config::{CONFIG_FILE, Config};

/// One sentence for each name of `rulelink.toml` that has no effect.
///
/// Empty if the configuration cannot be loaded: the command that needs it reports why.
#[must_use]
pub fn warnings(root: &Path) -> Vec<String> {
    let unknown = Config::load(root).map(|config| config.unknown_keys);
    unknown
        .unwrap_or_default()
        .iter()
        // A quoted TOML name may hold a line break: it must not add lines to the output.
        .map(|name| {
            format!(
                "{CONFIG_FILE}: unknown section or key `{}` is ignored; check its spelling",
                name.escape_debug()
            )
        })
        .collect()
}
