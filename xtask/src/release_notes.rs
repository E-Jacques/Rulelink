//! `release-notes`: Release notes and an RSS feed per Product, generated from commit messages.
//!
//! Output is written into the public documentation (`docs/public/release-notes/<product>.md`)
//! and the static assets of the site (`site/public/release-notes/<product>.xml`), so both are
//! part of the assembled site. Feed URLs are absolute, built from `SITE_URL` in
//! `site/site.config.mjs`.

use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::process::Command;

use serde::Deserialize;

/// The groups of a release, in the order they are shown: breaking changes first.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Group {
    Breaking,
    Added,
    Changed,
    Deprecated,
    Fixed,
}

impl Group {
    const ALL: [Group; 5] = [
        Group::Breaking,
        Group::Added,
        Group::Changed,
        Group::Deprecated,
        Group::Fixed,
    ];

    fn title(self) -> &'static str {
        match self {
            Group::Breaking => "Breaking",
            Group::Added => "Added",
            Group::Changed => "Changed",
            Group::Deprecated => "Deprecated",
            Group::Fixed => "Fixed",
        }
    }
}

/// One commit as read from `git log`, newest first in a list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Commit {
    /// Commit date, `YYYY-MM-DD`.
    pub date: String,
    /// Commit date, RFC 2822 (RSS `pubDate`).
    pub rfc_date: String,
    /// The version tag on this commit, if any (`v0.1.0` or `0.1.0`).
    pub tag: Option<String>,
    pub subject: String,
    pub body: String,
}

/// One Product and the commit scopes that belong to it.
#[derive(Debug, Clone, Deserialize)]
pub struct Product {
    pub id: String,
    pub title: String,
    pub scopes: Vec<String>,
}

/// The single scope-to-Product mapping (`xtask/release-notes.toml`).
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(rename = "product")]
    pub products: Vec<Product>,
    #[serde(default)]
    pub ignore: Vec<String>,
}

/// A parsed conventional commit.
#[derive(Debug, PartialEq, Eq)]
struct Parsed {
    kind: String,
    scope: Option<String>,
    bang: bool,
    description: String,
    migration: Option<String>,
    breaking_footer: bool,
}

/// One line of the notes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub text: String,
    /// The migration note of a breaking change.
    pub migration: Option<String>,
}

/// The changes of one version (or the unreleased ones) of a Product.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Release {
    /// `None` for changes after the latest tag.
    pub version: Option<String>,
    pub date: String,
    pub rfc_date: String,
    pub groups: Vec<(Group, Vec<Entry>)>,
}

impl Release {
    fn heading(&self) -> String {
        self.version
            .clone()
            .unwrap_or_else(|| "Unreleased".to_string())
    }
}

const NO_MIGRATION: &str = "No migration note was recorded for this change.";

fn parse_commit(subject: &str, body: &str) -> Option<Parsed> {
    let (head, description) = subject.split_once(": ")?;
    let (head, bang) = head
        .strip_suffix('!')
        .map_or((head, false), |rest| (rest, true));
    let (kind, scope) = match head.split_once('(') {
        Some((kind, rest)) => (kind, Some(rest.strip_suffix(')')?.to_string())),
        None => (head, None),
    };
    if kind.is_empty() || !kind.chars().all(|c| c.is_ascii_lowercase()) {
        return None;
    }
    let (migration, breaking_footer) = breaking_footer(body);
    Some(Parsed {
        kind: kind.to_string(),
        scope,
        bang,
        description: strip_pr_number(description.trim()),
        migration,
        breaking_footer,
    })
}

fn strip_pr_number(text: &str) -> String {
    if let Some(open) = text.rfind(" (#") {
        let tail = &text[open + 3..];
        if tail.ends_with(')') && tail[..tail.len() - 1].chars().all(|c| c.is_ascii_digit()) {
            return text[..open].to_string();
        }
    }
    text.to_string()
}

fn is_footer_start(line: &str) -> bool {
    line.split_once(": ").is_some_and(|(token, _)| {
        !token.is_empty() && token.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
    })
}

/// Reads the `BREAKING CHANGE:` footer: its text runs until a blank line or another footer.
fn breaking_footer(body: &str) -> (Option<String>, bool) {
    let mut lines = body.lines();
    while let Some(line) = lines.next() {
        let text = line
            .strip_prefix("BREAKING CHANGE:")
            .or_else(|| line.strip_prefix("BREAKING-CHANGE:"));
        let Some(first) = text else { continue };
        let mut parts = vec![first.trim().to_string()];
        for next in lines.by_ref() {
            if next.trim().is_empty() || is_footer_start(next) {
                break;
            }
            parts.push(next.trim().to_string());
        }
        let note = parts.join(" ").trim().to_string();
        return ((!note.is_empty()).then_some(note), true);
    }
    (None, false)
}

fn group_of(kind: &str) -> Option<Group> {
    match kind {
        "feat" => Some(Group::Added),
        "fix" => Some(Group::Fixed),
        "perf" | "change" => Some(Group::Changed),
        "deprecate" => Some(Group::Deprecated),
        _ => None,
    }
}

fn capitalise(text: &str) -> String {
    let mut chars = text.chars();
    chars.next().map_or_else(String::new, |c| {
        c.to_uppercase().collect::<String>() + chars.as_str()
    })
}

/// Builds the releases of every Product, newest first. Fails on a user-facing commit whose
/// scope is neither mapped to a Product nor ignored.
pub fn build(commits: &[Commit], config: &Config) -> Result<Vec<(Product, Vec<Release>)>, String> {
    let mut result = Vec::new();
    for product in &config.products {
        result.push((product.clone(), build_product(commits, config, product)?));
    }
    Ok(result)
}

fn build_product(
    commits: &[Commit],
    config: &Config,
    product: &Product,
) -> Result<Vec<Release>, String> {
    // Walk oldest first so a tag closes the commits that came before it.
    let mut releases = Vec::new();
    let mut pending: Vec<(Group, Entry)> = Vec::new();
    let mut latest: Option<&Commit> = None;
    for commit in commits.iter().rev() {
        if let Some(parsed) = parse_commit(&commit.subject, &commit.body) {
            if let Some(entry) = classify(&parsed, config, product, &commit.subject)? {
                pending.push(entry);
                latest = Some(commit);
            }
        }
        if let Some(tag) = &commit.tag {
            if !pending.is_empty() {
                releases.push(release(Some(tag), commit, &std::mem::take(&mut pending)));
            }
            latest = None;
        }
    }
    if let Some(last) = latest {
        releases.push(release(None, last, &pending));
    }
    releases.reverse();
    Ok(releases)
}

fn unmapped(subject: &str) -> String {
    format!(
        "commit `{subject}` has no Product: map its scope in xtask/release-notes.toml (`scopes` or `ignore`)"
    )
}

/// `Some(true)` when the scope belongs to this Product, `Some(false)` when it belongs to
/// another Product or is ignored, `None` when it is not mapped at all.
fn in_scope(scope: &str, config: &Config, product: &Product) -> Option<bool> {
    let has = |scopes: &[String]| scopes.iter().any(|s| s == scope);
    if has(&product.scopes) {
        Some(true)
    } else if has(&config.ignore) || config.products.iter().any(|p| has(&p.scopes)) {
        Some(false)
    } else {
        None
    }
}

fn classify(
    parsed: &Parsed,
    config: &Config,
    product: &Product,
    subject: &str,
) -> Result<Option<(Group, Entry)>, String> {
    let breaking = parsed.bang || parsed.breaking_footer;
    let group = if breaking {
        Some(Group::Breaking)
    } else {
        group_of(&parsed.kind)
    };
    let Some(group) = group else {
        return Ok(None);
    };
    let Some(scope) = parsed.scope.as_deref() else {
        return Err(unmapped(subject));
    };
    if !in_scope(scope, config, product).ok_or_else(|| unmapped(subject))? {
        return Ok(None);
    }
    let migration = breaking.then(|| {
        parsed
            .migration
            .clone()
            .unwrap_or_else(|| NO_MIGRATION.to_string())
    });
    Ok(Some((
        group,
        Entry {
            text: capitalise(&parsed.description),
            migration,
        },
    )))
}

fn release(tag: Option<&String>, commit: &Commit, entries: &[(Group, Entry)]) -> Release {
    let groups = Group::ALL
        .iter()
        .filter_map(|group| {
            let items: Vec<Entry> = entries
                .iter()
                .filter(|(g, _)| g == group)
                .map(|(_, e)| e.clone())
                .fold(Vec::new(), |mut all, entry| {
                    if !all.contains(&entry) {
                        all.push(entry);
                    }
                    all
                });
            (!items.is_empty()).then_some((*group, items))
        })
        .collect();
    Release {
        version: tag.map(|t| t.trim_start_matches('v').to_string()),
        date: commit.date.clone(),
        rfc_date: commit.rfc_date.clone(),
        groups,
    }
}

/// GitHub-style heading slug, which Starlight also produces for the headings it renders.
fn slug(heading: &str) -> String {
    heading
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-')
        .map(|c| if c == ' ' { '-' } else { c })
        .collect()
}

fn escape_md(text: &str) -> String {
    text.replace('<', "&lt;").replace('{', "&#123;")
}

fn escape_xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn page_path(product: &Product) -> String {
    format!("release-notes/{}", product.id)
}

fn feed_url(site_url: &str, product: &Product) -> String {
    format!(
        "{}/{}.xml",
        site_url.trim_end_matches('/'),
        page_path(product)
    )
}

/// The Markdown page of a Product: front matter accepted by `docs-check` and the site build.
pub fn render_page(product: &Product, releases: &[Release], site_url: &str) -> String {
    let mut out = String::new();
    let _ = write!(
        out,
        "---\ntitle: {title} release notes\ndescription: What changed in each version of the Rulelink {title}.\n---\n\n# {title} release notes\n\nWhat changed in each version of the Rulelink {title}, breaking changes first. These notes are generated from commit messages. Subscribe with the [RSS feed]({feed}).\n",
        title = product.title,
        feed = feed_url(site_url, product),
    );
    if releases.is_empty() {
        out.push_str("\nNo user-facing changes yet.\n");
    }
    for release in releases {
        let _ = write!(out, "\n## {}\n", release.heading());
        if release.version.is_some() {
            let _ = write!(out, "\nReleased {}.\n", release.date);
        }
        for (group, entries) in &release.groups {
            let _ = write!(out, "\n### {}\n\n", group.title());
            for entry in entries {
                let _ = writeln!(out, "- {}", escape_md(&entry.text));
                if let Some(note) = &entry.migration {
                    let _ = write!(out, "\n  Migration: {}\n\n", escape_md(note));
                }
            }
        }
    }
    out
}

/// The RSS 2.0 feed of a Product, one item per release, every URL absolute.
pub fn render_rss(product: &Product, releases: &[Release], site_url: &str) -> String {
    let page = format!("{}/{}/", site_url.trim_end_matches('/'), page_path(product));
    let mut out = String::new();
    let _ = write!(
        out,
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<rss version=\"2.0\" xmlns:atom=\"http://www.w3.org/2005/Atom\">\n<channel>\n<title>Rulelink {title} release notes</title>\n<link>{page}</link>\n<description>What changed in each version of the Rulelink {title}.</description>\n<atom:link href=\"{feed}\" rel=\"self\" type=\"application/rss+xml\"/>\n",
        title = escape_xml(&product.title),
        feed = feed_url(site_url, product),
    );
    if let Some(first) = releases.first() {
        let _ = writeln!(out, "<lastBuildDate>{}</lastBuildDate>", first.rfc_date);
    }
    for release in releases {
        let heading = release.heading();
        let link = format!("{page}#{}", slug(&heading));
        let _ = write!(
            out,
            "<item>\n<title>Rulelink {} {}</title>\n<link>{link}</link>\n<guid isPermaLink=\"true\">{link}</guid>\n<pubDate>{}</pubDate>\n<description>{}</description>\n</item>\n",
            escape_xml(&product.title),
            escape_xml(&heading),
            release.rfc_date,
            escape_xml(&release_html(release)),
        );
    }
    out.push_str("</channel>\n</rss>\n");
    out
}

fn release_html(release: &Release) -> String {
    let mut html = String::new();
    for (group, entries) in &release.groups {
        let _ = write!(html, "<h3>{}</h3><ul>", group.title());
        for entry in entries {
            let _ = write!(html, "<li>{}", escape_xml(&entry.text));
            if let Some(note) = &entry.migration {
                let _ = write!(html, "<br/>Migration: {}", escape_xml(note));
            }
            html.push_str("</li>");
        }
        html.push_str("</ul>");
    }
    html
}

/// Reads `git log`, newest first.
fn read_commits() -> Result<Vec<Commit>, String> {
    let format = "%x1e%cs%x1f%cd%x1f%D%x1f%s%x1f%b";
    let output = Command::new("git")
        .args(["log", "--date=rfc2822", &format!("--format={format}")])
        .output()
        .map_err(|e| format!("cannot run git: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "git log failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(parse_log(&String::from_utf8_lossy(&output.stdout)))
}

fn parse_log(log: &str) -> Vec<Commit> {
    log.split('\x1e')
        .filter(|record| !record.trim().is_empty())
        .filter_map(|record| {
            let mut fields = record.splitn(5, '\x1f');
            let date = fields.next()?.trim().to_string();
            let rfc_date = fields.next()?.trim().to_string();
            let refs = fields.next()?;
            let subject = fields.next()?.trim().to_string();
            let body = fields.next().unwrap_or("").to_string();
            Some(Commit {
                date,
                rfc_date,
                tag: version_tag(refs),
                subject,
                body,
            })
        })
        .collect()
}

/// The version tag among `git log` decorations (`HEAD -> main, tag: v0.1.0`).
fn version_tag(refs: &str) -> Option<String> {
    refs.split(", ")
        .filter_map(|r| r.trim().strip_prefix("tag: "))
        .find(|tag| {
            let parts: Vec<&str> = tag.trim_start_matches('v').split('.').collect();
            parts.len() == 3
                && parts
                    .iter()
                    .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
        })
        .map(str::to_string)
}

fn site_url(config_js: &str) -> Result<String, String> {
    config_js
        .lines()
        .find_map(|line| line.strip_prefix("export const SITE_URL = '"))
        .and_then(|rest| rest.split('\'').next())
        .map(str::to_string)
        .ok_or_else(|| "SITE_URL not found in site/site.config.mjs".to_string())
}

/// Generates the files, or with `check` verifies they are up to date.
pub fn run(check: bool) -> Result<(), String> {
    let read = |path: &str| fs::read_to_string(path).map_err(|e| format!("{path}: {e}"));
    let config: Config = toml::from_str(&read("xtask/release-notes.toml")?)
        .map_err(|e| format!("xtask/release-notes.toml: {e}"))?;
    let url = site_url(&read("site/site.config.mjs")?)?;
    let products = build(&read_commits()?, &config)?;
    let mut stale = Vec::new();
    for (product, releases) in &products {
        let files = [
            (
                format!("docs/public/{}.md", page_path(product)),
                render_page(product, releases, &url),
            ),
            (
                format!("site/public/{}.xml", page_path(product)),
                render_rss(product, releases, &url),
            ),
        ];
        for (path, content) in files {
            if check {
                if fs::read_to_string(&path).ok().as_deref() != Some(content.as_str()) {
                    stale.push(path);
                }
            } else {
                if let Some(dir) = Path::new(&path).parent() {
                    fs::create_dir_all(dir).map_err(|e| format!("{path}: {e}"))?;
                }
                fs::write(&path, content).map_err(|e| format!("{path}: {e}"))?;
            }
        }
    }
    if stale.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "out of date, run `cargo xtask release-notes`: {}",
            stale.join(", ")
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config() -> Config {
        toml::from_str(
            r#"
            ignore = ["site", "docs"]
            [[product]]
            id = "cli"
            title = "CLI"
            scopes = ["cli", "rules"]
            [[product]]
            id = "vscode"
            title = "VS Code extension"
            scopes = ["vscode"]
            "#,
        )
        .expect("config")
    }

    fn commit(subject: &str, body: &str, tag: Option<&str>) -> Commit {
        Commit {
            date: "2026-11-25".into(),
            rfc_date: "Wed, 25 Nov 2026 10:00:00 +0100".into(),
            tag: tag.map(str::to_string),
            subject: subject.into(),
            body: body.into(),
        }
    }

    fn cli(commits: &[Commit]) -> Vec<Release> {
        build(commits, &config()).expect("build").remove(0).1
    }

    #[test]
    fn groups_follow_the_agreed_order_with_breaking_first() {
        let releases = cli(&[
            commit("fix(cli): stop a crash (#4)", "", None),
            commit("deprecate(cli): old flag", "", None),
            commit("perf(rules): faster list", "", None),
            commit("feat(cli): add where", "", None),
            commit(
                "feat(cli)!: rename init",
                "BREAKING CHANGE: use `start`.",
                None,
            ),
        ]);
        let groups: Vec<Group> = releases[0].groups.iter().map(|(g, _)| *g).collect();
        assert_eq!(
            groups,
            [
                Group::Breaking,
                Group::Added,
                Group::Changed,
                Group::Deprecated,
                Group::Fixed
            ]
        );
        assert_eq!(releases[0].groups[4].1[0].text, "Stop a crash");
    }

    #[test]
    fn a_breaking_change_carries_the_migration_note_of_its_footer() {
        let body = "Why.\n\nBREAKING CHANGE: the `--id` flag is gone.\nUse `--key` instead.\n\nAssisted-by: x\n";
        let releases = cli(&[commit("feat(cli)!: drop --id", body, None)]);
        let entry = &releases[0].groups[0].1[0];
        assert_eq!(
            entry.migration.as_deref(),
            Some("the `--id` flag is gone. Use `--key` instead.")
        );
    }

    #[test]
    fn a_footer_alone_marks_a_change_as_breaking() {
        let releases = cli(&[commit(
            "fix(cli): change exit code",
            "BREAKING CHANGE: exit 2 now.\n",
            None,
        )]);
        assert_eq!(releases[0].groups[0].0, Group::Breaking);
    }

    #[test]
    fn a_breaking_change_without_footer_says_so() {
        let releases = cli(&[commit("feat(cli)!: drop it", "", None)]);
        assert_eq!(
            releases[0].groups[0].1[0].migration.as_deref(),
            Some(NO_MIGRATION)
        );
    }

    #[test]
    fn tags_split_versions_and_later_commits_are_unreleased() {
        let releases = cli(&[
            commit("feat(cli): three", "", None),
            commit("fix(cli): two", "", Some("v0.1.1")),
            commit("feat(cli): one", "", Some("v0.1.0")),
        ]);
        let names: Vec<String> = releases.iter().map(Release::heading).collect();
        assert_eq!(names, ["Unreleased", "0.1.1", "0.1.0"]);
    }

    #[test]
    fn a_version_without_user_facing_change_is_left_out() {
        let releases = cli(&[
            commit("chore: release", "", Some("v0.2.0")),
            commit("feat(cli): one", "", Some("v0.1.0")),
        ]);
        assert_eq!(releases.len(), 1);
    }

    #[test]
    fn scopes_are_mapped_to_one_product_and_others_are_ignored() {
        let commits = [
            commit("feat(vscode): add a panel", "", None),
            commit("feat(site): new page", "", None),
            commit("docs(cli): words", "", None),
            commit("feat(rules): list", "", None),
        ];
        let built = build(&commits, &config()).expect("build");
        assert_eq!(built[0].1[0].groups[0].1.len(), 1);
        assert_eq!(built[1].1[0].groups[0].1[0].text, "Add a panel");
    }

    #[test]
    fn a_crate_name_is_not_a_product_and_unmapped_scopes_fail() {
        let err = build(&[commit("feat(registry): internals", "", None)], &config())
            .expect_err("unmapped scope");
        assert!(err.contains("feat(registry)"), "{err}");
        let err =
            build(&[commit("fix: no scope", "", None)], &config()).expect_err("unmapped scope");
        assert!(err.contains("no scope"), "{err}");
    }

    #[test]
    fn identical_lines_in_a_group_appear_once() {
        let releases = cli(&[
            commit("feat(cli): same", "", None),
            commit("feat(cli): same", "", None),
        ]);
        assert_eq!(releases[0].groups[0].1.len(), 1);
    }

    #[test]
    fn non_conventional_and_internal_commits_are_skipped() {
        let releases = cli(&[
            commit("Merge ticket #40: site foundation", "", None),
            commit("chore(ia): tidy", "", None),
            commit("docs: words", "", None),
        ]);
        assert_eq!(releases.len(), 0);
    }

    #[test]
    fn the_page_has_front_matter_groups_and_an_absolute_feed_link() {
        let product = config().products[0].clone();
        let releases = cli(&[
            commit("fix(cli): crash", "", None),
            commit(
                "feat(cli)!: rename <init>",
                "BREAKING CHANGE: use start.",
                Some("v0.1.0"),
            ),
        ]);
        let page = render_page(&product, &releases, "https://rulelink.dev");
        assert!(page.starts_with("---\ntitle: CLI release notes\ndescription: "));
        assert!(page.contains("[RSS feed](https://rulelink.dev/release-notes/cli.xml)"));
        assert!(page.contains("## 0.1.0\n\nReleased 2026-11-25."));
        let breaking = page.find("### Breaking").expect("breaking");
        assert!(page.find("## Unreleased").expect("unreleased") < breaking);
        assert!(page.contains("Migration: use start."));
        assert!(page.contains("Rename &lt;init>"));
    }

    #[test]
    fn the_feed_has_absolute_urls_and_escaped_content() {
        let product = config().products[0].clone();
        let releases = cli(&[commit("feat(cli): handle a & b", "", Some("v0.1.0"))]);
        let rss = render_rss(&product, &releases, "https://rulelink.dev/");
        assert!(rss.contains("<link>https://rulelink.dev/release-notes/cli/</link>"));
        assert!(rss.contains(
            "<guid isPermaLink=\"true\">https://rulelink.dev/release-notes/cli/#010</guid>"
        ));
        assert!(rss.contains("href=\"https://rulelink.dev/release-notes/cli.xml\""));
        assert!(rss.contains("a &amp;amp; b"));
        assert!(rss.contains("<pubDate>Wed, 25 Nov 2026 10:00:00 +0100</pubDate>"));
    }

    #[test]
    fn git_log_records_are_parsed_with_their_version_tag() {
        let log = "\x1e2026-11-25\x1fWed, 25 Nov 2026 10:00:00 +0100\x1fHEAD -> main, tag: v0.1.0, origin/main\x1ffeat(cli): x\x1fBody\n\x1e2026-11-24\x1fTue, 24 Nov 2026 10:00:00 +0100\x1f\x1ffix(cli): y\x1f";
        let commits = parse_log(log);
        assert_eq!(commits.len(), 2);
        assert_eq!(commits[0].tag.as_deref(), Some("v0.1.0"));
        assert_eq!(commits[1].tag, None);
        assert_eq!(commits[1].subject, "fix(cli): y");
    }

    #[test]
    fn the_site_url_is_read_from_the_site_config() {
        let js = "// c\nexport const SITE_URL = 'https://rulelink.dev';\nexport const X = 1;\n";
        assert_eq!(site_url(js).as_deref(), Ok("https://rulelink.dev"));
    }
}
