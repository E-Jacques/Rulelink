//! Rule use cases: create a Rule and move it along its lifecycle.

use std::path::{Path, PathBuf};

use rulelink_config::Config;
use rulelink_model::{Rule, RuleDraft, RuleId, RuleSet, RuleStatus, SearchText};

use crate::{Error, Rewrite};

/// Loads the configuration, then reads the Source it points to.
///
/// Returns the configuration, the Source's path and its Rules.
pub(crate) fn read_source(root: &Path) -> Result<(Config, PathBuf, RuleSet), Error> {
    let config = Config::load(root)?;
    let source_path = root.join(&config.source_path);
    let rule_set = rulelink_adapter_json::read_source(&source_path)?;
    Ok((config, source_path, rule_set))
}

/// Loads the configuration and the Source, applies `change`, then writes the Source once.
///
/// Any failure, including one from `change`, leaves the Source as it was.
fn update_source<T>(
    root: &Path,
    change: impl FnOnce(&Config, &mut RuleSet) -> Result<T, Error>,
) -> Result<T, Error> {
    let (config, source_path, mut rule_set) = read_source(root)?;
    let out = change(&config, &mut rule_set)?;
    rulelink_adapter_json::write_source(&source_path, &rule_set)?;
    Ok(out)
}

/// Appends `draft` to the Source as a `proposed` Rule and returns its new Identifier.
///
/// # Errors
/// [`Error::Input`] if a related Rule does not exist, [`Error::Config`] for a missing or
/// invalid `rulelink.toml`, [`Error::Source`] for a missing, invalid or unsupported Source or
/// if no Identifier is left, [`Error::Internal`] on I/O failure.
pub fn add(root: &Path, draft: RuleDraft) -> Result<RuleId, Error> {
    update_source(root, |config, set| Ok(set.add(&config.id_prefix, draft)?))
}

/// The Rules that contain `text` (any, if `None`) and are in `status` (any, if `None`), in
/// Identifier order. Reads the Source; never writes it.
///
/// # Errors
/// [`Error::Config`] for a missing or invalid `rulelink.toml`, [`Error::Source`] for a
/// missing, invalid or unsupported Source.
pub fn select(
    root: &Path,
    text: Option<&SearchText>,
    status: Option<RuleStatus>,
) -> Result<Vec<Rule>, Error> {
    let (_, _, rule_set) = read_source(root)?;
    Ok(rule_set.select(text, status).into_iter().cloned().collect())
}

/// Rewrites the Source into Canonical form, leaving it untouched if it already is.
///
/// # Errors
/// [`Error::Config`] for a missing or invalid `rulelink.toml`, [`Error::Source`] for a
/// missing, invalid or unsupported Source (which is left as it was), [`Error::Internal`] on
/// I/O failure.
pub fn format(root: &Path) -> Result<Formatted, Error> {
    let config = Config::load(root)?;
    let rewrite = rulelink_adapter_json::rewrite_source(&root.join(&config.source_path))?;
    Ok(Formatted {
        path: config.source_path,
        rewrite,
    })
}

/// Result of [`format`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Formatted {
    /// The Source, as `rulelink.toml` names it (relative to the project root).
    pub path: PathBuf,
    /// Whether the file had to change.
    pub rewrite: Rewrite,
}

/// Moves a `proposed` Rule to `validated`.
///
/// # Errors
/// [`Error::Input`] if the Rule does not exist or is not `proposed`; otherwise as [`add`].
pub fn approve(root: &Path, id: &RuleId) -> Result<(), Error> {
    update_source(root, |_, set| Ok(set.approve(id)?))
}

/// Moves a `proposed` Rule to `rejected` and retires its Identifier.
///
/// # Errors
/// [`Error::Input`] if the Rule does not exist or is not `proposed`; otherwise as [`add`].
pub fn reject(root: &Path, id: &RuleId) -> Result<(), Error> {
    update_source(root, |_, set| Ok(set.reject(id)?))
}

/// Moves a `validated` Rule to `obsolete`, retires its Identifier and records the Rule that
/// replaces it, if any.
///
/// # Errors
/// [`Error::Input`] if the Rule does not exist or is not `validated`, or the replacement is
/// unknown or the Rule itself; otherwise as [`add`].
pub fn obsolete(root: &Path, id: &RuleId, superseded_by: Option<&RuleId>) -> Result<(), Error> {
    update_source(root, |_, set| Ok(set.obsolete(id, superseded_by)?))
}
