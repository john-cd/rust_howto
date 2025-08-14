# README

## Install `typst`

```sh
# If snap is not installed:
sudo apt update
sudo apt install snapd

sudo snap install typst
```

## Use `typst`

```sh
# Create `file.pdf` in working directory:
typst compile bk/book/typst/book.typst

# Create PDF file at the desired path:
typst compile path/to/source.typ path/to/output.pdf
```

### Watches source files and recompiles on changes

```sh
typst watch file.typ
```

```sh
typst help
# Prints detailed usage of a subcommand
typst help watch
```

## Useful Typst Packages

[typst~codly][typst~codly]

[typst~codly]: https://typst.app/universe/package/codly

## Useful Templates

[typst~ilm][typst~ilm]

[typst~ilm]: https://typst.app/universe/package/ilm

[typst~wonderous-book][typst~wonderous-book]

[typst~wonderous-book]: https://github.com/typst/templates/tree/main/wonderous-book

## mdBook Backends

```sh
cargo install mdbook-typst
```

Add an entry to `book.toml`:

```toml
[output.typst]
```

- Code: [mdbook-typst][mdbook-typst~github].

- Configuration options: see [mdbook-typst~config~docs][mdbook-typst~config~docs].

[mdbook-typst~config~docs]: <https://docs.rs/crate/mdbook-typst/0.1.7/source/src/config.rs>
[mdbook-typst~github]: <https://github.com/LegNeato/mdbook-typst>

```toml
[output.typst.output]
format = "pdf" # pdf, svg, png, #[default] typst
name = "name"

[output.typst.style]
paper = "us-letter"
text_size = "11pt"
text_font = "Helvetica"
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

Example:

```toml
[output.typst]

[output.typst.output]

# format = "pdf"

[output.typst.style]
# paper = "us-letter"
# text_size = "11pt"
text_font = "libertinus serif"
# paragraph_spacing  = "2em"
# paragraph_leading = ".8em"
# heading_numbering = ""
# heading_below = "2em"
# heading_above = "2em"
# link_underline = true
# link_color = "blue"
```

## Bibliography

<https://github.com/typst/hayagriva>

## References

[Exploring Typst, a new typesetting system similar to LaTeX][typst]

[typst]: https://blog.jreyesr.com/posts/typst

[typst examplesbook][typstexamplesbook]

[typstexamplesbook]: https://sitandr.github.io/typst-examples-book/book/basics/tutorial/functions.html

See also:

- [asciidoc.org][asciidoc~website].
- [mdbook-pandoc~crates.io][mdbook-pandoc~crates.io].
- [pullup~lib.rs][pullup~lib.rs].
- [mdbook-typstpdf~lib.rs][mdbook-typstpdf~lib.rs].
- [mdbook-typst-pdf~lib.rs][mdbook-typst-pdf~lib.rs].

[pullup~lib.rs]: <https://lib.rs/crates/pullup>
[asciidoc~website]: <https://asciidoc.org>
[mdbook-pandoc~crates.io]: <https://crates.io/crates/mdbook-pandoc>
[mdbook-typstpdf~lib.rs]: <https://lib.rs/crates/mdbook-typstpdf>
[mdbook-typst-pdf~lib.rs]: <https://lib.rs/crates/mdbook-typst-pdf>
