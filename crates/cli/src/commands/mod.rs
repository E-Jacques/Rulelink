//! Command groups of the CLI. No business logic here: arguments become application commands.
//!
//! Adding a command: add a variant to [`Command`] with its own `clap::Args` struct, a handler
//! function returning `Result<(), AppError>`, and one line in [`dispatch`].

use clap::error::ErrorKind;
use clap::{Parser, Subcommand};

mod init;
mod rules_add;

use checkmate_app::runtime::{
    AppContext, AppError, CancellationToken, Entrypoint, Outcome, ShutdownReason, TelemetryConfig,
    UsageError,
};
use init::InitArgs;
use rules_add::RulesAddArgs;

/// Command line grammar, derived by `clap`.
#[derive(Debug, Parser)]
#[command(name = "checkmate", version, about, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

/// Regular CLI commands (`lsp` is routed by `main` before this grammar is used).
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Create `checkmate.toml` and an empty rules file in the current directory.
    Init(InitArgs),
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
        "checkmate"
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
    ) -> Result<AppContext, AppError> {
        Ok(AppContext { cancel })
    }

    async fn run(&self, _ctx: &AppContext) -> Outcome {
        // `Bootstrap` already validated the command line; parsing again is side-effect free.
        let result = match self.parse_args() {
            Ok(args) => dispatch(args),
            Err(err) => Err(AppError::Internal(err.to_string())),
        };
        match result {
            Ok(()) => Outcome::Success,
            Err(err) => Outcome::Failed(err),
        }
    }

    async fn shutdown(&self, _ctx: &AppContext, _reason: ShutdownReason) {}
}

/// One line per command: the handlers hold the logic.
fn dispatch(args: CliArgs) -> Result<(), AppError> {
    match args {
        CliArgs::Display(text) => {
            print!("{text}");
            Ok(())
        }
        CliArgs::Run(Command::Init(args)) => init::run(&args),
        CliArgs::Run(Command::Rules {
            action: RulesAction::Add(args),
        }) => rules_add::run(&args),
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
        std::iter::once("checkmate")
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
        assert_eq!(init.id_prefix, checkmate_app::DEFAULT_ID_PREFIX);
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
    fn rules_add_requires_title_and_statement() {
        assert!(parse(args(&["rules", "add", "--title", "T"])).is_err());
        assert!(parse(args(&["rules", "add", "--statement", "S"])).is_err());
    }

    #[test]
    fn version_is_displayed_not_run() {
        assert!(matches!(
            parse(args(&["--version"])),
            Ok(CliArgs::Display(text)) if text.starts_with("checkmate ")
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
