# HTML Parsing

{{#include html.incl.md}}

## Parse HTML with `html5ever` {#html5ever}

[![html5ever][c~html5ever~docs~badge]][c~html5ever~docs] [![html5ever~crates.io][c~html5ever~crates.io~badge]][c~html5ever~crates.io] [![html5ever~repo][c~html5ever~repo~badge]][c~html5ever~repo] [![html5ever~lib.rs][c~html5ever~lib.rs~badge]][c~html5ever~lib.rs]{{hi:html5ever}}{{hi:Parser}}{{hi:Parsing}}{{hi:Html}}{{hi:Html5}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}} [![cat~parser-implementations][cat~parser-implementations~badge]][cat~parser-implementations]{{hi:Parser implementations}}

[`html5ever`][c~html5ever~docs]↗{{hi:html5ever}} is a high-performance browser-grade HTML5 parser.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/html/html5ever.rs:example}}
```

## Parse HTML with `tl` {#tl}

[![tl][c~tl~docs~badge]][c~tl~docs] [![tl~crates.io][c~tl~crates.io~badge]][c~tl~crates.io] [![tl~repo][c~tl~repo~badge]][c~tl~repo] [![tl~lib.rs][c~tl~lib.rs~badge]][c~tl~lib.rs]{{hi:tl}}{{hi:Parser}}{{hi:Html}} [![cat~parsing][cat~parsing~badge]][cat~parsing]{{hi:Parsing tools}} [![cat~parser-implementations][cat~parser-implementations~badge]][cat~parser-implementations]{{hi:Parser implementations}}

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

[![cssparser][c~cssparser~docs~badge]][c~cssparser~docs] [![cssparser~crates.io][c~cssparser~crates.io~badge]][c~cssparser~crates.io] [![cssparser~repo][c~cssparser~repo~badge]][c~cssparser~repo] [![cssparser~lib.rs][c~cssparser~lib.rs~badge]][c~cssparser~lib.rs]{{hi:cssparser}}{{hi:Css}}{{hi:Syntax}}{{hi:Parser}}

[`cssparser`][c~cssparser~docs]↗{{hi:cssparser}} is a Rust implementation of CSS Syntax Level 3.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/html/cssparser.rs:example}}
```

## `scraper` {#scraper}

[![scraper][c~scraper~docs~badge]][c~scraper~docs] [![scraper~crates.io][c~scraper~crates.io~badge]][c~scraper~crates.io] [![scraper~repo][c~scraper~repo~badge]][c~scraper~repo] [![scraper~lib.rs][c~scraper~lib.rs~badge]][c~scraper~lib.rs]{{hi:scraper}}{{hi:Css}}{{hi:Html}}{{hi:Selector}}{{hi:Scraping}}

HTML parsing and querying with CSS selectors. [`scraper`][c~scraper~docs]↗{{hi:scraper}} uses CSS selectors.

## Other Options {#other-options .skip}

[`select`][c~select~docs]↗{{hi:select}} is another option. [`kuchiki`][c~kuchiki~docs]↗{{hi:kuchiki}} is a fast and robust HTML parser.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1187)
</div>
