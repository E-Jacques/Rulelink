//! Command groups of the CLI. No business logic here: arguments become application commands.
//!
//! Adding a command: add a variant to [`Command`] with its own `clap::Args` struct, a handler
//! function returning `Result<(), Error>`, and one line in [`dispatch`]. A command that can run
//! to the end and still fail the run, like `check`, returns `Result<Outcome, Error>` instead.
//! A handler writes only through its [`Presenter`], and gets the project from [`open`].

use clap::error::ErrorKind;
use clap::{Parser, Subcommand};

mod check;
mod init;
mod rules_add;
mod rules_format;
mod rules_lifecycle;
mod rules_query;
mod where_cmd;

use crate::presenter::Presenter;
use check::CheckArgs;
use init::InitArgs;
use rulelink_app::runtime::{
    AppContext, CancellationToken, Entrypoint, Outcome, ShutdownReason, TelemetryConfig, UsageError,
};
use rulelink_app::{Error, Workspace};
use rules_add::RulesAddArgs;
use rules_lifecycle::{RuleRefArgs, RulesObsoleteArgs};
use rules_query::{RulesListArgs, RulesSearchArgs};
use where_cmd::WhereArgs;

/// Command line grammar, derived by `clap`.
#[derive(Debug, Parser)]
#[command(name = "rulelink", version, about, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

/// Regular CLI commands (`lsp` is routed by `main` before this grammar is used).
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Create `rulelink.toml` and an empty rules file in the current directory.
    Init(InitArgs),
    /// Report the Findings about the rules file and the links between Rules and code.
    ///
    /// Verifies the link between a Rule and the code, not that the code is correct.
    Check(CheckArgs),
    /// List the code locations that cite a Rule.
    Where(WhereArgs),
    /// Create, inspect and change Rules.
    Rules {
        /// What to do with Rules.
        #[command(subcommand)]
        action: RulesAction,
    },
}

/// Subcommands of `rules`.
#[derive(Debug, Subcommand)]
pub enum RulesAction {
    /// Create a `proposed` Rule with a newly allocated Identifier.
    Add(RulesAddArgs),
    /// Mark a `proposed` Rule as `validated`.
    Approve(RuleRefArgs),
    /// Decline a `proposed` Rule and retire its Identifier.
    Reject(RuleRefArgs),
    /// Retire a `validated` Rule, optionally recording its replacement.
    Obsolete(RulesObsoleteArgs),
    /// Rewrite the rules file into Canonical form.
    Format,
    /// List the Rules, optionally only those in one status.
    List(RulesListArgs),
    /// Find the Rules that contain a text, ignoring case and accents.
    Search(RulesSearchArgs),
}

/// Parsed command line.
#[derive(Debug)]
pub enum CliArgs {
    /// A command to run.
    Run(Command),
    /// `--help` or `--version`: text to print, nothing to run.
    Display(String),
}

/// `Entrypoint` of the regular CLI commands.
#[derive(Debug)]
pub struct CliEntrypoint;

impl Entrypoint for CliEntrypoint {
    type Args = CliArgs;

    fn name(&self) -> &'static str {
        "rulelink"
    }

    fn parse_args(&self) -> Result<Self::Args, UsageError> {
        parse(std::env::args())
    }

    fn telemetry(&self, _args: &Self::Args) -> TelemetryConfig {
        TelemetryConfig::default()
    }

    fn build_context(
        &self,
        _args: &Self::Args,
        cancel: CancellationToken,
    ) -> Result<AppContext, Error> {
        Ok(AppContext { cancel })
    }

    async fn run(&self, _ctx: &AppContext) -> Outcome {
        // `Bootstrap` already validated the command line; parsing again is side-effect free.
        let (mut out, mut err) = (std::io::stdout(), std::io::stderr());
        let mut presenter = Presenter::new(&mut out, &mut err);
        let result = match self.parse_args() {
            Ok(args) => dispatch(args, &mut presenter),
            Err(err) => Err(Error::Internal(err.to_string())),
        };
        result.unwrap_or_else(Outcome::Failed)
    }

    fn report_failure(&self, failure: &dyn std::fmt::Display) {
        Presenter::new(&mut std::io::stdout(), &mut std::io::stderr()).error(self.name(), failure);
    }

    async fn shutdown(&self, _ctx: &AppContext, _reason: ShutdownReason) {}
}

/// The project root: the directory the command is run from.
fn project_root() -> Result<std::path::PathBuf, Error> {
    std::env::current_dir().map_err(|e| Error::Internal(e.to_string()))
}

/// Opens the project in the current directory, and shows what its configuration warns about
/// before the command does anything.
fn open(out: &mut Presenter<'_>) -> Result<Workspace, Error> {
    let opened = Workspace::open(&project_root()?)?;
    for warning in &opened.warnings {
        out.warn(warning)?;
    }
    Ok(opened.configuration)
}

/// Runs the command line. Only `check` can run to the end and still fail the run.
fn dispatch(args: CliArgs, out: &mut Presenter<'_>) -> Result<Outcome, Error> {
    let succeeded = |()| Outcome::Success;
    match args {
        // `clap` ends its text with the line break that `log` adds.
        CliArgs::Display(text) => out
            .log(text.strip_suffix('\n').unwrap_or(&text))
            .map(succeeded),
        CliArgs::Run(Command::Check(args)) => check::run(&args, out),
        CliArgs::Run(Command::Init(args)) => init::run(&args, out).map(succeeded),
        CliArgs::Run(Command::Where(args)) => where_cmd::run(&args, out).map(succeeded),
        CliArgs::Run(Command::Rules { action }) => rules(action, out).map(succeeded),
    }
}

/// One line per `rules` command: the handlers hold the logic.
fn rules(action: RulesAction, out: &mut Presenter<'_>) -> Result<(), Error> {
    match action {
        RulesAction::Add(args) => rules_add::run(&args, out),
        RulesAction::Approve(args) => rules_lifecycle::approve(&args, out),
        RulesAction::Reject(args) => rules_lifecycle::reject(&args, out),
        RulesAction::Obsolete(args) => rules_lifecycle::obsolete(&args, out),
        RulesAction::Format => rules_format::run(out),
        RulesAction::List(args) => rules_query::list(&args, out),
        RulesAction::Search(args) => rules_query::search(&args, out),
    }
}

fn parse(args: impl IntoIterator<Item = String>) -> Result<CliArgs, UsageError> {
    match Cli::try_parse_from(args) {
        Ok(cli) => Ok(CliArgs::Run(cli.command)),
        Err(err)
            if matches!(
                err.kind(),
                ErrorKind::DisplayHelp | ErrorKind::DisplayVersion
            ) =>
        {
            Ok(CliArgs::Display(err.to_string()))
        }
        Err(err) => Err(UsageError(err.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(list: &[&str]) -> Vec<String> {
        std::iter::once("rulelink")
            .chain(list.iter().copied())
            .map(str::to_owned)
            .collect()
    }

    #[test]
    fn init_uses_default_prefix() {
        let parsed = parse(args(&["init"])).unwrap_or_else(|e| panic!("{e}"));
        let CliArgs::Run(Command::Init(init)) = parsed else {
            panic!("expected init");
        };
        assert_eq!(init.id_prefix, rulelink_app::DEFAULT_ID_PREFIX);
    }

    #[test]
    fn init_accepts_custom_prefix() {
        let parsed = parse(args(&["init", "--id-prefix", "BR"])).unwrap_or_else(|e| panic!("{e}"));
        let CliArgs::Run(Command::Init(init)) = parsed else {
            panic!("expected init");
        };
        assert_eq!(init.id_prefix, "BR");
    }

    #[test]
    fn rules_add_collects_repeated_options() {
        let parsed = parse(args(&[
            "rules",
            "add",
            "--title",
            "T",
            "--statement",
            "S",
            "--app",
            "a",
            "--app",
            "b",
        ]))
        .unwrap_or_else(|e| panic!("{e}"));
        let CliArgs::Run(Command::Rules {
            action: RulesAction::Add(add),
        }) = parsed
        else {
            panic!("expected rules add");
        };
        assert_eq!(add.applications, ["a", "b"]);
        assert_eq!(add.module, None);
    }

    #[test]
    fn rules_list_and_search_parse_the_status() {
        let parsed = parse(args(&["rules", "search", "tva", "--status", "validated"]))
            .unwrap_or_else(|e| panic!("{e}"));
        let CliArgs::Run(Command::Rules {
            action: RulesAction::Search(search),
        }) = parsed
        else {
            panic!("expected rules search");
        };
        assert_eq!(
            (search.text.as_str(), search.status),
            ("tva", Some(rulelink_app::RuleStatus::Validated))
        );
        assert!(parse(args(&["rules", "list", "--status", "draft"])).is_err());
        assert!(parse(args(&["rules", "search"])).is_err());
    }

    #[test]
    fn rules_add_requires_title_and_statement() {
        assert!(parse(args(&["rules", "add", "--title", "T"])).is_err());
        assert!(parse(args(&["rules", "add", "--statement", "S"])).is_err());
    }

    #[test]
    fn a_displayed_text_ends_with_a_single_line_break() {
        let (mut out, mut err) = (Vec::new(), Vec::new());
        let text = CliArgs::Display("usage\n\nmore\n".to_owned());
        dispatch(text, &mut Presenter::new(&mut out, &mut err)).unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(String::from_utf8_lossy(&out), "usage\n\nmore\n");
    }

    #[test]
    fn version_is_displayed_not_run() {
        assert!(matches!(
            parse(args(&["--version"])),
            Ok(CliArgs::Display(text)) if text.starts_with("rulelink ")
        ));
    }

    #[test]
    fn unknown_argument_is_a_usage_error() {
        assert!(parse(args(&["init", "--nope"])).is_err());
        assert!(parse(args(&["bogus"])).is_err());
    }

    #[test]
    fn no_argument_is_a_usage_error() {
        assert!(parse(args(&[])).is_err());
    }
}
