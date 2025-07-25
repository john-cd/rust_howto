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

```txt
[![rand~website][c~rand~website~badge]][c~rand~website] [![rand][c~rand~docs~badge]][c~rand~docs] [![rand~crates.io][c~rand~crates.io~badge]][c~rand~crates.io] [![rand~github][c~rand~github~badge]][c~rand~github] [![rand~lib.rs][c~rand~lib.rs~badge]][c~rand~lib.rs]{{hi:rand}}{{hi:Random}}{{hi:Rng}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

Random number generators and other randomness functionality.

[c~rand~docs~badge]: https://img.shields.io/crates/v/rand?label=rand
[c~rand~crates.io~badge]: https://img.shields.io/badge/crates.io-rand-crimson
[c~rand~crates.io]: https://crates.io/crates/rand
[c~rand~github~badge]: https://img.shields.io/badge/rand-steelblue?logo=github
[c~rand~github]: https://github.com/rust-random/rand
[c~rand~lib.rs~badge]: https://img.shields.io/badge/lib.rs-rand-yellow
[c~rand~lib.rs]: https://lib.rs/crates/rand
[c~rand~docs]: https://docs.rs/rand
[c~rand~website~badge]: https://img.shields.io/badge/rand-coral
[c~rand~website]: https://rust-random.github.io/book
```

`rbe` returns a badge that points to the `Rust by example` book, given a concept / chapter name:

```markdown
[![Rust by example - attributes][book~rust-by-example~attributes~badge]][book~rust-by-example~attributes]

[book~rust-by-example~attribute~badge]: https://img.shields.io/badge/Rust_By_Example-attribute-violet?logo=mdbook
[book~rust-by-example~attribute]: https://doc.rust-lang.org/rust-by-example/attribute.html
```

`info` returns a JSON object from the `crates.io` API:

- categories
- crate_data
- keywords
- versions
