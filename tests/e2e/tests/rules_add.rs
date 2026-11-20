//! `rulelink rules add`: create a proposed Rule with an allocated Identifier.

use rulelink_e2e::Repo;

fn initialised() -> Repo {
    let repo = Repo::new();
    assert_eq!(repo.run(&["init"]).code, 0);
    repo
}

fn add(repo: &Repo, title: &str) -> rulelink_e2e::Run {
    repo.run(&[
        "rules",
        "add",
        "--title",
        title,
        "--statement",
        "A statement.",
    ])
}

#[test]
fn documented_example_creates_a_proposed_rule() {
    let repo = initialised();
    let run = repo.run(&[
        "rules",
        "add",
        "--title",
        "Line-level VAT rounding",
        "--statement",
        "VAT is rounded to the cent on each invoice line, then summed.",
    ]);
    assert_eq!(run.code, 0, "stderr: {}", run.stderr);
    assert!(run.stdout.contains("RULE-001"), "stdout: {}", run.stdout);
    let source = repo.read("rules/rules.json").expect("source");
    assert!(source.contains("\"id\": \"RULE-001\""));
    assert!(source.contains("\"status\": \"proposed\""));
    assert!(source.contains("\"RULE\": 1"));
}

#[test]
fn identifiers_are_allocated_in_sequence() {
    let repo = initialised();
    assert!(add(&repo, "One").stdout.contains("RULE-001"));
    assert!(add(&repo, "Two").stdout.contains("RULE-002"));
}

#[test]
fn custom_prefix_from_configuration_is_used() {
    let repo = Repo::new();
    assert_eq!(repo.run(&["init", "--id-prefix", "BR"]).code, 0);
    assert!(add(&repo, "One").stdout.contains("BR-001"));
}

#[test]
fn source_is_written_in_canonical_form() {
    let repo = initialised();
    add(&repo, "Two words");
    let source = repo.read("rules/rules.json").expect("source");
    let expected = "{\n  \"format\": 1,\n  \"sequences\": {\n    \"RULE\": 1\n  },\n  \"retired\": [],\n  \"rules\": [\n    {\n      \"id\": \"RULE-001\",\n      \"title\": \"Two words\",\n      \"statement\": \"A statement.\",\n      \"status\": \"proposed\"\n    }\n  ]\n}\n";
    assert_eq!(source, expected);
}

#[test]
fn optional_fields_are_stored() {
    let repo = initialised();
    let run = repo.run(&[
        "rules",
        "add",
        "--title",
        "T",
        "--statement",
        "S",
        "--app",
        "billing",
        "--app",
        "web",
        "--module",
        "VAT",
        "--covered",
        "B2B",
        "--excluded",
        "Export",
    ]);
    assert_eq!(run.code, 0, "stderr: {}", run.stderr);
    let source = repo.read("rules/rules.json").expect("source");
    for needle in [
        "\"billing\"",
        "\"web\"",
        "\"module\": \"VAT\"",
        "\"B2B\"",
        "\"Export\"",
    ] {
        assert!(source.contains(needle), "missing {needle} in {source}");
    }
}

#[test]
fn values_may_be_in_any_language() {
    let repo = initialised();
    let run = add(&repo, "Arrondi de la TVA par ligne été 🧾");
    assert_eq!(run.code, 0, "stderr: {}", run.stderr);
    let source = repo.read("rules/rules.json").expect("source");
    assert!(source.contains("Arrondi de la TVA par ligne été 🧾"));
}

#[test]
fn retired_identifier_is_never_allocated() {
    let repo = initialised();
    repo.write(
        "rules/rules.json",
        "{\n  \"format\": 1,\n  \"sequences\": {},\n  \"retired\": [\n    \"RULE-007\"\n  ],\n  \"rules\": []\n}\n",
    );
    assert!(add(&repo, "One").stdout.contains("RULE-008"));
}

#[test]
fn identifier_of_a_rule_deleted_by_hand_is_never_reused() {
    let repo = initialised();
    add(&repo, "One");
    add(&repo, "Two");
    // Delete RULE-002 by hand, leaving `sequences` as the tool wrote it.
    let source = repo.read("rules/rules.json").expect("source");
    let start = source
        .find(",\n    {\n      \"id\": \"RULE-002\"")
        .expect("second rule");
    let end = source.rfind("\n    }").expect("end") + "\n    }".len();
    let edited = format!("{}{}", &source[..start], &source[end..]);
    repo.write("rules/rules.json", &edited);
    assert!(
        !repo
            .read("rules/rules.json")
            .expect("source")
            .contains("RULE-002\"")
    );
    assert!(add(&repo, "Three").stdout.contains("RULE-003"));
}

#[test]
fn sequence_lowered_by_hand_does_not_reuse_a_present_rule() {
    let repo = initialised();
    add(&repo, "One");
    let source = repo.read("rules/rules.json").expect("source");
    repo.write(
        "rules/rules.json",
        &source.replace("\"RULE\": 1", "\"RULE\": 0"),
    );
    assert!(add(&repo, "Two").stdout.contains("RULE-002"));
}

#[test]
fn invalid_input_is_refused_and_the_source_is_untouched() {
    let repo = initialised();
    let before = repo.read("rules/rules.json");
    let long = "x".repeat(121);
    for args in [
        vec!["rules", "add", "--title", "  ", "--statement", "S"],
        vec!["rules", "add", "--title", "T", "--statement", ""],
        vec!["rules", "add", "--title", long.as_str(), "--statement", "S"],
        vec![
            "rules",
            "add",
            "--title",
            "T",
            "--statement",
            "S",
            "--covered",
            " ",
        ],
        vec![
            "rules",
            "add",
            "--title",
            "T",
            "--statement",
            "S",
            "--app",
            "a",
            "--app",
            "a",
        ],
        vec![
            "rules",
            "add",
            "--title",
            "T",
            "--statement",
            "S",
            "--module",
            "",
        ],
    ] {
        let run = repo.run(&args);
        assert_eq!(run.code, 2, "args {args:?}, stderr: {}", run.stderr);
        assert_ne!(run.stderr, "");
        assert_eq!(repo.read("rules/rules.json"), before, "args {args:?}");
    }
}

#[test]
fn missing_required_option_is_a_usage_error() {
    let repo = initialised();
    assert_eq!(repo.run(&["rules", "add", "--title", "T"]).code, 2);
}

#[test]
fn without_init_the_command_explains_what_to_do() {
    let repo = Repo::new();
    let run = add(&repo, "One");
    assert_eq!(run.code, 2);
    assert!(
        run.stderr.contains("rulelink init"),
        "stderr: {}",
        run.stderr
    );
}

#[test]
fn unsupported_source_kind_is_refused() {
    let repo = initialised();
    let config = repo.read("rulelink.toml").expect("config");
    repo.write(
        "rulelink.toml",
        &config.replace("kind = \"json\"", "kind = \"csv\""),
    );
    let run = add(&repo, "One");
    assert_eq!(run.code, 2);
    assert!(run.stderr.contains("csv"), "stderr: {}", run.stderr);
}

#[test]
fn source_path_escaping_the_project_is_refused() {
    let repo = initialised();
    let config = repo.read("rulelink.toml").expect("config");
    repo.write(
        "rulelink.toml",
        &config.replace("rules/rules.json", "../outside.json"),
    );
    let run = add(&repo, "One");
    assert_eq!(run.code, 2);
    assert!(run.stderr.contains("source.path"), "stderr: {}", run.stderr);
}

#[test]
fn corrupted_source_is_reported_and_never_overwritten() {
    let repo = initialised();
    repo.write("rules/rules.json", "{ not json");
    let run = add(&repo, "One");
    assert_eq!(run.code, 3);
    assert!(
        run.stderr.contains("rules/rules.json"),
        "stderr: {}",
        run.stderr
    );
    assert_eq!(repo.read("rules/rules.json").as_deref(), Some("{ not json"));
}

#[test]
fn missing_source_is_reported_with_exit_code_3() {
    let repo = initialised();
    std::fs::remove_file(repo.path().join("rules/rules.json")).expect("remove");
    let run = add(&repo, "One");
    assert_eq!(run.code, 3);
    assert!(run.stderr.contains("not found"), "stderr: {}", run.stderr);
}

#[test]
fn unsupported_format_version_is_refused() {
    let repo = initialised();
    repo.write("rules/rules.json", "{\"format\": 99}\n");
    let run = add(&repo, "One");
    assert_eq!(run.code, 3);
    assert!(run.stderr.contains("format"), "stderr: {}", run.stderr);
}

#[test]
fn source_with_unknown_fields_is_refused_not_rewritten() {
    let repo = initialised();
    let hand_edited = "{\n  \"format\": 1,\n  \"note\": \"mine\",\n  \"rules\": []\n}\n";
    repo.write("rules/rules.json", hand_edited);
    let run = add(&repo, "One");
    assert_eq!(run.code, 3);
    assert_eq!(repo.read("rules/rules.json").as_deref(), Some(hand_edited));
}
