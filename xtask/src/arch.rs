//! Dependency rules of the hexagonal architecture (docs/repository/01, section 6.2).

use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::process::Command;

const DOMAIN: &[&str] = &[
    "rulelink-model",
    "rulelink-registry",
    "rulelink-link",
    "rulelink-check",
];
const FORBIDDEN: &[&str] = &[
    "tokio",
    "reqwest",
    "hyper",
    "async-std",
    "mio",
    "ureq",
    "async-io",
];
/// Packages allowed to define `fn main`.
const MAIN_ALLOWED: &[&str] = &["rulelink-cli", "xtask"];

pub fn run() -> Result<(), String> {
    let out = Command::new("cargo")
        .args(["metadata", "--format-version", "1", "--locked"])
        .output()
        .map_err(|e| format!("cargo metadata: {e}"))?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).into_owned());
    }
    let meta: Value = serde_json::from_slice(&out.stdout).map_err(|e| e.to_string())?;
    let mut errors = Vec::new();
    check_forbidden(&meta, &mut errors);
    check_main(&meta, &mut errors);
    check_workspace_deps(&meta, &mut errors);
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("\n"))
    }
}

fn check_forbidden(meta: &Value, errors: &mut Vec<String>) {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut names: HashMap<&str, &str> = HashMap::new();
    for pkg in meta["packages"].as_array().into_iter().flatten() {
        if let (Some(id), Some(name)) = (pkg["id"].as_str(), pkg["name"].as_str()) {
            names.insert(id, name);
        }
    }
    for node in meta["resolve"]["nodes"].as_array().into_iter().flatten() {
        let Some(id) = node["id"].as_str() else {
            continue;
        };
        let deps = node["dependencies"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .collect();
        graph.insert(id, deps);
    }
    for (id, name) in &names {
        if !DOMAIN.contains(name) {
            continue;
        }
        let mut seen = HashSet::new();
        let mut stack = vec![*id];
        while let Some(cur) = stack.pop() {
            for dep in graph.get(cur).into_iter().flatten() {
                if seen.insert(*dep) {
                    if let Some(dep_name) = names.get(dep) {
                        if FORBIDDEN.contains(dep_name) {
                            errors.push(format!(
                                "rule `no-io-in-domain`: `{name}` reaches forbidden crate `{dep_name}`"
                            ));
                        }
                    }
                    stack.push(dep);
                }
            }
        }
    }
}

fn check_main(meta: &Value, errors: &mut Vec<String>) {
    for pkg in meta["packages"].as_array().into_iter().flatten() {
        let (Some(name), Some(manifest)) = (pkg["name"].as_str(), pkg["manifest_path"].as_str())
        else {
            continue;
        };
        if !name.starts_with("rulelink-") && name != "xtask" {
            continue;
        }
        if MAIN_ALLOWED.contains(&name) {
            continue;
        }
        let Some(dir) = std::path::Path::new(manifest).parent() else {
            continue;
        };
        if let Ok(src) = std::fs::read_to_string(dir.join("src/lib.rs")) {
            if src.contains("fn main(") {
                errors.push(format!("rule `single-main`: `{name}` defines `fn main`"));
            }
        }
        if dir.join("src/main.rs").exists() {
            errors.push(format!("rule `single-main`: `{name}` has a src/main.rs"));
        }
    }
}

/// Every dependency of a workspace member is inherited from `[workspace.dependencies]`
/// (`name.workspace = true` or `name = { workspace = true }`), or is a path dependency.
fn check_workspace_deps(meta: &Value, errors: &mut Vec<String>) {
    let members: HashSet<&str> = meta["workspace_members"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect();
    for pkg in meta["packages"].as_array().into_iter().flatten() {
        let (Some(id), Some(name), Some(manifest)) = (
            pkg["id"].as_str(),
            pkg["name"].as_str(),
            pkg["manifest_path"].as_str(),
        ) else {
            continue;
        };
        if !members.contains(id) {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(manifest) else {
            continue;
        };
        for dep in local_dependencies(&text) {
            errors.push(format!(
                "rule `workspace-deps`: `{name}` declares `{dep}` itself; add it to [workspace.dependencies] and use `{dep}.workspace = true`"
            ));
        }
    }
}

/// Names of dependencies declared with their own version in a member manifest.
fn local_dependencies(manifest: &str) -> Vec<String> {
    let mut in_deps = false;
    let mut found = Vec::new();
    for line in manifest.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_deps = line.ends_with("dependencies]");
            continue;
        }
        if !in_deps || line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((name, value)) = line.split_once('=') else {
            continue;
        };
        let (name, value) = (name.trim(), value.trim());
        let inherited = name.ends_with(".workspace") || value.contains("workspace = true");
        let local = value.contains("path =");
        if !inherited && !local {
            found.push(name.to_owned());
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::local_dependencies;

    #[test]
    fn flags_dependencies_that_bypass_the_workspace() {
        let manifest = "[dependencies]\ntoml = \"0.8\"\nserde.workspace = true\nx = { workspace = true }\ny = { path = \"../y\" }\n[dev-dependencies]\nz = { version = \"1\" }\n";
        assert_eq!(local_dependencies(manifest), vec!["toml", "z"]);
    }
}
