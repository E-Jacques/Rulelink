use std::any::{Any, TypeId};
use std::collections::BTreeMap;

use crate::{ParseResult, RawValue};

/// One setting: it decides whether it is required, its default, its type and what is valid.
///
/// Keep the fields of the implementing type private, so that parsing is the only way to build
/// one from the settings a user wrote.
///
/// ```
/// use rulelink_adapter_sdk::{ParseResult, Parsed, Property, RawValue, required_text};
///
/// #[derive(Debug)]
/// pub struct Space(String);
///
/// impl Property for Space {
///     const KEY: &'static str = "source.space";
///
///     fn parse(raw: Option<&RawValue>) -> ParseResult<Self> {
///         let text = required_text(Self::KEY, raw)?;
///         Ok(Parsed::new(Self(text.to_owned())))
///     }
/// }
///
/// assert!(Space::parse(None).is_err());
/// ```
pub trait Property: Any + std::fmt::Debug + Send + Sync + Sized {
    /// The setting as written from the top of the file, such as `source.path`.
    const KEY: &'static str;

    /// Reads the setting. `raw` is `None` when the file does not write it.
    ///
    /// # Errors
    /// [`ParseError`](crate::ParseError) if the setting is missing or refused.
    fn parse(raw: Option<&RawValue>) -> ParseResult<Self>;
}

/// A property was read that no parser declared: a bug, not a user mistake.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("the setting `{key}` was read but never parsed")]
pub struct UndeclaredProperty {
    /// The setting.
    pub key: &'static str,
}

struct Entry {
    key: &'static str,
    value: Box<dyn Any + Send + Sync>,
}

/// Parsed settings, read back by their [`Property`] type.
///
/// Only a [`TableParser`](crate::TableParser) can fill one.
///
/// ```
/// # use std::collections::BTreeMap;
/// # use rulelink_adapter_sdk::{ParseResult, Parsed, Property, RawTable, RawValue, TableParser};
/// # #[derive(Debug)] struct Flag;
/// # impl Property for Flag {
/// #     const KEY: &'static str = "flag";
/// #     fn parse(_: Option<&RawValue>) -> ParseResult<Self> { Ok(Parsed::new(Flag)) }
/// # }
/// let table = RawTable::new("", BTreeMap::new());
/// let parsed = TableParser::new(table).property::<Flag>().finish()?;
/// assert!(parsed.configuration.get::<Flag>().is_ok());
/// # Ok::<(), rulelink_adapter_sdk::ParseError>(())
/// ```
pub struct ConfigMap {
    entries: BTreeMap<TypeId, Entry>,
}

impl ConfigMap {
    pub(crate) fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    pub(crate) fn insert<P: Property>(&mut self, property: P) {
        self.entries.insert(
            TypeId::of::<P>(),
            Entry {
                key: P::KEY,
                value: Box::new(property),
            },
        );
    }

    /// The parsed property `P`.
    ///
    /// # Errors
    /// [`UndeclaredProperty`] only if no parser declared `P`.
    pub fn get<P: Property>(&self) -> Result<&P, UndeclaredProperty> {
        self.entries
            .get(&TypeId::of::<P>())
            .and_then(|entry| entry.value.downcast_ref())
            .ok_or(UndeclaredProperty { key: P::KEY })
    }

    /// Adds the settings of `other`.
    pub fn merge(&mut self, other: Self) {
        self.entries.extend(other.entries);
    }
}

impl std::fmt::Debug for ConfigMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set()
            .entries(self.entries.values().map(|entry| entry.key))
            .finish()
    }
}
