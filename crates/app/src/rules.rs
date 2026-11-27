//! Rule use cases: create a Rule and move it along its lifecycle.

use std::path::PathBuf;

use rulelink_model::{Rule, RuleDraft, RuleId, RuleSet, RuleStatus, SearchText};

use crate::{Error, Rewrite, Workspace};

/// Reads the Source the project points to. Returns its path and its Rules.
pub(crate) fn read_source(project: &Workspace) -> Result<(PathBuf, RuleSet), Error> {
    let source_path = project.root().join(project.source_path()?);
    let rule_set = rulelink_adapter_json::read_source(&source_path)?;
    Ok((source_path, rule_set))
}

/// Reads the Source, applies `change`, then writes the Source once.
///
/// Any failure, including one from `change`, leaves the Source as it was.
fn update_source<T>(
    project: &Workspace,
    change: impl FnOnce(&mut RuleSet) -> Result<T, Error>,
) -> Result<T, Error> {
    let (source_path, mut rule_set) = read_source(project)?;
    let out = change(&mut rule_set)?;
    rulelink_adapter_json::write_source(&source_path, &rule_set)?;
    Ok(out)
}

/// Appends `draft` to the Source as a `proposed` Rule and returns its new Identifier.
///
/// # Errors
/// [`Error::Input`] if a related Rule does not exist, [`Error::Source`] for a missing,
/// invalid or unsupported Source or if no Identifier is left, [`Error::Internal`] on I/O failure.
pub fn add(project: &Workspace, draft: RuleDraft) -> Result<RuleId, Error> {
    let prefix = project.id_prefix()?;
    update_source(project, |set| Ok(set.add(prefix, draft)?))
}

/// The Rules that contain `text` (any, if `None`) and are in `status` (any, if `None`), in
/// Identifier order. Reads the Source; never writes it.
///
/// # Errors
/// [`Error::Source`] for a missing, invalid or unsupported Source.
pub fn select(
    project: &Workspace,
    text: Option<&SearchText>,
    status: Option<RuleStatus>,
) -> Result<Vec<Rule>, Error> {
    let (_, rule_set) = read_source(project)?;
    Ok(rule_set.select(text, status).into_iter().cloned().collect())
}

/// Rewrites the Source into Canonical form, leaving it untouched if it already is.
///
/// # Errors
/// [`Error::Source`] for a missing, invalid or unsupported Source (which is left as it was),
/// [`Error::Internal`] on I/O failure.
pub fn format(project: &Workspace) -> Result<Formatted, Error> {
    let path = project.source_path()?;
    let rewrite = rulelink_adapter_json::rewrite_source(&project.root().join(path))?;
    Ok(Formatted {
        path: path.to_path_buf(),
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
pub fn approve(project: &Workspace, id: &RuleId) -> Result<(), Error> {
    update_source(project, |set| Ok(set.approve(id)?))
}

/// Moves a `proposed` Rule to `rejected` and retires its Identifier.
///
/// # Errors
/// [`Error::Input`] if the Rule does not exist or is not `proposed`; otherwise as [`add`].
pub fn reject(project: &Workspace, id: &RuleId) -> Result<(), Error> {
    update_source(project, |set| Ok(set.reject(id)?))
}

/// Moves a `validated` Rule to `obsolete`, retires its Identifier and records the Rule that
/// replaces it, if any.
///
/// # Errors
/// [`Error::Input`] if the Rule does not exist or is not `validated`, or the replacement is
/// unknown or the Rule itself; otherwise as [`add`].
pub fn obsolete(
    project: &Workspace,
    id: &RuleId,
    superseded_by: Option<&RuleId>,
) -> Result<(), Error> {
    update_source(project, |set| Ok(set.obsolete(id, superseded_by)?))
}
