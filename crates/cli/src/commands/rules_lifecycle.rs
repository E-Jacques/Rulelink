//! `rules approve`, `rules reject` and `rules obsolete`.

use clap::Args;
use rulelink_app::{Error, RuleId};

use super::project_root;

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

pub(super) fn approve(args: &RuleRefArgs) -> Result<(), Error> {
    let id = RuleId::parse(&args.id, "the Rule Identifier")?;
    rulelink_app::rules::approve(&project_root()?, &id)?;
    println!("{id} is now validated");
    Ok(())
}

pub(super) fn reject(args: &RuleRefArgs) -> Result<(), Error> {
    let id = RuleId::parse(&args.id, "the Rule Identifier")?;
    rulelink_app::rules::reject(&project_root()?, &id)?;
    println!("{id} is now rejected; its Identifier is retired");
    Ok(())
}

pub(super) fn obsolete(args: &RulesObsoleteArgs) -> Result<(), Error> {
    let id = RuleId::parse(&args.id, "the Rule Identifier")?;
    let replacement = args
        .superseded_by
        .as_deref()
        .map(|raw| RuleId::parse(raw, "--superseded-by"))
        .transpose()?;
    rulelink_app::rules::obsolete(&project_root()?, &id, replacement.as_ref())?;
    println!("{id} is now obsolete; its Identifier is retired");
    Ok(())
}
