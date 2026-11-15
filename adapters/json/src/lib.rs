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
