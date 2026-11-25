/** Copies `text` and flips the button label to "Copied" for two seconds. */
export async function copyText(button: HTMLElement, text: string) {
  const label = button.querySelector<HTMLElement>('[data-copy-label]');
  try {
    await navigator.clipboard.writeText(text);
    button.dataset.copied = 'true';
    if (label) label.textContent = 'Copied';
  } catch {
    if (label) label.textContent = 'Press Ctrl+C';
  }
  setTimeout(() => {
    delete button.dataset.copied;
    if (label) label.textContent = 'Copy';
  }, 2000);
}
