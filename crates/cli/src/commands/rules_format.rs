//! `rules format`.

use rulelink_app::rules::Formatted;
use rulelink_app::{Error, Rewrite};

use super::open;
use crate::presenter::Presenter;

pub(super) fn run(out: &mut Presenter<'_>) -> Result<(), Error> {
    let Formatted { path, rewrite } = rulelink_app::rules::format(&open(out)?)?;
    let path = path.display();
    match rewrite {
        Rewrite::Rewritten => out.log(format_args!("Rewrote {path} into Canonical form")),
        Rewrite::Unchanged => out.log(format_args!("{path} is already in Canonical form")),
    }
}
