//! Application layer: use cases, wiring and process runtime skeleton.

mod error;
pub mod init;
pub mod rules;
pub mod runtime;

pub use error::Error;

pub use rulelink_config::DEFAULT_ID_PREFIX;
pub use rulelink_model::{Rule, RuleDraft, RuleId, RuleStatus, SearchText};
