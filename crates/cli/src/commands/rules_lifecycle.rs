//! `rules approve`, `rules reject` and `rules obsolete`.

use clap::Args;
use rulelink_app::{Error, RuleId};

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

fn root() -> Result<std::path::PathBuf, Error> {
    std::env::current_dir().map_err(|e| Error::Internal(e.to_string()))
}

pub(super) fn approve(args: &RuleRefArgs) -> Result<(), Error> {
    let id = RuleId(args.id.clone());
    rulelink_app::rules::approve(&root()?, &id)?;
    println!("{id} is now validated");
    Ok(())
}

pub(super) fn reject(args: &RuleRefArgs) -> Result<(), Error> {
    let id = RuleId(args.id.clone());
    rulelink_app::rules::reject(&root()?, &id)?;
    println!("{id} is now rejected; its Identifier is retired");
    Ok(())
}

pub(super) fn obsolete(args: &RulesObsoleteArgs) -> Result<(), Error> {
    let id = RuleId(args.id.clone());
    let replacement = args.superseded_by.clone().map(RuleId);
    rulelink_app::rules::obsolete(&root()?, &id, replacement.as_ref())?;
    println!("{id} is now obsolete; its Identifier is retired");
    Ok(())
}
