//! Warnings about `rulelink.toml`, through the built binary.

use rulelink_e2e::Repo;

fn warning(name: &str) -> String {
    format!("rulelink: warning: rulelink.toml: unknown section or key `{name}` is ignored\n")
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

#[test]
fn known_extension_and_adapter_names_do_not_warn() {
    let all = "[scan]\nexclude = [\"vendor/\"]\n[check.levels]\norphan-rule = \"off\"\n[x-team]\nowner = \"a\"\n[adapters.notion]\ndb = \"d\"";
    for repo in [project("x-note = 1", all), project("", "")] {
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
