use rulelink_app::Error;
use rulelink_app::runtime::{
    AppContext, CancellationToken, Entrypoint, Outcome, ShutdownReason, TelemetryConfig, UsageError,
};

/// `Entrypoint` of the `rulelink lsp` subcommand.
#[derive(Debug, Default)]
pub struct LspEntrypoint;

impl Entrypoint for LspEntrypoint {
    type Args = ();

    fn name(&self) -> &'static str {
        "rulelink-lsp"
    }

    fn parse_args(&self) -> Result<Self::Args, UsageError> {
        Ok(())
    }

    fn telemetry(&self, (): &Self::Args) -> TelemetryConfig {
        TelemetryConfig::default()
    }

    fn build_context(
        &self,
        (): &Self::Args,
        cancel: CancellationToken,
    ) -> Result<AppContext, Error> {
        Ok(AppContext { cancel })
    }

    async fn run(&self, _ctx: &AppContext) -> Outcome {
        Outcome::Failed(Error::Internal(
            "language server not implemented (V1)".into(),
        ))
    }

    async fn shutdown(&self, _ctx: &AppContext, _reason: ShutdownReason) {}
}
