//! Command groups of the CLI. No business logic here: arguments become application commands.

use checkmate_app::runtime::{
    AppContext, AppError, CancellationToken, Entrypoint, Outcome, ShutdownReason, TelemetryConfig,
    UsageError,
};

/// Parsed command line.
#[derive(Debug)]
pub enum CliArgs {
    /// `--version`
    Version,
    /// `--help` or no argument.
    Help,
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
        match std::env::args().nth(1).as_deref() {
            None | Some("--help" | "-h") => Ok(CliArgs::Help),
            Some("--version" | "-V") => Ok(CliArgs::Version),
            Some(other) => Err(UsageError(format!("unknown argument `{other}`"))),
        }
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
        match self.parse_args() {
            Ok(CliArgs::Version) => println!("checkmate {}", env!("CARGO_PKG_VERSION")),
            _ => println!(
                "checkmate {}\nUsage: checkmate [--version | --help]",
                env!("CARGO_PKG_VERSION")
            ),
        }
        Outcome::Success
    }

    async fn shutdown(&self, _ctx: &AppContext, _reason: ShutdownReason) {}
}
