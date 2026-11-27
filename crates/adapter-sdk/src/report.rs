/// What a successful parse returns: the result and what to warn about.
///
/// ```
/// use rulelink_adapter_sdk::Parsed;
///
/// let parsed = Parsed::new(1);
/// assert!(parsed.warnings.is_empty());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parsed<T> {
    /// What was parsed.
    pub configuration: T,
    /// What the user should know, although nothing is refused.
    pub warnings: Vec<Warning>,
}

impl<T> Parsed<T> {
    /// A result with no warning.
    #[must_use]
    pub fn new(configuration: T) -> Self {
        Self {
            configuration,
            warnings: Vec::new(),
        }
    }
}

/// The result of a parse. A failed one carries no warning.
pub type ParseResult<T> = Result<Parsed<T>, ParseError>;

/// A setting that is accepted but deserves attention.
///
/// ```
/// use rulelink_adapter_sdk::Warning;
///
/// let warning = Warning::new("a.b", "`a.b` is deprecated");
/// assert_eq!(warning.to_string(), "`a.b` is deprecated");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{message}")]
pub struct Warning {
    /// The setting, as written from the top of the file.
    pub key: String,
    /// What to know and what to do. It names the setting.
    pub message: String,
}

impl Warning {
    /// A warning about `key`. Control characters of `message` are escaped: text taken from
    /// the file cannot add lines to the output.
    #[must_use]
    pub fn new(key: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            message: on_one_line(&message.into()),
        }
    }
}

fn on_one_line(text: &str) -> String {
    let mut line = String::with_capacity(text.len());
    for c in text.chars() {
        if c.is_control() {
            line.extend(c.escape_default());
        } else {
            line.push(c);
        }
    }
    line
}

/// One reason to refuse the settings.
///
/// ```
/// use rulelink_adapter_sdk::Problem;
///
/// let problem = Problem::new("version", "missing `version`");
/// assert_eq!(problem.to_string(), "missing `version`");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{message}")]
pub struct Problem {
    /// The setting, as written from the top of the file; empty if none is at fault.
    pub key: String,
    /// What is wrong and how to fix it. It names the setting.
    pub message: String,
}

impl Problem {
    /// A problem with `key`. Control characters of `message` are escaped: text taken from
    /// the file cannot add lines to the output.
    #[must_use]
    pub fn new(key: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            message: on_one_line(&message.into()),
        }
    }

    /// A problem with the whole file, whose `message` keeps its lines. Only for a parser's
    /// own report, which quotes the line at fault and marks it.
    #[must_use]
    pub fn verbatim(message: impl Into<String>) -> Self {
        Self {
            key: String::new(),
            message: message.into(),
        }
    }
}

/// Why settings are refused: at least one [`Problem`].
///
/// Displayed as the problem alone, or as an indented list when there are several.
///
/// ```
/// use rulelink_adapter_sdk::{ParseError, Problem};
///
/// let error = ParseError::new(Problem::new("a", "missing `a`"));
/// assert_eq!(error.problems().count(), 1);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    first: Problem,
    rest: Vec<Problem>,
}

impl ParseError {
    /// An error for one problem.
    #[must_use]
    pub fn new(first: Problem) -> Self {
        Self {
            first,
            rest: Vec::new(),
        }
    }

    /// An error for all of `problems`, or `None` if there are none.
    #[must_use]
    pub fn from_problems(problems: impl IntoIterator<Item = Problem>) -> Option<Self> {
        let mut problems = problems.into_iter();
        let first = problems.next()?;
        Some(Self {
            first,
            rest: problems.collect(),
        })
    }

    /// The problems, in the order they were found.
    pub fn problems(&self) -> impl Iterator<Item = &Problem> {
        std::iter::once(&self.first).chain(&self.rest)
    }

    /// This error, followed by `problem` if there is one.
    #[must_use]
    pub fn with(mut self, problem: Option<Problem>) -> Self {
        self.rest.extend(problem);
        self
    }

    /// Adds the problems of `other` after these.
    pub(crate) fn append(&mut self, other: Self) {
        self.rest.push(other.first);
        self.rest.extend(other.rest);
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.rest.is_empty() {
            return write!(f, "{}", self.first);
        }
        for problem in self.problems() {
            write!(f, "\n  - {problem}")?;
        }
        Ok(())
    }
}

impl std::error::Error for ParseError {}
