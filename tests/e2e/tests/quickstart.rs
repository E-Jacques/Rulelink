//! The Quickstart page is replayed against the real binary, so it cannot go stale.
//!
//! Every `console` block of the page is a list of `$ command` lines, each followed by the
//! output it prints (if any). The test runs each command in a fixture repository and checks
//! the exit code and the output.

use std::path::PathBuf;
use std::process::Command;

use rulelink_e2e::{Repo, rulelink_bin};

/// One `$ command` of a `console` block and the output the page shows for it.
#[derive(Debug, PartialEq, Eq)]
struct Step {
    command: String,
    expected: Vec<String>,
}

/// Reads the steps of every `console` block of a page.
fn steps(page: &str) -> Vec<Step> {
    let mut steps: Vec<Step> = Vec::new();
    let mut in_block = false;
    for line in page.lines() {
        if line.starts_with("```") {
            in_block = !in_block && line.trim() == "```console";
            continue;
        }
        if !in_block {
            continue;
        }
        if let Some(command) = line.strip_prefix("$ ") {
            steps.push(Step {
                command: command.to_string(),
                expected: Vec::new(),
            });
        } else if let Some(step) = steps.last_mut() {
            step.expected.push(line.to_string());
        }
    }
    steps
}

/// Replays the steps in `repo`. Returns the first divergence between the page and the CLI.
fn replay(repo: &Repo, steps: &[Step]) -> Result<(), String> {
    let bin = rulelink_bin();
    let bin_dir = bin.parent().expect("binary directory").to_path_buf();
    let inherited = std::env::var_os("PATH").unwrap_or_default();
    let path =
        std::env::join_paths(std::iter::once(bin_dir).chain(std::env::split_paths(&inherited)))
            .expect("PATH");
    for step in steps {
        let output = Command::new("sh")
            .args(["-c", &step.command])
            .current_dir(repo.path())
            .env("PATH", &path)
            .output()
            .expect("shell runs");
        if !output.status.success() {
            return Err(format!(
                "`{}` failed: {}",
                step.command,
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        let actual: Vec<&str> = stdout.lines().collect();
        if actual != step.expected {
            return Err(format!(
                "`{}` printed {actual:?}, the page shows {:?}",
                step.command, step.expected
            ));
        }
    }
    Ok(())
}

fn page() -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/public/get-started/quickstart.md");
    std::fs::read_to_string(path).expect("the Quickstart page exists")
}

#[test]
fn the_quickstart_runs_as_written() {
    let steps = steps(&page());
    for command in ["rulelink init", "rulelink rules add", "rulelink where"] {
        assert!(
            steps.iter().any(|s| s.command.starts_with(command)),
            "the page has no `{command}` step: {steps:?}"
        );
    }
    let repo = Repo::new();
    if let Err(divergence) = replay(&repo, &steps) {
        panic!("the Quickstart and the CLI diverge: {divergence}");
    }
    assert!(repo.read("rules/rules.json").is_some());
}

#[test]
fn a_command_the_cli_rejects_is_a_divergence() {
    let page = "```console\n$ rulelink init\nInitialised Rulelink: wrote rulelink.toml and rules/rules.json\n$ rulelink check\n```\n";
    let result = replay(&Repo::new(), &steps(page));
    assert!(result.is_err_and(|e| e.contains("rulelink check")));
}

#[test]
fn an_output_the_cli_does_not_print_is_a_divergence() {
    let page = "```console\n$ rulelink init\nSet up!\n```\n";
    let result = replay(&Repo::new(), &steps(page));
    assert!(result.is_err_and(|e| e.contains("Set up!")));
}
