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
/// [`Error::NotFound`] if the file is missing, [`Error::Invalid`] if it is not a valid
/// Source of the current format, [`Error::Io`] on other I/O failures.
pub fn read_source(path: &Path) -> Result<RuleSet, Error> {
    let text = fs::read_to_string(path).map_err(|e| match e.kind() {
        std::io::ErrorKind::NotFound => Error::NotFound,
        _ => Error::Io(e),
    })?;
    let set: RuleSet = serde_json::from_str(&text).map_err(|e| Error::Invalid(e.to_string()))?;
    if set.format != checkmate_model::FORMAT_VERSION {
        return Err(Error::Invalid(format!(
            "unsupported format {} (expected {})",
            set.format,
            checkmate_model::FORMAT_VERSION
        )));
    }
    Ok(set)
}

/// Replaces the content of an existing Source file with `rules` in Canonical form.
///
/// The new content is written next to the file and renamed over it, so a failure never
/// leaves a half-written Source.
///
/// # Errors
/// [`Error::Io`] on I/O failure.
pub fn write_source(path: &Path, rules: &RuleSet) -> Result<(), Error> {
    let mut staging = path.as_os_str().to_owned();
    staging.push(".tmp");
    let staging = std::path::PathBuf::from(staging);
    let result =
        fs::write(&staging, rules.to_canonical()).and_then(|()| fs::rename(&staging, path));
    if result.is_err() {
        let _ = fs::remove_file(&staging);
    }
    Ok(result?)
}
