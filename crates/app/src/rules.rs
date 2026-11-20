//! `rules add`: create a proposed Rule in the Source.

use std::path::Path;

use checkmate_config::Config;
use checkmate_model::{RuleDraft, RuleId};

use crate::Error;

/// Appends `draft` to the Source as a `proposed` Rule and returns its new Identifier.
///
/// Reads the configuration and the Source, allocates, then writes once at the end: any
/// failure leaves the Source as it was.
///
/// # Errors
/// [`Error::Config`] for a missing or invalid `checkmate.toml`, [`Error::Source`] for a
/// missing, invalid or unsupported Source, [`Error::Source`] if no Identifier is left,
/// [`Error::Internal`] on I/O failure.
pub fn add(root: &Path, draft: RuleDraft) -> Result<RuleId, Error> {
    let config = Config::load(root)?;
    let source_path = root.join(&config.source_path);
    let mut rule_set = checkmate_adapter_json::read_source(&source_path)?;
    let id = rule_set.add(&config.id_prefix, draft)?;
    checkmate_adapter_json::write_source(&source_path, &rule_set)?;
    Ok(id)
}
