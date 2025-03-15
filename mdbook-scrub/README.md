# `mdbook-scrub`

This is a **Work In Progress**. The API are subject to change.

`mdbook-scrub` is a simple `mdbook` preprocessor that

- removes the hidden sections between `<div class="hidden">` and `</div>`
- removes `{{#includes }}` that points to hidden files (files that start with `_`)

## Installation

Make sure that

- [`Rust`](https://www.rust-lang.org/)
- [`mdbook`](https://github.com/rust-lang/mdBook)

are set up correctly, then install the pre-processor:

```sh
cargo install mdbook-scrub
```

Create a book using `mdbook init`, if not done already.

Add a new table to `book.toml` and update the configuration as required:

```toml
[preprocessor.scrub]
# Remove all markdown within HTML tags with class `hidden` from the book (default = true)
remove-hidden-sections = true
# Do not include hidden files i.e. files that start with `chapter-prefix` below, typically _ (default = true)
do-not-include-hidden-chapters = true
# Define the prefix for hidden chapters (default = '_')
hidden-chapter-prefix = "_"
# The order in which preprocessors are run can be controlled with the `before` and `after` fields.
# https://rust-lang.github.io/mdBook/format/configuration/preprocessors.html
before = [ "links" ]
#after = [ "" ]
```

Please review the `test_book\book.toml` for a complete example.

## Remove Hidden Sections

```html
<div class="hidden">This is not be seen.</div>
```

`mdBook` hides the Markdown content within HTML tags with class `hidden`, but do not remove them from the output HTML.
The hidden content remains searchable and can be read in the underlying HTML.
If `remove-hidden-sections` is set to `true`, this preprocessor strips these sections from the Markdown before it is passed to `mdbook` renderers.

## Do not include hidden chapters

The `links` mdBook preprocessor is built-in and included by default.
It expands the `{{ #playground }}`, `{{ #include }}`, and `{{ #rustdoc_include }}` syntaxes:

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

[`mdbook-private`](https://github.com/RealAtix/mdbook-private) is a preprocessor for defining and optionally removing private sections and chapters in a mdbook. However, it does not remove included hidden files.

When your `book.toml` uses this preprocessor and `do-not-include-hidden-chapters` is set to `true`, any `include`, `playground`, or `rustdoc_include` statements that refer to a file starting with `hidden-chapter-prefix` (typically `_`) are removed from the Markdown.
For example, the following would be removed:

```markdown
{{#include _hiddenfile.rs}}
```
