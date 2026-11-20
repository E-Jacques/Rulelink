//! `rules add`.

use checkmate_app::RuleDraft;
use checkmate_app::runtime::AppError;
use clap::Args;

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
}

pub(super) fn run(args: &RulesAddArgs) -> Result<(), AppError> {
    let draft = RuleDraft::new(
        &args.title,
        &args.statement,
        &args.applications,
        args.module.as_deref(),
        &args.covered,
        &args.excluded,
    )?;
    let root = std::env::current_dir().map_err(|e| AppError::Internal(e.to_string()))?;
    let id = checkmate_app::rules::add(&root, draft)?;
    println!("Created {} (proposed)", id.0);
    Ok(())
}
