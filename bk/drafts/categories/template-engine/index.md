<!-- TODO Reviewed ??-??-2026 -->
# Template Engine

[![cat~template-engine][cat~template-engine~badge]][cat~template-engine]{{hi:Template engines}}

Template engines render text by combining a template with data. Choose by where you want to write templates, when they are parsed, and whether you need HTML or another text format. Each engine has its own syntax and escaping rules.

## Choose a Template Workflow

| Workflow | Crates | How they fit |
| --- | --- | --- |
| Compile templates with Rust | [`askama`][c~askama~docs]↗{{hi:askama}} | Jinja-like template files are bound to a Rust `struct` or `enum` with a derive macro and checked during compilation. |
| Write HTML in Rust | [`maud`][c~maud~docs]↗{{hi:maud}}, [`markup`][c~markup~docs]↗{{hi:markup}} | HTML or markup is written with Rust macro syntax and rendered as a Rust value. |
| Load Jinja-style templates at runtime | [`minijinja`][c~minijinja~docs]↗{{hi:minijinja}}, [`tera`][c~tera~docs]↗{{hi:tera}} | Both use Jinja-inspired syntax. MiniJinja emphasizes a small dependency footprint; Tera includes template inheritance, filters, and custom functions. |
| Use Handlebars templates | [`handlebars`][c~handlebars~docs]↗{{hi:handlebars}} | Register named templates and helpers in a runtime registry; useful when you already have Handlebars templates. |
| Keep text templating minimal | [`tinytemplate`][c~tinytemplate~docs]↗{{hi:tinytemplate}} | A small runtime engine for interpolation and simple control flow. |

For HTML output, check the selected engine's auto-escaping behavior and only bypass escaping for trusted content.

## Included Examples

### [Create HTML Files with Tera](tera.md)

{{#include tera.incl.md}}

### [Create Markdown Fragments with TinyTemplate](tinytemplate.md)

{{#include tinytemplate.incl.md}}

## Related Tools

Web UI frameworks and site generators produce HTML as part of larger application workflows. [`yew`][c~yew~docs]↗{{hi:yew}} provides a component framework with an HTML macro; [`perseus`][c~perseus~docs]↗{{hi:perseus}} supports static generation and server-side rendering.

See [Web Programming](../web-programming/index.md) and [Static Website Generators](../web-programming_http-server/static_website_generators.md) for those topics.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
