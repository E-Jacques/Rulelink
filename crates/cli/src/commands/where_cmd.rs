//! `where`.

use clap::Args;
use rulelink_app::{Error, RuleId, RuleStatus};

use super::open;
use crate::presenter::Presenter;

/// Arguments of `where`.
#[derive(Debug, Args)]
pub struct WhereArgs {
    /// Identifier of the Rule to look for.
    pub(super) id: String,
}

/// Prints `file:line` for each Annotation citing the Rule, or says why there is none.
pub(super) fn run(args: &WhereArgs, out: &mut Presenter<'_>) -> Result<(), Error> {
    let id = RuleId::parse(&args.id, "the Rule Identifier")?;
    let found = rulelink_app::links::locate(&open(out)?, &id)?;
    if matches!(found.status, RuleStatus::Obsolete | RuleStatus::Rejected) {
        out.log(format_args!(
            "{id} is {}: its Identifier is retired",
            found.status
        ))?;
    } else if found.locations.is_empty() {
        out.log(format_args!("{id} is not cited by any code"))?;
    }
    for location in &found.locations {
        out.log(format_args!("{}:{}", location.path, location.line))?;
    }
    Ok(())
}
