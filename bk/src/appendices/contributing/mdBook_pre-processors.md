<!-- TODO Reviewed ??-??-2026 -->

# mdBook pre-processors

{{#include mdBook_pre-processors.incl.md}}

## Preprocessors Used in the Book {#preprocessors}

This book is built from its markdown sources using [`mdbook`][c~mdbook~docs]↗{{hi:mdbook}}.

The [`bk/book.toml`][rust-howto~book.toml~repo]↗ config file configures several [mdbook preprocessors][book~mdbook~configuration-preprocessors]↗ and [backends][book~mdbook~configuration-renderers]↗.

- [`mdbook-private`][c~mdbook-private~repo]↗ hides private sections and chapters in the book.
- `mdbook-scrub` is a custom preprocessor written specifically for this book. See below.
- The custom `directives` stage of `mdbook-scrub` expands the book's directives before later preprocessors run.
- [`mdbook-indexing`][mdbook-indexing/tree/main~repo]↗ builds a word index via annotations in the book's Markdown.
- [`mdbook-linkcheck`][c~mdbook-linkcheck~repo]↗{{hi:mdbook-linkcheck}} is a [backend][book~mdbook~backends]↗ that checks links. Install the pinned version with `cargo install mdbook-linkcheck --version 0.7.7 --locked`. The active `[output.linkcheck]` configuration in [`book.toml`][c~mdbook~book.toml]↗{{hi:book.toml}} invokes this repository's `mdbook-linkcheck-compat` shim, which adapts the tool's command-line contract for the configured mdBook version.
- The default [`html`][book~mdbook~renderers]↗{{hi:html}} backend is used to generate the HTML for deployment on GitHub Pages.

The [[mdbook | `mdbook`]] chapter describes these preprocessors and backends in more detail.

## Use `mdbook-scrub`, a Custom Preprocessor for `mdbook` {#mdbook-scrub}

[`mdbook-scrub`][rust-howto~mdbook-scrub~repo]↗ is a [custom preprocessor][book~mdbook~preprocessors]↗ written specifically for this book. It is a simple tool that

- removes the hidden sections between <div class="hidden"> and </div> from the markdown before rendering. By default, [`mdbook`][book~mdbook]↗{{hi:mdbook}} hides <div class="hidden"></div> sections, but they remain searchable.
- removes the legacy `{{#includes }}` directive for hidden files (files that start with `_`), if any. See [mdBook's include documentation][book~mdbook~including-files]↗ for the related syntax. This feature fixes a gap of `mdbook-private`.
- expands the book-specific directives stage used by `bk/book.toml` before the other preprocessors.

Consult the project's [README][mdbook-scrub-readme~repo]↗ for more details.

## Related Topics {#related-topics .skip}

- [[book_editing_and_example_code_development | Book Editing and Example Code Development]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">

- [[mdbook | Mdbook]].

accuracy review: contributing guide

</div>
