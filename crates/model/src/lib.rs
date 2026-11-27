//! Core domain model of Rulelink: rules, identifiers, statuses and links.

mod draft;
mod error;
mod id_prefix;
mod lifecycle;
mod rule;
mod ruleset;
mod search;
mod sequences;

pub use draft::{RuleDraft, TITLE_MAX_CHARS};
pub use error::{
    AddError, BlankRuleId, BlankSearchText, DraftError, Error, InvalidIdPrefix, LifecycleError,
    UnknownStatus,
};
pub use id_prefix::{DEFAULT_ID_PREFIX, IdPrefix};
pub use rule::{Origin, Rule, RuleId, RuleStatus, Scope};
pub use ruleset::{FORMAT_VERSION, RuleSet};
pub use search::SearchText;
