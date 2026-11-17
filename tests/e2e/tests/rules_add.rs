//! `checkmate rules add`: creating proposed Rules with allocated Identifiers.

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
        "A statement.",
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
        "Line-level VAT rounding",
        "--statement",
        "VAT is rounded on each invoice line, not on the invoice total.",
    ]);
    assert_eq!(run.code, 0, "stderr: {}", run.stderr);
    assert!(run.stdout.contains("RULE-001"), "stdout: {}", run.stdout);
    let source = repo.read("rules/rules.json").expect("source");
    assert_eq!(
        source,
        "{\n  \"format\": 1,\n  \"sequences\": {\n    \"RULE\": 1\n  },\n  \"retired\": [],\n  \"rules\": [\n    {\n      \"id\": \"RULE-001\",\n      \"title\": \"Line-level VAT rounding\",\n      \"statement\": \"VAT is rounded on each invoice line, not on the invoice total.\",\n      \"status\": \"proposed\"\n    }\n  ]\n}\n"
    );
}

#[test]
fn identifiers_increase_and_follow_the_configured_prefix() {
    let repo = initialised(Some("BR"));
    assert!(add(&repo, "One").stdout.contains("BR-001"));
    assert!(add(&repo, "Two").stdout.contains("BR-002"));
    let source = repo.read("rules/rules.json").expect("source");
    assert!(source.contains("\"BR\": 2"));
}

#[test]
fn optional_fields_are_written() {
    let repo = initialised(None);
    add(&repo, "First");
    let run = repo.run(&[
        "rules",
        "add",
        "--title",
        "Second",
        "--statement",
        "S.",
        "--app",
        "billing",
        "--app",
        "web",
        "--module",
        "invoicing",
        "--covered",
        "EUR",
        "--excluded",
        "Credit notes",
        "--related",
        "RULE-001",
        "--origin",
        "jira:BILL-42",
    ]);
    assert_eq!(run.code, 0, "stderr: {}", run.stderr);
    let source = repo.read("rules/rules.json").expect("source");
    for needle in [
        "\"billing\"",
        "\"web\"",
        "\"module\": \"invoicing\"",
        "\"EUR\"",
        "\"Credit notes\"",
        "\"related\": [\n        \"RULE-001\"",
        "\"kind\": \"jira\"",
        "\"ref\": \"BILL-42\"",
    ] {
        assert!(source.contains(needle), "missing {needle} in {source}");
    }
}

#[test]
fn output_is_canonical_even_if_the_source_was_not() {
    let repo = initialised(None);
    repo.write(
        "rules/rules.json",
        "{\"format\":1,\"sequences\":{\"RULE\":2},\"retired\":[],\"rules\":[\
{\"id\":\"RULE-010\",\"title\":\"b\",\"statement\":\"s\",\"status\":\"proposed\"},\
{\"id\":\"RULE-002\",\"title\":\"a\",\"statement\":\"s\",\"status\":\"validated\"}]}",
    );
    let run = add(&repo, "New");
    assert_eq!(run.code, 0, "stderr: {}", run.stderr);
    let source = repo.read("rules/rules.json").expect("source");
    let pos = |id: &str| source.find(id).unwrap_or_else(|| panic!("{id} missing"));
    assert!(pos("RULE-002") < pos("RULE-010"));
    assert!(pos("RULE-010") < pos("RULE-011"));
    assert!(source.ends_with("}\n") && !source.contains('\r'));
}

#[test]
fn a_rule_deleted_by_hand_never_gets_its_identifier_back() {
    let repo = initialised(None);
    add(&repo, "One");
    add(&repo, "Two");
    // Delete RULE-002 from the Source by hand, leaving the sequence untouched.
    let source = repo.read("rules/rules.json").expect("source");
    let start = source
        .find("    {\n      \"id\": \"RULE-002\"")
        .expect("rule two");
    let end = source[start..].find("\n    }").expect("end") + start + "\n    }".len();
    let edited = format!("{}{}", &source[..start - 2], &source[end..]);
    repo.write("rules/rules.json", &edited);
    assert!(!edited.contains("RULE-002"));
    assert!(add(&repo, "Three").stdout.contains("RULE-003"));
}

#[test]
fn deleted_rule_is_not_reused_even_if_the_sequence_was_lost() {
    let repo = initialised(None);
    repo.write(
        "rules/rules.json",
        "{\"format\":1,\"sequences\":{},\"retired\":[\"RULE-007\"],\"rules\":[\
{\"id\":\"RULE-003\",\"title\":\"a\",\"statement\":\"s\",\"status\":\"proposed\"}]}",
    );
    assert!(add(&repo, "New").stdout.contains("RULE-008"));
}

#[test]
fn invalid_input_is_refused_and_the_source_is_untouched() {
    let repo = initialised(None);
    let before = repo.read("rules/rules.json");
    for args in [
        vec!["rules", "add", "--title", "", "--statement", "S."],
        vec![
            "rules",
            "add",
            "--title",
            &"x".repeat(121),
            "--statement",
            "S.",
        ],
        vec!["rules", "add", "--title", "T", "--statement", " "],
        vec![
            "rules",
            "add",
            "--title",
            "T",
            "--statement",
            "S.",
            "--related",
            "RULE-099",
        ],
        vec![
            "rules",
            "add",
            "--title",
            "T",
            "--statement",
            "S.",
            "--origin",
            "nocolon",
        ],
    ] {
        let run = repo.run(&args);
        assert_eq!(run.code, 2, "args {args:?} stderr: {}", run.stderr);
        assert_ne!(run.stderr.trim(), "");
        assert_eq!(repo.read("rules/rules.json"), before);
    }
}

#[test]
fn missing_setup_is_reported() {
    let repo = Repo::new();
    let run = add(&repo, "T");
    assert_eq!(run.code, 2);
    assert!(run.stderr.contains("init"), "stderr: {}", run.stderr);
}

#[test]
fn values_may_be_in_any_language() {
    let repo = initialised(None);
    let run = repo.run(&[
        "rules",
        "add",
        "--title",
        "Arrondi de la TVA",
        "--statement",
        "La TVA est arrondie à la ligne — 税込み。",
    ]);
    assert_eq!(run.code, 0, "stderr: {}", run.stderr);
    let source = repo.read("rules/rules.json").expect("source");
    assert!(source.contains("La TVA est arrondie à la ligne — 税込み。"));
    assert!(run.stderr.is_empty() || run.stderr.is_ascii());
}
