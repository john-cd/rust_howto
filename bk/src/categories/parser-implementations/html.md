# HTML Parsing

{{#include html.incl.md}}

## Parse HTML with `html5ever` {#html5ever}

[![html5ever][c-html5ever-badge]][c-html5ever] [![html5ever-crates.io][c-html5ever-crates.io-badge]][c-html5ever-crates.io] [![html5ever-github][c-html5ever-github-badge]][c-html5ever-github] [![html5ever-lib.rs][c-html5ever-lib.rs-badge]][c-html5ever-lib.rs]{{hi:html5ever}}{{hi:Parser}}{{hi:Parsing}}{{hi:Html}}{{hi:Html5}} [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]{{hi:Web programming}} [![cat-parser-implementations][cat-parser-implementations-badge]][cat-parser-implementations]{{hi:Parser implementations}}

`html5ever` is a high-performance browser-grade HTML5 parser.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/tests/html/html5ever.rs:example}}
```

## Parse HTML with `tl` {#tl}

[![tl][c-tl-badge]][c-tl] [![tl-crates.io][c-tl-crates.io-badge]][c-tl-crates.io] [![tl-github][c-tl-github-badge]][c-tl-github] [![tl-lib.rs][c-tl-lib.rs-badge]][c-tl-lib.rs]{{hi:tl}}{{hi:Parser}}{{hi:Html}} [![cat-parsing][cat-parsing-badge]][cat-parsing]{{hi:Parsing tools}} [![cat-parser-implementations][cat-parser-implementations-badge]][cat-parser-implementations]{{hi:Parser implementations}}

`tl` is a fast HTML parser written in pure Rust.

Add the following to your `Cargo.toml`:

```toml
[dependencies]
tl = "0.7.8"
# or, with explicit SIMD support
# (requires a nightly compiler!)
tl = { version = "0.7.8", features = ["simd"] }
```

```rust,editable
{{#include ../../../crates/cats/parser_implementations/tests/html/tl.rs:example}}
```

## Parse CSS with `cssparser` {#css-rs}

[![cssparser][c-cssparser-badge]][c-cssparser] [![cssparser-crates.io][c-cssparser-crates.io-badge]][c-cssparser-crates.io] [![cssparser-github][c-cssparser-github-badge]][c-cssparser-github] [![cssparser-lib.rs][c-cssparser-lib.rs-badge]][c-cssparser-lib.rs]{{hi:cssparser}}{{hi:Css}}{{hi:Syntax}}{{hi:Parser}}

`cssparser` is a Rust implementation of CSS Syntax Level 3.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/tests/html/cssparser.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P2 write

[`scraper`][c-scraper]⮳{{hi:scraper}}, [`select`][c-select]⮳{{hi:select}}, [`kuchiki`][c-kuchiki]⮳{{hi:kuchiki}}

[`scraper`][c-scraper]⮳{{hi:scraper}} uses CSS selectors. [`select`][c-select]⮳{{hi:select}} is another option. [`kuchiki`][c-kuchiki]⮳{{hi:kuchiki}} is a fast and robust HTML parser.

</div>
