<!-- TODO Reviewed ??-??-2026 -->

# Sanitize HTML

{{#include html_sanitization.incl.md}}

Strip dangerous markup from user-supplied or scraped HTML before showing or storing it. For an email's plain-text alternative, see [[send_emails | Send Emails]].

## Sanitize Untrusted HTML (`ammonia`) {#ammonia}

[![ammonia][c~ammonia~docs~badge]][c~ammonia~docs] [![ammonia~crates.io][c~ammonia~crates.io~badge]][c~ammonia~crates.io]{{hi:ammonia}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Sanitization}}

[`ammonia`][c~ammonia~docs]↗{{hi:ammonia}} cleans HTML with an allow list of tags, attributes and URL schemes, which removes `<script>` elements, event-handler attributes and `javascript:` links. `ammonia::clean` uses safe defaults; a `Builder` narrows or widens the policy. Sanitizing is for HTML you render; it does not replace escaping in a template engine.

```rust,editable
{{#include ../../../crates/cats/web_programming/examples/html_text/ammonia_clean.rs:example}}
```

## Render Markdown to Safe HTML {#markdown-to-safe-html}

[![pulldown-cmark][c~pulldown-cmark~docs~badge]][c~pulldown-cmark~docs] [![pulldown-cmark~crates.io][c~pulldown-cmark~crates.io~badge]][c~pulldown-cmark~crates.io] [![ammonia][c~ammonia~docs~badge]][c~ammonia~docs] [![ammonia~crates.io][c~ammonia~crates.io~badge]][c~ammonia~crates.io]{{hi:pulldown-cmark}}{{hi:ammonia}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Markdown}}

Markdown allows raw HTML, so cleaning the Markdown source is not enough. Render with [`pulldown-cmark`][c~pulldown-cmark~docs]↗{{hi:pulldown-cmark}} first, then pass the HTML output through [`ammonia`][c~ammonia~docs]↗{{hi:ammonia}}.

```rust,editable
{{#include ../../../crates/cats/web_programming/examples/html_text/markdown_to_safe_html.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Web Scraping](scraping.md).
- [Template Engine](../template-engine/index.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
