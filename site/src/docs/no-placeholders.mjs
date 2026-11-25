import { readdirSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

// Text that must never reach a published page.
const PLACEHOLDER =
  /\b(TODO|TBD|FIXME|XXX)\b|lorem ipsum|\{\{[^}]*\}\}|\[(?:your|organi[sz]ation|domain|npm package)[^\]]*\]|example\.com/i;

/** Fails the build when a rendered page still contains a placeholder. */
export function noPlaceholders() {
  return {
    name: 'no-placeholders',
    hooks: {
      'astro:build:done': ({ dir }) => {
        const out = fileURLToPath(dir);
        const bad = [];
        for (const f of readdirSync(out, { recursive: true })) {
          if (!String(f).endsWith('.html') || String(f).startsWith('pagefind')) continue;
          const text = readFileSync(path.join(out, String(f)), 'utf8')
            .replace(/<(script|style)[\s\S]*?<\/\1>/g, '')
            .replace(/<[^>]+>/g, ' ');
          const m = PLACEHOLDER.exec(text);
          if (m) bad.push(`${f}: \`${m[0]}\``);
        }
        if (bad.length) throw new Error(`Placeholder in rendered pages:\n${bad.join('\n')}`);
      },
    },
  };
}
