//! `rules format`.

use rulelink_app::rules::Formatted;
use rulelink_app::{Error, Rewrite};

use super::project_root;

pub(super) fn run() -> Result<(), Error> {
    let Formatted { path, rewrite } = rulelink_app::rules::format(&project_root()?)?;
    let path = path.display();
    match rewrite {
        Rewrite::Rewritten => println!("Rewrote {path} into Canonical form"),
        Rewrite::Unchanged => println!("{path} is already in Canonical form"),
    }
    Ok(())
}
