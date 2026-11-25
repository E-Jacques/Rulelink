//! Filesystem code scanning adapter: walks the project and indexes its Annotations.

mod error;

use std::path::Path;

use ignore::WalkBuilder;
use ignore::overrides::OverrideBuilder;
use rulelink_link::LinkIndex;

pub use error::Error;

/// Name of the Rulelink-specific ignore file, in `.gitignore` syntax.
const IGNORE_FILE: &str = ".rulelinkignore";
/// The byte that marks a file as binary.
const NUL: char = '\0';
/// Files above this size are skipped: they are data or generated code, not hand-written code.
const MAX_FILE_BYTES: u64 = 2 * 1024 * 1024;

/// Scans the files under `root` for Annotations.
///
/// Honours `.gitignore`, `.ignore` and `.rulelinkignore` (also outside a Git repository),
/// plus `exclude`, a list of `.gitignore`-style patterns relative to `root`. Skips hidden
/// files, symbolic links, files over 2 MB, binary files and files that are not UTF-8.
///
/// # Errors
/// [`Error::InvalidExclude`] for a pattern that is not a valid glob, [`Error::Unreadable`] for
/// a directory or file that cannot be read.
pub fn scan(root: &Path, exclude: &[String]) -> Result<LinkIndex, Error> {
    let walk = files_to_walk(root, exclude)?;
    let mut index = LinkIndex::new();
    for entry in walk {
        let entry = entry.map_err(|e| Error::Unreadable {
            path: root.to_path_buf(),
            message: e.to_string(),
        })?;
        index_entry(root, &entry, &mut index)?;
    }
    Ok(index)
}

/// The walk over `root`: ignore files and `exclude` patterns applied, nothing read yet.
fn files_to_walk(root: &Path, exclude: &[String]) -> Result<ignore::Walk, Error> {
    let invalid = |pattern: String, e: ignore::Error| Error::InvalidExclude {
        pattern,
        message: e.to_string(),
    };
    let mut overrides = OverrideBuilder::new(root);
    for pattern in exclude {
        // In an override, a leading `!` means "ignore".
        overrides
            .add(&format!("!{pattern}"))
            .map_err(|e| invalid(pattern.clone(), e))?;
    }
    let overrides = overrides
        .build()
        .map_err(|e| invalid(exclude.join(", "), e))?;
    Ok(WalkBuilder::new(root)
        .require_git(false)
        .add_custom_ignore_filename(IGNORE_FILE)
        .overrides(overrides)
        .build())
}

/// Adds the Annotations of one walk entry to `index`, skipping what is not hand-written text.
fn index_entry(root: &Path, entry: &ignore::DirEntry, index: &mut LinkIndex) -> Result<(), Error> {
    if !entry.file_type().is_some_and(|t| t.is_file()) {
        return Ok(());
    }
    let path = entry.path();
    let unreadable = |e: &dyn std::fmt::Display| Error::Unreadable {
        path: path.to_path_buf(),
        message: e.to_string(),
    };
    if entry.metadata().map_err(|e| unreadable(&e))?.len() > MAX_FILE_BYTES {
        return Ok(());
    }
    let bytes = std::fs::read(path).map_err(|e| unreadable(&e))?;
    let Ok(text) = String::from_utf8(bytes) else {
        return Ok(());
    };
    // A NUL byte is valid UTF-8 but never in hand-written text: it marks a binary format
    // (image, archive, database) that happens to decode, the same heuristic `grep` uses.
    if !text.contains(NUL) {
        index.add_file(&relative(root, path), &text);
    }
    Ok(())
}

/// `path` relative to `root`, always `/`-separated.
///
/// Built from path components rather than the raw string, so a Windows path (`a\\b.rs`) comes out
/// as `a/b.rs` too: the same Location on every platform.
fn relative(root: &Path, path: &Path) -> String {
    let rel = path.strip_prefix(root).unwrap_or(path);
    rel.components()
        .map(|c| c.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rulelink_model::RuleId;

    fn cited(dir: &tempfile::TempDir, exclude: &[&str]) -> Vec<String> {
        let exclude: Vec<String> = exclude.iter().map(|s| (*s).to_owned()).collect();
        scan(dir.path(), &exclude)
            .unwrap_or_else(|e| panic!("{e}"))
            .locations(&RuleId("RULE-1".to_owned()))
            .into_iter()
            .map(|l| format!("{}:{}", l.path, l.line))
            .collect()
    }

    fn write(dir: &tempfile::TempDir, rel: &str, content: &[u8]) {
        let path = dir.path().join(rel);
        std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
        std::fs::write(path, content).expect("write");
    }

    #[test]
    fn skips_binary_non_utf8_and_oversized_files() {
        let dir = tempfile::tempdir().expect("dir");
        write(&dir, "ok.rs", b"// @rule RULE-1\n");
        write(&dir, "bin.dat", b"\0\0// @rule RULE-1\n");
        write(&dir, "latin.txt", b"// caf\xe9 @rule RULE-1\n");
        let mut big = b"// @rule RULE-1\n".to_vec();
        big.resize(usize::try_from(MAX_FILE_BYTES).expect("fits") + 1, b' ');
        write(&dir, "big.rs", &big);
        assert_eq!(cited(&dir, &[]), ["ok.rs:1"]);
    }

    #[test]
    fn nested_paths_use_forward_slashes_and_exclusions_apply() {
        let dir = tempfile::tempdir().expect("dir");
        write(&dir, "a/b/c.rs", b"// @rule RULE-1\n");
        write(&dir, "gen/x.rs", b"// @rule RULE-1\n");
        assert_eq!(cited(&dir, &["gen/"]), ["a/b/c.rs:1"]);
    }

    #[test]
    fn an_invalid_exclusion_names_the_pattern() {
        let dir = tempfile::tempdir().expect("dir");
        let err = scan(dir.path(), &["[".to_owned()]).expect_err("refused");
        assert!(matches!(err, Error::InvalidExclude { ref pattern, .. } if pattern == "["));
    }
}
