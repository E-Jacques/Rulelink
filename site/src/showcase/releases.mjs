// Reads the generated release notes (docs/public/release-notes/cli.md) so the Versions page
// never invents a version. Pure function over the markdown text, tested in tests/.

/**
 * @param {string} markdown the generated release notes
 * @returns {{ versions: {version: string, date?: string, groups: {title: string, items: string[]}[]}[], unreleased: {title: string, items: string[]}[] }}
 */
export function parseReleaseNotes(markdown) {
  const out = { versions: [], unreleased: [] };
  let current = null;
  let group = null;
  for (const line of markdown.split(/\r?\n/)) {
    const h2 = /^##\s+(.+?)\s*$/.exec(line);
    const h3 = /^###\s+(.+?)\s*$/.exec(line);
    const li = /^[-*]\s+(.+?)\s*$/.exec(line);
    if (h2) {
      group = null;
      const m = /^v?(\d+\.\d+\.\d+\S*)(?:\s*[-(–—]\s*\(?(\d{4}-\d{2}-\d{2})\)?)?$/.exec(h2[1]);
      if (/^unreleased$/i.test(h2[1])) current = { unreleased: true, groups: [] };
      else if (m) current = { version: m[1], date: m[2], groups: [] };
      else current = null;
      if (current?.unreleased) out.unreleasedRef = current;
      else if (current) out.versions.push(current);
    } else if (h3 && current) {
      group = { title: h3[1], items: [] };
      current.groups.push(group);
    } else if (li && group) {
      group.items.push(li[1]);
    }
  }
  out.unreleased = out.unreleasedRef?.groups ?? [];
  delete out.unreleasedRef;
  return out;
}
