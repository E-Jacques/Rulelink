//! `rulelink where`: list the code locations that cite a Rule.

use rulelink_e2e::{Repo, Run};

fn project() -> Repo {
    let repo = Repo::new();
    assert_eq!(repo.run(&["init"]).code, 0);
    for title in ["One", "Two", "Three"] {
        let run = repo.run(&["rules", "add", "--title", title, "--statement", "S"]);
        assert_eq!(run.code, 0, "{}", run.stderr);
    }
    repo
}

fn lines(run: &Run) -> Vec<&str> {
    run.stdout.lines().collect()
}

fn with_scan_section(repo: &Repo, section: &str) {
    let config = repo.read("rulelink.toml").expect("config");
    repo.write("rulelink.toml", &format!("{config}\n{section}\n"));
}

// Documented example.

#[test]
fn documented_example_lists_files_and_lines_sorted() {
    let repo = project();
    repo.write("tests/pricing_test.rs", "fn t() {}\n// @rule RULE-001\n");
    repo.write(
        "src/pricing.rs",
        "\n\n// @rule RULE-001 -- rounding\nfn f() {}\n",
    );
    let run = repo.run(&["where", "RULE-001"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(lines(&run), ["src/pricing.rs:3", "tests/pricing_test.rs:2"]);
}

// Defaults: a Rule nothing cites.

#[test]
fn an_uncited_rule_says_so() {
    let repo = project();
    let run = repo.run(&["where", "RULE-002"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(lines(&run), ["RULE-002 is not cited by any code"]);
}

// Languages and several Annotations.

#[test]
fn finds_annotations_in_many_comment_styles() {
    let repo = project();
    repo.write("a.py", "x = 1  # @rule RULE-001\n");
    repo.write("b.sql", "-- @rule RULE-001\n");
    repo.write(
        "c.c",
        "/* @rule RULE-001 */\n/**\n * @rule RULE-002, RULE-001\n */\n",
    );
    repo.write("d.html", "<!-- @rule RULE-001 -->\n");
    repo.write("e.rs", "let s = \"@rule RULE-001\";\n");
    let run = repo.run(&["where", "RULE-001"]);
    assert_eq!(
        lines(&run),
        ["a.py:1", "b.sql:1", "c.c:1", "c.c:3", "d.html:1"]
    );
}

// Ignore files and exclusions.

#[test]
fn ignore_files_and_configured_exclusions_are_respected() {
    let repo = project();
    repo.write(".gitignore", "build/\n");
    repo.write(".rulelinkignore", "generated.rs\n");
    repo.write("build/out.rs", "// @rule RULE-001\n");
    repo.write("generated.rs", "// @rule RULE-001\n");
    repo.write("vendor/lib.rs", "// @rule RULE-001\n");
    repo.write("src/ok.rs", "// @rule RULE-001\n");
    with_scan_section(&repo, "[scan]\nexclude = [\"vendor/\"]");
    let run = repo.run(&["where", "RULE-001"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(lines(&run), ["src/ok.rs:1"]);
}

#[test]
fn an_invalid_exclusion_is_a_configuration_error() {
    let repo = project();
    with_scan_section(&repo, "[scan]\nexclude = [\"[\"]");
    let run = repo.run(&["where", "RULE-001"]);
    assert_eq!(run.code, 2);
    assert!(run.stderr.contains('['), "{}", run.stderr);
}

// Unknown, retired.

#[test]
fn an_unknown_identifier_is_refused_by_name() {
    let repo = project();
    let run = repo.run(&["where", "RULE-999"]);
    assert_eq!(run.code, 2);
    assert!(run.stderr.contains("RULE-999"), "{}", run.stderr);
}

#[test]
fn a_retired_rule_is_flagged_and_its_leftover_citations_listed() {
    let repo = project();
    assert_eq!(repo.run(&["rules", "reject", "RULE-003"]).code, 0);
    repo.write("old.rs", "// @rule RULE-003\n");
    let run = repo.run(&["where", "RULE-003"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(
        lines(&run),
        [
            "RULE-003 is rejected: its Identifier is retired",
            "old.rs:1"
        ]
    );
}

#[test]
fn a_blank_identifier_is_refused() {
    let repo = project();
    assert_eq!(repo.run(&["where", " "]).code, 2);
}
