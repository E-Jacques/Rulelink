use std::str::FromStr;

use rulelink_link::Location;
use rulelink_model::{RuleId, RuleStatus};

use crate::{Severity, UnknownFinding};

/// The kinds of Finding, each with a code (`RL001`) and the name that sets its level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Code {
    /// `RL001`: the code cites an Identifier that no Rule has.
    UnknownReference,
    /// `RL002`: the code cites an `obsolete` Rule.
    ObsoleteReference,
    /// `RL003`: several Rules share an Identifier.
    DuplicateId,
    /// `RL004`: no code cites a `validated` Rule.
    OrphanRule,
    /// `RL005`: the code cites a `proposed` Rule.
    UnvalidatedReference,
    /// `RL009`: the code cites a `rejected` Rule or a Retired identifier that has no Rule.
    RetiredReference,
    /// `RL100`: the Source is JSON of the wrong shape.
    InvalidShape,
    /// `RL101`: an Identifier in the Source does not follow the grammar.
    InvalidIdentifier,
    /// `RL102`: a `proposed` or `validated` Rule has a Retired identifier.
    ReusedRetiredId,
    /// `RL103`: `related` names a Rule that is not in the Source.
    UnknownRelated,
    /// `RL105`: a required field of a Rule is empty.
    EmptyField,
    /// `RL106`: the Source is not in Canonical form.
    NotCanonical,
}

impl Code {
    /// Every kind, in code order.
    pub(crate) const ALL: [Self; 12] = [
        Self::UnknownReference,
        Self::ObsoleteReference,
        Self::DuplicateId,
        Self::OrphanRule,
        Self::UnvalidatedReference,
        Self::RetiredReference,
        Self::InvalidShape,
        Self::InvalidIdentifier,
        Self::ReusedRetiredId,
        Self::UnknownRelated,
        Self::EmptyField,
        Self::NotCanonical,
    ];

    /// The code shown in reports, such as `RL001`.
    #[must_use]
    pub fn id(self) -> &'static str {
        match self {
            Self::UnknownReference => "RL001",
            Self::ObsoleteReference => "RL002",
            Self::DuplicateId => "RL003",
            Self::OrphanRule => "RL004",
            Self::UnvalidatedReference => "RL005",
            Self::RetiredReference => "RL009",
            Self::InvalidShape => "RL100",
            Self::InvalidIdentifier => "RL101",
            Self::ReusedRetiredId => "RL102",
            Self::UnknownRelated => "RL103",
            Self::EmptyField => "RL105",
            Self::NotCanonical => "RL106",
        }
    }

    /// The name a team uses to set the level, such as `unknown-reference`.
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::UnknownReference => "unknown-reference",
            Self::ObsoleteReference => "obsolete-reference",
            Self::DuplicateId => "duplicate-id",
            Self::OrphanRule => "orphan-rule",
            Self::UnvalidatedReference => "unvalidated-reference",
            Self::RetiredReference => "retired-reference",
            Self::InvalidShape => "invalid-shape",
            Self::InvalidIdentifier => "invalid-identifier",
            Self::ReusedRetiredId => "reused-retired-id",
            Self::UnknownRelated => "unknown-related",
            Self::EmptyField => "empty-field",
            Self::NotCanonical => "not-canonical",
        }
    }

    /// Whether the Rules cannot be trusted while such a Finding stands. It is then always an
    /// error, whatever a team sets, and the code is not checked.
    pub(crate) fn is_source_error(self) -> bool {
        match self {
            Self::DuplicateId
            | Self::InvalidShape
            | Self::InvalidIdentifier
            | Self::ReusedRetiredId
            | Self::EmptyField => true,
            Self::UnknownReference
            | Self::ObsoleteReference
            | Self::OrphanRule
            | Self::UnvalidatedReference
            | Self::RetiredReference
            | Self::UnknownRelated
            | Self::NotCanonical => false,
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

/// Where the Source holds an Identifier, with the Rule that holds it if it has one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentifierAt {
    /// The `id` of a Rule.
    Id,
    /// The `retired` list.
    Retired,
    /// The `related` list of a Rule.
    Related(Option<RuleId>),
    /// The `superseded_by` of a Rule.
    SupersededBy(Option<RuleId>),
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
    /// Several Rules have the Identifier.
    DuplicateId {
        /// How many.
        rules: usize,
    },
    /// The Source is JSON of the wrong shape.
    WrongShape {
        /// What does not fit.
        detail: String,
    },
    /// An Identifier does not follow the grammar.
    InvalidIdentifier {
        /// The Identifier as written.
        value: String,
        /// Where it is written.
        at: IdentifierAt,
    },
    /// The Rule is in use, but its Identifier is retired.
    ReusedRetiredId {
        /// The status of the Rule.
        status: RuleStatus,
    },
    /// `related` names a Rule that is not in the Source.
    UnknownRelated {
        /// The Identifier no Rule has.
        related: RuleId,
    },
    /// A required field holds nothing.
    EmptyField {
        /// Its key in the Source.
        field: &'static str,
        /// The title of the Rule, if it has one: it identifies a Rule with no Identifier.
        title: Option<String>,
    },
    /// The Source is not written in Canonical form.
    NotCanonical,
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
            Self::DuplicateId { .. } => Code::DuplicateId,
            Self::WrongShape { .. } => Code::InvalidShape,
            Self::InvalidIdentifier { .. } => Code::InvalidIdentifier,
            Self::ReusedRetiredId { .. } => Code::ReusedRetiredId,
            Self::UnknownRelated { .. } => Code::UnknownRelated,
            Self::EmptyField { .. } => Code::EmptyField,
            Self::NotCanonical => Code::NotCanonical,
        }
    }
}

/// Where a Finding points.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Place {
    /// The Annotation at fault.
    Code(Location),
    /// The Source: no line of code is at fault.
    RulesFile {
        /// The line at fault, from 1, when it is known.
        line: Option<u32>,
    },
}

/// One result of a check, about a Rule, an Annotation or the Source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    /// How much it weighs, after the team's levels.
    pub severity: Severity,
    /// The Identifier it is about; `None` if there is none to name.
    pub rule: Option<RuleId>,
    /// What is wrong.
    pub problem: Problem,
    /// Where it points.
    pub place: Place,
}

impl Finding {
    /// One sentence saying what is wrong, without severity, code or location.
    #[must_use]
    pub fn message(&self) -> String {
        let id = subject(self.rule.as_ref());
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
            Problem::DuplicateId { rules } => {
                format!("{id} is the Identifier of {rules} Rules")
            }
            Problem::WrongShape { detail } => {
                format!("the rules file has the wrong shape: {detail}")
            }
            Problem::InvalidIdentifier { value, at } => invalid_identifier(value, at),
            Problem::ReusedRetiredId { status } => {
                format!("{id} is retired but its Rule is {status}")
            }
            Problem::UnknownRelated { related } => {
                format!("{id} is related to {related}, which is not a Rule of this project")
            }
            Problem::EmptyField { field, title } => match (&self.rule, title) {
                (None, Some(title)) => format!("the Rule titled `{title}` has an empty `{field}`"),
                _ => format!("{id} has an empty `{field}`"),
            },
            Problem::NotCanonical => {
                "the rules file is not in Canonical form: run `rulelink rules format`".to_owned()
            }
        }
    }
}

/// How a message names the Rule it is about.
fn subject(rule: Option<&RuleId>) -> String {
    rule.map_or_else(|| "a Rule with no Identifier".to_owned(), RuleId::to_string)
}

fn invalid_identifier(value: &str, at: &IdentifierAt) -> String {
    let place = match at {
        IdentifierAt::Id => "`id`".to_owned(),
        IdentifierAt::Retired => "`retired`".to_owned(),
        IdentifierAt::Related(holder) => format!("`related` of {}", subject(holder.as_ref())),
        IdentifierAt::SupersededBy(holder) => {
            format!("`superseded_by` of {}", subject(holder.as_ref()))
        }
    };
    format!("`{value}`, in {place}, is not a valid Identifier")
}
