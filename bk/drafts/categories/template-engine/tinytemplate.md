<!-- TODO Reviewed ??-??-2026 -->

# Tinytemplate

{{#include tinytemplate.incl.md}}

## Create Markdown Fragments from a Template {#tinytemplate}

{{#badges tinytemplate template-engine="Template engine"}}{{hi:Html}}{{hi:Template}}

[`tinytemplate`][c~tinytemplate~docs]↗{{hi:tinytemplate}} is a small runtime template engine for string interpolation and control flow. It is a good fit when templates are short, known at build time, and rendered to a `String`. The tool templates in `tools/tool_lib/src/templates/` use this pattern with embedded template constants and custom formatters.

```rust,editable
{{#include ../../../crates/cats/template_engine/examples/tinytemplate/tinytemplate.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Template Engines][p~template-engine]
- [Tera][ex~template-engine~create-html-from-template] for file-based templates, filters, and HTML autoescaping.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
