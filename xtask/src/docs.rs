//! `docs-check`: the public documentation must cover the CLI and stay consistent.

use std::fs;
use std::path::{Path, PathBuf};
use std::process;

/// A leaf command of the CLI and the long options its `--help` reports.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    /// Words typed after `rulelink`, e.g. `["rules", "add"]`.
    pub path: Vec<String>,
    /// Long options, with their dashes, e.g. `--title`.
    pub options: Vec<String>,
}

/// Reads a `--help` text: the subcommands it lists and its long options.
pub fn parse_help(help: &str) -> (Vec<String>, Vec<String>) {
    let mut subcommands = Vec::new();
    let mut options = Vec::new();
    let mut section = "";
    for line in help.lines() {
        if !line.starts_with(' ') {
            section = line.trim_end_matches(':');
            continue;
        }
        let words: Vec<&str> = line.split_whitespace().collect();
        match section {
            "Commands" => {
                if let Some(name) = words.first().filter(|name| **name != "help") {
                    subcommands.push((*name).to_string());
                }
            }
            "Options" => {
                let long = words
                    .iter()
                    .map(|word| word.trim_end_matches(','))
                    .find(|word| word.starts_with("--"));
                if let Some(option) = long.filter(|o| *o != "--help" && *o != "--version") {
                    options.push(option.to_string());
                }
            }
            _ => {}
        }
    }
    (subcommands, options)
}

/// Returns one message per problem found under `docs_root` (`docs/public`).
pub fn check(docs_root: &Path, commands: &[Command]) -> Vec<String> {
    let mut problems = Vec::new();
    for command in commands {
        check_command(docs_root, command, &mut problems);
    }
    let mut pages = Vec::new();
    collect_pages(docs_root, &mut pages);
    for page in pages {
        check_links(docs_root, &page, &mut problems);
    }
    problems
}

fn check_command(docs_root: &Path, command: &Command, problems: &mut Vec<String>) {
    let typed = format!("rulelink {}", command.path.join(" "));
    let relative = format!("cli/{}.md", command.path.join("-"));
    let Ok(text) = fs::read_to_string(docs_root.join(&relative)) else {
        problems.push(format!("`{typed}` has no page: expected {relative}"));
        return;
    };
    check_front_matter(&relative, &text, problems);
    for option in &command.options {
        if !text.contains(&format!("`{option}`")) {
            problems.push(format!(
                "{relative}: option `{option}` of `{typed}` is not documented"
            ));
        }
    }
}

/// A CLI page starts with a front matter holding `title` and a `since` version.
fn check_front_matter(relative: &str, text: &str, problems: &mut Vec<String>) {
    let Some(rest) = text.strip_prefix("---\n") else {
        problems.push(format!(
            "{relative}: missing front matter (`title` and `since`)"
        ));
        return;
    };
    let front = rest.split("\n---").next().unwrap_or_default();
    let field = |name: &str| {
        front
            .lines()
            .find_map(|line| line.strip_prefix(&format!("{name}:")))
            .map(str::trim)
            .filter(|value| !value.is_empty())
    };
    if field("title").is_none() {
        problems.push(format!("{relative}: front matter field `title` is missing"));
    }
    match field("since") {
        None => problems.push(format!("{relative}: front matter field `since` is missing")),
        Some(version) if !is_version(version) => problems.push(format!(
            "{relative}: front matter `since` must be a version like 0.1.0, found `{version}`"
        )),
        Some(_) => {}
    }
}

fn is_version(text: &str) -> bool {
    let parts: Vec<&str> = text.split('.').collect();
    parts.len() == 3
        && parts
            .iter()
            .all(|part| !part.is_empty() && part.chars().all(|c| c.is_ascii_digit()))
}

fn collect_pages(dir: &Path, pages: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    let mut paths: Vec<PathBuf> = entries.flatten().map(|entry| entry.path()).collect();
    paths.sort();
    for path in paths {
        if path.is_dir() {
            collect_pages(&path, pages);
        } else if path.extension().is_some_and(|ext| ext == "md") {
            pages.push(path);
        }
    }
}

/// Every relative link of a page must point to a file that exists.
fn check_links(docs_root: &Path, page: &Path, problems: &mut Vec<String>) {
    let Ok(text) = fs::read_to_string(page) else {
        return;
    };
    let name = page
        .strip_prefix(docs_root)
        .unwrap_or(page)
        .display()
        .to_string();
    let mut in_fence = false;
    for line in text.lines() {
        if line.trim_start().starts_with("```") {
            in_fence = !in_fence;
        }
        if in_fence {
            continue;
        }
        for target in link_targets(line) {
            let path = target.split('#').next().unwrap_or_default();
            let external = target.contains("://") || target.starts_with("mailto:");
            if path.is_empty() || external {
                continue;
            }
            let resolved = page.parent().unwrap_or(docs_root).join(path);
            if !resolved.exists() {
                problems.push(format!("{name}: broken link to `{target}`"));
            }
        }
    }
}

/// The targets of the Markdown links `[text](target)` on one line.
fn link_targets(line: &str) -> Vec<&str> {
    let mut targets = Vec::new();
    let mut rest = line;
    while let Some(start) = rest.find("](") {
        rest = &rest[start + 2..];
        let Some(end) = rest.find(')') else {
            break;
        };
        targets.push(&rest[..end]);
        rest = &rest[end..];
    }
    targets
}

/// Runs the check on `docs/public` against the commands the real binary reports.
pub fn run() -> Result<(), String> {
    let commands = discover_commands()?;
    let problems = check(Path::new("docs/public"), &commands);
    if problems.is_empty() {
        Ok(())
    } else {
        Err(problems.join("\n"))
    }
}

/// Walks `rulelink --help` down to the leaf commands.
fn discover_commands() -> Result<Vec<Command>, String> {
    let mut leaves = Vec::new();
    let mut pending = vec![Vec::<String>::new()];
    while let Some(path) = pending.pop() {
        let (subcommands, options) = parse_help(&help_of(&path)?);
        if subcommands.is_empty() {
            leaves.push(Command { path, options });
        } else {
            pending.extend(subcommands.into_iter().map(|name| {
                let mut child = path.clone();
                child.push(name);
                child
            }));
        }
    }
    leaves.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(leaves)
}

fn help_of(path: &[String]) -> Result<String, String> {
    let output = process::Command::new("cargo")
        .args(["run", "--quiet", "-p", "rulelink-cli", "--"])
        .args(path)
        .arg("--help")
        .output()
        .map_err(|e| format!("cannot run the CLI: {e}"))?;
    if !output.status.success() {
        return Err(format!("`rulelink {} --help` failed", path.join(" ")));
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAGE: &str = "---\ntitle: rulelink init\nsince: 0.1.0\n---\n\n# rulelink init\n\n## Options\n\n| name | description | example |\n| --- | --- | --- |\n| `--id-prefix` | Prefix. | `--id-prefix BR` |\n";

    fn init_command() -> Command {
        Command {
            path: vec!["init".into()],
            options: vec!["--id-prefix".into()],
        }
    }

    /// Builds a documentation tree in a fresh temporary folder.
    fn tree(name: &str, files: &[(&str, &str)]) -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!("xtask-docs-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        for (path, text) in files {
            let file = root.join(path);
            fs::create_dir_all(file.parent().expect("a parent folder")).expect("create the folder");
            fs::write(file, text).expect("write the page");
        }
        root
    }

    fn problems(name: &str, files: &[(&str, &str)], commands: &[Command]) -> Vec<String> {
        check(&tree(name, files), commands)
    }

    #[test]
    fn a_documented_command_passes() {
        let found = problems("ok", &[("cli/init.md", PAGE)], &[init_command()]);
        assert!(found.is_empty(), "{found:?}");
    }

    #[test]
    fn a_command_without_a_page_fails() {
        let found = problems("nopage", &[("index.md", "# Hi\n")], &[init_command()]);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].contains("rulelink init") && found[0].contains("cli/init.md"));
    }

    #[test]
    fn a_subcommand_page_is_named_after_its_path() {
        let add = Command {
            path: vec!["rules".into(), "add".into()],
            options: vec![],
        };
        let found = problems("sub", &[("cli/init.md", PAGE)], &[add]);
        assert!(found[0].contains("cli/rules-add.md"), "{found:?}");
    }

    #[test]
    fn an_undocumented_option_fails() {
        let page = PAGE.replace("--id-prefix", "--other");
        let found = problems("opt", &[("cli/init.md", &page)], &[init_command()]);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].contains("--id-prefix") && found[0].contains("cli/init.md"));
    }

    #[test]
    fn a_cli_page_needs_title_and_a_valid_since() {
        let no_front = "# rulelink init\n\n`--id-prefix`\n";
        let found = problems("nofm", &[("cli/init.md", no_front)], &[init_command()]);
        assert!(
            found.iter().any(|m| m.contains("front matter")),
            "{found:?}"
        );

        let bad = PAGE.replace("since: 0.1.0", "since: soon");
        let found = problems("badsince", &[("cli/init.md", &bad)], &[init_command()]);
        assert!(found.iter().any(|m| m.contains("since")), "{found:?}");

        let no_title = PAGE.replace("title: rulelink init\n", "");
        let found = problems("notitle", &[("cli/init.md", &no_title)], &[init_command()]);
        assert!(found.iter().any(|m| m.contains("title")), "{found:?}");
    }

    #[test]
    fn a_broken_internal_link_fails() {
        let page = format!(
            "{PAGE}\nSee [rules](rules.md), [site](https://example.com/x.md) and [top](#options).\n"
        );
        let found = problems("link", &[("cli/init.md", &page)], &[init_command()]);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].contains("rules.md"));
    }

    #[test]
    fn a_valid_relative_link_with_an_anchor_passes() {
        let page = format!("{PAGE}\nSee [format](../concepts/rule-format.md#identifier).\n");
        let found = problems(
            "goodlink",
            &[
                ("cli/init.md", &page),
                ("concepts/rule-format.md", "# Rule format\n"),
            ],
            &[init_command()],
        );
        assert!(found.is_empty(), "{found:?}");
    }

    #[test]
    fn help_output_yields_subcommands_and_options() {
        let help = "Create things\n\nUsage: rulelink rules <COMMAND>\n\nCommands:\n  add   Create a Rule\n  help  Print this message\n\nOptions:\n      --title <TITLE>  Short title\n  -h, --help           Print help\n  -V, --version        Print version\n";
        let (subs, options) = parse_help(help);
        assert_eq!(subs, ["add"]);
        assert_eq!(options, ["--title"]);
    }
}
