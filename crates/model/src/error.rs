/// The Identifier prefix is malformed.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error(
    "invalid Identifier prefix `{0}`: use upper-case letters, digits and `_`, starting with a letter"
)]
pub struct InvalidIdPrefix(pub String);

/// An Identifier given on the command line is blank.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{field} must not be empty or blank")]
pub struct BlankRuleId {
    /// Name of the offending value.
    pub field: &'static str,
}

/// A value given for a new Rule is refused.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DraftError {
    /// A required text value is empty once trimmed.
    #[error("{field} must not be empty or blank")]
    Empty {
        /// Name of the offending value.
        field: &'static str,
    },
    /// The title is longer than allowed.
    #[error("title is too long: {len} characters, at most {max} are allowed")]
    TitleTooLong {
        /// Length of the stored title, in characters.
        len: usize,
        /// Maximum length.
        max: usize,
    },
    /// A list value appears twice.
    #[error("{field} lists `{value}` more than once")]
    Duplicate {
        /// Name of the offending value.
        field: &'static str,
        /// The repeated value.
        value: String,
    },
}

use crate::{RuleId, RuleStatus};

/// A lifecycle command is refused. Every variant names the value or Rule at fault.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LifecycleError {
    /// No Rule has this Identifier.
    #[error("no Rule has the Identifier `{0}`: run `rulelink rules list` to see the Identifiers")]
    RuleNotFound(RuleId),
    /// The Rule is not in the status the transition starts from.
    #[error("cannot {action} {id}: it is {status}, but a {required} Rule is required")]
    TransitionRefused {
        /// The Rule.
        id: RuleId,
        /// Its current status.
        status: RuleStatus,
        /// The only status the transition accepts.
        required: RuleStatus,
        /// The refused action (`approve`, `reject`, `obsolete`).
        action: &'static str,
    },
    /// The replacement Rule does not exist.
    #[error(
        "cannot record `{0}` as the replacement (--superseded-by): no Rule has this Identifier; run `rulelink rules list` to see the Identifiers"
    )]
    ReplacementNotFound(RuleId),
    /// A Rule cannot replace itself.
    #[error("{0} cannot be its own replacement")]
    SelfReplacement(RuleId),
}

/// Errors raised by `rulelink-model` outside input validation.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// The requested capability is not implemented yet.
    #[error("not implemented: {0}")]
    NotImplemented(&'static str),
}

/// A new Rule cannot be added to the Source.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum AddError {
    /// No more Identifiers can be allocated for the prefix: the Source is full.
    #[error("no Identifier is left for prefix `{0}`")]
    IdentifierSpaceExhausted(String),
    /// A `--related` Identifier names no Rule of the Source.
    #[error(
        "cannot record `{0}` as a related Rule (--related): no Rule has this Identifier; run `rulelink rules list` to see the Identifiers"
    )]
    UnknownRelated(RuleId),
}

/// A status given by the user is not one of the four.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("unknown status `{value}`: use proposed, validated, obsolete or rejected")]
pub struct UnknownStatus {
    /// The value as typed.
    pub value: String,
}

/// The text to search for is empty once trimmed.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("the search text must not be empty or blank")]
pub struct BlankSearchText;
