import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import { SITE_URL, REPO_URL } from './site.config.mjs';
import { remarkPublicDocs } from './src/docs/remark-public-docs.mjs';

export default defineConfig({
  site: SITE_URL,
  markdown: { remarkPlugins: [remarkPublicDocs] },
  integrations: [
    starlight({
      title: 'Rulelink',
      customCss: [
        '@fontsource/ibm-plex-sans/400.css',
        '@fontsource/ibm-plex-sans/600.css',
        '@fontsource/ibm-plex-mono/400.css',
        './src/docs/tokens.css',
        './src/docs/theme.css',
      ],
      editLink: { baseUrl: `${REPO_URL}/edit/main/docs/public/` },
      social: [{ icon: 'github', label: 'GitHub', href: REPO_URL }],
      sidebar: [
        { label: 'Concepts', autogenerate: { directory: 'concepts' } },
        { label: 'CLI reference', autogenerate: { directory: 'cli' } },
      ],
    }),
  ],
});
