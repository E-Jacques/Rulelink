// The one place that knows where the site is served from and where its content lives.
// Everything else (canonical URLs, sitemap, "Edit this page") reads these values.
export const SITE_URL = 'https://rulelink.dev';
export const REPO_URL = 'https://github.com/E-Jacques/Rulelink';
export const DOCS_SOURCE = '../docs/public'; // relative to site/
export const EDIT_BRANCH = 'main';

// Routes the showcase links to. Each is the route of a page under docs/public, so the home
// page never repeats a path by hand. tests/boundaries.test.mjs checks that each one exists.
export const ROUTES = {
  introduction: '/introduction/',
  installation: '/installation/',
  quickstart: '/get-started/quickstart/',
  roadmap: '/roadmap/',
  guides: '/guides/write-a-good-rule/',
  cli: '/cli/',
  crates: '/developers/crates/',
  glossary: '/reference/glossary/',
  releaseNotes: '/release-notes/cli/',
};
export const route = (name) => ROUTES[name];
export const INSTALLER_URL = `${REPO_URL}/releases/latest/download`;
