---
name: public-documentation
description: How to create or edit a guide in the public documentation (tone, structure, front matter, review checklist). Use when adding or changing a page under `docs/public`, except CLI command pages.
---

# Public documentation

Guides live in `docs/public/guides/`, one Markdown file each. Command pages follow the `cli-documentation` skill instead. Concept pages follow the same tone as guides.

## Tone

Two audiences, two registers.

- **Guides, first steps and concepts** (Introduction, Installation, Quickstart, `guides/`, `concepts/`): simple professional English. Short sentences, one idea each. Active voice, present tense. Speak to the reader as "you". Use the terms of `GLOSSARY.md` and no others (Rule, Source, Annotation, Finding, not ticket or requirement). Do not name crates, types or modules. Explain a command by what the reader sees, not how it works inside.
- **CLI reference and crate reference** (`cli/`, `developers/`): jargon is accepted. Crate and type names are fine there.

Describe only what the current version does. Announced features belong on the Roadmap only. Never document an option or command that does not exist.

## Front matter

Every guide starts with two fields, both required and non-empty. `cargo xtask docs-check` fails without them.

```yaml
---
title: Write a good Rule
description: Learn how to write a Rule that states one criterion anyone can verify.
---
```

- `title`: a short name, in sentence case. The first `# heading` repeats it.
- `description`: one sentence, in plain words, saying what the reader will learn or be able to do. Start with a verb (Learn, Run, Find). No jargon, no marketing words, no more than about 20 words.

## Structure of a guide

Start from [templates/guide-template.md](templates/guide-template.md). Keep the sections in this order.

1. **Title** (`# …`), same as `title`.
2. **Introduction**: two or three sentences. Say what the reader will do and why.
3. **Before you start**: what the reader needs. Omit it if nothing is needed.
4. **Steps** or **topic sections**: one `##` heading per step or idea. Each step has an example the reader can copy and run as written.
5. **Do and do not**: for guides that teach a practice, show good and bad examples side by side.
6. **Next steps**: links to related guides, concept pages and command pages.

Link to other pages with relative `.md` paths so they work on GitHub and on the site. Link a command by its page in `cli/`.

## Create a guide

1. Check the guide does not exist. Search `docs/public` for the topic.
2. Check every term against `GLOSSARY.md`. If a term is missing, add it to the glossary first.
3. Copy the template to `docs/public/guides/<topic>.md`. Use lower-case words joined by hyphens.
4. Fill in the front matter, replace every `{{ placeholder }}` and delete the HTML comments.
5. Run each command in the examples against the real binary.
6. Link the guide from a related page so readers can find it.
7. Run the checklist below, then `moon run :docs-check`.

## Edit a guide

1. Change the text and its examples together. Re-run every example you touched.
2. Keep the terms, the tone and the section order. Do not rewrite sentences you have no reason to change.
3. Update `description` if the guide now teaches something different.
4. If the change comes from a code change, make it in the same pull request as the code.
5. Run the checklist below, then `moon run :docs-check`.

## Review checklist

- [ ] The front matter has a `title` and a one-sentence `description` in plain words that says what the reader will learn.
- [ ] Sentences are short. One idea each.
- [ ] Only terms from `GLOSSARY.md`. No crate, type or module names.
- [ ] Every example runs as written on the current version.
- [ ] Nothing described is only announced. No placeholder remains.
- [ ] Links are relative `.md` paths and point to pages that exist.
- [ ] The sections follow the order above.
- [ ] `moon run :docs-check` passes.
