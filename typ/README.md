# README

TODO

- Need to convert book to typst?? or keep as markdown?
- Select the proper mdbook backend to convert Markdown to Typst, then PDF.
- Typst template for printed book.

## Typst

### Install `typst`

```sh
# If snap is not installed:
sudo apt update
sudo apt install snapd

sudo snap install typst
```

### Use `typst`

```sh
# Get help:
typst help
# Prints detailed usage of a subcommand:
typst help watch
```

```sh
# Create `book.pdf` in the working directory:
typst compile bk/book/typst/book.typst

# Create a PDF file at the desired path:
typst compile path/to/source.typ path/to/output.pdf
```

#### Watches Source Files and Recompiles on Changes

```sh
typst watch main.typ
```

## Useful Typst Packages

- [typst~codly][typst~codly].

## Useful Typst Templates

- [typst~ilm][typst~ilm].
- [typst~wonderous-book][typst~wonderous-book].

Initialize with e.g.:

```sh
typst init @preview/ilm
```

## Typst Bibliography

<https://github.com/typst/hayagriva>

## References

- [Exploring Typst, a new typesetting system similar to LaTeX][blog~typst].
- [Typst Examples Book][typstexamplesbook].

---

## `mdBook` Backends that Convert Markdown to Typst

- [mdbook-pandoc][mdbook-pandoc~crates.io].
- [mdbook-typst][mdbook-typst~github].
- [mdbook-typstpdf][mdbook-typstpdf~lib.rs].
- [mdbook-typst-pdf][mdbook-typst-pdf~lib.rs].

### `mdbook-typst`

Install with:

```sh
cargo install mdbook-typst
```

Add an entry to `book.toml` to enable the backend:

```toml
[output.typst]
```

See [configuration options][mdbook-typst~config~docs].

```toml
[output.typst.output]
format = "pdf" # pdf, svg, png, #[default] typst
name = "name"

[output.typst.style]
paper = "us-letter"
text_size = "11pt"
text_font = "Helvetica" # text_font = "libertinus serif"
paragraph_spacing  = "2em"
paragraph_leading = ".8em"
heading_numbering = ""
heading_below = "2em"
heading_above = "2em"
link_underline = true
link_color = "blue"

[output.typst.markup]
horizontal_rule = "#v(1em)\n#line(length: 100%)\n#v(1em)"

[output.typst.toc]
enable = true
depth = 2
indent = "2em"
entry_show_rules = [
{ level = 1, text_size = "11pt", strong = true }
]

[output.typst.advanced]
#typst_markup_header = ""
#typst_markup_footer = ""
```

---

## See Also

- [asciidoc.org][asciidoc~website].
- [Environment Variables - mdBook Documentation][book~mdbook~environment-variables].
- [`mdbook-pdf`][c~mdbook-pdf~crates.io].
- [`mdbook-tools`][c~mdbook-tools~lib.rs].
- [pullup][pullup~lib.rs].
- [quarkdown~github][quarkdown~github]

[asciidoc~website]: https://asciidoc.org
[blog~typst]: https://blog.jreyesr.com/posts/typst
[book~mdbook~environment-variables]: https://rust-lang.github.io/mdBook/format/configuration/environment-variables.html
[mdbook-pandoc~crates.io]: https://crates.io/crates/mdbook-pandoc
[mdbook-typst-pdf~lib.rs]: https://lib.rs/crates/mdbook-typst-pdf
[mdbook-typst~config~docs]: https://docs.rs/crate/mdbook-typst/0.1.7/source/src/config.rs
[mdbook-typst~github]: https://github.com/LegNeato/mdbook-typst
[mdbook-typstpdf~lib.rs]: https://lib.rs/crates/mdbook-typstpdf
[pullup~lib.rs]: https://lib.rs/crates/pullup
[quarkdown~github]: https://github.com/iamgio/quarkdown
[typst~codly]: https://typst.app/universe/package/codly
[typst~ilm]: https://typst.app/universe/package/ilm
[typst~wonderous-book]: https://github.com/typst/templates/tree/main/wonderous-book
[typstexamplesbook]: https://sitandr.github.io/typst-examples-book/book/basics/tutorial/functions.html
