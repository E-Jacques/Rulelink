import { existsSync, readdirSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { DOCS_SOURCE } from '../../site.config.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..', DOCS_SOURCE);

const hasPage = (slug) => existsSync(path.join(root, `${slug}.md`));
const titleCase = (s) => s.replace(/-/g, ' ').replace(/^./, (c) => c.toUpperCase());

/**
 * The slugs of every page in a folder, its README (the folder's index) first.
 * Starlight's `autogenerate` only sees pages under `src/content/docs`, so it returns nothing
 * for pages read in place from the public documentation.
 */
function folderPages(dir) {
  const full = path.join(root, dir);
  if (!existsSync(full)) return [];
  return readdirSync(full, { withFileTypes: true })
    .sort((a, b) => (b.name === 'README.md') - (a.name === 'README.md') || a.name.localeCompare(b.name))
    .flatMap((f) => {
      if (f.isDirectory()) return folderPages(`${dir}/${f.name}`);
      if (!f.name.endsWith('.md')) return [];
      return [`${dir}/${f.name === 'README.md' ? 'index' : f.name.slice(0, -3)}`];
    });
}

/** CLI commands that have subcommands: their pages (`<command>-<sub>`) share one section. */
const COMMAND_GROUPS = ['rules'];

/** Replaces a command's subcommand pages with one section, where the first of them was. */
function groupSubcommands(pages) {
  return COMMAND_GROUPS.reduce((items, command) => {
    const isSub = (i) => typeof i === 'string' && i.startsWith(`cli/${command}-`);
    const subs = items.filter(isSub);
    if (!subs.length) return items;
    const at = items.findIndex(isSub);
    const rest = items.filter((i) => !isSub(i));
    return rest.toSpliced(at, 0, { label: `rulelink ${command}`, items: subs });
  }, pages);
}

/**
 * The agreed navigation. An entry is a page (`slug`) or a folder (`dir`, one entry per page).
 * Entries without content are left out, and so are groups left empty, so a group appears
 * as soon as its first page is added and never shows an empty or broken entry.
 */
const NAVIGATION = [
  {
    label: 'Get started',
    entries: ['introduction', 'roadmap', 'installation', 'get-started/quickstart'].map((slug) => ({ slug })),
  },
  { label: 'Guides', entries: [{ dir: 'guides' }] },
  { label: 'Concepts', entries: [{ dir: 'concepts' }] },
  { label: 'CLI reference', entries: [{ dir: 'cli' }] },
  {
    label: 'Reference',
    entries: [{ dir: 'developers' }, { dir: 'release-notes' }, { slug: 'reference/glossary' }],
  },
];

export function sidebar() {
  return NAVIGATION.flatMap(({ label, entries }) => {
    // A group that is one folder lists that folder's pages directly.
    const flat = entries.length === 1 && entries[0].dir;
    const items = entries.flatMap((e) => {
      if (e.slug) return hasPage(e.slug) ? [e.slug] : [];
      const pages = folderPages(e.dir);
      if (flat) return groupSubcommands(pages);
      return pages.length ? [{ label: titleCase(e.dir), items: pages }] : [];
    });
    return items.length ? [{ label, items }] : [];
  });
}
