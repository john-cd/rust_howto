<!-- TODO Reviewed ??-??-2026 -->

# Tera

{{#include tera.incl.md}}

## Create HTML Files from a Template {#create-html-from-template}

[![tera~website][c~tera~website~badge]][c~tera~website] {{#badges tera}} {{hi:Django}} {{hi:Markup}} {{hi:Html}} {{hi:Jinja2}} {{hi:Template}} {{#categories template-engine="Template engine"}}

[`tera`][c~tera~docs]↗{{hi:tera}} is a [template engine][p~template-engine]
based on Jinja2 and Django templates. It renders text from a serializable
context, with control flow, filters, and functions. Tera is useful for HTML,
configuration files, and other text output. HTML templates are autoescaped by
default; preserve that behavior for untrusted values unless the output is
deliberately safe markup.

The example loads its template files relative to the crate manifest, serializes
a page into a `Context`, and registers a custom filter. This is a common shape
for a documentation or site-generation tool: Rust supplies the data and the
template controls its presentation. The second included template adapts the
crate-badge generation pattern used by this repository's `tools/templ`; that
tool currently uses TinyTemplate internally, while this sample renders the
same kind of Markdown with Tera and fixed local data.

```rust,editable
{{#include ../../../crates/cats/template_engine/examples/tera/tera.rs:example}}
```

## Sanitize or Escape: What Protects a Template {#sanitize-vs-escape}

[![tera][c~tera~docs~badge]][c~tera~docs]{{hi:tera}}{{hi:Escaping}}{{hi:Sanitizing}} [![cat~template-engine][cat~template-engine~badge]][cat~template-engine]{{hi:Template engine}}

Escaping and sanitizing solve different problems. _Escaping_ turns text into harmless data for one output context: `<` becomes `&lt;`, so user input is displayed instead of interpreted. [`tera`][c~tera~docs]↗{{hi:tera}} escapes automatically for templates whose name ends in `.html`, `.htm` or `.xml` (`Tera::autoescape_on`); the `safe` filter turns it off for one value and should be reserved for HTML you produced yourself. Escaping does not make a URL safe (a `javascript:` link stays a `javascript:` link), and it applies to the context you are in: an HTML-escaped value is not safe inside a `<script>` block or a CSS rule.

_Sanitizing_ keeps some of the markup and removes the dangerous part, for the case where users are allowed to submit HTML (a rich-text comment) or when Markdown is rendered. Run user-supplied HTML through [`ammonia`][c~ammonia~docs]↗{{hi:ammonia}} before it reaches a template, and only then mark it `safe`. See [[html_sanitization | Sanitize HTML]]. As a rule: escape everything by default, sanitize only the values that must contain markup, and never use `safe` on unsanitized input.

## Related Topics {#related-topics .skip}

- [Template Engines][p~template-engine]
- [TinyTemplate][ex~template-engine~tinytemplate]
- [Tera built-in filters][doc~tera-built-in-filters]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
