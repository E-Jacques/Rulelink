//! `rulelink check` on the rules file itself: RL003, RL100–RL103, RL105 and RL106.

use rulelink_e2e::{Repo, Run};
use serde_json::{Value, json};

const PROMISE: &str =
    "check verifies the link between a Rule and the code, not that the code is correct.";
const NOT_CHECKED: &str = "The code is not checked until the errors in the rules file are fixed.";
const NOT_CANONICAL: &str = "warning RL106: the rules file is not in Canonical form: run `rulelink rules format` (rules/rules.json)";

fn rule(id: &str, status: &str) -> Value {
    json!({"id": id, "title": "T", "statement": "S", "status": status})
}

fn with(mut rule: Value, field: &str, value: Value) -> Value {
    rule[field] = value;
    rule
}

/// A project whose code cites an Identifier no Rule has, so that any code check shows.
fn project(text: &str) -> Repo {
    let repo = Repo::new();
    assert_eq!(repo.run(&["init"]).code, 0);
    repo.write("rules/rules.json", text);
    repo.write("a.rs", "// @rule RULE-099\n");
    repo
}

/// [`project`] with a rules file in Canonical form.
fn canonical(rules: &[Value], retired: &[&str]) -> Repo {
    let repo = project(&json!({"format": 1, "retired": retired, "rules": rules}).to_string());
    assert_eq!(repo.run(&["rules", "format"]).code, 0);
    repo
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

/// The report of a rules file whose only problem is the error `line`.
fn assert_stops(repo: &Repo, lines: &[&str]) {
    let totals = format!(
        "{} error{}, 0 warnings",
        lines.len(),
        if lines.len() == 1 { "" } else { "s" }
    );
    let expected = [&[NOT_CHECKED], lines, &[totals.as_str()]].concat();
    assert_report(&repo.run(&["check"]), 1, &expected);
}

#[test]
fn rl003_two_rules_with_one_identifier_as_after_a_bad_merge() {
    let repo = canonical(
        &[
            rule("RULE-001", "validated"),
            rule("RULE-001", "proposed"),
            rule("RULE-001", "validated"),
        ],
        &[],
    );
    assert_stops(
        &repo,
        &["error RL003: RULE-001 is the Identifier of 3 Rules (rules/rules.json)"],
    );
}

/// Valid JSON that is not a Source: the text, the line named after the file (if any) and
/// what the message must name.
const WRONG_SHAPES: [(&str, &str, &str); 10] = [
    ("{\"format\":1,\n\"mine\":true}", ":2", "mine"),
    (
        "{\"format\":1,\"rules\":[\n{\"id\":\"RULE-001\",\"title\":\"T\",\"statement\":\"S\",\"status\":\"draft\"}]}",
        ":2",
        "draft",
    ),
    (
        "{\"format\":1,\"rules\":[{\"id\":\"RULE-001\",\"statement\":\"S\",\"status\":\"proposed\"}]}",
        ":1",
        "missing field `title`",
    ),
    ("{\"rules\":[]}", ":1", "missing field `format`"),
    ("{\"format\":\"1\"}", ":1", "invalid type"),
    (
        "{\"format\":1,\"rules\":[],\"rules\":[]}",
        ":1",
        "duplicate field",
    ),
    // Read by position, these would be accepted and rewritten by `rules format`.
    ("[1]", "", "the rules file must be an object"),
    (
        "{\"format\":1,\"rules\":[[\"RULE-001\",\"T\",\"S\",\"proposed\"]]}",
        "",
        "`rules[0]` must be an object",
    ),
    (
        "{\"format\":1,\"rules\":[{\"id\":\"RULE-001\",\"title\":\"T\",\"statement\":\"S\",\"status\":{\"proposed\":null}}]}",
        "",
        "`rules[0].status` must be a string",
    ),
    (
        "{\"format\":1,\"rules\":[{\"id\":\"RULE-001\",\"title\":\"T\",\"statement\":\"S\",\"status\":\"proposed\",\"origin\":[\"jira\"]}]}",
        "",
        "`rules[0].origin` must be an object",
    ),
];

#[test]
fn rl100_valid_json_of_the_wrong_shape_is_a_finding_with_its_line() {
    for (text, line, named) in WRONG_SHAPES {
        let run = project(text).run(&["check"]);
        let lines: Vec<&str> = run.stdout.lines().collect();
        assert_eq!(run.code, 1, "{text}: {}{}", run.stdout, run.stderr);
        assert_eq!(lines.len(), 4, "{text}: {}", run.stdout);
        assert_eq!(
            (lines[0], lines[2]),
            (NOT_CHECKED, "1 error, 0 warnings"),
            "{text}"
        );
        assert!(
            lines[1].starts_with("error RL100: the rules file has the wrong shape: ")
                && lines[1].ends_with(&format!("(rules/rules.json{line})"))
                && lines[1].contains(named),
            "{text}: {}",
            lines[1]
        );
    }
}

#[test]
fn rl101_a_malformed_identifier_wherever_the_file_holds_one() {
    let broken = with(
        with(rule("rule-1", "obsolete"), "related", json!(["RULE 2"])),
        "superseded_by",
        json!("RULE-"),
    );
    let run = canonical(&[broken], &["RULE-04a"]).run(&["check"]);
    assert_report(
        &run,
        1,
        &[
            NOT_CHECKED,
            "error RL101: `RULE-04a`, in `retired`, is not a valid Identifier (rules/rules.json)",
            "error RL101: `rule-1`, in `id`, is not a valid Identifier (rules/rules.json)",
            "error RL101: `RULE 2`, in `related` of rule-1, is not a valid Identifier (rules/rules.json)",
            "error RL101: `RULE-`, in `superseded_by` of rule-1, is not a valid Identifier (rules/rules.json)",
            "warning RL103: rule-1 is related to RULE 2, which is not a Rule of this project (rules/rules.json)",
            "4 errors, 1 warning",
        ],
    );
}

#[test]
fn rl101_does_not_depend_on_the_configured_prefix() {
    let repo = canonical(&[rule("BR-7", "proposed")], &["PRICING_V2-A1"]);
    let run = repo.run(&["check"]);
    assert!(!run.stdout.contains("RL101"), "{}", run.stdout);
}

#[test]
fn rl102_a_live_rule_with_a_retired_identifier() {
    for status in ["proposed", "validated"] {
        let repo = canonical(&[rule("RULE-001", status)], &["RULE-001"]);
        assert_stops(
            &repo,
            &[&format!(
                "error RL102: RULE-001 is retired but its Rule is {status} (rules/rules.json)"
            )],
        );
    }
}

#[test]
fn rl103_an_unknown_related_rule_is_a_warning_and_the_code_is_still_checked() {
    let related = with(
        rule("RULE-001", "proposed"),
        "related",
        json!(["RULE-007", "RULE-007"]),
    );
    assert_report(
        &canonical(&[related], &[]).run(&["check"]),
        1,
        &[
            "error RL001: RULE-099 is not a Rule of this project (a.rs:1)",
            "warning RL103: RULE-001 is related to RULE-007, which is not a Rule of this project (rules/rules.json)",
            "1 error, 1 warning",
        ],
    );
}

#[test]
fn rl105_an_empty_required_field() {
    for field in ["title", "statement"] {
        let repo = canonical(
            &[with(rule("RULE-001", "proposed"), field, json!(" \t"))],
            &[],
        );
        assert_stops(
            &repo,
            &[&format!(
                "error RL105: RULE-001 has an empty `{field}` (rules/rules.json)"
            )],
        );
    }
    let origin = with(rule("RULE-001", "proposed"), "origin", json!({"kind": ""}));
    assert_stops(
        &canonical(&[origin], &[]),
        &["error RL105: RULE-001 has an empty `origin.kind` (rules/rules.json)"],
    );
    assert_stops(
        &canonical(&[rule("", "proposed")], &[]),
        &["error RL105: the Rule titled `T` has an empty `id` (rules/rules.json)"],
    );
}

#[test]
fn rl106_disappears_after_rules_format() {
    let source = json!({"format": 1, "rules": [rule("RULE-001", "proposed")]});
    let repo = project(&source.to_string());
    repo.write("a.rs", "// @rule RULE-001\n");
    assert_report(
        &repo.run(&["check"]),
        0,
        &[
            "warning RL005: RULE-001 is still proposed (a.rs:1)",
            NOT_CANONICAL,
            "0 errors, 2 warnings",
        ],
    );
    assert_eq!(repo.run(&["rules", "format"]).code, 0);
    assert_report(
        &repo.run(&["check"]),
        0,
        &[
            "warning RL005: RULE-001 is still proposed (a.rs:1)",
            "0 errors, 1 warning",
        ],
    );
}

#[test]
fn crlf_line_endings_are_not_canonical() {
    let repo = canonical(&[], &[]);
    let text = repo.read("rules/rules.json").expect("source");
    repo.write("rules/rules.json", &text.replace('\n', "\r\n"));
    let run = repo.run(&["check"]);
    assert!(run.stdout.contains("RL106"), "{}", run.stdout);
}

fn set_level(repo: &Repo, name: &str, level: &str) {
    let config = repo.read("rulelink.toml").expect("config");
    repo.write(
        "rulelink.toml",
        &format!("{config}\n[check.levels]\n{name} = \"{level}\"\n"),
    );
}

#[test]
fn the_warnings_can_be_raised_without_stopping_the_code_checks_or_turned_off() {
    let related = with(rule("RULE-001", "proposed"), "related", json!(["RULE-007"]));
    let source = json!({"format": 1, "rules": [related]}).to_string();
    for name in ["unknown-related", "not-canonical"] {
        let repo = project(&source);
        set_level(&repo, name, "error");
        let run = repo.run(&["check"]);
        assert_eq!(run.code, 1, "{name}");
        assert!(run.stdout.contains("RL001"), "{name}: {}", run.stdout);
        assert!(!run.stdout.contains(NOT_CHECKED), "{name}: {}", run.stdout);
        assert!(run.stdout.contains("2 errors, 1 warning"), "{}", run.stdout);

        let repo = project(&source);
        set_level(&repo, name, "off");
        let run = repo.run(&["check"]);
        assert!(run.stdout.contains("1 error, 1 warning"), "{}", run.stdout);
    }
}

#[test]
fn a_source_error_cannot_be_lowered_or_turned_off() {
    for name in [
        "duplicate-id",
        "invalid-shape",
        "invalid-identifier",
        "reused-retired-id",
        "empty-field",
    ] {
        for level in ["warning", "off"] {
            let repo = canonical(&[], &[]);
            set_level(&repo, name, level);
            let run = repo.run(&["check"]);
            assert_eq!((run.code, run.stdout.as_str()), (2, ""), "{name} {level}");
            assert!(
                run.stderr.contains(&format!("check.levels.{name}")),
                "{}",
                run.stderr
            );
        }
        let repo = canonical(&[], &[]);
        set_level(&repo, name, "error");
        let run = repo.run(&["check"]);
        assert_eq!(run.code, 0, "{name} may be set to error: {}", run.stderr);
    }
}

/// As after a bad merge: keeping one of the two would let `rules format` lower the counter,
/// and the Identifier of a deleted Rule be allocated again.
#[test]
fn a_repeated_sequence_is_a_wrong_shape_that_rules_format_leaves_alone() {
    let text = "{\"format\":1,\"sequences\":{\"RULE\":9,\"RULE\":1}}";
    let repo = project(text);
    let run = repo.run(&["check"]);
    assert!(run.stdout.contains("error RL100: "), "{}", run.stdout);
    assert!(run.stdout.contains("`RULE`"), "{}", run.stdout);
    assert_eq!(repo.run(&["rules", "format"]).code, 3);
    assert_eq!(repo.read("rules/rules.json").as_deref(), Some(text));
}

#[test]
fn rules_that_cannot_be_told_apart_are_each_counted() {
    let unnamed = with(rule("", "proposed"), "title", json!(""));
    assert_stops(
        &canonical(&[unnamed.clone(), unnamed], &[]),
        &[
            "error RL105: the Rule at `rules[0]` has an empty `id` (rules/rules.json)",
            "error RL105: the Rule at `rules[0]` has an empty `title` (rules/rules.json)",
            "error RL105: the Rule at `rules[1]` has an empty `id` (rules/rules.json)",
            "error RL105: the Rule at `rules[1]` has an empty `title` (rules/rules.json)",
        ],
    );
}

#[test]
fn an_entry_repeated_in_one_list_is_reported_once() {
    let related = with(
        rule("RULE-001", "proposed"),
        "related",
        json!(["rule 2", "rule 2"]),
    );
    let run = canonical(&[related], &["bad", "bad"]).run(&["check"]);
    assert!(run.stdout.contains("2 errors, 1 warning"), "{}", run.stdout);
}

#[test]
fn what_is_not_a_finding_exits_3() {
    for text in [
        "<<<<<<< HEAD",
        "{\"format\":1",
        "{\"format\":2,\"mine\":1}",
        "",
    ] {
        let repo = project(text);
        let run = repo.run(&["check"]);
        assert_eq!((run.code, run.stdout.as_str()), (3, ""), "{text}");
        assert!(run.stderr.contains("rules.json"), "{}", run.stderr);
        assert_eq!(repo.read("rules/rules.json").as_deref(), Some(text));
    }
}

#[test]
fn another_command_still_refuses_a_wrong_shape_with_exit_3() {
    let text = "{\"format\":1,\"mine\":true}";
    for command in [&["rules", "list"][..], &["rules", "format"]] {
        let repo = project(text);
        let run = repo.run(command);
        assert_eq!(run.code, 3, "{command:?}");
        assert!(run.stderr.contains("mine"), "{}", run.stderr);
        assert_eq!(repo.read("rules/rules.json").as_deref(), Some(text));
    }
}

#[test]
fn a_source_finding_leaves_out_rule_and_line_from_the_json_report_when_unknown() {
    let finding = |text: &str| -> Value {
        let run = project(text).run(&["check", "--format", "json"]);
        let report: Value = serde_json::from_str(&run.stdout).expect("JSON");
        report["findings"][0].clone()
    };
    assert_eq!(
        finding("{\"format\":1,\n\"mine\":true}")
            .as_object()
            .map(|f| (f.get("rule"), f.get("file"), f.get("line"))),
        Some((None, Some(&json!("rules/rules.json")), Some(&json!(2))))
    );
    assert_eq!(
        finding(&json!({"format": 1, "rules": [rule("", "proposed")]}).to_string()),
        json!({
            "code": "RL105", "name": "empty-field", "severity": "error",
            "message": "the Rule titled `T` has an empty `id`",
            "file": "rules/rules.json"
        })
    );
}

/// The second example of `docs/public/cli/check.md`, as written.
#[test]
fn the_documented_example_of_a_rules_file_error() {
    let rules = [rule("RULE-004", "validated"), rule("RULE-004", "proposed")];
    let repo = project(&json!({"format": 1, "rules": rules}).to_string());
    let page = include_str!("../../../docs/public/cli/check.md").replace("\r\n", "\n");
    let shown = page
        .split_once("## Errors in the rules file")
        .and_then(|(_, rest)| rest.split("```text\n").nth(1))
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

/// The level the CI guide tells the reader to set.
#[test]
fn the_ci_guide_fails_the_job_on_a_rules_file_that_is_not_canonical() {
    let page =
        include_str!("../../../docs/public/guides/run-rulelink-in-ci.md").replace("\r\n", "\n");
    let levels = page
        .split("```toml\n")
        .nth(1)
        .and_then(|rest| rest.split("```").next())
        .expect("the guide shows a setting");
    let repo = project("{\"format\":1}");
    repo.write("a.rs", "");
    assert_eq!(repo.run(&["check"]).code, 0);
    let config = repo.read("rulelink.toml").expect("config");
    repo.write("rulelink.toml", &format!("{config}\n{levels}"));
    assert_eq!(repo.run(&["check"]).code, 1);
    assert_eq!(repo.run(&["rules", "format"]).code, 0);
    assert_eq!(repo.run(&["check"]).code, 0);
}
