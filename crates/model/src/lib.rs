//! Core domain model of Checkmate: rules, identifiers, statuses and links.

mod error;
mod rule;
mod ruleset;

pub use error::Error;
pub use rule::{Origin, Rule, RuleId, RuleStatus, Scope};
pub use ruleset::{FORMAT_VERSION, RuleSet};
