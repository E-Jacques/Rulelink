use std::path::{Component, Path, PathBuf};

/// A path that was refused because it may lead outside the project.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("must be a relative path inside the project, without `..`")]
pub struct OutsideProject;

/// A relative path that stays inside the project, as far as its text says: symbolic links are
/// not followed.
///
/// ```
/// use rulelink_adapter_sdk::ProjectPath;
///
/// assert!(ProjectPath::parse("rules/rules.json").is_ok());
/// assert!(ProjectPath::parse("../rules.json").is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectPath(PathBuf);

impl ProjectPath {
    /// Checks `raw`.
    ///
    /// # Errors
    /// [`OutsideProject`] for an empty or absolute path, or one holding `..`.
    pub fn parse(raw: &str) -> Result<Self, OutsideProject> {
        let path = PathBuf::from(raw);
        let plain = !raw.is_empty()
            && path
                .components()
                .all(|c| matches!(c, Component::Normal(_) | Component::CurDir));
        if plain {
            Ok(Self(path))
        } else {
            Err(OutsideProject)
        }
    }

    /// The path, relative to the project root.
    #[must_use]
    pub fn as_path(&self) -> &Path {
        &self.0
    }
}
