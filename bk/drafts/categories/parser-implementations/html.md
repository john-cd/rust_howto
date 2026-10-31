<!-- TODO Reviewed ??-??-2026 -->

# HTML Parsing

{{#include html.incl.md}}

## Parse HTML with `html5ever` {#html5ever}

{{#badges html5ever web-programming="Web programming" parser-implementations="Parser implementations"}}{{hi:Parser}}{{hi:Parsing}}{{hi:Html}}{{hi:Html5}}

[`html5ever`][c~html5ever~docs]↗{{hi:html5ever}} is a high-performance browser-grade HTML5 parser.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/html/html5ever.rs:example}}
```

## Parse HTML with `tl` {#tl}

{{#badges tl parsing="Parsing tools" parser-implementations="Parser implementations"}}{{hi:Parser}}{{hi:Html}}

[`tl`][c~tl~docs]↗{{hi:tl}} is a fast HTML parser written in pure Rust.

Add the following to your [`Cargo.toml`][book~cargo~cargo-toml]↗{{hi:Cargo.toml}}:

```toml
[dependencies]
tl = "0.7.8"
# or, with explicit SIMD support
# (requires a nightly compiler!)
tl = { version = "0.7.8", features = ["simd"] }
```

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/html/tl.rs:example}}
```

## Parse CSS with `cssparser` {#css-rs}

{{#badges cssparser}}{{hi:Css}}{{hi:Syntax}}{{hi:Parser}}

[`cssparser`][c~cssparser~docs]↗{{hi:cssparser}} is a Rust implementation of CSS Syntax Level 3.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/html/cssparser.rs:example}}
```

## `scraper` {#scraper}

{{#badges scraper}}{{hi:Css}}{{hi:Html}}{{hi:Selector}}{{hi:Scraping}}

[`scraper`][c~scraper~docs]↗{{hi:scraper}} provides a tree-backed interface for selecting elements with CSS selectors. Use it to extract text and attributes from the HTML you received; it does not execute JavaScript or fetch pages for you.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/html/scraper.rs:example}}
```

## Other Options {#other-options .skip}

[`select`][c~select~docs]↗{{hi:select}} is another option. [`kuchiki`][c~kuchiki~docs]↗{{hi:kuchiki}} is a fast and robust HTML parser.

## Related Topics {#related-topics .skip}

- [XML parsing][p~xml-parsing] for XML-specific parsers.
- [Web scraping][p~web-scraping] for extracting links and other page data.
- [Template engines][p~template-engine] for generating HTML from structured data.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  This chapter now distinguishes HTML5 parsing, DOM-style CSS selection, and CSS syntax parsing, adds a tested `scraper` recipe, and links related XML, scraping, and templating guidance.
</div>
