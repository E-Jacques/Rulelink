import { readFileSync } from 'node:fs';
import { glob } from 'astro/loaders';
import type { Loader } from 'astro/loaders';
import { DOCS_SOURCE } from '../../site.config.mjs';

/** The H1 of a Markdown file, which GitHub shows and Starlight needs as `title`. */
function firstHeading(path: string): string | undefined {
  return /^#\s+(.+)$/m.exec(readFileSync(path, 'utf8'))?.[1].trim();
}

/**
 * Reads `docs/public` in place (no copy). Two accommodations for pages written for GitHub:
 * - a missing `title` is taken from the page's first H1;
 * - `README.md` (what GitHub shows for a folder) becomes the folder's index page.
 */
export function publicDocsLoader(): Loader {
  const inner = glob({
    pattern: '**/*.md',
    base: DOCS_SOURCE,
    generateId: ({ entry }) =>
      entry.replace(/\.md$/, '').replace(/(^|\/)README$/, '$1index'),
  });
  return {
    name: 'public-docs',
    async load(ctx) {
      const parseData: typeof ctx.parseData = (entry) => {
        const data = { ...entry.data } as Record<string, unknown>;
        if (!data.title && entry.filePath) data.title = firstHeading(entry.filePath);
        return ctx.parseData({ ...entry, data } as typeof entry);
      };
      await inner.load({ ...ctx, parseData });
    },
  };
}
