//! JSON file adapter.

mod error;

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use checkmate_model::{FORMAT_VERSION, RuleSet};

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
/// [`Error::NotFound`], [`Error::Invalid`] for malformed content, [`Error::UnsupportedFormat`],
/// [`Error::Io`] for other I/O failures.
pub fn read_source(path: &Path) -> Result<RuleSet, Error> {
    let text = fs::read_to_string(path).map_err(|e| match e.kind() {
        std::io::ErrorKind::NotFound => Error::NotFound(path.to_owned()),
        std::io::ErrorKind::InvalidData => Error::Invalid {
            path: path.to_owned(),
            message: "not valid UTF-8".to_owned(),
        },
        _ => Error::Unreadable {
            path: path.to_owned(),
            message: e.to_string(),
        },
    })?;
    check_format(path, &text)?;
    let set: RuleSet = serde_json::from_str(&text).map_err(|e| Error::Invalid {
        path: path.to_owned(),
        message: e.to_string(),
    })?;
    Ok(set)
}

/// Reads `format` alone first, so a file of another format is reported as such rather than as
/// a shape error.
fn check_format(path: &Path, text: &str) -> Result<(), Error> {
    #[derive(serde::Deserialize)]
    struct Probe {
        format: u32,
    }
    let probe: Probe = serde_json::from_str(text).map_err(|e| Error::Invalid {
        path: path.to_owned(),
        message: e.to_string(),
    })?;
    if probe.format == FORMAT_VERSION {
        Ok(())
    } else {
        Err(Error::UnsupportedFormat {
            path: path.to_owned(),
            found: probe.format,
            expected: FORMAT_VERSION,
        })
    }
}

/// Replaces the Source file with `rules` in Canonical form.
///
/// The content goes to a temporary file in the same directory, which then replaces the
/// target, so a failure leaves the previous file intact.
///
/// # Errors
/// [`Error::Io`] on I/O failure.
pub fn write_source(path: &Path, rules: &RuleSet) -> Result<(), Error> {
    let tmp = temporary_sibling(path);
    let result = fs::write(&tmp, rules.to_canonical()).and_then(|()| fs::rename(&tmp, path));
    if result.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    Ok(result?)
}

fn temporary_sibling(path: &Path) -> PathBuf {
    let mut name = path.file_name().unwrap_or_default().to_owned();
    name.push(format!(".{}.tmp", std::process::id()));
    path.with_file_name(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn path(dir: &tempfile::TempDir) -> PathBuf {
        dir.path().join("rules.json")
    }

    #[test]
    fn written_source_reads_back_and_leaves_no_temporary_file() {
        let dir = tempfile::tempdir().unwrap_or_else(|e| panic!("{e}"));
        let mut set = RuleSet::empty();
        set.sequences.insert("RULE".into(), 3);
        write_source(&path(&dir), &set).unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(
            read_source(&path(&dir)).unwrap_or_else(|e| panic!("{e}")),
            set
        );
        assert_eq!(fs::read_dir(dir.path()).map_or(0, Iterator::count), 1);
    }

    #[test]
    fn writing_replaces_an_existing_source() {
        let dir = tempfile::tempdir().unwrap_or_else(|e| panic!("{e}"));
        fs::write(path(&dir), "old").unwrap_or_else(|e| panic!("{e}"));
        write_source(&path(&dir), &RuleSet::empty()).unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(
            fs::read_to_string(path(&dir)).unwrap_or_default(),
            RuleSet::empty().to_canonical()
        );
    }

    #[test]
    fn unknown_fields_are_refused_instead_of_dropped_on_the_next_write() {
        let dir = tempfile::tempdir().unwrap_or_else(|e| panic!("{e}"));
        fs::write(path(&dir), "{\"format\": 1, \"mine\": true}").unwrap_or_else(|e| panic!("{e}"));
        assert!(matches!(
            read_source(&path(&dir)),
            Err(Error::Invalid { .. })
        ));
    }

    #[test]
    fn a_directory_is_unreadable_not_internal() {
        let dir = tempfile::tempdir().unwrap_or_else(|e| panic!("{e}"));
        assert!(matches!(
            read_source(dir.path()),
            Err(Error::Unreadable { .. })
        ));
    }

    #[test]
    fn missing_invalid_and_foreign_format_are_distinguished() {
        let dir = tempfile::tempdir().unwrap_or_else(|e| panic!("{e}"));
        assert!(matches!(read_source(&path(&dir)), Err(Error::NotFound(_))));
        fs::write(path(&dir), "{ nope").unwrap_or_else(|e| panic!("{e}"));
        assert!(matches!(
            read_source(&path(&dir)),
            Err(Error::Invalid { .. })
        ));
        fs::write(path(&dir), "{\"format\": 9, \"other\": 1}").unwrap_or_else(|e| panic!("{e}"));
        assert!(matches!(
            read_source(&path(&dir)),
            Err(Error::UnsupportedFormat { found: 9, .. })
        ));
    }
}
