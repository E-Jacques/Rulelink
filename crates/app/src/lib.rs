//! Application layer: use cases, wiring and process runtime skeleton.

pub mod checks;
mod error;
pub mod init;
pub mod links;
pub mod rules;
pub mod runtime;
mod workspace;

pub use error::Error;
pub use workspace::Workspace;

pub use rulelink_adapter_json::Rewrite;
pub use rulelink_adapter_sdk::{Parsed, Warning};
pub use rulelink_check::{Finding, Severity};
pub use rulelink_config::DEFAULT_ID_PREFIX;
pub use rulelink_link::Location;
pub use rulelink_model::{Rule, RuleDraft, RuleId, RuleStatus, SearchText};
