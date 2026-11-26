// The theme picker of the showcase pages. It behaves like the documentation's
// (Starlight's ThemeSelect), and shares its stored preference.
type Theme = 'auto' | 'dark' | 'light';

const storageKey = 'starlight-theme';
const parseTheme = (value: unknown): Theme =>
  value === 'dark' || value === 'light' ? value : 'auto';

function loadTheme(): Theme {
  try { return parseTheme(localStorage.getItem(storageKey)); } catch { return 'auto'; }
}

const system = matchMedia('(prefers-color-scheme: light)');
const select = document.querySelector<HTMLSelectElement>('[data-theme-select]');

function apply(theme: Theme) {
  if (select) select.value = theme;
  document.documentElement.dataset.theme =
    theme === 'auto' ? (system.matches ? 'light' : 'dark') : theme;
}

select?.addEventListener('change', () => {
  const theme = parseTheme(select.value);
  apply(theme);
  // Auto is stored as an empty string, as the documentation does.
  try { localStorage.setItem(storageKey, theme === 'auto' ? '' : theme); } catch {}
});

system.addEventListener('change', () => {
  if (loadTheme() === 'auto') apply('auto');
});

// A choice made in another tab, or on a documentation page reached with Back.
addEventListener('pageshow', () => apply(loadTheme()));

apply(loadTheme());
