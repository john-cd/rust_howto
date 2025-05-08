# Tera

{{#include tera.incl.md}}

## Create HTML Files from a Template {#create-html-from-template}

[![tera-website][c-tera-website-badge]][c-tera-website] [![tera][c-tera-badge]][c-tera] [![tera-crates.io][c-tera-crates.io-badge]][c-tera-crates.io] [![tera-github][c-tera-github-badge]][c-tera-github] [![tera-lib.rs][c-tera-lib.rs-badge]][c-tera-lib.rs]{{hi:tera}}{{hi:Django}}{{hi:Markup}}{{hi:Html}}{{hi:Jinja2}}{{hi:Template}} [![cat-template-engine][cat-template-engine-badge]][cat-template-engine]{{hi:Template engine}}

[`tera`][c-tera]⮳{{hi:tera}} is a [template engine][p-template-engine] based on Jinja2/Django templates. It allows developers to embed dynamic content within text-based templates, supporting features like control flow (if/else, for loops), expressions, filters, and custom functions.  Tera excels at generating various text-based outputs, from HTML for web applications to configuration files.

```rust,editable
{{#include ../../../crates/cats/template_engine/tests/tera/tera.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[tera: write; add sample from tools code](https://github.com/john-cd/rust_howto/issues/483)
</div>
