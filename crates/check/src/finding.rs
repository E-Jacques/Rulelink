use std::str::FromStr;

use rulelink_link::Location;
use rulelink_model::RuleId;

use crate::{Level, Severity, UnknownFinding};

/// The kinds of Finding, each with a code (`RL001`) and the name that sets its level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Code {
    /// `RL001`: the code cites an Identifier that no Rule has.
    UnknownReference,
    /// `RL002`: the code cites an `obsolete` Rule.
    ObsoleteReference,
    /// `RL004`: no code cites a `validated` Rule.
    OrphanRule,
    /// `RL005`: the code cites a `proposed` Rule.
    UnvalidatedReference,
    /// `RL009`: the code cites a `rejected` Rule or a Retired identifier that has no Rule.
    RetiredReference,
}

impl Code {
    /// Every kind, in code order.
    pub(crate) const ALL: [Self; 5] = [
        Self::UnknownReference,
        Self::ObsoleteReference,
        Self::OrphanRule,
        Self::UnvalidatedReference,
        Self::RetiredReference,
    ];

    /// The code shown in reports, such as `RL001`.
    #[must_use]
    pub fn id(self) -> &'static str {
        match self {
            Self::UnknownReference => "RL001",
            Self::ObsoleteReference => "RL002",
            Self::OrphanRule => "RL004",
            Self::UnvalidatedReference => "RL005",
            Self::RetiredReference => "RL009",
        }
    }

    /// The name a team uses to set the level, such as `unknown-reference`.
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::UnknownReference => "unknown-reference",
            Self::ObsoleteReference => "obsolete-reference",
            Self::OrphanRule => "orphan-rule",
            Self::UnvalidatedReference => "unvalidated-reference",
            Self::RetiredReference => "retired-reference",
        }
    }

    pub(crate) fn default_level(self, no_rules: bool) -> Level {
        match self {
            // Annotations written before the first Rule must not fail a build.
            Self::UnknownReference if no_rules => Level::Warning,
            Self::UnknownReference | Self::ObsoleteReference | Self::RetiredReference => {
                Level::Error
            }
            Self::OrphanRule | Self::UnvalidatedReference => Level::Warning,
        }
    }
}

impl FromStr for Code {
    type Err = UnknownFinding;

    /// Reads the name of a Finding, such as `unknown-reference`. A code such as `RL001` is not a name.
    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|code| code.name() == raw)
            .ok_or_else(|| UnknownFinding {
                value: raw.to_owned(),
            })
    }
}

/// What is wrong, with what is needed to say it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Problem {
    /// No Rule has the cited Identifier, and it is not retired.
    Unknown,
    /// The cited Rule is `obsolete`.
    Obsolete {
        /// The Rule that replaces it, as the Source records it.
        superseded_by: Option<RuleId>,
    },
    /// The Rule is `validated` and nothing cites it.
    Orphan,
    /// The cited Rule is `proposed`.
    Unvalidated,
    /// The cited Rule is `rejected`.
    Rejected,
    /// The cited Identifier is retired and its Rule is no longer in the Source.
    Retired,
}

impl Problem {
    /// The kind of Finding this problem is.
    #[must_use]
    pub fn code(&self) -> Code {
        match self {
            Self::Unknown => Code::UnknownReference,
            Self::Obsolete { .. } => Code::ObsoleteReference,
            Self::Orphan => Code::OrphanRule,
            Self::Unvalidated => Code::UnvalidatedReference,
            Self::Rejected | Self::Retired => Code::RetiredReference,
        }
    }
}

/// One result of a check, about a Rule or an Annotation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    /// How much it weighs, after the team's levels.
    pub severity: Severity,
    /// The Identifier it is about.
    pub rule: RuleId,
    /// What is wrong.
    pub problem: Problem,
    /// The Annotation at fault; `None` for a Finding about a Rule that no code cites.
    pub location: Option<Location>,
}

impl Finding {
    /// One sentence saying what is wrong, without severity, code or location.
    #[must_use]
    pub fn message(&self) -> String {
        let id = &self.rule;
        match &self.problem {
            Problem::Unknown => format!("{id} is not a Rule of this project"),
            Problem::Obsolete {
                superseded_by: Some(replacement),
            } => format!("{id} is obsolete, superseded by {replacement}"),
            Problem::Obsolete {
                superseded_by: None,
            } => format!("{id} is obsolete"),
            Problem::Orphan => format!("{id} is validated but no code cites it"),
            Problem::Unvalidated => format!("{id} is still proposed"),
            Problem::Rejected => format!("{id} is rejected: its Identifier is retired"),
            Problem::Retired => format!("{id} is a Retired identifier"),
        }
    }
}
