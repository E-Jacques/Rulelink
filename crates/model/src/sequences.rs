//! Reading `sequences`.

use std::collections::BTreeMap;
use std::fmt;

use serde::Deserializer;
use serde::de::{Error, MapAccess, Visitor};

/// Reads `sequences`, refusing a prefix given twice, as a bad merge can leave it: keeping
/// one of the two numbers could lower the sequence, and an Identifier be allocated again.
pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<BTreeMap<String, u64>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_map(UniquePrefixes)
}

struct UniquePrefixes;

impl<'de> Visitor<'de> for UniquePrefixes {
    type Value = BTreeMap<String, u64>;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a map from Identifier prefix to number")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut sequences = BTreeMap::new();
        while let Some((prefix, number)) = map.next_entry::<String, u64>()? {
            if sequences.contains_key(&prefix) {
                return Err(A::Error::custom(format_args!(
                    "`sequences` gives `{prefix}` twice"
                )));
            }
            sequences.insert(prefix, number);
        }
        Ok(sequences)
    }
}
