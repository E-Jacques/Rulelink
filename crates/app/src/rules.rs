//! `rules add`: create a proposed Rule with an allocated Identifier.

use std::fs;
use std::path::Path;

use checkmate_config::{CONFIG_FILE, Config};
use checkmate_model::{Origin, Rule, RuleId, RuleStatus, Scope};

use crate::runtime::AppError;

/// What the user supplies for a new Rule; the Identifier and status are decided here.
#[derive(Debug, Clone, Default)]
pub struct NewRule {
    /// Short title.
    pub title: String,
    /// Statement of the Rule.
    pub statement: String,
    /// Applications the Rule applies to.
    pub applications: Vec<String>,
    /// Functional module.
    pub module: Option<String>,
    /// Covered cases.
    pub covered: Vec<String>,
    /// Excluded cases.
    pub excluded: Vec<String>,
    /// Related Rules.
    pub related: Vec<String>,
    /// Origin as `kind:reference`.
    pub origin: Option<String>,
}

/// Adds `new` to the Source of the project at `root` as a `proposed` Rule.
///
/// Nothing is written unless the whole operation succeeds.
///
/// # Errors
/// [`AppError::Config`] when the project is not initialised or the input is invalid,
/// [`AppError::Source`] when the Source cannot be read or written.
pub fn add(root: &Path, new: NewRule) -> Result<RuleId, AppError> {
    let config_text = fs::read_to_string(root.join(CONFIG_FILE)).map_err(|_| {
        AppError::Config(format!(
            "{CONFIG_FILE} not found: run `checkmate init` at the root of your project first"
        ))
    })?;
    let config = Config::parse(&config_text).map_err(|e| AppError::Config(e.to_string()))?;
    let source_path = root.join(&config.source_path);
    let mut set = checkmate_adapter_json::read_source(&source_path)
        .map_err(|e| AppError::Source(e.to_string()))?;

    let origin = new.origin.as_deref().map(parse_origin).transpose()?;
    let id = set.allocate_id(config.prefix.as_str());
    let rule = Rule {
        id: id.clone(),
        title: new.title,
        statement: new.statement,
        status: RuleStatus::Proposed,
        applications: new.applications,
        module: new.module,
        scope: (!new.covered.is_empty() || !new.excluded.is_empty()).then_some(Scope {
            covered: new.covered,
            excluded: new.excluded,
        }),
        origin,
        related: new.related.into_iter().map(RuleId).collect(),
        superseded_by: None,
    };
    set.add(rule).map_err(|e| AppError::Config(e.to_string()))?;
    checkmate_adapter_json::write_source(&source_path, &set)
        .map_err(|e| AppError::Source(e.to_string()))?;
    Ok(id)
}

fn parse_origin(raw: &str) -> Result<Origin, AppError> {
    match raw.split_once(':') {
        Some((kind, reference)) if !kind.is_empty() && !reference.is_empty() => Ok(Origin {
            kind: kind.to_owned(),
            reference: Some(reference.to_owned()),
            url: None,
        }),
        _ => Err(AppError::Config(format!(
            "invalid origin `{raw}`: expected `kind:reference`, e.g. `jira:BILL-42`"
        ))),
    }
}
