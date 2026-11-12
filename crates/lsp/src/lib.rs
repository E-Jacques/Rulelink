//! Language server loop, mounted by the CLI as the `lsp` subcommand.
#![allow(clippy::unused_async, clippy::unused_async_trait_impl)] // skeleton: hooks become truly async later

mod error;
mod server;

pub use error::Error;
pub use server::LspEntrypoint;
