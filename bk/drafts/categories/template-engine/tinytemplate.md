<!-- TODO Reviewed ??-??-2026 -->

# Tinytemplate

{{#include tinytemplate.incl.md}}

## Create Markdown Fragments from a Template {#tinytemplate}

[![tinytemplate][c~tinytemplate~docs~badge]][c~tinytemplate~docs] [![tinytemplate~crates.io][c~tinytemplate~crates.io~badge]][c~tinytemplate~crates.io] [![tinytemplate~repo][c~tinytemplate~repo~badge]][c~tinytemplate~repo] [![tinytemplate~lib.rs][c~tinytemplate~lib.rs~badge]][c~tinytemplate~lib.rs]{{hi:tinytemplate}}{{hi:Html}}{{hi:Template}} [![cat~template-engine][cat~template-engine~badge]][cat~template-engine]{{hi:Template engine}}

[`tinytemplate`][c~tinytemplate~docs]↗{{hi:tinytemplate}} is a small runtime template engine for string interpolation and control flow. It is a good fit when templates are short, known at build time, and rendered to a `String`. The tool templates in `tools/tool_lib/src/templates/` use this pattern with embedded template constants and custom formatters.

```rust,editable
{{#include ../../../crates/cats/template_engine/examples/tinytemplate/tinytemplate.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Template Engines][p~template-engine]
- [Tera][ex~template-engine~create-html-from-template] for file-based templates, filters, and HTML autoescaping.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
