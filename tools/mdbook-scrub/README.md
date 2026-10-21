# `mdbook-scrub`

This is a **Work In Progress**. The API are subject to change.

`mdbook-scrub` provides two mdBook preprocessing stages. The first removes hidden
sections and hidden file helpers before mdBook expands native includes. The
second expands the book's custom directives after those includes, including
directives in included Markdown. Both stages work on the in-memory book; they do
not rewrite source files.

Custom directives in code spans, fenced code blocks and HTML comments remain
literal. An unresolved custom directive or example path fails the build with a
chapter-specific error.

## Installation

The helper workspace also builds `mdbook-linkcheck-compat`, selected by the
book's linkcheck output command. It adapts the mdBook 0.5 book container and
unset config defaults for mdbook-linkcheck 0.7.7's older protocol (advertised
as 0.4.43 to its version gate), and forwards
the checker's exit status. Only the checker's copy of Rust edition metadata
is mapped from 2024 to 2021; rendered examples keep edition 2024.
Build the local helpers with `just release` from `tools/`.

Make sure that:

- [`Rust`][rust-lang]
- [`mdbook`][mdbook]

are set up correctly, then install the pre-processor:

```sh
cargo install mdbook-scrub
```

Create a book using `mdbook init`, if not done already.

Configure both stages in `book.toml`:

```toml
[preprocessor.scrub]
command = "../bin/mdbook-scrub"
# Remove all markdown within HTML tags with class `hidden` from the book
# (default = true)
remove-hidden-sections = true
# Do not include hidden files i.e. files that start with `chapter-prefix` below,
# typically _ (default = true)
do-not-include-hidden-chapters = true
# Define the prefix for hidden chapters (default = '_')
hidden-chapter-prefix = "_"
# Remove hidden includes before mdBook's built-in links preprocessor expands them.
# <https://rust-lang.github.io/mdBook/format/configuration/preprocessors.html>
after = [ "private" ]
before = [ "links" ]

[preprocessor.directives]
command = "../bin/mdbook-scrub --stage directives"
after = [ "links" ]
# Directive families can be disabled independently (all default to true).
process-crate-block-directives = true
process-example-directives = true
process-link-and-badge-directives = true
process-wikilinks = true
```

Please review the `test_book\book.toml` for a complete example.

## Expand Custom Directives

The directive stage supports crate and category blocks, crate and category
links or badges, examples, and wikilinks. For example:

```md
{{#crate serde filesystem}}
{{#categories filesystem=Filesystem,testing="Testing"}}
{{#example ../../crates/language/examples/constants_and_statics/statics.rs:example}}
```

An example path is relative to its chapter's source file and must resolve to a
Rust file inside the book root. The optional anchor defaults to `example`.
Anchored snippets use `// ANCHOR: name` and `// ANCHOR_END: name` comments;
only the lines between those markers are emitted in a Rust code fence. Missing
files or anchors stop the build instead of silently dropping the example.

The order is `private` → `scrub` → mdBook's `links` → `directives`. This lets
mdBook expand its own include, rustdoc-include, and playground helpers first,
then applies custom directives to the included content while preserving code.

## Remove Hidden Sections

```html
<div class="hidden">This is not be seen.</div>
```

`mdBook` hides the Markdown content within HTML tags with class `hidden`,
but do not remove them from the output HTML.
The hidden content remains searchable and can be read in the underlying HTML.
If `remove-hidden-sections` is set to `true`, this preprocessor strips these sections
from the Markdown before it is passed to `mdbook` renderers.

## Do not include hidden chapters

The `links` mdBook preprocessor is built-in and included by default.
It expands the `{{ #playground }}`, `{{ #include }}`,
and `{{ #rustdoc_include }}` syntaxes:

```md
Include a file into your book:

{{#include file.rs}}

Partially include a file into your book:

Second line only: {{#include file.rs:2}}
Up to line 10: {{#include file.rs::10}}
From line 2: {{#include file.rs:2:}}
Between lines 2 and 10: {{#include file.rs:2:10}}

Partially include a file into your book using anchors:

{{#include file.rs:component}}

Including a file but initially hiding all except specified lines:

{{#rustdoc_include file.rs:2}}

Inserting runnable Rust files:

{{#playground file.rs}}
```

The path to the file is relative from the current source file.

[`mdbook-private`][c~mdbook-private~repo]↗ is a preprocessor
for defining and optionally removing private sections and chapters in a mdbook.
However, it does not remove included hidden files.

When your `book.toml` uses this preprocessor and `do-not-include-hidden-chapters`
is set to `true`, any `include`, `playground`, or `rustdoc_include` statements
that refer to a file starting with `hidden-chapter-prefix` (typically `_`)
are removed from the Markdown.

For example, the following would be removed:

```markdown
{{#include _hiddenfile.rs}}
```

[mdbook]: https://github.com/rust-lang/mdBook
[rust-lang]: https://rust-lang.org
[c~mdbook-private~repo]: https://github.com/RealAtix/mdbook-private
