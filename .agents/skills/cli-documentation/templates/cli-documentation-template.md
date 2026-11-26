---
title: {{ title }}
since: {{ version }}
---

# {{ title }}

{{ description }}

## Example

```bash
{{ example }}
```

<!-- If applicable -->

## Arguments

| name             | description              | example             |
| ---------------- | ------------------------ | ------------------- |
| {{ arg_name_1 }} |  {{ arg_description_1 }} | {{ arg_example_1 }} |
| {{ arg_name_2 }} |  {{ arg_description_2 }} | {{ arg_example_2 }} |

<!-- endif -->

<!-- If applicable -->

## Options

| name                | description                 | example                |
| ------------------- | --------------------------- | ---------------------- |
| {{ option_name_1 }} |  {{ option_description_1 }} | {{ option_example_1 }} |
| {{ option_name_2 }} |  {{ option_description_2 }} | {{ option_example_2 }} |

<!-- endif -->

## Exit codes

| code | meaning               |
| ---- | --------------------- |
| `0`  | {{ success_meaning }} |
| `2`  | {{ invalid_meaning }} |

## See also

- [{{ related_command }}]({{ related_page }}.md)
