//! `rules add`: create a Rule in the Source.

use std::path::Path;

use checkmate_config::{CONFIG_FILE, Config};
use checkmate_model::{NewRule, RuleId};

use crate::runtime::AppError;

/// Creates a `proposed` Rule in the Source configured in `root` and returns its Identifier.
///
/// The Source is rewritten in Canonical form, and only when the Rule is valid.
///
/// # Errors
/// [`AppError::Config`] if the project is not initialised or the Rule is invalid,
/// [`AppError::Source`] if the Source cannot be read, [`AppError::Internal`] on write failure.
pub fn add(root: &Path, new: NewRule) -> Result<RuleId, AppError> {
    let config = load_config(root)?;
    let source_path = root.join(&config.source_path);
    let mut set = checkmate_adapter_json::read_source(&source_path)
        .map_err(|e| AppError::Source(format!("{}: {e}", config.source_path)))?;
    let id = set
        .add_proposed(config.id_prefix.as_str(), new)
        .map_err(|e| AppError::Config(e.to_string()))?;
    checkmate_adapter_json::write_source(&source_path, &set)
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(id)
}

fn load_config(root: &Path) -> Result<Config, AppError> {
    let text = std::fs::read_to_string(root.join(CONFIG_FILE)).map_err(|e| match e.kind() {
        std::io::ErrorKind::NotFound => AppError::Config(format!(
            "{CONFIG_FILE} not found: run `checkmate init` first"
        )),
        _ => AppError::Internal(e.to_string()),
    })?;
    Config::parse(&text).map_err(|e| AppError::Config(e.to_string()))
}
