//! `rulelink check --format json`: the report, its published schema and exit codes.

use rulelink_e2e::{Repo, Run};
use serde_json::{Value, json};

const PAGE: &str = include_str!("../../../docs/public/cli/check.md");
const SCHEMA: &str = include_str!("../../../schemas/check-report.schema.json");

fn rule(id: &str, status: &str) -> Value {
    json!({"id": id, "title": "T", "statement": "S", "status": status})
}

/// A project whose Source holds `rules`, with `retired` Identifiers.
fn project(rules: &[Value], retired: &[&str]) -> Repo {
    let repo = Repo::new();
    assert_eq!(repo.run(&["init"]).code, 0);
    let source = json!({"format": 1, "retired": retired, "rules": rules});
    repo.write("rules/rules.json", &source.to_string());
    repo
}

/// A file whose line `line` cites `id`.
fn cite_at(repo: &Repo, file: &str, line: usize, id: &str) {
    repo.write(file, &format!("{}// @rule {id}\n", "\n".repeat(line - 1)));
}

/// The project of the first example of the `check` page.
fn documented() -> Repo {
    let mut replaced = rule("RULE-003", "obsolete");
    replaced["superseded_by"] = json!("RULE-007");
    let repo = project(
        &[
            replaced,
            rule("RULE-005", "validated"),
            rule("RULE-007", "validated"),
        ],
        &["RULE-003"],
    );
    cite_at(&repo, "src/billing.rs", 42, "RULE-099");
    cite_at(&repo, "src/vat.rs", 3, "RULE-003");
    cite_at(&repo, "src/total.rs", 1, "RULE-007");
    repo
}

/// One Finding of each code.
fn every_code() -> Repo {
    let repo = project(
        &[
            rule("RULE-001", "obsolete"),
            rule("RULE-002", "validated"),
            rule("RULE-003", "proposed"),
            rule("RULE-004", "rejected"),
        ],
        &["RULE-001", "RULE-004"],
    );
    repo.write("src/a.rs", "// @rule RULE-099 RULE-001 RULE-003 RULE-004\n");
    repo
}

/// The published schema, refusing in addition every field it does not declare: the contract
/// lets a later version add fields, this one must not print any by accident.
fn closed_schema() -> Value {
    fn close(node: &mut Value) {
        match node {
            Value::Object(map) => {
                if map.contains_key("properties") {
                    map.insert("additionalProperties".into(), Value::Bool(false));
                }
                map.values_mut().for_each(close);
            }
            Value::Array(items) => items.iter_mut().for_each(close),
            _ => {}
        }
    }
    let mut schema = published_schema();
    close(&mut schema);
    schema
}

fn published_schema() -> Value {
    serde_json::from_str(SCHEMA).expect("the schema is JSON")
}

fn errors(schema: &Value, instance: &Value) -> Vec<String> {
    jsonschema::validator_for(schema)
        .expect("the schema is a valid JSON Schema")
        .iter_errors(instance)
        .map(|error| format!("{} at {}", error, error.instance_path()))
        .collect()
}

/// The report printed by `run`, checked against the published schema.
fn report(run: &Run) -> Value {
    let report: Value = serde_json::from_str(&run.stdout)
        .unwrap_or_else(|e| panic!("stdout is not JSON ({e}): {}{}", run.stdout, run.stderr));
    assert_eq!(errors(&closed_schema(), &report), Vec::<String>::new());
    report
}

fn check_json(repo: &Repo) -> Run {
    repo.run(&["check", "--format", "json"])
}

#[test]
fn the_documented_example_prints_the_documented_report() {
    let section = PAGE
        .split_once("## JSON report")
        .expect("the page has a JSON report section")
        .1;
    let block = section
        .split_once("```json")
        .expect("the section has an example")
        .1
        .split_once("```")
        .expect("the example is closed")
        .0;
    let expected: Vec<&str> = block.trim().lines().collect();

    let run = check_json(&documented());

    assert_eq!(run.stdout.lines().collect::<Vec<_>>(), expected);
    assert_eq!((run.code, run.stderr.as_str()), (1, ""));
    report(&run);
}

#[test]
fn every_code_validates_with_the_exit_code_of_the_human_report() {
    let repo = every_code();
    let run = check_json(&repo);
    let report = report(&run);
    let codes: Vec<&str> = report["findings"]
        .as_array()
        .expect("findings")
        .iter()
        .filter_map(|f| f["code"].as_str())
        .collect();
    assert_eq!(codes, ["RL001", "RL009", "RL002", "RL004", "RL005"]);
    assert_eq!(report["totals"], json!({"errors": 2, "warnings": 3}));
    assert_eq!((run.code, repo.run(&["check"]).code), (1, 1));
}

#[test]
fn warnings_alone_succeed_and_hidden_findings_are_not_counted() {
    let repo = project(&[rule("RULE-001", "validated")], &[]);
    let run = check_json(&repo);
    assert_eq!(run.code, 0);
    assert_eq!(report(&run)["totals"], json!({"errors": 0, "warnings": 1}));

    let config = repo.read("rulelink.toml").expect("config");
    repo.write(
        "rulelink.toml",
        &format!("{config}\n[check.levels]\norphan-rule = \"off\"\n"),
    );
    let run = check_json(&repo);
    assert_eq!(
        report(&run),
        json!({
            "schema": "rulelink/check-report/v1",
            "findings": [],
            "totals": {"errors": 0, "warnings": 0}
        })
    );
}

#[test]
fn a_source_without_rules_adds_nothing_to_the_report() {
    let repo = project(&[], &[]);
    let run = check_json(&repo);
    assert_eq!(run.code, 0);
    assert_eq!(
        report(&run),
        json!({
            "schema": "rulelink/check-report/v1",
            "findings": [],
            "totals": {"errors": 0, "warnings": 0}
        })
    );
}

#[test]
fn the_rules_file_is_named_like_a_scanned_file() {
    let repo = project(&[rule("RULE-001", "validated")], &[]);
    let config = repo.read("rulelink.toml").expect("config");
    repo.write(
        "rulelink.toml",
        &config.replace("rules/rules.json", "./rules/./rules.json"),
    );
    assert_eq!(
        report(&check_json(&repo))["findings"][0]["file"],
        "rules/rules.json"
    );
}

#[test]
fn human_is_the_default_format() {
    let repo = documented();
    let default = repo.run(&["check"]);
    let human = repo.run(&["check", "--format", "human"]);
    assert_eq!(
        (default.code, default.stdout),
        (human.code, human.stdout.clone())
    );
    assert!(
        human.stdout.starts_with("error RL001: "),
        "{}",
        human.stdout
    );
}

#[test]
fn another_format_is_refused_naming_the_accepted_ones() {
    for format in ["sarif", "JSON", ""] {
        let run = documented().run(&["check", "--format", format]);
        assert_eq!((run.code, run.stdout.as_str()), (2, ""), "{format}");
        assert!(
            run.stderr.contains("--format") && run.stderr.contains("human, json"),
            "{}",
            run.stderr
        );
    }
}

#[test]
fn a_run_without_a_report_leaves_stdout_empty() {
    let invalid_config = documented();
    invalid_config.write("rulelink.toml", "version = 2\n");
    let broken_source = documented();
    broken_source.write("rules/rules.json", "{");
    for (repo, code, named) in [
        (invalid_config, 2, "rulelink.toml"),
        (broken_source, 3, "rules.json"),
    ] {
        let run = check_json(&repo);
        assert_eq!((run.code, run.stdout.as_str()), (code, ""));
        assert!(run.stderr.contains(named), "{}", run.stderr);
        assert_eq!(repo.run(&["check"]).code, code);
    }
}

#[test]
fn the_schema_lets_a_later_version_add_codes_and_fields_only() {
    let later = json!({
        "schema": "rulelink/check-report/v1",
        "findings": [{
            "code": "RL106", "name": "not-canonical", "severity": "warning",
            "message": "m", "column": 4
        }],
        "totals": {"errors": 0, "warnings": 1},
        "added": true
    });
    assert_eq!(errors(&published_schema(), &later), Vec::<String>::new());

    for (pointer, value) in [
        ("/schema", json!("rulelink/check-report/v2")),
        ("/findings/0/code", json!("XX1")),
        ("/findings/0/name", json!("Not Canonical")),
        ("/findings/0/severity", json!("info")),
        ("/findings/0/line", json!(0)),
        ("/findings/0/line", Value::Null),
        ("/findings/0/rule", Value::Null),
        ("/findings/0/file", Value::Null),
    ] {
        let mut broken = later.clone();
        let (parent, key) = pointer.rsplit_once('/').expect("pointer");
        broken.pointer_mut(parent).expect("parent")[key] = value.clone();
        assert!(
            !errors(&published_schema(), &broken).is_empty(),
            "{pointer} = {value} must be refused"
        );
    }
}
