//! `rulelink rules approve | reject | obsolete`: the Rule lifecycle.

use rulelink_e2e::{Repo, Run};

fn initialised() -> Repo {
    let repo = Repo::new();
    assert_eq!(repo.run(&["init"]).code, 0);
    repo
}

fn add(repo: &Repo, title: &str) {
    let run = repo.run(&["rules", "add", "--title", title, "--statement", "S."]);
    assert_eq!(run.code, 0, "stderr: {}", run.stderr);
}

/// A repository holding RULE-001 in the given status.
fn with_rule(status: &str) -> Repo {
    let repo = initialised();
    add(&repo, "One");
    match status {
        "proposed" => {}
        "validated" => ok(&repo.run(&["rules", "approve", "RULE-001"])),
        "rejected" => ok(&repo.run(&["rules", "reject", "RULE-001"])),
        "obsolete" => {
            ok(&repo.run(&["rules", "approve", "RULE-001"]));
            ok(&repo.run(&["rules", "obsolete", "RULE-001"]));
        }
        other => panic!("unknown status {other}"),
    }
    repo
}

fn ok(run: &Run) {
    assert_eq!(run.code, 0, "stderr: {}", run.stderr);
}

fn source(repo: &Repo) -> String {
    repo.read("rules/rules.json").expect("source")
}

fn status_of(repo: &Repo) -> String {
    let text = source(repo);
    let line = text
        .lines()
        .find(|l| l.contains("\"status\""))
        .expect("status line");
    line.split('"').nth(3).expect("status value").to_owned()
}

// Documented examples.

#[test]
fn documented_approve_example_validates_the_rule() {
    let repo = with_rule("proposed");
    let run = repo.run(&["rules", "approve", "RULE-001"]);
    ok(&run);
    assert!(run.stdout.contains("RULE-001"), "stdout: {}", run.stdout);
    assert_eq!(status_of(&repo), "validated");
}

#[test]
fn documented_reject_example_rejects_the_rule_and_retires_it() {
    let repo = with_rule("proposed");
    ok(&repo.run(&["rules", "reject", "RULE-001"]));
    assert_eq!(status_of(&repo), "rejected");
    assert!(source(&repo).contains("\"retired\": [\n    \"RULE-001\"\n  ]"));
}

#[test]
fn documented_obsolete_example_records_the_replacement() {
    let repo = with_rule("validated");
    add(&repo, "Two");
    let run = repo.run(&[
        "rules",
        "obsolete",
        "RULE-001",
        "--superseded-by",
        "RULE-002",
    ]);
    ok(&run);
    let text = source(&repo);
    assert!(text.contains("\"status\": \"obsolete\""));
    assert!(text.contains("\"superseded_by\": \"RULE-002\""));
    assert!(text.contains("\"retired\": [\n    \"RULE-001\"\n  ]"));
}

#[test]
fn obsolete_without_replacement_works() {
    let repo = with_rule("validated");
    ok(&repo.run(&["rules", "obsolete", "RULE-001"]));
    assert_eq!(status_of(&repo), "obsolete");
    assert!(!source(&repo).contains("superseded_by"));
}

// Retired Identifiers are never allocated again.

#[test]
fn rejected_identifier_is_not_reallocated() {
    let repo = with_rule("rejected");
    let run = repo.run(&["rules", "add", "--title", "T", "--statement", "S"]);
    assert!(run.stdout.contains("RULE-002"), "stdout: {}", run.stdout);
}

#[test]
fn obsolete_identifier_is_not_reallocated_even_if_the_rule_is_deleted_by_hand() {
    let repo = with_rule("obsolete");
    repo.write(
        "rules/rules.json",
        "{\n  \"format\": 1,\n  \"sequences\": {},\n  \"retired\": [\"RULE-001\"],\n  \"rules\": []\n}\n",
    );
    let run = repo.run(&["rules", "add", "--title", "T", "--statement", "S"]);
    assert!(run.stdout.contains("RULE-002"), "stdout: {}", run.stdout);
}

// Every transition not listed in the ticket is refused and leaves the Source unchanged.

fn refused(from: &str, args: &[&str]) {
    let repo = with_rule(from);
    let before = source(&repo);
    let run = repo.run(args);
    assert_eq!(run.code, 2, "{from} {args:?} stdout: {}", run.stdout);
    assert!(
        run.stderr.contains("RULE-001") && run.stderr.contains(from),
        "message must name the Rule and its status: {}",
        run.stderr
    );
    assert_eq!(source(&repo), before, "{from} {args:?} changed the Source");
}

#[test]
fn approve_is_refused_unless_proposed() {
    for from in ["validated", "rejected", "obsolete"] {
        refused(from, &["rules", "approve", "RULE-001"]);
    }
}

#[test]
fn refusal_message_reads_naturally() {
    let repo = with_rule("rejected");
    let run = repo.run(&["rules", "reject", "RULE-001"]);
    assert!(
        run.stderr
            .contains("cannot reject RULE-001: it is rejected, but a proposed Rule is required"),
        "stderr: {}",
        run.stderr
    );
}

#[test]
fn reject_is_refused_unless_proposed() {
    for from in ["validated", "rejected", "obsolete"] {
        refused(from, &["rules", "reject", "RULE-001"]);
    }
}

#[test]
fn obsolete_is_refused_unless_validated() {
    for from in ["proposed", "rejected", "obsolete"] {
        refused(from, &["rules", "obsolete", "RULE-001"]);
    }
}

// Boundaries.

fn refused_input(repo: &Repo, args: &[&str], needle: &str) {
    let before = source(repo);
    let run = repo.run(args);
    assert_eq!(run.code, 2, "{args:?} stdout: {}", run.stdout);
    assert!(run.stderr.contains(needle), "stderr: {}", run.stderr);
    assert_eq!(source(repo), before);
}

#[test]
fn unknown_rule_is_refused_naming_the_identifier() {
    let repo = with_rule("proposed");
    for action in ["approve", "reject", "obsolete"] {
        refused_input(&repo, &["rules", action, "RULE-099"], "RULE-099");
    }
}

#[test]
fn unknown_replacement_is_refused() {
    let repo = with_rule("validated");
    refused_input(
        &repo,
        &[
            "rules",
            "obsolete",
            "RULE-001",
            "--superseded-by",
            "RULE-099",
        ],
        "RULE-099",
    );
}

#[test]
fn a_rule_cannot_replace_itself() {
    let repo = with_rule("validated");
    refused_input(
        &repo,
        &[
            "rules",
            "obsolete",
            "RULE-001",
            "--superseded-by",
            "RULE-001",
        ],
        "RULE-001",
    );
}

#[test]
fn missing_identifier_is_a_usage_error() {
    let repo = with_rule("proposed");
    assert_eq!(repo.run(&["rules", "approve"]).code, 2);
}

#[test]
fn without_init_the_command_fails_with_a_configuration_error() {
    let repo = Repo::new();
    assert_eq!(repo.run(&["rules", "approve", "RULE-001"]).code, 2);
}
