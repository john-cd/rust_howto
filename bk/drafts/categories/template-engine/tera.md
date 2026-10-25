<!-- TODO Reviewed ??-??-2026 -->

# Tera

{{#include tera.incl.md}}

## Create HTML Files from a Template {#create-html-from-template}

[![tera~website][c~tera~website~badge]][c~tera~website] [![tera][c~tera~docs~badge]][c~tera~docs] [![tera~crates.io][c~tera~crates.io~badge]][c~tera~crates.io] [![tera~repo][c~tera~repo~badge]][c~tera~repo] [![tera~lib.rs][c~tera~lib.rs~badge]][c~tera~lib.rs]{{hi:tera}}{{hi:Django}}{{hi:Markup}}{{hi:Html}}{{hi:Jinja2}}{{hi:Template}} [![cat~template-engine][cat~template-engine~badge]][cat~template-engine]{{hi:Template engine}}

[`tera`][c~tera~docs]↗{{hi:tera}} is a [template engine][p~template-engine] based on Jinja2/Django templates. It allows developers to embed dynamic content within text-based templates, supporting features like control flow (if/else, for loops), expressions, filters, and custom functions.  Tera excels at generating various text-based outputs, from HTML for web applications to configuration files.

```rust,editable
{{#include ../../../crates/cats/template_engine/examples/tera/tera.rs:example}}
```

## Sanitize or Escape: What Protects a Template {#sanitize-vs-escape}

[![tera][c~tera~docs~badge]][c~tera~docs]{{hi:tera}}{{hi:Escaping}}{{hi:Sanitizing}} [![cat~template-engine][cat~template-engine~badge]][cat~template-engine]{{hi:Template engine}}

Escaping and sanitizing solve different problems. *Escaping* turns text into harmless data for one output context: `<` becomes `&lt;`, so user input is displayed instead of interpreted. [`tera`][c~tera~docs]↗{{hi:tera}} escapes automatically for templates whose name ends in `.html`, `.htm` or `.xml` (`Tera::autoescape_on`); the `safe` filter turns it off for one value and should be reserved for HTML you produced yourself. Escaping does not make a URL safe (a `javascript:` link stays a `javascript:` link), and it applies to the context you are in: an HTML-escaped value is not safe inside a `<script>` block or a CSS rule.

*Sanitizing* keeps some of the markup and removes the dangerous part, for the case where users are allowed to submit HTML (a rich-text comment) or when Markdown is rendered. Run user-supplied HTML through [`ammonia`][c~ammonia~docs]↗{{hi:ammonia}} before it reaches a template, and only then mark it `safe`. See [[html_sanitization | Sanitize HTML]]. As a rule: escape everything by default, sanitize only the values that must contain markup, and never use `safe` on unsanitized input.

## Related Topics {#related-topics .skip}

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[tera: write; add sample from tools code](https://github.com/john-cd/rust_howto/issues/483)

- [Tera Built-in Filters][doc~tera-built-in-filters]↗.

</div>
