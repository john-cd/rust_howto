# mdBook pre-processors

{{#include mdBook_pre-processors.incl.md}}

## Preprocessors Used in the Book {#preprocessors}

This book is built from its markdown sources using [`mdbook`][c~mdbook~docs]↗{{hi:mdbook}}.

The [`bk/book.toml`][rust-howto~book.toml~repo]↗ config file configures several [mdbook preprocessors][book~mdbook~configuration-preprocessors]↗ and [backends][book~mdbook~configuration-renderers]↗.

- [`mdbook-private`][c~mdbook-private~repo]↗ hides private sections and chapters in the book.
- `mdbook-scrub` is a custom preprocessor written specifically for this book. See below.
- [`mdbook-indexing`][mdbook-indexing/tree/main~repo]↗ builds a word index via annotations in the book's Markdown.
- [`mdbook-linkcheck`][c~mdbook-linkcheck~repo]↗{{hi:mdbook-linkcheck}} is a [backend][book~mdbook~backends]↗ that checks (internal) links. Install with `cargo install mdbook-linkcheck`. Un-comment the related section in [`book.toml`][c~mdbook~book.toml]↗{{hi:book.toml}}.
- The default [`html`][book~mdbook~renderers]↗{{hi:html}} backend is used to generate the HTML for deployment on GitHub Pages.

The [[mdbook | `mdbook`]] chapter describes these preprocessors and backends in more detail.

## Use `mdbook-scrub`, a Custom Preprocessor for `mdbook` {#mdbook-scrub}

[`mdbook-scrub`][rust-howto~mdbook-scrub~repo]↗ is a [custom preprocessor][book~mdbook~preprocessors]↗ written specifically for this book. It is a simple tool that

- removes the hidden sections between <div class="hidden"> and </div> from the markdown before rendering. By default, [`mdbook`][book~mdbook]↗{{hi:mdbook}} hides <div class="hidden"></div> sections, but they remain searchable.
- removes [`{{#includes }}`][book~mdbook~including-files]↗{{hi:{{#includes }}}} that points to hidden files (files that start with `_`), if any. This feature fixes a gap of `mdbook-private`.

Consult the project's [README][mdbook-scrub-readme~repo]↗ for more details.

## Related Topics {#related-topics .skip}

- [[book_editing_and_example_code_development | Book Editing and Example Code Development]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">

- [[mdbook | Mdbook]].

</div>
