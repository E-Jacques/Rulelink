//! Telling apart the ways a text can fail to be a Source.

use std::path::Path;

use rulelink_model::{FORMAT_VERSION, RuleSet};
use serde_json::Value;

use crate::Error;

/// Why valid JSON is not a Source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WrongShape {
    /// What is wrong, naming the key or value at fault.
    pub detail: String,
    /// The line at fault, from 1, when the parser knows it.
    pub line: Option<u32>,
}

/// Parses the content of the Source file at `path`; `path` only names the file in errors.
pub(crate) fn parse(path: &Path, text: &str) -> Result<RuleSet, Error> {
    let value: Value = serde_json::from_str(text).map_err(|e| Error::invalid(path, e))?;
    // Before the shape: a file of another format is reported as such.
    check_format(path, &value)?;
    let wrong_shape = |shape| Error::WrongShape {
        path: path.to_owned(),
        shape,
    };
    refuse_positional(&value).map_err(wrong_shape)?;
    serde_json::from_str(text).map_err(|e| {
        wrong_shape(WrongShape {
            detail: e.to_string(),
            line: u32::try_from(e.line()).ok().filter(|line| *line > 0),
        })
    })
}

/// A missing `format`, or one that is not a whole number, is a wrong shape.
fn check_format(path: &Path, value: &Value) -> Result<(), Error> {
    match value.get("format").and_then(Value::as_u64) {
        Some(found) if found != u64::from(FORMAT_VERSION) => Err(Error::UnsupportedFormat {
            path: path.to_owned(),
            found,
            expected: FORMAT_VERSION,
        }),
        _ => Ok(()),
    }
}

/// Refuses what serde would accept by position: an array where an object belongs, and
/// `{"proposed": null}` for a status. Accepted, `rules format` would rewrite them.
fn refuse_positional(source: &Value) -> Result<(), WrongShape> {
    expect(source.is_object(), "the rules file", "an object")?;
    let rules = source.get("rules").and_then(Value::as_array);
    for (index, rule) in rules.into_iter().flatten().enumerate() {
        let at = format!("`rules[{index}]`");
        expect(rule.is_object(), &at, "an object")?;
        for key in ["scope", "origin"] {
            let fits = rule.get(key).is_none_or(|v| v.is_object() || v.is_null());
            expect(fits, &format!("`rules[{index}].{key}`"), "an object")?;
        }
        let fits = rule.get("status").is_none_or(Value::is_string);
        expect(fits, &format!("`rules[{index}].status`"), "a string")?;
    }
    Ok(())
}

fn expect(fits: bool, what: &str, kind: &str) -> Result<(), WrongShape> {
    if fits {
        Ok(())
    } else {
        Err(WrongShape {
            detail: format!("{what} must be {kind}"),
            line: None,
        })
    }
}
