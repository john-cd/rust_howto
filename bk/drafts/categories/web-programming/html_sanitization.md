<!-- TODO Reviewed ??-??-2026 -->

# Sanitize HTML

{{#include html_sanitization.incl.md}}

Strip dangerous markup from user-supplied or scraped HTML before showing or storing it, and produce a plain-text version of HTML.

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

## Convert HTML to Plain Text (`html2text`) {#html2text}

[![html2text][c~html2text~docs~badge]][c~html2text~docs] [![html2text~crates.io][c~html2text~crates.io~badge]][c~html2text~crates.io]{{hi:html2text}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Email}}

[`html2text`][c~html2text~docs]↗{{hi:html2text}} renders HTML as wrapped plain text, keeping headings, list items and link targets. It is handy for the `text/plain` alternative of an HTML email or for a terminal preview.

```rust,editable
{{#include ../../../crates/cats/web_programming/examples/html_text/html2text.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[scraping | Web Scraping]].
- [[template-engine | Template Engine]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
