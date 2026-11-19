//! `checkmate rules add`: Rule creation and Identifier allocation.

use checkmate_e2e::Repo;

fn initialised(prefix: Option<&str>) -> Repo {
    let repo = Repo::new();
    let run = match prefix {
        Some(p) => repo.run(&["init", "--id-prefix", p]),
        None => repo.run(&["init"]),
    };
    assert_eq!(run.code, 0, "init failed: {}", run.stderr);
    repo
}

fn add(repo: &Repo, title: &str) -> checkmate_e2e::Run {
    repo.run(&[
        "rules",
        "add",
        "--title",
        title,
        "--statement",
        "Some statement.",
    ])
}

/// The exact example of `crates/cli/docs/reference/rules-add.md`.
#[test]
fn documentation_example_creates_a_proposed_rule() {
    let repo = initialised(None);
    let run = repo.run(&[
        "rules",
        "add",
        "--title",
        "Refund window",
        "--statement",
        "A customer can request a refund within 30 days of delivery.",
    ]);
    assert_eq!(run.code, 0, "stderr: {}", run.stderr);
    assert!(run.stdout.contains("RULE-001"), "stdout: {}", run.stdout);
    let expected = concat!(
        "{\n",
        "  \"format\": 1,\n",
        "  \"sequences\": {\n",
        "    \"RULE\": 1\n",
        "  },\n",
        "  \"retired\": [],\n",
        "  \"rules\": [\n",
        "    {\n",
        "      \"id\": \"RULE-001\",\n",
        "      \"title\": \"Refund window\",\n",
        "      \"statement\": \"A customer can request a refund within 30 days of delivery.\",\n",
        "      \"status\": \"proposed\"\n",
        "    }\n",
        "  ]\n",
        "}\n",
    );
    assert_eq!(repo.read("rules/rules.json").as_deref(), Some(expected));
}

#[test]
fn identifiers_are_allocated_in_sequence() {
    let repo = initialised(None);
    assert!(add(&repo, "One").stdout.contains("RULE-001"));
    assert!(add(&repo, "Two").stdout.contains("RULE-002"));
    let source = repo.read("rules/rules.json").expect("source");
    assert!(source.contains("\"RULE\": 2"));
}

#[test]
fn configured_prefix_is_used() {
    let repo = initialised(Some("BR"));
    let run = add(&repo, "One");
    assert_eq!(run.code, 0, "stderr: {}", run.stderr);
    assert!(run.stdout.contains("BR-001"), "stdout: {}", run.stdout);
}

#[test]
fn optional_fields_are_written() {
    let repo = initialised(None);
    add(&repo, "First");
    let run = repo.run(&[
        "rules",
        "add",
        "--title",
        "Délai",
        "--statement",
        "Le client peut annuler.",
        "--app",
        "billing",
        "--app",
        "shop",
        "--module",
        "payments",
        "--covered",
        "Biens physiques",
        "--excluded",
        "Cartes cadeaux",
        "--related",
        "RULE-001",
        "--origin",
        "jira:SHOP-42",
    ]);
    assert_eq!(run.code, 0, "stderr: {}", run.stderr);
    let source = repo.read("rules/rules.json").expect("source");
    for needle in [
        "\"Délai\"",
        "\"billing\"",
        "\"shop\"",
        "\"payments\"",
        "\"Biens physiques\"",
        "\"Cartes cadeaux\"",
        "\"RULE-001\"",
        "\"jira\"",
        "\"SHOP-42\"",
    ] {
        assert!(source.contains(needle), "missing {needle} in {source}");
    }
}

#[test]
fn output_is_canonical_whatever_the_input_layout() {
    let repo = initialised(None);
    // A compact, unsorted hand edit: RULE-10 listed before RULE-2.
    repo.write(
        "rules/rules.json",
        r#"{"format":1,"sequences":{"RULE":10},"retired":[],"rules":[
{"id":"RULE-10","title":"b","statement":"b","status":"proposed"},
{"id":"RULE-2","title":"a","statement":"a","status":"proposed"}]}"#,
    );
    let run = add(&repo, "c");
    assert_eq!(run.code, 0, "stderr: {}", run.stderr);
    let source = repo.read("rules/rules.json").expect("source");
    let positions: Vec<_> = ["RULE-2\"", "RULE-10\"", "RULE-011\""]
        .iter()
        .map(|id| source.find(id).unwrap_or_else(|| panic!("{id} missing")))
        .collect();
    assert!(positions.windows(2).all(|w| w[0] < w[1]), "{source}");
    assert!(source.ends_with("}\n"));
    assert!(source.contains("\n  \"rules\": [\n    {\n      \"id\": "));
}

#[test]
fn retired_identifier_is_never_reused() {
    let repo = initialised(None);
    repo.write(
        "rules/rules.json",
        "{\n  \"format\": 1,\n  \"sequences\": {},\n  \"retired\": [\"RULE-007\"],\n  \"rules\": []\n}\n",
    );
    let run = add(&repo, "x");
    assert_eq!(run.code, 0, "stderr: {}", run.stderr);
    assert!(run.stdout.contains("RULE-008"), "stdout: {}", run.stdout);
}

#[test]
fn identifier_of_a_rule_deleted_by_hand_is_never_reused() {
    let repo = initialised(None);
    // RULE-002 was allocated (sequence 2) and then removed from the file by hand.
    repo.write(
        "rules/rules.json",
        "{\n  \"format\": 1,\n  \"sequences\": {\n    \"RULE\": 2\n  },\n  \"retired\": [],\n  \"rules\": [\n    {\n      \"id\": \"RULE-001\",\n      \"title\": \"a\",\n      \"statement\": \"a\",\n      \"status\": \"proposed\"\n    }\n  ]\n}\n",
    );
    let run = add(&repo, "Three");
    assert_eq!(run.code, 0, "stderr: {}", run.stderr);
    assert!(run.stdout.contains("RULE-003"), "stdout: {}", run.stdout);
}

#[test]
fn identifier_is_not_reused_when_bookkeeping_was_lost_too() {
    let repo = initialised(None);
    repo.write(
        "rules/rules.json",
        "{\n  \"format\": 1,\n  \"sequences\": {},\n  \"retired\": [],\n  \"rules\": [\n    {\n      \"id\": \"RULE-004\",\n      \"title\": \"a\",\n      \"statement\": \"a\",\n      \"status\": \"validated\"\n    }\n  ]\n}\n",
    );
    let run = add(&repo, "x");
    assert!(run.stdout.contains("RULE-005"), "stdout: {}", run.stdout);
}

#[test]
fn invalid_input_is_refused_and_leaves_the_source_untouched() {
    let repo = initialised(None);
    let before = repo.read("rules/rules.json");
    let cases: [&[&str]; 5] = [
        &[
            "rules",
            "add",
            "--title",
            "t",
            "--statement",
            "s",
            "--app",
            " ",
        ],
        &["rules", "add", "--title", "", "--statement", "s"],
        &["rules", "add", "--title", "t", "--statement", "  "],
        &[
            "rules",
            "add",
            "--title",
            "t",
            "--statement",
            "s",
            "--related",
            "RULE-099",
        ],
        &[
            "rules",
            "add",
            "--title",
            "t",
            "--statement",
            "s",
            "--origin",
            "nocolon",
        ],
    ];
    for args in cases {
        let run = repo.run(args);
        assert_eq!(run.code, 2, "{args:?} should fail");
        assert!(!run.stderr.is_empty(), "{args:?} gave no message");
        assert_eq!(repo.read("rules/rules.json"), before, "{args:?}");
    }
    let long = "x".repeat(121);
    let run = add(&repo, &long);
    assert_ne!(run.code, 0);
    assert!(run.stderr.contains("120"), "stderr: {}", run.stderr);
    assert_eq!(repo.read("rules/rules.json"), before);
}

#[test]
fn missing_title_is_a_usage_error() {
    let repo = initialised(None);
    let run = repo.run(&["rules", "add", "--statement", "s"]);
    assert_eq!(run.code, 2);
}

#[test]
fn without_init_the_command_explains_what_to_do() {
    let repo = Repo::new();
    let run = add(&repo, "x");
    assert_eq!(run.code, 2);
    assert!(run.stderr.contains("init"), "stderr: {}", run.stderr);
}

#[test]
fn corrupt_source_is_refused_and_not_overwritten() {
    let repo = initialised(None);
    repo.write("rules/rules.json", "not json");
    let run = add(&repo, "x");
    assert_eq!(run.code, 3, "stderr: {}", run.stderr);
    assert_eq!(repo.read("rules/rules.json").as_deref(), Some("not json"));
}
