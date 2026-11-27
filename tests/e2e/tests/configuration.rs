//! Mistakes in `rulelink.toml`, through the built binary.

use rulelink_e2e::Repo;

const PAGE: &str = include_str!("../../../docs/public/concepts/configuration.md");

/// Git may check text files out with Windows line endings, which reach the page above and
/// the configuration template built into the binary.
fn unix_lines(text: &str) -> String {
    text.replace("\r\n", "\n")
}

/// Whether the documentation shows `output`.
fn page_shows(output: &str) -> bool {
    unix_lines(PAGE).contains(output.trim_end())
}

/// What a command prints when the configuration has exactly one problem.
fn one(problem: &str) -> String {
    format!("rulelink: configuration error: invalid rulelink.toml: {problem}\n")
}

/// What a command prints when the configuration has several problems.
fn several(problems: &[&str]) -> String {
    format!(
        "rulelink: configuration error: invalid rulelink.toml:\n  - {}\n",
        problems.join("\n  - ")
    )
}

fn initialised() -> Repo {
    let repo = Repo::new();
    assert_eq!(repo.run(&["init"]).code, 0);
    repo
}

/// An initialised project with `before` added above the configuration and `after` below it.
fn project(before: &str, after: &str) -> Repo {
    let repo = initialised();
    let config = repo.read("rulelink.toml").expect("config");
    repo.write("rulelink.toml", &format!("{before}\n{config}\n{after}\n"));
    repo
}

/// An initialised project with `from` replaced by `to` in its configuration.
fn edited(from: &str, to: &str) -> Repo {
    let repo = initialised();
    let config = unix_lines(&repo.read("rulelink.toml").expect("config"));
    assert!(config.contains(from), "{config}");
    repo.write("rulelink.toml", &config.replace(from, to));
    repo
}

/// Runs `check` and returns its exit code and error output.
fn check(repo: &Repo) -> (i32, String) {
    let run = repo.run(&["check"]);
    (run.code, run.stderr)
}

/// The first example of `docs/public/concepts/configuration.md`.
#[test]
fn a_misspelled_key_is_refused_with_the_closest_known_key() {
    let repo = project("", "[scan]\nexcludes = [\"vendor/\"]");
    let expected = one("unknown section or key `scan.excludes`; did you mean `scan.exclude`?");
    assert_eq!(check(&repo), (2, expected.clone()));
    assert!(page_shows(&expected), "the page shows it");
}

/// The second example of the page.
#[test]
fn every_problem_is_listed_at_once() {
    let repo = edited("prefix =", "prefx =");
    let expected = several(&[
        "missing `rules.prefix`",
        "unknown section or key `rules.prefx`; did you mean `rules.prefix`?",
    ]);
    assert_eq!(check(&repo), (2, expected.clone()));
    assert!(page_shows(&expected), "the page shows it");
}

#[test]
fn every_command_that_reads_the_configuration_refuses_it_and_changes_nothing() {
    let repo = project("", "[scan]\nexclude = [\"[\"]");
    let source = repo.read("rules/rules.json");
    let add = ["rules", "add", "--title", "T", "--statement", "S"];
    let commands: [&[&str]; 9] = [
        &add,
        &["rules", "list"],
        &["rules", "search", "x"],
        &["rules", "approve", "RULE-001"],
        &["rules", "reject", "RULE-001"],
        &["rules", "obsolete", "RULE-001"],
        &["rules", "format"],
        &["where", "RULE-001"],
        &["check"],
    ];
    for command in commands {
        let run = repo.run(command);
        assert_eq!(run.code, 2, "{command:?}: {}", run.stderr);
        assert!(
            run.stderr.contains("`scan.exclude` pattern \"[\""),
            "{}",
            run.stderr
        );
        assert_eq!(run.stdout, "", "{command:?}");
    }
    assert_eq!(repo.read("rules/rules.json"), source);
}

#[test]
fn an_unknown_key_is_refused_in_every_section() {
    for (from, to, name) in [
        ("[source]", "[source]\nbranch = \"main\"", "source.branch"),
        ("[rules]", "[rules]\nid_width = 4", "rules.id_width"),
        ("[rules]", "[check]\nlvl = 1\n[rules]", "check.lvl"),
        ("version = 1", "version = 1\nlvls = 1", "lvls"),
    ] {
        let (code, stderr) = check(&edited(from, to));
        assert_eq!(code, 2, "{stderr}");
        assert!(
            stderr.contains(&format!("unknown section or key `{name}`; ")),
            "{stderr}"
        );
    }
}

#[test]
fn a_name_with_no_close_known_key_says_how_to_keep_it() {
    let repo = project("lvls = 1", "");
    let expected =
        one("unknown section or key `lvls`; remove it, or start its name with `x-` to keep it");
    assert_eq!(check(&repo), (2, expected));
}

#[test]
fn a_quoted_name_is_told_apart_and_cannot_add_lines() {
    let repo = project("\"rules.prefx\" = 1\n\"a\\nb\" = 2", "");
    let (code, stderr) = check(&repo);
    assert_eq!(code, 2);
    assert!(stderr.contains("`\"rules.prefx\"`"), "{stderr}");
    assert!(stderr.contains("`a\\nb`"), "{stderr}");
    assert_eq!(stderr.lines().count(), 3, "{stderr}");
}

#[test]
fn known_extension_and_adapter_names_are_accepted() {
    let all = "[scan]\nexclude = [\"vendor/\"]\nx-note = 99999999999999999999\n[check]\nx-why = 1\n[check.levels]\norphan-rule = \"off\"\n[x-team]\nowner = \"a\"\n[adapters.notion]\ndb = \"d\"";
    for repo in [
        project("x-note = 1\nx-big = 99999999999999999999", all),
        project("", ""),
        edited("[source]", "[source]\nx-owner = \"a\""),
        edited("[rules]", "[rules]\nx-owner = \"a\""),
    ] {
        assert_eq!(check(&repo), (0, String::new()));
    }
}

#[test]
fn a_section_that_is_not_a_table_is_refused() {
    let (code, stderr) = check(&edited("[rules]\nprefix = \"RULE\"", ""));
    assert_eq!((code, stderr), (2, one("missing `rules.prefix`")));
    let repo = initialised();
    repo.write(
        "rulelink.toml",
        "version = 1\nrules = [\"RULE\"]\n[source]\nkind = \"json\"\npath = \"rules/rules.json\"\n",
    );
    let (code, stderr) = check(&repo);
    assert_eq!(
        (code, stderr),
        (2, one("`rules` must be a section, found a list"))
    );
}

#[test]
fn invalid_toml_is_reported_with_its_line() {
    let (code, stderr) = check(&project("", "[scan"));
    assert_eq!(code, 2);
    assert!(stderr.contains("[scan"), "{stderr}");
    assert!(stderr.contains("line "), "{stderr}");
}
