/// Application-level failure.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Invalid input: a value the user supplied is refused.
    #[error("invalid input: {0}")]
    Input(String),
    /// Invalid configuration.
    #[error("configuration error: {0}")]
    Config(String),
    /// Source unreadable, invalid or not found.
    #[error("source error: {0}")]
    Source(String),
    /// Unexpected internal error.
    #[error("internal error: {0}")]
    Internal(String),
}

impl Error {
    /// Exit code of the error (docs/architecture/00, section 9.2).
    #[must_use]
    pub fn exit_code(&self) -> u8 {
        match self {
            Self::Input(_) | Self::Config(_) => 2,
            Self::Source(_) => 3,
            Self::Internal(_) => 10,
        }
    }
}

/// Every variant is a refused user value.
impl From<rulelink_model::DraftError> for Error {
    fn from(err: rulelink_model::DraftError) -> Self {
        Self::Input(err.to_string())
    }
}

/// Every variant is a Rule or replacement the user named, or a transition they asked for.
impl From<rulelink_model::LifecycleError> for Error {
    fn from(err: rulelink_model::LifecycleError) -> Self {
        Self::Input(err.to_string())
    }
}

impl From<rulelink_config::Error> for Error {
    fn from(err: rulelink_config::Error) -> Self {
        use rulelink_config::Error as E;
        match err {
            E::InvalidIdPrefix(_) | E::NotFound(_) | E::Unreadable { .. } | E::Invalid { .. } => {
                Self::Config(err.to_string())
            }
        }
    }
}

impl From<rulelink_adapter_json::Error> for Error {
    fn from(err: rulelink_adapter_json::Error) -> Self {
        use rulelink_adapter_json::Error as E;
        match err {
            E::NotFound(_)
            | E::Invalid { .. }
            | E::UnsupportedFormat { .. }
            | E::Unreadable { .. } => Self::Source(err.to_string()),
            E::AlreadyExists | E::Io(_) => Self::Internal(err.to_string()),
        }
    }
}

impl From<rulelink_model::Error> for Error {
    fn from(err: rulelink_model::Error) -> Self {
        use rulelink_model::Error as E;
        match err {
            // The Source holds the highest number: it is the Source that is full.
            E::IdentifierSpaceExhausted(_) => Self::Source(err.to_string()),
            E::NotImplemented(_) => Self::Internal(err.to_string()),
        }
    }
}
