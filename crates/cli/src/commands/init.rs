//! `init`.

use clap::Args;
use rulelink_app::Error;

/// Arguments of `init`.
#[derive(Debug, Args)]
pub struct InitArgs {
    /// Prefix of the generated rule identifiers.
    #[arg(long, default_value = rulelink_app::DEFAULT_ID_PREFIX)]
    pub(super) id_prefix: String,
}

pub(super) fn run(args: &InitArgs) -> Result<(), Error> {
    let root = super::project_root()?;
    rulelink_app::init::init(&root, &args.id_prefix)?;
    println!("Initialised Rulelink: wrote rulelink.toml and rules/rules.json");
    Ok(())
}
