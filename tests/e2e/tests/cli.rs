//! End-to-end tests of the `rulelink` binary, built on demand from the workspace.

use std::path::PathBuf;
use std::process::Command;

fn rulelink_bin() -> PathBuf {
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
}

#[test]
fn prints_version() {
    let output = Command::new(rulelink_bin())
        .arg("--version")
        .output()
        .expect("binary runs");
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).starts_with("rulelink "));
}
