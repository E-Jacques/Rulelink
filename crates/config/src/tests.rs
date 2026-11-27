use rulelink_adapter_sdk::{RawValue, Warning};

use super::*;

/// A property of the fake adapter, which warns each time it is written.
#[derive(Debug, PartialEq)]
struct Note(bool);

impl Property for Note {
    const KEY: &'static str = "source.note";

    fn parse(raw: Option<&RawValue>) -> ParseResult<Self> {
        let mut parsed = Parsed::new(Self(raw.is_some()));
        if raw.is_some() {
            parsed
                .warnings
                .push(Warning::new(Self::KEY, "`source.note` is deprecated"));
        }
        Ok(parsed)
    }
}

struct Fake;

impl AdapterConfig for Fake {
    fn kind(&self) -> &'static str {
        "json"
    }

    fn parse(&self, table: RawTable) -> ParseResult<ConfigMap> {
        TableParser::new(table).property::<Note>().finish()
    }
}

/// A scanner without settings.
struct NoScan;

impl AdapterConfig for NoScan {
    fn kind(&self) -> &'static str {
        "none"
    }

    fn parse(&self, table: RawTable) -> ParseResult<ConfigMap> {
        TableParser::new(table).finish()
    }
}

const CATALOG: AdapterCatalog<'static> = AdapterCatalog {
    sources: &[&Fake],
    scanner: &NoScan,
};

const VALID: &str = "version = 1\n[source]\nkind = \"json\"\n[rules]\nprefix = \"BR\"\n";

fn problems(text: &str) -> Vec<String> {
    let error = parse(text, CATALOG).expect_err("refused");
    error.problems().map(|p| p.message.clone()).collect()
}

#[test]
fn every_property_is_declared() {
    let map = parse(VALID, CATALOG).expect("valid").configuration;
    assert!(map.get::<Version>().is_ok());
    assert_eq!(map.get::<SourceKind>().map(SourceKind::as_str), Ok("json"));
    assert_eq!(
        map.get::<RulesPrefix>().map(|p| p.prefix().as_str()),
        Ok("BR")
    );
    assert!(map.get::<CheckLevels>().is_ok());
    assert!(map.get::<Note>().is_ok());
}

#[test]
fn a_warning_of_an_adapter_comes_with_the_map() {
    let text = VALID.replace("[source]", "[source]\nnote = 1");
    let parsed = parse(&text, CATALOG).expect("valid");
    assert_eq!(parsed.warnings.len(), 1);
}

#[test]
fn independent_problems_are_all_reported() {
    let text = "version = 1\n[source]\nkind = \"json\"\nbad = 1\n[rules]\nprefix = \"bad\"\n[scan]\nz = 1\n[check.levels]\nnope = \"off\"\n";
    let found = problems(text);
    assert_eq!(found.len(), 4, "{found:?}");
}

#[test]
fn invalid_toml_and_the_version_are_reported_alone() {
    assert_eq!(problems("not toml {").len(), 1);
    assert_eq!(problems(""), ["missing `version`"]);
    assert_eq!(
        problems("version = 2\nzzz = 1\n"),
        ["unsupported version 2, expected 1"]
    );
    assert_eq!(
        problems("version = 99999999999999999999\n"),
        ["unsupported version 99999999999999999999, expected 1"]
    );
}

#[test]
fn a_name_written_twice_is_invalid_toml() {
    let found = problems("version = 1\nversion = 1\n");
    assert!(found[0].contains("duplicate"), "{found:?}");
}

#[test]
fn an_unknown_kind_lists_the_available_ones_and_leaves_its_keys_unchecked() {
    let text = VALID.replace("\"json\"", "\"csv\"\nwhatever = 1");
    assert_eq!(
        problems(&text),
        ["unsupported `source.kind` \"csv\", only \"json\" is available"]
    );
}

#[test]
fn numbers_are_read_in_any_base() {
    assert!(parse(&VALID.replace("version = 1", "version = 0x1"), CATALOG).is_ok());
}

#[test]
fn every_refused_level_is_reported() {
    let text = format!("{VALID}[check.levels]\nnope = \"off\"\norphan-rule = 1\n");
    let found = problems(&text);
    assert_eq!(found.len(), 2, "{found:?}");
    assert!(found[0].starts_with("`check.levels.nope`: "), "{found:?}");
    assert!(found[1].contains("found a number"), "{found:?}");
}

#[test]
fn one_problem_stays_on_the_line_and_several_are_listed_below() {
    let invalid = |text: &str| Error::Invalid(parse(text, CATALOG).expect_err("refused"));
    assert_eq!(
        invalid("").to_string(),
        "invalid rulelink.toml: missing `version`"
    );
    assert!(
        invalid("version = 1\n")
            .to_string()
            .starts_with("invalid rulelink.toml:\n  - ")
    );
}

#[test]
fn text_of_the_file_cannot_add_lines_to_a_message() {
    for text in [
        format!("{VALID}[check.levels]\n\"a\\nb\" = \"off\"\n"),
        format!("{VALID}[check.levels]\norphan-rule = \"x\\ny\"\n"),
        VALID.replace("\"BR\"", "\"a\\nb\""),
        VALID.replace("\"json\"", "\"c\\nsv\""),
    ] {
        for message in problems(&text) {
            assert!(!message.contains('\n'), "{text}: {message}");
        }
    }
}

#[test]
fn a_typo_of_version_or_kind_is_named_with_the_missing_key() {
    assert_eq!(
        problems(&VALID.replace("version", "verion")),
        [
            "missing `version`",
            "unknown section or key `verion`; did you mean `version`?"
        ]
    );
    assert_eq!(
        problems(&VALID.replace("kind", "knd")),
        [
            "missing `source.kind`",
            "unknown section or key `source.knd`; did you mean `source.kind`?"
        ]
    );
}

#[test]
fn a_key_close_to_one_parsed_early_or_reserved_gets_a_suggestion() {
    for (text, meant) in [
        (format!("versions = 1\n{VALID}"), "`version`?"),
        (
            VALID.replace("[source]", "[source]\nkinds = 1"),
            "`source.kind`?",
        ),
        (format!("{VALID}[adapter.x]\n"), "`adapters`?"),
    ] {
        let found = problems(&text);
        assert!(found[0].ends_with(meant), "{text}: {found:?}");
    }
}

#[test]
fn an_extension_name_is_accepted_among_the_levels() {
    let text = format!("{VALID}[check.levels]\nx-note = \"why\"\n");
    assert!(parse(&text, CATALOG).is_ok());
}

#[test]
fn missing_file_points_to_init() {
    let dir = std::env::temp_dir().join("rulelink-config-missing-test");
    let err = load(&dir, CATALOG)
        .expect_err("no config in an empty directory")
        .to_string();
    assert!(err.contains("rulelink init"), "{err}");
}
