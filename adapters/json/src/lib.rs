//! JSON file adapter.

mod error;

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

use checkmate_model::RuleSet;

pub use error::Error;

/// Creates a Source file holding `rules` in Canonical form.
///
/// Parent directories are created. The file must not exist yet.
///
/// # Errors
/// Returns [`Error::AlreadyExists`] if the file exists, [`Error::Io`] on other I/O failures.
pub fn create_source(path: &Path, rules: &RuleSet) -> Result<(), Error> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| match e.kind() {
            std::io::ErrorKind::AlreadyExists => Error::AlreadyExists,
            _ => Error::Io(e),
        })?;
    file.write_all(rules.to_canonical().as_bytes())?;
    Ok(())
}

/// Reads a Source file.
///
/// # Errors
/// Returns [`Error::Io`] if the file cannot be read, [`Error::Invalid`] if it is not a
/// Source, and [`Error::UnsupportedFormat`] if its `format` is not the current one.
pub fn read_source(path: &Path) -> Result<RuleSet, Error> {
    let text = fs::read_to_string(path)?;
    let set: RuleSet = serde_json::from_str(&text).map_err(|e| Error::Invalid(e.to_string()))?;
    if set.format != checkmate_model::FORMAT_VERSION {
        return Err(Error::UnsupportedFormat(set.format));
    }
    Ok(set)
}

/// Replaces a Source file with `rules` in Canonical form.
///
/// The content is written to a sibling file first and renamed into place, so a failure
/// never leaves a truncated Source behind.
///
/// # Errors
/// Returns [`Error::Io`] on I/O failure.
pub fn write_source(path: &Path, rules: &RuleSet) -> Result<(), Error> {
    let mut tmp = path.as_os_str().to_owned();
    tmp.push(".tmp");
    let tmp = std::path::PathBuf::from(tmp);
    let result = fs::write(&tmp, rules.to_canonical()).and_then(|()| fs::rename(&tmp, path));
    if result.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    Ok(result?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_then_read_round_trips() {
        let dir = tempfile::tempdir().unwrap_or_else(|e| panic!("{e}"));
        let path = dir.path().join("rules.json");
        create_source(&path, &RuleSet::empty()).unwrap_or_else(|e| panic!("{e}"));
        let mut set = read_source(&path).unwrap_or_else(|e| panic!("{e}"));
        set.sequences.insert("RULE".into(), 3);
        write_source(&path, &set).unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(read_source(&path).unwrap_or_else(|e| panic!("{e}")), set);
        fs::write(&path, "nope").unwrap_or_else(|e| panic!("{e}"));
        assert!(matches!(read_source(&path), Err(Error::Invalid(_))));
    }
}
