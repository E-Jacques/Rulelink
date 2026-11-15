# 06: Annotation scan and `where`

**What to build:** A lexical, language-agnostic scan finds `@rule <ID>` Annotations in comments across many languages and comment styles, respecting ignore files and configured exclusions. One file can cite several Rules and one Rule can appear in several places, tests included. `where RULE-042` lists the code locations and behaves clearly for unknown, retired and uncited Identifiers.

**Blocked by:** 02 (Create Rules and allocate Identifiers)

**Status:** ready-for-agent

- [ ] Annotations are found in line and block comments of several languages (e.g. `//`, `#`, `--`, `/* */`, `<!-- -->`)
- [ ] Multiple Annotations per file and multiple locations per Rule are all reported
- [ ] `.gitignore`-style ignore files and configured exclusions are respected
- [ ] `where` lists file and line for each location
- [ ] `where` on an unknown, retired or uncited Identifier gives a distinct, useful answer
- [ ] No tree-sitter and no enclosing-symbol resolution
