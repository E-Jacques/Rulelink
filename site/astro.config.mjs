import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import sitemap from '@astrojs/sitemap';
import { SITE_URL, REPO_URL, EDIT_BRANCH } from './site.config.mjs';
import { sidebar } from './src/docs/sidebar.mjs';
import { noPlaceholders } from './src/docs/no-placeholders.mjs';
import { remarkPublicDocs } from './src/docs/remark-public-docs.mjs';

export default defineConfig({
  site: SITE_URL,
  // Every page is a directory (`/guides/page/index.html`), so a plain Apache host serves it.
  trailingSlash: 'always',
  build: { format: 'directory' },
  markdown: { remarkPlugins: [remarkPublicDocs] },
  integrations: [
    // The component library is a maintainer page, not content to index.
    sitemap({ filter: (page) => !page.includes('/components/') }),
    starlight({
      title: 'Rulelink',
      // The mark sits beside the title, which stays visible.
      logo: { src: './src/assets/logo.svg', alt: '' },
      customCss: [
        '@fontsource/ibm-plex-sans/400.css',
        '@fontsource/ibm-plex-sans/600.css',
        '@fontsource/ibm-plex-mono/400.css',
        './src/docs/tokens.css',
        './src/docs/theme.css',
      ],
      editLink: { baseUrl: `${REPO_URL}/edit/${EDIT_BRANCH}/site/` },
      social: [{ icon: 'github', label: 'GitHub', href: REPO_URL }],
      sidebar: sidebar(),
    }),
    noPlaceholders(),
  ],
});
