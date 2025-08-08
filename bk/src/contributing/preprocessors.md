# 'mdBook' pre-processors

{{#include preprocessors.incl.md}}

## Preprocessors Used in the Book {#preprocessors}

This book is built from its markdown sources using [`mdbook`][c~mdbook~docs]{{hi:mdbook}}↗.

The [`bk/book.toml`](https://github.com/john-cd/rust_howto/blob/main/bk/book.toml)↗ config file configures several [mdbook preprocessors](https://rust-lang.github.io/mdBook/format/configuration/preprocessors.html)↗ and [backends](https://rust-lang.github.io/mdBook/format/configuration/renderers.html)↗.

- [`mdbook-private`][c~mdbook-private~github]↗ hides private sections and chapters in the book.
- `mdbook-scrub` is a custom preprocessor written specifically for this book. See below.
- [`mdbook-indexing`](https://github.com/daviddrysdale/mdbook-indexing/tree/main)↗ builds a word index via annotations in the book's Markdown.
- [`mdbook-linkcheck`][c~mdbook-linkcheck~github]{{hi:mdbook-linkcheck}}↗ is a [backend](https://rust-lang.github.io/mdBook/for_developers/backends.html)↗ that checks (internal) links. Install with `cargo install mdbook-linkcheck`. Un-comment the related section in [`book.toml`][c~mdbook~book.toml]↗{{hi:book.toml}}.
- The default `html` backend is used to generate the HTML for deployment on GitHub Pages.

The [[mdbook | `mdbook`]] chapter describes these preprocessors and backends in more detail.

## `mdbook-scrub` {#mdbook-scrub}

[`mdbook-scrub`](https://github.com/john-cd/rust_howto/blob/main/mdbook-scrub)↗ is a [custom preprocessor](https://rust-lang.github.io/mdBook/for_developers/preprocessors.html)↗ written specifically for this book. It is a simple tool that

- removes the hidden sections between <div class="hidden"> and </div> from the markdown before rendering. By default, [`mdbook`](https://rust-lang.github.io/mdBook)↗{{hi:mdbook}} hides <div class="hidden"></div> sections, but they remain searchable.
- removes `{{#includes }}` that points to hidden files (files that start with `_`), if any. This feature fixes a gap of `mdbook-private`.

Consult the project's [README](https://github.com/john-cd/rust_howto/blob/main/mdbook-scrub/README.md)↗ for more details.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
