# HTML Parsing

{{#include html.incl.md}}

## Parse HTML with `html5ever` {#html5ever}

[![html5ever][c-html5ever-badge]][c-html5ever] [![html5ever-crates.io][c-html5ever-crates.io-badge]][c-html5ever-crates.io] [![html5ever-github][c-html5ever-github-badge]][c-html5ever-github] [![html5ever-lib.rs][c-html5ever-lib.rs-badge]][c-html5ever-lib.rs]{{hi:html5ever}}{{hi:Parser}}{{hi:Parsing}}{{hi:Html}}{{hi:Html5}} [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]{{hi:Web programming}} [![cat-parser-implementations][cat-parser-implementations-badge]][cat-parser-implementations]{{hi:Parser implementations}}

`html5ever` is a high-performance browser-grade HTML5 parser.

{{#example html5ever}}

## Parse HTML with `tl` {#tl}

[![tl][c-tl-badge]][c-tl] [![tl-crates.io][c-tl-crates.io-badge]][c-tl-crates.io] [![tl-github][c-tl-github-badge]][c-tl-github] [![tl-lib.rs][c-tl-lib.rs-badge]][c-tl-lib.rs]{{hi:tl}}{{hi:Parser}}{{hi:Html}} [![cat-parsing][cat-parsing-badge]][cat-parsing]{{hi:Parsing tools}} [![cat-parser-implementations][cat-parser-implementations-badge]][cat-parser-implementations]{{hi:Parser implementations}}

`tl` is a fast HTML parser written in pure Rust.

{{#example tl}}

## Parse CSS with `css` {#css-rs}

[![css-website][c-css-website-badge]][c-css-website] [![css][c-css-badge]][c-css] [![css-crates.io][c-css-crates.io-badge]][c-css-crates.io] [![css-github][c-css-github-badge]][c-css-github] [![css-lib.rs][c-css-lib.rs-badge]][c-css-lib.rs]{{hi:css}}{{hi:css}}

`css` is a crate that uses the cssparser and selectors crates to provide a domain model for CSS Stylesheets. Intended to allow effective minification of CSS and CSS transformations such as autoprefixing and removal by other crates

{{#example css}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P2 write

[`scraper`][c-scraper]⮳{{hi:scraper}}, [`select`][c-select]⮳{{hi:select}}, [`kuchiki`][c-kuchiki]⮳{{hi:kuchiki}}

[`scraper`][c-scraper]⮳{{hi:scraper}} uses CSS selectors. [`select`][c-select]⮳{{hi:select}} is another option. [`kuchiki`][c-kuchiki]⮳{{hi:kuchiki}} is a fast and robust HTML parser.

</div>
