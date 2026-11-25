// Run with `node --test tests` from site/ (no install needed).
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readdirSync, readFileSync, existsSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { ROUTES, DOCS_SOURCE } from '../site.config.mjs';

const site = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const files = (dir) =>
  readdirSync(path.join(site, dir), { recursive: true })
    .map(String)
    .filter((f) => /\.(astro|mjs|ts|css)$/.test(f))
    .map((f) => path.join(site, dir, f));

test('the documentation part does not depend on the showcase part', () => {
  for (const dir of ['src/docs', 'src/components']) {
    for (const f of files(dir)) {
      assert.doesNotMatch(readFileSync(f, 'utf8'), /showcase/i, `${path.relative(site, f)} mentions the showcase`);
    }
  }
});

test('the showcase imports nothing from the docs part but the shared tokens', () => {
  for (const f of files('src/showcase')) {
    const imports = [...readFileSync(f, 'utf8').matchAll(/from\s+['"]([^'"]+)['"]|import\s+['"]([^'"]+)['"]/g)].map((m) => m[1] ?? m[2]);
    for (const i of imports.filter((i) => i.includes('/docs/'))) {
      assert.ok(i.endsWith('tokens.css'), `${path.relative(site, f)} imports ${i}`);
    }
  }
});

test('the Versions page tells the truth about the release notes', async () => {
  const { parseReleaseNotes } = await import('../src/showcase/releases.mjs');
  const md = readFileSync(path.join(site, DOCS_SOURCE, 'release-notes/cli.md'), 'utf8');
  assert.ok(parseReleaseNotes(md).unreleased.length > 0, 'the generated notes list unreleased changes');
  const sample = parseReleaseNotes('## Unreleased\n\n### Added\n\n- A\n\n## 0.1.0 - 2026-01-02\n\n### Fixed\n\n- B\n');
  assert.deepEqual(sample.unreleased, [{ title: 'Added', items: ['A'] }]);
  assert.equal(sample.versions[0].version, '0.1.0');
  assert.equal(sample.versions[0].date, '2026-01-02');
  assert.deepEqual(parseReleaseNotes('# Notes\n').versions, []);
});

test('every route the showcase links to is a page of docs/public', () => {
  const root = path.resolve(site, DOCS_SOURCE);
  for (const [name, route] of Object.entries(ROUTES)) {
    const rel = route.replace(/^\/|\/$/g, '');
    const ok = existsSync(path.join(root, `${rel}.md`)) || existsSync(path.join(root, rel, 'README.md'));
    assert.ok(ok, `route "${name}" (${route}) has no page in ${DOCS_SOURCE}`);
  }
});
