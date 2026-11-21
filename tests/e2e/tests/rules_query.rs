//! `rulelink rules list | search` and `rules add --related`: find Rules and link them.

use rulelink_e2e::{Repo, Run};

fn initialised() -> Repo {
    let repo = Repo::new();
    assert_eq!(repo.run(&["init"]).code, 0);
    repo
}

fn ok(run: &Run) {
    assert_eq!(run.code, 0, "stderr: {}", run.stderr);
}

fn add(repo: &Repo, title: &str, statement: &str) {
    ok(&repo.run(&["rules", "add", "--title", title, "--statement", statement]));
}

fn source(repo: &Repo) -> String {
    repo.read("rules/rules.json").expect("source")
}

/// RULE-001 `validated`, RULE-002 and RULE-003 `proposed`.
fn three_rules() -> Repo {
    let repo = initialised();
    add(&repo, "Arrondi de la TVA", "La TVA est arrondie par ligne.");
    add(&repo, "Late fees", "A late fee applies after 30 days.");
    add(&repo, "Énoncé du devis", "Un devis est valable 30 jours.");
    ok(&repo.run(&["rules", "approve", "RULE-001"]));
    repo
}

fn ids(run: &Run) -> Vec<&str> {
    run.stdout
        .lines()
        .filter_map(|l| l.split_whitespace().next())
        .collect()
}

// Documented examples.

#[test]
fn documented_list_example_shows_only_proposed_rules() {
    let repo = three_rules();
    let run = repo.run(&["rules", "list", "--status", "proposed"]);
    ok(&run);
    assert_eq!(
        ids(&run),
        ["RULE-002", "RULE-003"],
        "stdout: {}",
        run.stdout
    );
    assert!(run.stdout.contains("Late fees"));
}

#[test]
fn documented_search_example_finds_the_rule() {
    let repo = three_rules();
    let run = repo.run(&["rules", "search", "tva"]);
    ok(&run);
    assert_eq!(ids(&run), ["RULE-001"], "stdout: {}", run.stdout);
}

// Defaults.

#[test]
fn list_without_options_shows_every_rule_in_identifier_order() {
    let repo = three_rules();
    let run = repo.run(&["rules", "list"]);
    ok(&run);
    assert_eq!(ids(&run), ["RULE-001", "RULE-002", "RULE-003"]);
    assert!(
        run.stdout
            .lines()
            .next()
            .unwrap_or("")
            .contains("validated")
    );
}

#[test]
fn list_of_an_empty_source_prints_nothing() {
    let run = initialised().run(&["rules", "list"]);
    ok(&run);
    assert_eq!(run.stdout, "");
}

// Search.

#[test]
fn search_ignores_case_and_accents_in_both_directions() {
    let repo = three_rules();
    assert_eq!(ids(&repo.run(&["rules", "search", "ENONCE"])), ["RULE-003"]);
    assert_eq!(ids(&repo.run(&["rules", "search", "énoncé"])), ["RULE-003"]);
    assert_eq!(
        ids(&repo.run(&["rules", "search", "LATE FEE"])),
        ["RULE-002"]
    );
}

#[test]
fn search_looks_in_the_statement_and_can_be_filtered_by_status() {
    let repo = three_rules();
    let all = repo.run(&["rules", "search", "30"]);
    assert_eq!(ids(&all), ["RULE-002", "RULE-003"]);
    let validated = repo.run(&["rules", "search", "30", "--status", "validated"]);
    ok(&validated);
    assert_eq!(validated.stdout, "");
}

#[test]
fn search_without_match_succeeds_and_prints_nothing() {
    let run = three_rules().run(&["rules", "search", "zzz"]);
    ok(&run);
    assert_eq!(run.stdout, "");
}

#[test]
fn blank_search_text_is_refused_with_the_input_exit_code() {
    let run = three_rules().run(&["rules", "search", "   "]);
    assert_eq!(run.code, 2, "stdout: {}", run.stdout);
    assert!(run.stderr.contains("search text"), "stderr: {}", run.stderr);
}

#[test]
fn unknown_status_is_a_usage_error_naming_the_value() {
    let run = three_rules().run(&["rules", "list", "--status", "draft"]);
    assert_ne!(run.code, 0);
    assert!(run.stderr.contains("draft"), "stderr: {}", run.stderr);
    assert!(run.stderr.contains("proposed"), "stderr: {}", run.stderr);
}

// Related.

#[test]
fn a_rule_records_its_related_identifiers_in_canonical_form() {
    let repo = three_rules();
    ok(&repo.run(&[
        "rules",
        "add",
        "--title",
        "T",
        "--statement",
        "S",
        "--related",
        "RULE-001",
        "--related",
        "RULE-003",
    ]));
    let source = source(&repo);
    assert!(
        source.contains("\"related\": [\n        \"RULE-001\",\n        \"RULE-003\"\n      ]"),
        "source: {source}"
    );
}

#[test]
fn an_unknown_related_identifier_is_refused_and_nothing_is_written() {
    let repo = three_rules();
    let before = source(&repo);
    let run = repo.run(&[
        "rules",
        "add",
        "--title",
        "T",
        "--statement",
        "S",
        "--related",
        "RULE-999",
    ]);
    assert_eq!(run.code, 2, "stdout: {}", run.stdout);
    assert!(run.stderr.contains("RULE-999"), "stderr: {}", run.stderr);
    assert_eq!(source(&repo), before);
}

#[test]
fn a_repeated_or_blank_related_identifier_is_refused() {
    let repo = three_rules();
    let dup = repo.run(&[
        "rules",
        "add",
        "--title",
        "T",
        "--statement",
        "S",
        "--related",
        "RULE-001",
        "--related",
        " RULE-001 ",
    ]);
    assert_eq!(dup.code, 2);
    assert!(dup.stderr.contains("RULE-001"), "stderr: {}", dup.stderr);
    let blank = repo.run(&[
        "rules",
        "add",
        "--title",
        "T",
        "--statement",
        "S",
        "--related",
        " ",
    ]);
    assert_eq!(blank.code, 2);
    assert!(
        blank.stderr.contains("--related"),
        "stderr: {}",
        blank.stderr
    );
}
