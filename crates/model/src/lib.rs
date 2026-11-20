//! Core domain model of Rulelink: rules, identifiers, statuses and links.

mod draft;
mod error;
mod id_prefix;
mod rule;
mod ruleset;

pub use draft::{RuleDraft, TITLE_MAX_CHARS};
pub use error::{BlankRuleId, DraftError, Error, InvalidIdPrefix, LifecycleError};
pub use id_prefix::{DEFAULT_ID_PREFIX, IdPrefix};
pub use rule::{Origin, Rule, RuleId, RuleStatus, Scope};
pub use ruleset::{FORMAT_VERSION, RuleSet};
