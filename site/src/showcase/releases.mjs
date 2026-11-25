// Reads the generated release notes (docs/public/release-notes/cli.md) so the Versions page
// never invents a version. Pure function over the markdown text, tested in tests/.
//
// The format is the one `cargo xtask release-notes` writes (xtask/src/release_notes.rs):
//
//   ## Unreleased | ## 0.1.0
//   Released 2026-01-02.            (versions only)
//   ### Added
//   - An item                       (text escaped with escape_md)
//     Migration: a note             (breaking changes only, after a blank line)

/** Undoes `escape_md` in the generator, so the page shows what the commit said. */
export function unescapeMd(text) {
  return text.replaceAll('&lt;', '<').replaceAll('&#123;', '{');
}

/**
 * @typedef {{ text: string, migration?: string }} Item
 * @typedef {{ title: string, items: Item[] }} Group
 * @param {string} markdown the generated release notes
 * @returns {{ versions: { version: string, date?: string, groups: Group[] }[], unreleased: Group[] }}
 */
export function parseReleaseNotes(markdown) {
  const versions = [];
  let unreleased = [];
  let current = null;
  let group = null;
  let item = null;
  for (const line of markdown.split(/\r?\n/)) {
    const h2 = /^##\s+(.+?)\s*$/.exec(line);
    const h3 = /^###\s+(.+?)\s*$/.exec(line);
    const li = /^[-*]\s+(.+?)\s*$/.exec(line);
    const released = /^Released (\d{4}-\d{2}-\d{2})\.$/.exec(line);
    const migration = /^\s+Migration:\s*(.+?)\s*$/.exec(line);
    if (h2) {
      group = item = null;
      const version = /^v?(\d+\.\d+\.\d+\S*)$/.exec(h2[1]);
      if (/^unreleased$/i.test(h2[1])) {
        current = { groups: [] };
        unreleased = current.groups;
      } else if (version) {
        current = { version: version[1], date: undefined, groups: [] };
        versions.push(current);
      } else {
        current = null;
      }
    } else if (released && current && !group) {
      current.date = released[1];
    } else if (h3 && current) {
      item = null;
      group = { title: h3[1], items: [] };
      current.groups.push(group);
    } else if (li && group) {
      item = { text: unescapeMd(li[1]) };
      group.items.push(item);
    } else if (migration && item) {
      item.migration = unescapeMd(migration[1]);
    }
  }
  return { versions, unreleased };
}
