//! `checkmate init`: configuration and empty Source.

use checkmate_e2e::Repo;

const EMPTY_SOURCE: &str =
    "{\n  \"format\": 1,\n  \"sequences\": {},\n  \"retired\": [],\n  \"rules\": []\n}\n";

#[test]
fn init_creates_config_and_empty_canonical_source() {
    let repo = Repo::new();
    let run = repo.run(&["init"]);
    assert_eq!(run.code, 0, "stderr: {}", run.stderr);
    assert_eq!(repo.read("rules/rules.json").as_deref(), Some(EMPTY_SOURCE));
    let config = repo.read("checkmate.toml").expect("config written");
    assert!(config.contains("version = 1"));
    assert!(config.contains("path = \"rules/rules.json\""));
    assert!(config.contains("prefix = \"RULE\""));
}

#[test]
fn identifier_prefix_is_configurable() {
    let repo = Repo::new();
    let run = repo.run(&["init", "--id-prefix", "ACME"]);
    assert_eq!(run.code, 0, "stderr: {}", run.stderr);
    let config = repo.read("checkmate.toml").expect("config written");
    assert!(config.contains("prefix = \"ACME\""));
    assert!(!config.contains("\"RULE\""));
}

#[test]
fn invalid_prefix_is_refused_and_writes_nothing() {
    let repo = Repo::new();
    let run = repo.run(&["init", "--id-prefix", "not valid"]);
    assert_eq!(run.code, 2);
    assert!(run.stderr.contains("prefix"), "stderr: {}", run.stderr);
    assert!(repo.read("checkmate.toml").is_none());
    assert!(repo.read("rules/rules.json").is_none());
}

#[test]
fn init_refuses_existing_config_and_leaves_files_untouched() {
    let repo = Repo::new();
    repo.write("checkmate.toml", "# mine\n");
    let run = repo.run(&["init"]);
    assert_eq!(run.code, 2);
    assert!(run.stderr.contains("already"), "stderr: {}", run.stderr);
    assert_eq!(repo.read("checkmate.toml").as_deref(), Some("# mine\n"));
    assert!(repo.read("rules/rules.json").is_none());
}

#[test]
fn init_refuses_existing_source_and_leaves_files_untouched() {
    let repo = Repo::new();
    repo.write("rules/rules.json", "{\"mine\": true}\n");
    let run = repo.run(&["init"]);
    assert_eq!(run.code, 2);
    assert_eq!(
        repo.read("rules/rules.json").as_deref(),
        Some("{\"mine\": true}\n")
    );
    assert!(repo.read("checkmate.toml").is_none());
}
