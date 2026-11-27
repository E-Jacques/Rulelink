//! `rules list` and `rules search`.

use clap::Args;
use rulelink_app::{Error, Rule, RuleStatus, SearchText};

use super::open;
use crate::presenter::Presenter;

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

/// Prints every Rule, or only those in `--status`, one per line.
///
/// # Errors
/// As [`rulelink_app::rules::select`], or if the project cannot be opened.
pub(super) fn list(args: &RulesListArgs, out: &mut Presenter<'_>) -> Result<(), Error> {
    print_selection(None, args.status, out)
}

/// Prints the Rules containing the text, optionally limited to `--status`.
///
/// # Errors
/// [`Error::Input`] if the text is empty or made only of accents; otherwise as [`list`].
pub(super) fn search(args: &RulesSearchArgs, out: &mut Presenter<'_>) -> Result<(), Error> {
    let text = SearchText::parse(&args.text)?;
    print_selection(Some(&text), args.status, out)
}

/// Prints one line per Rule selected by `text` and `status`, in Identifier order.
fn print_selection(
    text: Option<&SearchText>,
    status: Option<RuleStatus>,
    out: &mut Presenter<'_>,
) -> Result<(), Error> {
    for rule in rulelink_app::rules::select(&open(out)?, text, status)? {
        out.log(line(&rule))?;
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
