import { visit } from 'unist-util-visit';
import { existsSync, statSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { DOCS_SOURCE } from '../../site.config.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..', DOCS_SOURCE);

/**
 * Makes pages written for GitHub work on the site:
 * - drops the leading H1 (Starlight renders the title itself);
 * - turns relative links (`rules-add.md`, `../cli/`) into site routes;
 * - fails the build when such a link points to nothing.
 *
 * Why not starlight-links-validator: it assumes the docs sit in `src/content/docs`
 * and mis-reads pages loaded from `docs/public`.
 */
export function remarkPublicDocs() {
  return (tree, file) => {
    const from = file.path;
    if (!from || !path.resolve(from).startsWith(root + path.sep)) return;
    const first = tree.children[0];
    if (first?.type === 'heading' && first.depth === 1) tree.children.shift();
    visit(tree, 'link', (node) => {
      if (/^([a-z][a-z0-9+.-]*:|#|\/)/i.test(node.url)) return;
      const [target, hash] = node.url.split('#');
      let abs = path.resolve(path.dirname(from), target);
      // A link to a folder is valid on GitHub when the folder has a README.md.
      if (existsSync(abs) && statSync(abs).isDirectory()) abs = path.join(abs, 'README.md');
      if (!abs.startsWith(root + path.sep) || !abs.endsWith('.md') || !existsSync(abs)) {
        throw new Error(`${path.relative(root, from)}: broken link \`${node.url}\``);
      }
      const rel = path
        .relative(root, abs)
        .split(path.sep)
        .join('/')
        .replace(/(^|\/)README\.md$/, '')
        .replace(/\.md$/, '');
      node.url = (rel ? `/${rel}/` : '/') + (hash ? `#${hash}` : '');
    });
  };
}
