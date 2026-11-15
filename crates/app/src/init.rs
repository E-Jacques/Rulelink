//! `init`: create the configuration and an empty Source.

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

use checkmate_config::{CONFIG_FILE, DEFAULT_SOURCE_PATH, IdPrefix};
use checkmate_model::RuleSet;

use crate::runtime::AppError;

/// Initialises Checkmate in `root`. Refuses, touching nothing, if a setup already exists.
///
/// # Errors
/// Returns [`AppError::Config`] for an invalid prefix or an existing setup, and
/// [`AppError::Internal`] on I/O failure.
pub fn init(root: &Path, prefix: &str) -> Result<(), AppError> {
    let prefix = IdPrefix::parse(prefix).map_err(|e| AppError::Config(e.to_string()))?;
    let config_path = root.join(CONFIG_FILE);
    let source_path = root.join(DEFAULT_SOURCE_PATH);
    for (path, name) in [
        (&config_path, CONFIG_FILE),
        (&source_path, DEFAULT_SOURCE_PATH),
    ] {
        if path.exists() {
            return Err(AppError::Config(format!(
                "already initialised: {name} exists, nothing was changed"
            )));
        }
    }
    checkmate_adapter_json::create_source(&source_path, &RuleSet::empty()).map_err(internal)?;
    if let Err(err) = write_new(&config_path, &checkmate_config::render_initial(&prefix)) {
        let _ = fs::remove_file(&source_path);
        return Err(internal(err));
    }
    Ok(())
}

fn write_new(path: &Path, content: &str) -> std::io::Result<()> {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?
        .write_all(content.as_bytes())
}

fn internal(err: impl std::fmt::Display) -> AppError {
    AppError::Internal(err.to_string())
}
