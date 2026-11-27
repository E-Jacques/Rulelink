//! Process skeleton shared by every entry point (`Entrypoint`, `Bootstrap`).
//!
//! See `docs/architecture/10-runtime-lifecycle.md`. Libraries never create a
//! process: `main()` lives in `rulelink-cli` only.

use std::future::Future;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::Error;

/// Cooperative cancellation flag shared between the signal handlers and the run loop.
#[derive(Debug, Clone, Default)]
pub struct CancellationToken(Arc<AtomicBool>);

impl CancellationToken {
    /// Requests cancellation.
    pub fn cancel(&self) {
        self.0.store(true, Ordering::SeqCst);
    }

    /// Whether cancellation was requested.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }
}

/// Invalid command line usage.
#[derive(Debug, thiserror::Error)]
#[error("usage error: {0}")]
pub struct UsageError(pub String);

/// Tracing configuration requested by an entry point.
#[derive(Debug, Clone, Copy, Default)]
pub struct TelemetryConfig {
    /// Verbose logging.
    pub verbose: bool,
}

/// Dependencies shared by a run, built by the entry point.
#[derive(Debug, Clone, Default)]
pub struct AppContext {
    /// Cancellation token bound to the signal handlers.
    pub cancel: CancellationToken,
}

/// Result of a run.
#[derive(Debug)]
pub enum Outcome {
    /// Success, no blocking finding.
    Success,
    /// At least one error-level finding.
    ChecksFailed,
    /// The run failed.
    Failed(Error),
}

/// Why the process is shutting down.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShutdownReason {
    /// The run ended normally.
    Completed,
    /// SIGINT.
    Interrupted,
    /// SIGTERM.
    Terminated,
    /// The parent process exited.
    ParentExited,
    /// The client asked for it (LSP `exit`).
    ClientRequested,
}

/// Fixed skeleton of a process; each entry point implements only its hooks (Template Method).
pub trait Entrypoint {
    /// Parsed arguments.
    type Args;
    /// Name used in logs.
    fn name(&self) -> &'static str;
    /// Parses the command line.
    ///
    /// # Errors
    /// Returns [`UsageError`] on invalid usage.
    fn parse_args(&self) -> Result<Self::Args, UsageError>;
    /// Tracing configuration.
    fn telemetry(&self, args: &Self::Args) -> TelemetryConfig;
    /// Builds the shared context.
    ///
    /// # Errors
    /// Returns [`Error`] when discovery or configuration fails.
    fn build_context(
        &self,
        args: &Self::Args,
        cancel: CancellationToken,
    ) -> Result<AppContext, Error>;
    /// Runs the entry point.
    fn run(&self, ctx: &AppContext) -> impl Future<Output = Outcome>;
    /// Tells the user why the run failed, the way this surface shows errors: the shared
    /// skeleton writes nothing itself.
    fn report_failure(&self, failure: &dyn std::fmt::Display);
    /// Releases resources.
    fn shutdown(&self, ctx: &AppContext, reason: ShutdownReason) -> impl Future<Output = ()>;
}

/// Drives an [`Entrypoint`] through the fixed steps and maps the outcome to an exit code.
#[derive(Debug, Clone, Copy, Default)]
pub struct Bootstrap;

impl Bootstrap {
    /// Executes the entry point and returns the process exit code.
    ///
    /// Panic hook, tracing and signal handlers are installed by the binary crate
    /// for now (TODO: move them here once a runtime dependency is accepted).
    pub async fn execute<E: Entrypoint>(entry: &E) -> u8 {
        let args = match entry.parse_args() {
            Ok(args) => args,
            Err(err) => {
                entry.report_failure(&err);
                return 2;
            }
        };
        let _telemetry = entry.telemetry(&args);
        let cancel = CancellationToken::default();
        let ctx = match entry.build_context(&args, cancel) {
            Ok(ctx) => ctx,
            Err(err) => {
                entry.report_failure(&err);
                return err.exit_code();
            }
        };
        let outcome = entry.run(&ctx).await;
        entry.shutdown(&ctx, ShutdownReason::Completed).await;
        match outcome {
            Outcome::Success => 0,
            Outcome::ChecksFailed => 1,
            Outcome::Failed(err) => {
                entry.report_failure(&err);
                err.exit_code()
            }
        }
    }
}
