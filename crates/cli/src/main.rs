//! The only `main()` of the product (ADR-004).

#![allow(clippy::unused_async, clippy::unused_async_trait_impl)] // skeleton: hooks become truly async later

mod commands;

use rulelink_app::runtime::Bootstrap;

#[tokio::main(flavor = "current_thread")]
async fn main() -> std::process::ExitCode {
    let sub = std::env::args().nth(1);
    let code = match sub.as_deref() {
        #[cfg(feature = "lsp")]
        Some("lsp") => Bootstrap::execute(&rulelink_lsp::LspEntrypoint).await,
        _ => Bootstrap::execute(&commands::CliEntrypoint).await,
    };
    std::process::ExitCode::from(code)
}
