//! `rules add`.

use clap::Args;
use rulelink_app::Error;
use rulelink_app::{RuleDraft, RuleId};

use super::open;
use crate::presenter::Presenter;

/// Arguments of `rules add`.
#[derive(Debug, Args)]
pub struct RulesAddArgs {
    /// Short title of the Rule (1 to 120 characters).
    #[arg(long)]
    pub(super) title: String,
    /// Statement of the Rule.
    #[arg(long)]
    pub(super) statement: String,
    /// Application the Rule applies to; repeat for several.
    #[arg(long = "app")]
    pub(super) applications: Vec<String>,
    /// Functional module of the Rule.
    #[arg(long)]
    pub(super) module: Option<String>,
    /// Case the Rule covers; repeat for several.
    #[arg(long)]
    pub(super) covered: Vec<String>,
    /// Case the Rule explicitly excludes; repeat for several.
    #[arg(long)]
    pub(super) excluded: Vec<String>,
    /// Identifier of a related Rule; repeat for several.
    #[arg(long)]
    pub(super) related: Vec<String>,
}

pub(super) fn run(args: &RulesAddArgs, out: &mut Presenter<'_>) -> Result<(), Error> {
    let related = args
        .related
        .iter()
        .map(|raw| RuleId::parse(raw, "--related"))
        .collect::<Result<Vec<_>, _>>()?;
    let draft = RuleDraft::new(
        &args.title,
        &args.statement,
        &args.applications,
        args.module.as_deref(),
        &args.covered,
        &args.excluded,
        &related,
    )?;
    let id = rulelink_app::rules::add(&open(out)?, draft)?;
    out.line(format_args!("Created {} (proposed)", id.0))
}
