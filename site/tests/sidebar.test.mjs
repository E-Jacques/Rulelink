import { test } from 'node:test';
import assert from 'node:assert/strict';
import { sidebar } from '../src/docs/sidebar.mjs';

const slugs = (items) => items.flatMap((i) => (typeof i === 'string' ? [i] : slugs(i.items ?? [])));

test('every navigation group and subgroup lists its pages', () => {
  const check = (group) => {
    assert.ok(group.items?.length > 0, `${group.label} has no entry`);
    group.items.filter((i) => typeof i !== 'string').forEach(check);
  };
  sidebar().forEach(check);
  assert.deepEqual(sidebar().map((g) => g.label), ['Get started', 'Guides', 'Concepts', 'CLI reference', 'Reference']);
  assert.ok(slugs(sidebar()).includes('guides/write-a-good-rule'));
  assert.ok(slugs(sidebar()).includes('cli/init'));
});

test('the subcommands of a command share one CLI reference section', () => {
  const cli = sidebar().find((g) => g.label === 'CLI reference');
  const rules = cli.items.find((i) => i.label === 'rulelink rules');
  assert.ok(rules.items.includes('cli/rules-add'));
  assert.ok(rules.items.every((i) => i.startsWith('cli/rules-')));
  assert.deepEqual(cli.items.filter((i) => typeof i === 'string' && i.startsWith('cli/rules-')), []);
});
