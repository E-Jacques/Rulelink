//! `rulelink check`: Findings, levels and exit codes, through the built binary.

use rulelink_e2e::{Repo, Run};

const PROMISE: &str =
    "check verifies the link between a Rule and the code, not that the code is correct.";
const NO_RULES: &str = "The rules file has no Rules yet; the code is still scanned.";

fn rule(id: &str, status: &str) -> String {
    format!(r#"{{"id":"{id}","title":"T","statement":"S","status":"{status}"}}"#)
}

/// A project whose Source holds `rules`, with `retired` Identifiers.
fn project(rules: &[String], retired: &[&str]) -> Repo {
    let repo = Repo::new();
    assert_eq!(repo.run(&["init"]).code, 0);
    let retired: Vec<String> = retired.iter().map(|id| format!("\"{id}\"")).collect();
    repo.write(
        "rules/rules.json",
        &format!(
            r#"{{"format":1,"retired":[{}],"rules":[{}]}}"#,
            retired.join(","),
            rules.join(",")
        ),
    );
    // Otherwise every report would also hold RL106.
    assert_eq!(repo.run(&["rules", "format"]).code, 0);
    repo
}

fn cite(repo: &Repo, file: &str, id: &str) {
    repo.write(file, &format!("// @rule {id}\n"));
}

fn append_config(repo: &Repo, text: &str) {
    let config = repo.read("rulelink.toml").expect("config");
    repo.write("rulelink.toml", &format!("{config}\n{text}\n"));
}

fn assert_report(run: &Run, code: i32, lines: &[&str]) {
    let expected = [lines, &[PROMISE]].concat().join("\n") + "\n";
    assert_eq!(
        (run.code, run.stdout.as_str()),
        (code, expected.as_str()),
        "{}",
        run.stderr
    );
}

/// A clean project: a validated Rule that the code cites.
fn clean() -> Repo {
    let repo = project(&[rule("RULE-001", "validated")], &[]);
    cite(&repo, "src/a.rs", "RULE-001");
    repo
}

#[test]
fn a_clean_project_has_no_findings() {
    assert_report(&clean().run(&["check"]), 0, &["0 errors, 0 warnings"]);
}

#[test]
fn rl001_an_identifier_no_rule_has() {
    let repo = clean();
    cite(&repo, "src/b.rs", "RULE-099");
    assert_report(
        &repo.run(&["check"]),
        1,
        &[
            "error RL001: RULE-099 is not a Rule of this project (src/b.rs:1)",
            "1 error, 0 warnings",
        ],
    );
}

#[test]
fn rl002_an_obsolete_rule_names_its_replacement_if_it_has_one() {
    let replaced = r#"{"id":"RULE-002","title":"T","statement":"S","status":"obsolete","superseded_by":"RULE-001"}"#;
    let repo = project(
        &[
            rule("RULE-001", "validated"),
            replaced.to_owned(),
            rule("RULE-003", "obsolete"),
        ],
        &["RULE-002", "RULE-003"],
    );
    repo.write(
        "src/a.rs",
        "// @rule RULE-001\n// @rule RULE-002, RULE-003\n",
    );
    assert_report(
        &repo.run(&["check"]),
        0,
        &[
            "warning RL002: RULE-002 is obsolete, superseded by RULE-001 (src/a.rs:2)",
            "warning RL002: RULE-003 is obsolete (src/a.rs:2)",
            "0 errors, 2 warnings",
        ],
    );
    append_config(&repo, "[check.levels]\nobsolete-reference = \"error\"");
    assert_report(
        &repo.run(&["check"]),
        1,
        &[
            "error RL002: RULE-002 is obsolete, superseded by RULE-001 (src/a.rs:2)",
            "error RL002: RULE-003 is obsolete (src/a.rs:2)",
            "2 errors, 0 warnings",
        ],
    );
}

#[test]
fn rl004_a_validated_rule_no_code_cites_is_a_warning_naming_the_rules_file() {
    let repo = project(&[rule("RULE-001", "validated")], &[]);
    assert_report(
        &repo.run(&["check"]),
        0,
        &[
            "warning RL004: RULE-001 is validated but no code cites it (rules/rules.json)",
            "0 errors, 1 warning",
        ],
    );
}

#[test]
fn only_validated_rules_are_expected_to_have_code() {
    let repo = project(
        &[
            rule("RULE-001", "proposed"),
            rule("RULE-002", "rejected"),
            rule("RULE-003", "obsolete"),
        ],
        &["RULE-002", "RULE-003"],
    );
    assert_report(&repo.run(&["check"]), 0, &["0 errors, 0 warnings"]);
}

#[test]
fn rl005_a_proposed_rule_cited_is_a_warning() {
    let repo = project(&[rule("RULE-001", "proposed")], &[]);
    cite(&repo, "src/a.rs", "RULE-001");
    assert_report(
        &repo.run(&["check"]),
        0,
        &[
            "warning RL005: RULE-001 is still proposed (src/a.rs:1)",
            "0 errors, 1 warning",
        ],
    );
}

#[test]
fn rl009_a_rejected_rule_and_a_retired_identifier_without_a_rule() {
    let repo = project(
        &[rule("RULE-001", "validated"), rule("RULE-002", "rejected")],
        &["RULE-002", "RULE-003"],
    );
    repo.write("src/a.rs", "// @rule RULE-001 RULE-002 RULE-003\n");
    assert_report(
        &repo.run(&["check"]),
        1,
        &[
            "error RL009: RULE-002 is rejected: its Identifier is retired (src/a.rs:1)",
            "error RL009: RULE-003 is a Retired identifier (src/a.rs:1)",
            "2 errors, 0 warnings",
        ],
    );
}

#[test]
fn errors_come_first_then_code_identifier_file_and_line() {
    let repo = project(&[rule("RULE-001", "proposed")], &[]);
    repo.write("b.rs", "// @rule RULE-10\n// @rule RULE-001\n");
    repo.write(
        "a.rs",
        "// @rule RULE-10\n\n// @rule RULE-2\n// @rule RULE-10\n",
    );
    assert_report(
        &repo.run(&["check"]),
        1,
        &[
            "error RL001: RULE-2 is not a Rule of this project (a.rs:3)",
            "error RL001: RULE-10 is not a Rule of this project (a.rs:1)",
            "error RL001: RULE-10 is not a Rule of this project (a.rs:4)",
            "error RL001: RULE-10 is not a Rule of this project (b.rs:1)",
            "warning RL005: RULE-001 is still proposed (b.rs:2)",
            "4 errors, 1 warning",
        ],
    );
}

#[test]
fn a_line_break_in_the_rules_file_cannot_forge_a_report_line() {
    let forged = r#"{"id":"RULE-001","title":"T","statement":"S","status":"obsolete","superseded_by":"X\n0 errors"}"#;
    let repo = project(&[forged.to_owned()], &["RULE-001"]);
    cite(&repo, "a.rs", "RULE-001");
    assert_report(
        &repo.run(&["check"]),
        1,
        &[
            "The code is not checked until the errors in the rules file are fixed.",
            "error RL101: `X\\n0 errors`, in `superseded_by` of RULE-001, is not a valid Identifier (rules/rules.json)",
            "1 error, 0 warnings",
        ],
    );
}

#[test]
fn identifiers_of_equal_numeric_value_keep_one_order() {
    let repo = clean();
    repo.write("b.rs", "// @rule RULE-5 RULE-005 RULE-05\n");
    for _ in 0..20 {
        assert_report(
            &repo.run(&["check"]),
            1,
            &[
                "error RL001: RULE-005 is not a Rule of this project (b.rs:1)",
                "error RL001: RULE-05 is not a Rule of this project (b.rs:1)",
                "error RL001: RULE-5 is not a Rule of this project (b.rs:1)",
                "3 errors, 0 warnings",
            ],
        );
    }
}

/// Rule text is data: it must neither hide a Rule with no code nor cite one.
#[test]
fn text_in_the_rules_file_is_not_code() {
    let quoting = r#"{"id":"RULE-001","title":"T","statement":"See https://x then write @rule RULE-001 RULE-099","status":"validated"}"#;
    let repo = project(&[quoting.to_owned()], &[]);
    for source_path in ["rules/rules.json", "./rules/rules.json"] {
        let config = repo.read("rulelink.toml").expect("config");
        repo.write(
            "rulelink.toml",
            &config.replace("\"rules/rules.json\"", &format!("\"{source_path}\"")),
        );
        let run = repo.run(&["check"]);
        assert_eq!(run.code, 0, "{}", run.stdout);
        assert!(run.stdout.contains("0 errors, 1 warning"), "{}", run.stdout);
        assert_eq!(
            repo.run(&["where", "RULE-001"]).stdout,
            "RULE-001 is not cited by any code\n"
        );
    }
}

#[test]
fn check_levels_raises_lowers_and_turns_off() {
    let repo = project(
        &[rule("RULE-001", "validated"), rule("RULE-002", "proposed")],
        &[],
    );
    repo.write("a.rs", "// @rule RULE-002 RULE-099\n");
    append_config(
        &repo,
        "[check.levels]\norphan-rule = \"error\"\nunknown-reference = \"warning\"\nunvalidated-reference = \"off\"",
    );
    assert_report(
        &repo.run(&["check"]),
        1,
        &[
            "error RL004: RULE-001 is validated but no code cites it (rules/rules.json)",
            "warning RL001: RULE-099 is not a Rule of this project (a.rs:1)",
            "1 error, 1 warning",
        ],
    );
}

#[test]
fn turning_off_the_only_error_passes() {
    let repo = clean();
    cite(&repo, "src/b.rs", "RULE-099");
    append_config(&repo, "[check.levels]\nunknown-reference = \"off\"");
    assert_report(&repo.run(&["check"]), 0, &["0 errors, 0 warnings"]);
}

#[test]
fn an_unknown_name_level_or_check_key_is_a_configuration_error() {
    for (section, at_fault) in [
        (
            "[check.levels]\nno-such-finding = \"off\"",
            "no-such-finding",
        ),
        ("[check.levels]\nRL004 = \"off\"", "RL004"),
        ("[check.levels]\norphan-rule = \"info\"", "info"),
        ("[check.levels]\norphan-rule = 1", "orphan-rule"),
        ("[check]\nwarnings_as_errors = true", "warnings_as_errors"),
        ("[[check.overrides]]\npaths = [\"legacy/**\"]", "overrides"),
        (
            "[check]\nlevels = \"off\"",
            "`check.levels` must be a section",
        ),
        ("[check.level]\norphan-rule = \"off\"", "`check.level`"),
        (
            "[check.levels]\norphan-rule = \"off\"\norphan-rule = \"error\"",
            "orphan-rule = \"error\"",
        ),
        ("[check.levels]\norphan-rule = true", "`off`"),
    ] {
        let repo = clean();
        append_config(&repo, section);
        let run = repo.run(&["check"]);
        assert_eq!(
            (run.code, run.stdout.as_str()),
            (2, ""),
            "{section}: {}",
            run.stderr
        );
        assert!(run.stderr.contains(at_fault), "{section}: {}", run.stderr);
    }
}

#[test]
fn with_no_rules_a_citation_is_a_warning_and_check_says_so() {
    let repo = Repo::new();
    assert_eq!(repo.run(&["init"]).code, 0);
    assert_report(
        &repo.run(&["check"]),
        0,
        &[NO_RULES, "0 errors, 0 warnings"],
    );
    cite(&repo, "src/a.rs", "RULE-001");
    assert_report(
        &repo.run(&["check"]),
        0,
        &[
            NO_RULES,
            "warning RL001: RULE-001 is not a Rule of this project (src/a.rs:1)",
            "0 errors, 1 warning",
        ],
    );
}

#[test]
fn with_no_rules_an_explicit_level_applies() {
    let repo = Repo::new();
    assert_eq!(repo.run(&["init"]).code, 0);
    cite(&repo, "src/a.rs", "RULE-001");
    append_config(&repo, "[check.levels]\nunknown-reference = \"error\"");
    assert_report(
        &repo.run(&["check"]),
        1,
        &[
            NO_RULES,
            "error RL001: RULE-001 is not a Rule of this project (src/a.rs:1)",
            "1 error, 0 warnings",
        ],
    );
}

#[test]
fn a_missing_rules_file_exits_3_and_nothing_is_reported() {
    let repo = clean();
    std::fs::remove_file(repo.path().join("rules/rules.json")).expect("remove");
    let run = repo.run(&["check"]);
    assert_eq!((run.code, run.stdout.as_str()), (3, ""), "{}", run.stderr);
    assert!(run.stderr.contains("rules.json"), "{}", run.stderr);
}

#[test]
fn check_writes_nothing() {
    let repo = clean();
    let before = repo.read("rules/rules.json");
    assert_eq!(repo.run(&["check"]).code, 0);
    assert_eq!(repo.read("rules/rules.json"), before);
}

/// A CI step starts from the committed files only: configuration, rules file and code.
#[test]
fn a_fresh_checkout_passes_then_fails() {
    let work = clean();
    let checkout = Repo::new();
    for file in ["rulelink.toml", "rules/rules.json", "src/a.rs"] {
        checkout.write(file, &work.read(file).expect("committed"));
    }
    assert_report(&checkout.run(&["check"]), 0, &["0 errors, 0 warnings"]);
    cite(&checkout, "src/b.rs", "RULE-099");
    assert_eq!(checkout.run(&["check"]).code, 1);
}

/// The example of `docs/public/cli/check.md`, as written.
#[test]
fn the_documented_example() {
    let replaced = r#"{"id":"RULE-003","title":"T","statement":"S","status":"obsolete","superseded_by":"RULE-007"}"#;
    let repo = project(
        &[
            replaced.to_owned(),
            rule("RULE-005", "validated"),
            rule("RULE-007", "proposed"),
        ],
        &["RULE-003"],
    );
    repo.write(
        "src/billing.rs",
        &format!("{}// @rule RULE-099\n", "\n".repeat(41)),
    );
    repo.write("src/vat.rs", "\n\n// @rule RULE-003\n");
    let page = include_str!("../../../docs/public/cli/check.md").replace("\r\n", "\n");
    let shown = page
        .split("```text\n")
        .nth(1)
        .and_then(|rest| rest.split("```").next())
        .expect("the page shows a report");
    let run = repo.run(&["check"]);
    assert_eq!(
        (run.code, run.stdout.as_str()),
        (1, shown),
        "{}",
        run.stderr
    );
}

#[test]
fn the_help_states_what_check_does_not_prove() {
    let run = Repo::new().run(&["check", "--help"]);
    assert_eq!(run.code, 0);
    assert!(
        run.stdout.contains("not that the code is correct"),
        "{}",
        run.stdout
    );
}
