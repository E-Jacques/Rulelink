//! The settings contract, through its public interface.

use std::collections::BTreeMap;
use std::path::Path;

use rulelink_adapter_sdk::{
    ParseError, ParseResult, Parsed, ProjectPath, Property, RawTable, RawValue, TableParser,
    Warning, required_text,
};

/// A required property.
#[derive(Debug, PartialEq)]
struct Name(String);

impl Property for Name {
    const KEY: &'static str = "demo.name";
    fn parse(raw: Option<&RawValue>) -> ParseResult<Self> {
        required_text(Self::KEY, raw).map(|text| Parsed::new(Self(text.to_owned())))
    }
}

/// An optional property with a default, which warns about one value.
#[derive(Debug, PartialEq)]
struct Mode(String);

impl Property for Mode {
    const KEY: &'static str = "demo.mode";
    fn parse(raw: Option<&RawValue>) -> ParseResult<Self> {
        let Some(value) = raw else {
            return Ok(Parsed::new(Self("plain".to_owned())));
        };
        let text = required_text(Self::KEY, Some(value))?;
        let mut parsed = Parsed::new(Self(text.to_owned()));
        if text == "old" {
            parsed
                .warnings
                .push(Warning::new(Self::KEY, "`demo.mode` \"old\" is deprecated"));
        }
        Ok(parsed)
    }
}

/// Never declared to a parser.
#[derive(Debug)]
struct Forgotten;

impl Property for Forgotten {
    const KEY: &'static str = "demo.forgotten";
    fn parse(_: Option<&RawValue>) -> ParseResult<Self> {
        Ok(Parsed::new(Self))
    }
}

fn table(entries: &[(&str, &str)]) -> RawTable {
    let entries: BTreeMap<String, RawValue> = entries
        .iter()
        .map(|(k, v)| ((*k).to_owned(), RawValue::String((*v).to_owned())))
        .collect();
    RawTable::new("demo", entries)
}

fn demo(entries: &[(&str, &str)]) -> ParseResult<rulelink_adapter_sdk::ConfigMap> {
    TableParser::new(table(entries))
        .property::<Name>()
        .property::<Mode>()
        .finish()
}

fn messages(error: &ParseError) -> Vec<&str> {
    error.problems().map(|p| p.message.as_str()).collect()
}

#[test]
fn a_parsed_map_holds_every_declared_property_with_its_default() {
    let parsed = demo(&[("name", "a")]).expect("valid");
    assert_eq!(parsed.configuration.get::<Name>(), Ok(&Name("a".to_owned())));
    assert_eq!(
        parsed.configuration.get::<Mode>(),
        Ok(&Mode("plain".to_owned()))
    );
    assert!(parsed.warnings.is_empty());
}

#[test]
fn a_property_that_was_never_declared_is_reported_by_key() {
    let parsed = demo(&[("name", "a")]).expect("valid");
    let error = parsed.configuration.get::<Forgotten>().expect_err("absent");
    assert!(error.to_string().contains("demo.forgotten"), "{error}");
}

#[test]
fn warnings_come_with_a_successful_parse() {
    let parsed = demo(&[("name", "a"), ("mode", "old")]).expect("valid");
    assert_eq!(parsed.warnings.len(), 1);
    assert_eq!(parsed.warnings[0].key, "demo.mode");
}

#[test]
fn every_problem_is_collected_and_extensions_are_accepted() {
    let error = demo(&[("nmae", "a"), ("x-note", "n")]).expect_err("invalid");
    assert_eq!(
        messages(&error),
        [
            "missing `demo.name`",
            "unknown section or key `demo.nmae`; did you mean `demo.name`?"
        ]
    );
}

#[test]
fn one_problem_is_displayed_alone_and_several_as_a_list() {
    let one = demo(&[]).expect_err("invalid");
    assert_eq!(one.to_string(), "missing `demo.name`");
    let two = demo(&[("zzzzzzzz", "a")]).expect_err("invalid");
    assert_eq!(
        two.to_string(),
        "\n  - missing `demo.name`\n  - unknown section or key `demo.zzzzzzzz`; remove it, or start its name with `x-` to keep it"
    );
}

#[test]
fn a_section_that_is_not_a_table_is_a_problem_and_a_reserved_name_is_not() {
    let root = RawTable::new(
        "",
        BTreeMap::from([
            ("demo".to_owned(), RawValue::Array(Vec::new())),
            ("adapters".to_owned(), RawValue::Boolean(true)),
        ]),
    );
    let error = TableParser::new(root)
        .section("demo", |t| TableParser::new(t).property::<Name>().finish())
        .reserved("adapters")
        .finish()
        .expect_err("invalid");
    assert_eq!(messages(&error), ["`demo` must be a section, found a list"]);
}

#[test]
fn an_absent_section_is_parsed_as_empty() {
    let error = TableParser::new(RawTable::new("", BTreeMap::new()))
        .section("demo", |t| TableParser::new(t).property::<Name>().finish())
        .finish()
        .expect_err("invalid");
    assert_eq!(messages(&error), ["missing `demo.name`"]);
}

#[test]
fn text_of_another_type_says_what_was_found() {
    let entries = BTreeMap::from([("name".to_owned(), RawValue::Integer(3))]);
    let error = TableParser::new(RawTable::new("demo", entries))
        .property::<Name>()
        .finish()
        .expect_err("invalid");
    assert_eq!(
        messages(&error),
        ["`demo.name` must be text in quotes, found a number"]
    );
}

#[test]
fn a_project_path_cannot_leave_the_project() {
    for raw in ["../x.json", "a/../../x.json", "/etc/x.json", ""] {
        assert!(ProjectPath::parse(raw).is_err(), "{raw}");
    }
    let path = ProjectPath::parse("./a/b.json").expect("inside");
    assert_eq!(path.as_path(), Path::new("./a/b.json"));
}
