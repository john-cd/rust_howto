# `templ` tool

`templ` is a simple CLI tool to generate markdown fragments, e.g. markdown links and reference definitions.
The outputs can be copy-pasted into the book's markdown where needed.

## Commands

Run the tool via `just templ`.

```txt
A simple CLI tool to generate markdown fragments, e.g. markdown links and reference definitions. The outputs can be copy-pasted into the book's markdown where needed.

Usage: templ [OPTIONS]
       templ badge [OPTIONS] <CRATE_NAME>...
       templ category_badge [OPTIONS] <CRATE_NAME>...
       templ rbe [OPTIONS] <CONCEPT>...
       templ info [OPTIONS] <CRATE_NAME>...
       templ help [COMMAND]...

Options:
  -v, --verbose  Use verbose output
  -h, --help     Print help
  -V, --version  Print version

templ badge:
Create the markdown for a crate badge
  -h, --help           Print help
  <CRATE_NAME>...  Enter the crate name(s)

templ category_badge:
Create the markdown for all category badges for (a) given crate name(s)
  -h, --help           Print help
  <CRATE_NAME>...  Enter the crate name(s)

templ rbe:
Create the markdown for a Rust By Example book badge
  -h, --help        Print help
  <CONCEPT>...  Enter the RBE book chapter name e.g. "attributes"

templ info:
Get crate information, including categories
  -h, --help           Print help
  <CRATE_NAME>...  Enter the crate name(s)

templ help:
Print this message or the help of the given subcommand(s)
  [COMMAND]...  Print help for the subcommand(s)
```

`badge` returns several badges that point to docs.rs, lib.rs, crates.io, etc..., given one or more crate names.

```md
[![rand][c-rand-badge]][c-rand]{{hi:rand}}
[![rand-crates.io][c-rand-crates.io-badge]][c-rand-crates.io]
[![rand-github][c-rand-github-badge]][c-rand-github]
[![rand-lib.rs][c-rand-lib.rs-badge]][c-rand-lib.rs]

[c-rand-badge]: https://img.shields.io/crates/v/rand?label=rand
[c-rand-crates.io-badge]: https://img.shields.io/badge/crates.io-rand-crimson
[c-rand-crates.io]: https://crates.io/crates/rand
[c-rand-github-badge]: https://img.shields.io/badge/rand-steelblue?logo=github
[c-rand-github]: https://github.com/_TODO P1
[c-rand-lib.rs-badge]: https://img.shields.io/badge/lib.rs-rand-yellow
[c-rand-lib.rs]: https://lib.rs/crates/rand
[c-rand]: https://docs.rs/rand
```

`rbe` returns a badge that points to the `Rust by example` book, given a concept / chapter name:

[![Rust by example - attributes][book-rust-by-example-attributes-badge]][book-rust-by-example-attributes]

[book-rust-by-example-attribute-badge]: https://img.shields.io/badge/Rust_By_Example-attribute-violet?logo=mdbook
[book-rust-by-example-attribute]: https://doc.rust-lang.org/rust-by-example/attribute.html

`info` returns a JSON object from the crates.io API:

- categories
- crate_data
- keywords
- versions
