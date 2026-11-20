//! Harness for end-to-end tests: runs the real `rulelink` binary against a fixture
//! repository in a temporary directory.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

/// Result of one `rulelink` invocation.
#[derive(Debug)]
pub struct Run {
    /// Process exit code.
    pub code: i32,
    /// Captured standard output.
    pub stdout: String,
    /// Captured standard error.
    pub stderr: String,
}

/// A throwaway repository on disk.
#[derive(Debug)]
pub struct Repo {
    dir: tempfile::TempDir,
}

impl Default for Repo {
    fn default() -> Self {
        Self::new()
    }
}

impl Repo {
    /// Creates an empty repository directory.
    ///
    /// # Panics
    /// Panics if the temporary directory cannot be created.
    #[must_use]
    pub fn new() -> Self {
        Self {
            dir: tempfile::tempdir().expect("temp dir"),
        }
    }

    /// Root of the repository.
    #[must_use]
    pub fn path(&self) -> &Path {
        self.dir.path()
    }

    /// Writes a file (creating parent directories).
    ///
    /// # Panics
    /// Panics on I/O failure.
    pub fn write(&self, rel: &str, content: &str) {
        let path = self.path().join(rel);
        fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
        fs::write(path, content).expect("write");
    }

    /// Reads a file, or `None` if it does not exist.
    #[must_use]
    pub fn read(&self, rel: &str) -> Option<String> {
        fs::read_to_string(self.path().join(rel)).ok()
    }

    /// Runs `rulelink <args>` in the repository root.
    ///
    /// # Panics
    /// Panics if the binary cannot be built or started.
    #[must_use]
    pub fn run(&self, args: &[&str]) -> Run {
        let output = Command::new(rulelink_bin())
            .args(args)
            .current_dir(self.path())
            .output()
            .expect("binary runs");
        Run {
            code: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        }
    }
}

/// Builds (once) and locates the `rulelink` binary.
///
/// # Panics
/// Panics if the build fails.
#[must_use]
pub fn rulelink_bin() -> PathBuf {
    static BIN: OnceLock<PathBuf> = OnceLock::new();
    BIN.get_or_init(|| {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".into());
        let status = Command::new(cargo)
            .args(["build", "--quiet", "-p", "rulelink-cli"])
            .current_dir(&root)
            .status()
            .expect("cargo runs");
        assert!(status.success(), "building rulelink-cli failed");
        let target =
            std::env::var_os("CARGO_TARGET_DIR").map_or_else(|| root.join("target"), PathBuf::from);
        target
            .join("debug")
            .join(format!("rulelink{}", std::env::consts::EXE_SUFFIX))
    })
    .clone()
}
