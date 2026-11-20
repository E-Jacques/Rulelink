/// Errors raised by `rulelink-lsp`.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The requested capability is not implemented yet.
    #[error("not implemented: {0}")]
    NotImplemented(&'static str),
}
