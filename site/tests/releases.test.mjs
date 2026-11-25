// Run with `node --test tests` from site/ (no install needed).
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { parseReleaseNotes } from '../src/showcase/releases.mjs';
import { DOCS_SOURCE } from '../site.config.mjs';

// The exact shape `cargo xtask release-notes` writes (see render_page in xtask/src/release_notes.rs).
const generated = `---
title: CLI release notes
---

# CLI release notes

Intro.

## Unreleased

### Added

- Add A

## 0.1.0

Released 2026-01-02.

### Breaking

- Rename &lt;init>

  Migration: use &#123;start} instead.

### Fixed

- Fix B
`;

test('parses the unreleased group and a released version with its date', () => {
  const { versions, unreleased } = parseReleaseNotes(generated);
  assert.deepEqual(unreleased, [{ title: 'Added', items: [{ text: 'Add A' }] }]);
  assert.equal(versions.length, 1);
  assert.equal(versions[0].version, '0.1.0');
  assert.equal(versions[0].date, '2026-01-02');
  assert.deepEqual(versions[0].groups.map((g) => g.title), ['Breaking', 'Fixed']);
});

test('keeps the migration note and unescapes what the generator escaped', () => {
  const [breaking] = parseReleaseNotes(generated).versions[0].groups;
  assert.deepEqual(breaking.items, [{ text: 'Rename <init>', migration: 'use {start} instead.' }]);
});

test('notes without a version list none', () => {
  assert.deepEqual(parseReleaseNotes('# Notes\n\nNo user-facing changes yet.\n').versions, []);
});

test('the committed release notes parse', () => {
  const site = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
  const md = readFileSync(path.join(site, DOCS_SOURCE, 'release-notes/cli.md'), 'utf8');
  const { versions, unreleased } = parseReleaseNotes(md);
  assert.ok(Array.isArray(versions) && Array.isArray(unreleased));
});
