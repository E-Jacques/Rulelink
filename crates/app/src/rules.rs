//! `rules add`: create a proposed Rule in the Source.

use std::path::Path;

use checkmate_config::Config;
use checkmate_model::{RuleDraft, RuleId};

use crate::runtime::AppError;

/// Appends `draft` to the Source as a `proposed` Rule and returns its new Identifier.
///
/// Reads the configuration and the Source, allocates, then writes once at the end: any
/// failure leaves the Source as it was.
///
/// # Errors
/// [`AppError::Config`] for a missing or invalid `checkmate.toml`, [`AppError::Source`] for a
/// missing, invalid or unsupported Source, [`AppError::Source`] if no Identifier is left,
/// [`AppError::Internal`] on I/O failure.
pub fn add(root: &Path, draft: RuleDraft) -> Result<RuleId, AppError> {
    let config = Config::load(root)?;
    let source_path = root.join(&config.source_path);
    let mut set = checkmate_adapter_json::read_source(&source_path)?;
    let id = set.add(&config.id_prefix, draft)?;
    checkmate_adapter_json::write_source(&source_path, &set)?;
    Ok(id)
}
