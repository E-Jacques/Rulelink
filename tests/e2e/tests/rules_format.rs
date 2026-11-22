//! `rulelink rules format`: rewrite the Source into Canonical form.

use std::process::Command;

use rulelink_e2e::{Repo, Run};

fn initialised() -> Repo {
    let repo = Repo::new();
    assert_eq!(repo.run(&["init"]).code, 0);
    repo
}

fn ok(run: &Run) {
    assert_eq!(run.code, 0, "stderr: {}", run.stderr);
}

fn add(repo: &Repo, title: &str) {
    ok(&repo.run(&[
        "rules",
        "add",
        "--title",
        title,
        "--statement",
        "A statement.",
    ]));
}

fn source(repo: &Repo) -> String {
    repo.read("rules/rules.json").expect("source")
}

/// A valid Source, hand-mangled: one line, RULE-10 before RULE-2, odd spacing.
const MANGLED: &str = r#"{"rules":[{"statement":"S10","title":"Ten","id":"RULE-10","status":"proposed"},
   {"title":"Two","status":"proposed","id":"RULE-2","statement":"S2","applications":["b","a"]}],
 "retired":[],   "sequences":{"RULE":10},"format":1}"#;

const CANONICAL: &str = r#"{
  "format": 1,
  "sequences": {
    "RULE": 10
  },
  "retired": [],
  "rules": [
    {
      "id": "RULE-2",
      "title": "Two",
      "statement": "S2",
      "status": "proposed",
      "applications": [
        "b",
        "a"
      ]
    },
    {
      "id": "RULE-10",
      "title": "Ten",
      "statement": "S10",
      "status": "proposed"
    }
  ]
}
"#;

// Documented example.

#[test]
fn documented_example_formats_a_mangled_source_into_the_exact_canonical_form() {
    let repo = initialised();
    repo.write("rules/rules.json", MANGLED);
    let run = repo.run(&["rules", "format"]);
    ok(&run);
    assert_eq!(source(&repo), CANONICAL);
    assert!(run.stdout.contains("rules/rules.json"), "{}", run.stdout);
}

// Defaults: a freshly initialised project.

#[test]
fn formatting_a_fresh_project_changes_nothing() {
    let repo = initialised();
    add(&repo, "One");
    let before = source(&repo);
    let run = repo.run(&["rules", "format"]);
    ok(&run);
    assert_eq!(source(&repo), before);
    assert!(run.stdout.contains("already"), "{}", run.stdout);
}

// Acceptance criteria.

#[test]
fn second_run_changes_nothing() {
    let repo = initialised();
    repo.write("rules/rules.json", MANGLED);
    ok(&repo.run(&["rules", "format"]));
    let first = source(&repo);
    let second = repo.run(&["rules", "format"]);
    ok(&second);
    assert_eq!(source(&repo), first);
    assert!(second.stdout.contains("already"), "{}", second.stdout);
}

#[test]
fn identifiers_are_ordered_naturally() {
    let repo = initialised();
    repo.write("rules/rules.json", MANGLED);
    ok(&repo.run(&["rules", "format"]));
    let text = source(&repo);
    let two = text.find("RULE-2\"").expect("RULE-2");
    let ten = text.find("RULE-10\"").expect("RULE-10");
    assert!(two < ten);
}

#[test]
fn a_file_written_by_rulelink_is_byte_identical_after_format() {
    let repo = initialised();
    for title in ["One", "Two", "Three"] {
        add(&repo, title);
    }
    ok(&repo.run(&["rules", "approve", "RULE-002"]));
    ok(&repo.run(&["rules", "reject", "RULE-003"]));
    let before = source(&repo);
    ok(&repo.run(&["rules", "format"]));
    assert_eq!(source(&repo), before);
}

#[test]
fn crlf_line_endings_are_normalised() {
    let repo = initialised();
    add(&repo, "One");
    let crlf = source(&repo).replace('\n', "\r\n");
    repo.write("rules/rules.json", &crlf);
    ok(&repo.run(&["rules", "format"]));
    assert!(!source(&repo).contains('\r'));
}

// Failures leave the file as it was.

#[test]
fn invalid_json_is_refused_and_left_untouched() {
    let repo = initialised();
    let broken = "<<<<<<< HEAD\n{ \"format\": 1 }\n=======\n>>>>>>> other\n";
    repo.write("rules/rules.json", broken);
    let run = repo.run(&["rules", "format"]);
    assert_eq!(run.code, 3, "stderr: {}", run.stderr);
    assert!(run.stderr.contains("rules/rules.json"), "{}", run.stderr);
    assert_eq!(source(&repo), broken);
}

#[test]
fn a_foreign_format_is_refused_and_left_untouched() {
    let repo = initialised();
    let foreign = "{\"format\": 9, \"other\": 1}";
    repo.write("rules/rules.json", foreign);
    let run = repo.run(&["rules", "format"]);
    assert_eq!(run.code, 3, "stderr: {}", run.stderr);
    assert!(run.stderr.contains("format 9"), "{}", run.stderr);
    assert_eq!(source(&repo), foreign);
}

#[test]
fn a_missing_source_is_reported_with_its_path() {
    let repo = initialised();
    std::fs::remove_file(repo.path().join("rules/rules.json")).expect("remove");
    let run = repo.run(&["rules", "format"]);
    assert_eq!(run.code, 3, "stderr: {}", run.stderr);
    assert!(run.stderr.contains("rules.json"), "{}", run.stderr);
}

#[test]
fn without_init_the_missing_configuration_is_reported() {
    let run = Repo::new().run(&["rules", "format"]);
    assert_eq!(run.code, 2, "stderr: {}", run.stderr);
    assert!(run.stderr.contains("rulelink.toml"), "{}", run.stderr);
}

#[test]
fn an_unexpected_argument_is_a_usage_error() {
    let repo = initialised();
    assert_eq!(repo.run(&["rules", "format", "other.json"]).code, 2);
}

// Merge behaviour (ADR-0003).

fn git(repo: &Repo, args: &[&str]) -> std::process::Output {
    Command::new("git")
        .args(["-c", "user.name=t", "-c", "user.email=t@example.com"])
        .args([
            "-c",
            "commit.gpgsign=false",
            "-c",
            "init.defaultBranch=main",
        ])
        .args(args)
        .current_dir(repo.path())
        .output()
        .expect("git runs")
}

fn git_ok(repo: &Repo, args: &[&str]) {
    let out = git(repo, args);
    assert!(
        out.status.success(),
        "git {args:?}: {}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn two_independent_edits_to_different_rules_merge_without_conflict() {
    let repo = initialised();
    for title in ["One", "Two", "Three"] {
        add(&repo, title);
    }
    git_ok(&repo, &["init", "-q"]);
    git_ok(&repo, &["add", "-A"]);
    git_ok(&repo, &["commit", "-q", "-m", "base"]);

    git_ok(&repo, &["checkout", "-q", "-b", "left"]);
    ok(&repo.run(&["rules", "approve", "RULE-001"]));
    git_ok(&repo, &["commit", "-q", "-am", "approve 1"]);

    git_ok(&repo, &["checkout", "-q", "main"]);
    git_ok(&repo, &["checkout", "-q", "-b", "right"]);
    ok(&repo.run(&["rules", "approve", "RULE-003"]));
    git_ok(&repo, &["commit", "-q", "-am", "approve 3"]);

    git_ok(&repo, &["merge", "-q", "--no-edit", "left"]);

    let merged = source(&repo);
    assert!(!merged.contains("<<<<<<<"), "{merged}");
    let run = repo.run(&["rules", "format"]);
    ok(&run);
    assert_eq!(
        source(&repo),
        merged,
        "the merge result is already canonical"
    );
    assert_eq!(merged.matches("\"validated\"").count(), 2);
}
