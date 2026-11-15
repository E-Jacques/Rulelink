# Every technical element is English; only rule values may be in another language

Format keys, CLI commands and options, configuration keys, report text, error messages and skills are English only. The tool has no `--lang` option, no translation catalogue and no `locale` setting. Rule values (`title`, `statement`, `scope`...) can be written in any language: the tool treats them as data and never checks their content. Only an agent may read them later to verify that a rule is respected and correctly implemented.

The rule format is a public contract (`"format": 1`) that is hard to change once published, and a single technical language keeps the format, the schema and the agent skills unambiguous. The brief names the fields in French (`titre`, `énoncé`, `statut`...); the mapping to English keys is kept in the registry design only as documentation.

## Considered options

- French keys, or both: rejected, accented keys and duplicated vocabularies complicate the schema and every consumer.
- English keys with localised display labels (the first draft of ADR-003): rejected, localisation adds a catalogue to maintain for a solo project and nothing requires it.

## Consequences

- Search stays accent- and case-insensitive so French values remain easy to find.
- The default identifier prefix is `RULE`, configurable per project. There is no language-specific template (the former `RG` convention, from the French "règle de gestion", is gone).
