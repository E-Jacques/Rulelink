//! `rules list` and `rules search`.

use clap::Args;
use rulelink_app::{Error, Rule, RuleStatus, SearchText};

use super::project_root;

/// Arguments of `rules list`.
#[derive(Debug, Args)]
pub struct RulesListArgs {
    /// List only the Rules in this status: proposed, validated, obsolete or rejected.
    #[arg(long)]
    pub(super) status: Option<RuleStatus>,
}

/// Arguments of `rules search`.
#[derive(Debug, Args)]
pub struct RulesSearchArgs {
    /// Text to look for, ignoring case and accents.
    pub(super) text: String,
    /// Keep only the Rules in this status: proposed, validated, obsolete or rejected.
    #[arg(long)]
    pub(super) status: Option<RuleStatus>,
}

pub(super) fn list(args: &RulesListArgs) -> Result<(), Error> {
    print_selection(None, args.status)
}

pub(super) fn search(args: &RulesSearchArgs) -> Result<(), Error> {
    let text = SearchText::parse(&args.text)?;
    print_selection(Some(&text), args.status)
}

fn print_selection(text: Option<&SearchText>, status: Option<RuleStatus>) -> Result<(), Error> {
    for rule in rulelink_app::rules::select(&project_root()?, text, status)? {
        println!("{}", line(&rule));
    }
    Ok(())
}

/// `RULE-001  validated  Title`: the status is padded to the longest one so titles align.
fn line(rule: &Rule) -> String {
    format!(
        "{}  {:<9}  {}",
        rule.id,
        rule.status.to_string(),
        rule.title
    )
}
