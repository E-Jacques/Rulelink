//! A project whose `rulelink.toml` has been accepted. Every use case that needs the
//! configuration takes one, so it cannot run before the configuration is checked.

use std::path::{Path, PathBuf};

use rulelink_adapter_json::{JsonConfig, SourcePath};
use rulelink_adapter_scan_fs::{ScanExclude, ScanFsConfig};
use rulelink_adapter_sdk::{ConfigMap, Parsed, Property};
use rulelink_check::Levels;
use rulelink_config::{AdapterCatalog, CheckLevels, IdPrefix, RulesPrefix};

use crate::Error;

/// The adapters built into this version.
const CATALOG: AdapterCatalog<'static> = AdapterCatalog {
    sources: &[&JsonConfig],
    scanner: &ScanFsConfig,
};

/// An open project: its root and its parsed settings.
#[derive(Debug)]
pub struct Workspace {
    root: PathBuf,
    settings: ConfigMap,
}

impl Workspace {
    /// Reads and checks all of `rulelink.toml` in `root`, and returns the project with what
    /// to warn about before using it.
    ///
    /// # Errors
    /// [`Error::Config`] for a missing, unreadable or invalid `rulelink.toml`.
    pub fn open(root: &Path) -> Result<Parsed<Self>, Error> {
        let parsed = rulelink_config::load(root, CATALOG)?;
        Ok(Parsed {
            configuration: Self {
                root: root.to_path_buf(),
                settings: parsed.configuration,
            },
            warnings: parsed.warnings,
        })
    }

    pub(crate) fn root(&self) -> &Path {
        &self.root
    }

    pub(crate) fn id_prefix(&self) -> Result<&IdPrefix, Error> {
        Ok(self.setting::<RulesPrefix>()?.prefix())
    }

    /// The Source, as `rulelink.toml` names it (relative to the project root).
    pub(crate) fn source_path(&self) -> Result<&Path, Error> {
        Ok(self.setting::<SourcePath>()?.as_path())
    }

    pub(crate) fn scan_exclude(&self) -> Result<&ScanExclude, Error> {
        self.setting::<ScanExclude>()
    }

    pub(crate) fn check_levels(&self) -> Result<&Levels, Error> {
        Ok(self.setting::<CheckLevels>()?.levels())
    }

    fn setting<P: Property>(&self) -> Result<&P, Error> {
        Ok(self.settings.get::<P>()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// What `init` writes must be accepted by the adapters it was written for.
    #[test]
    fn an_initialised_project_opens_with_every_setting() {
        let dir = tempfile::tempdir().unwrap_or_else(|e| panic!("{e}"));
        crate::init::init(dir.path(), "BR").unwrap_or_else(|e| panic!("{e}"));
        let opened = Workspace::open(dir.path()).unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(opened.warnings.len(), 0);
        let project = opened.configuration;
        assert_eq!(project.id_prefix().map(IdPrefix::as_str).ok(), Some("BR"));
        assert_eq!(
            project.source_path().ok(),
            Some(Path::new(rulelink_config::DEFAULT_SOURCE_PATH))
        );
        assert!(project.scan_exclude().is_ok());
        assert!(project.check_levels().is_ok());
    }
}
