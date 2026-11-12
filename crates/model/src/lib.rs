//! Core domain model of Checkmate: rules, identifiers, statuses and links.

mod error;
mod rule;

pub use error::Error;
pub use rule::{Rule, RuleId, RuleStatus};
