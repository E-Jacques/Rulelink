//! Warnings about `rulelink.toml`, through the built binary.

use rulelink_e2e::Repo;

fn warning(name: &str) -> String {
    format!(
        "rulelink: warning: rulelink.toml: unknown section or key `{name}` is ignored; check its spelling\n"
    )
}

/// An initialised project with `before` added above the configuration and `after` below it.
fn project(before: &str, after: &str) -> Repo {
    let repo = Repo::new();
    assert_eq!(repo.run(&["init"]).code, 0);
    let config = repo.read("rulelink.toml").expect("config");
    repo.write("rulelink.toml", &format!("{before}\n{config}\n{after}\n"));
    repo
}

/// The example of `docs/public/concepts/configuration.md`.
#[test]
fn a_misspelled_section_warns_and_changes_neither_result_nor_exit_code() {
    let repo = project("", "[checks.levels]\nunknown-reference = \"error\"");
    repo.write("a.rs", "// @rule RULE-001\n");
    let run = repo.run(&["check"]);
    assert_eq!((run.code, run.stderr), (0, warning("checks")));
    assert!(run.stdout.contains("0 errors, 1 warning"), "{}", run.stdout);
    let page = include_str!("../../../docs/public/concepts/configuration.md");
    assert!(
        page.contains(warning("checks").trim_end()),
        "the page shows the warning"
    );
}

#[test]
fn every_command_that_reads_the_configuration_warns() {
    let repo = project("", "[scann]\nexclude = [\"vendor/\"]");
    let add = ["rules", "add", "--title", "T", "--statement", "S"];
    let commands: [&[&str]; 5] = [
        &add,
        &["rules", "list"],
        &["rules", "format"],
        &["where", "RULE-001"],
        &["check"],
    ];
    for command in commands {
        let run = repo.run(command);
        assert_eq!((run.code, run.stderr), (0, warning("scann")), "{command:?}");
    }
}

#[test]
fn each_unknown_name_has_its_own_line_and_cannot_add_lines() {
    let repo = project(
        "lvls = 1\n\"a\\nb\" = 2",
        "[Check.levels]\norphan-rule = \"off\"",
    );
    let run = repo.run(&["check"]);
    let expected = [warning("Check"), warning("a\\nb"), warning("lvls")].concat();
    assert_eq!((run.code, run.stderr), (0, expected));
}

/// The second example of the page: the misspelled key excludes nothing, and says so.
#[test]
fn a_misspelled_key_in_scan_warns() {
    let repo = project("", "[scan]\nexcludes = [\"vendor/\"]");
    repo.write("vendor/a.rs", "// @rule RULE-001\n");
    let run = repo.run(&["check"]);
    assert_eq!((run.code, run.stderr), (0, warning("scan.excludes")));
    assert!(run.stdout.contains("vendor/a.rs:1"), "{}", run.stdout);
    let page = include_str!("../../../docs/public/concepts/configuration.md");
    assert!(
        page.contains(warning("scan.excludes").trim_end()),
        "the page shows the warning"
    );
}

/// Dotted keys at the top of the file, so that `[rules]` is not declared twice.
#[test]
fn unknown_keys_in_rules_and_scan_warn_but_extensions_do_not() {
    let repo = Repo::new();
    assert_eq!(repo.run(&["init"]).code, 0);
    let config = repo.read("rulelink.toml").expect("config");
    repo.write(
        "rulelink.toml",
        &format!(
            "{}\n[scan]\n\"a\\nb\" = 1\nx-note = 99999999999999999999\n",
            config.replace("[rules]", "[rules]\nid_width = 4\nx-owner = \"a\"")
        ),
    );
    let run = repo.run(&["check"]);
    let expected = [warning("rules.id_width"), warning("scan.a\\nb")].concat();
    assert_eq!((run.code, run.stderr), (0, expected));
}

#[test]
fn a_quoted_top_level_name_is_told_apart_from_a_key_of_a_section() {
    let repo = Repo::new();
    assert_eq!(repo.run(&["init"]).code, 0);
    let config = repo.read("rulelink.toml").expect("config");
    repo.write(
        "rulelink.toml",
        &format!(
            "\"rules.prefx\" = 1\n{}",
            config.replace("[rules]", "[rules]\nprefx = 2")
        ),
    );
    let run = repo.run(&["check"]);
    let expected = [warning("\"rules.prefx\""), warning("rules.prefx")].concat();
    assert_eq!((run.code, run.stderr), (0, expected));
}

/// TOML lists where a section is expected were accepted before keys were checked: looking
/// for unknown keys must not refuse a configuration that the commands can read.
#[test]
fn a_section_written_as_a_list_is_still_accepted() {
    let source = "[source]\nkind = \"json\"\npath = \"rules/rules.json\"\n";
    for config in [
        format!("version = 1\nrules = [\"RULE\"]\n{source}"),
        format!("version = 1\nscan = [[\"vendor/\"]]\n{source}[rules]\nprefix = \"RULE\"\n"),
    ] {
        let repo = Repo::new();
        assert_eq!(repo.run(&["init"]).code, 0);
        repo.write("rulelink.toml", &config);
        let run = repo.run(&["check"]);
        assert_eq!((run.code, run.stderr.as_str()), (0, ""), "{config}");
    }
}

/// Which keys of `[source]` belong to the adapter is not decided yet.
#[test]
fn an_unknown_key_in_source_does_not_warn() {
    let repo = Repo::new();
    assert_eq!(repo.run(&["init"]).code, 0);
    let config = repo.read("rulelink.toml").expect("config");
    repo.write(
        "rulelink.toml",
        &config.replace("[source]", "[source]\nbranch = \"main\""),
    );
    let run = repo.run(&["check"]);
    assert_eq!((run.code, run.stderr.as_str()), (0, ""));
}

#[test]
fn known_extension_and_adapter_names_do_not_warn() {
    let all = "[scan]\nexclude = [\"vendor/\"]\n[check.levels]\norphan-rule = \"off\"\n[x-team]\nowner = \"a\"\n[adapters.notion]\ndb = \"d\"";
    for repo in [
        project("x-note = 1\nx-big = 99999999999999999999", all),
        project("", ""),
    ] {
        let run = repo.run(&["check"]);
        assert_eq!((run.code, run.stderr.as_str()), (0, ""));
    }
}

/// The command reports what is wrong; a warning must not replace or double that.
#[test]
fn an_invalid_configuration_is_only_an_error() {
    let repo = project("", "[checks]\n[check]\nlevel = 1");
    let run = repo.run(&["check"]);
    assert_eq!(run.code, 2);
    assert!(!run.stderr.contains("warning:"), "{}", run.stderr);
    assert!(run.stderr.contains("level = 1"), "{}", run.stderr);
}
