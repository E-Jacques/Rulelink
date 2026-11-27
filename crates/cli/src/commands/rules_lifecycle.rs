//! `rules approve`, `rules reject` and `rules obsolete`.

use clap::Args;
use rulelink_app::{Error, RuleId};

use super::open;
use crate::presenter::Presenter;

/// Arguments of `rules approve` and `rules reject`.
#[derive(Debug, Args)]
pub struct RuleRefArgs {
    /// Identifier of the Rule.
    pub(super) id: String,
}

/// Arguments of `rules obsolete`.
#[derive(Debug, Args)]
pub struct RulesObsoleteArgs {
    /// Identifier of the Rule to retire.
    pub(super) id: String,
    /// Identifier of the Rule that replaces it.
    #[arg(long)]
    pub(super) superseded_by: Option<String>,
}

pub(super) fn approve(args: &RuleRefArgs, out: &mut Presenter<'_>) -> Result<(), Error> {
    let id = RuleId::parse(&args.id, "the Rule Identifier")?;
    rulelink_app::rules::approve(&open(out)?, &id)?;
    out.line(format_args!("{id} is now validated"))
}

pub(super) fn reject(args: &RuleRefArgs, out: &mut Presenter<'_>) -> Result<(), Error> {
    let id = RuleId::parse(&args.id, "the Rule Identifier")?;
    rulelink_app::rules::reject(&open(out)?, &id)?;
    out.line(format_args!(
        "{id} is now rejected; its Identifier is retired"
    ))
}

pub(super) fn obsolete(args: &RulesObsoleteArgs, out: &mut Presenter<'_>) -> Result<(), Error> {
    let id = RuleId::parse(&args.id, "the Rule Identifier")?;
    let replacement = args
        .superseded_by
        .as_deref()
        .map(|raw| RuleId::parse(raw, "--superseded-by"))
        .transpose()?;
    rulelink_app::rules::obsolete(&open(out)?, &id, replacement.as_ref())?;
    out.line(format_args!(
        "{id} is now obsolete; its Identifier is retired"
    ))
}
