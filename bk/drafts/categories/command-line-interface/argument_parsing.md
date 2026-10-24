<!-- TODO Reviewed ??-??-2026 -->

# Command-line Argument Parsing

{{#include argument_parsing.incl.md}}

## Plan a Command-line Interface {#plan-cli}

Treat command-line arguments as a user-facing interface. Give options clear names and help text, use positional arguments for the main input, and group related operations under subcommands. Keep standard output suitable for data and send diagnostics to standard error.

Use a parser when the program has options, help, validation, or subcommands. `std::env::args_os` preserves non-Unicode paths and values; `std::env::args` returns strings and can panic when an argument is not valid Unicode. The `--` separator conventionally ends option parsing, so later values can begin with a dash.

Keep parsing separate from application behavior. Convert parsed values into a typed configuration, then pass that configuration to the rest of the program. Define how command-line values interact with environment variables and config files, and document the precedence.

## Using `clap`'s Builder API {#clap-builder}

[![clap][c~clap~docs~badge]][c~clap~docs]{{hi:clap}} [![clap~examples][c~clap~examples~badge]][c~clap~examples] [![clap~repo][c~clap~repo~badge]][c~clap~repo] [![cat~command-line-interface][cat~command-line-interface~badge]][cat~command-line-interface]{{hi:Command-line interface}}

The builder API describes a command with a `Command` and one `Arg` for each option or positional value. Use it when you want to assemble a command dynamically or share definitions between commands.

`Arg::new` takes the internal argument ID. `short` and `long` set the spellings shown to the user; `value_parser` parses and validates typed values. Read parsed values with `get_one`, `get_many`, `get_flag`, or `get_count`, as appropriate. The example uses `get_matches_from` with fixed arguments so it can run deterministically under the book's example runner; a standalone program normally calls `get_matches`.

```rust,editable
{{#include ../../../crates/cats/command_line_interface/examples/arguments/clap_basic.rs:example}}
```

`clap` generates `--help` and `--version` output from the command definition. Run the built program with `--help` to inspect the exact usage for that version and set of arguments; avoid copying a help transcript from a different `clap` release.

## Advanced `clap` Builder Example with Subcommands {#clap-builder-advanced}

[![clap][c~clap~docs~badge]][c~clap~docs]{{hi:clap}} [![clap~examples][c~clap~examples~badge]][c~clap~examples] [![clap~repo][c~clap~repo~badge]][c~clap~repo] [![cat~command-line-interface][cat~command-line-interface~badge]][cat~command-line-interface]{{hi:Command-line interface}}

The following example, found in [`xmpl/clap_builder_xmpl`][rust-howto~clap_builder_xmpl~repo]↗, demonstrates how to build a full-featured CLI using [`clap`][c~clap~docs]↗{{hi:clap}}'s builder API. Because it spans multiple modules, it cannot be embedded directly into this page.

It showcases:

- **Subcommands**: `open` (one or more file paths), `query` (a sequence of query words), and `test` (a hidden command).
- **Global flags**: `--verbose` / `-v` (repeatable, for verbosity) and `--config` / `-c` (file path), both readable from environment variables (`TOOL_VERBOSE`, `TOOL_CONFIG_FILE`).
- **Custom styling** for the help output, using [`clap::builder::styling`][c~clap~docs]↗.
- **Inferred subcommands**: partial name matches (e.g., `te` matches `test`).
- **Builder pattern**: a `Config` struct with a dedicated `ConfigBuilder` separating CLI parsing from app configuration.

Browse the full source at [`xmpl/clap_builder_xmpl`][rust-howto~clap_builder_xmpl~repo]↗ or clone the repository and run:

```sh
cd xmpl/clap_builder_xmpl
cargo run -- --help
cargo run -- -vv open file1.csv file2.csv
cargo run -- --config config.toml query SELECT col FROM tbl
```

## Using `clap`'s Derive API {#clap-derive}

[![clap][c~clap~docs~badge]][c~clap~docs]{{hi:clap}} [(tutorial)][c~clap~tutorial]↗ [(cookbook)][c~clap~cookbook]↗ [![clap examples][c~clap~examples~badge]][c~clap~examples] [![cat~command-line-interface][cat~command-line-interface~badge]][cat~command-line-interface]{{hi:Command-line interface}}

[`clap_derive`][c~clap_derive~docs]↗{{hi:clap_derive}} simplifies CLI creation in Rust via a [`derive`][c~derive~docs]↗{{hi:derive}} macro, automatically generating command-line argument parsing logic from [`struct`][keyword~struct]↗{{hi:struct}} definitions.

Use field types to express the interface: `Option<T>` makes an argument optional, `Vec<T>` accepts repeated values, and a field of an enum that derives `ValueEnum` restricts input to named choices. Doc comments become help text. Use a `value_parser` for additional validation, and return errors from `try_parse` when the caller needs to handle parse failures instead of exiting immediately.

```rust,editable
{{#include ../../../crates/cats/command_line_interface/examples/arguments/clap.rs:example}}
```

## Generate Shell Completion Scripts with `clap_complete` {#clap_complete}

[![clap_complete][c~clap_complete~docs~badge]][c~clap_complete~docs] [![clap_complete~crates.io][c~clap_complete~crates.io~badge]][c~clap_complete~crates.io] [![clap_complete~repo][c~clap_complete~repo~badge]][c~clap_complete~repo] [![clap_complete~lib.rs][c~clap_complete~lib.rs~badge]][c~clap_complete~lib.rs]{{hi:clap_complete}}{{hi:Bash}}{{hi:Clap}}{{hi:Cli}}{{hi:Completion}} [![cat~command-line-interface][cat~command-line-interface~badge]][cat~command-line-interface]{{hi:Command-line interface}}

`clap_complete` generates shell completion scripts for your `clap::Command`.

This example defines a `clap::Command`, writes a Bash completion script to a temporary directory, and prints the generated file path. Install the generated completion using the target shell's conventions, and regenerate it when the CLI changes.

```rust,editable
{{#include ../../../crates/cats/command_line_interface/examples/arguments/clap_complete.rs:example}}
```

## `clap` Alternatives {#clap-alternatives .skip}

### Parse Command-line Arguments with `lexopt` {#lexopt}

[![lexopt][c~lexopt~docs~badge]][c~lexopt~docs]{{hi:lexopt}}
[![lexopt~crates.io][c~lexopt~crates.io~badge]][c~lexopt~crates.io]
[![lexopt~repo][c~lexopt~repo~badge]][c~lexopt~repo]
[![lexopt~lib.rs][c~lexopt~lib.rs~badge]][c~lexopt~lib.rs]

lexopt is a small, low-level parser for applications that need to control the parsing loop. It leaves help generation and much of the interface design to the application.

```rust,editable
{{#include ../../../crates/cats/command_line_interface/examples/arguments/lexopt.rs:example}}
```

### Parse Command-line Arguments with `pico-args` {#pico-args}

[![pico-args][c~pico-args~docs~badge]][c~pico-args~docs]{{hi:pico-args}}
[![pico-args~crates.io][c~pico-args~crates.io~badge]][c~pico-args~crates.io]
[![pico-args~repo][c~pico-args~repo~badge]][c~pico-args~repo]
[![pico-args~lib.rs][c~pico-args~lib.rs~badge]][c~pico-args~lib.rs]

The [`pico-args`][c~pico-args~docs]↗{{hi:pico-args}} crate is a small and fast library for parsing command-line arguments in Rust.

pico-args has a compact API for simple command-line interfaces. It has fewer built-in features than clap, so check whether its help, validation, and subcommand support meet the application's needs.

- No automatic help or version output.
- A small set of argument forms and parsing helpers.
- Non-Unicode argument support.

```rust,editable
{{#include ../../../crates/cats/command_line_interface/examples/arguments/pico_args.rs:example}}
```

### Legacy parser: structopt {#structopt .skip}

The structopt project has been superseded by clap's derive API. For existing applications, migrate to the derive API provided by the version of clap in use.

## References {#references .skip}

- [Parsing arguments in Rust, without using dependencies][blog~parsing-arguments-rust-no-dep]↗.

## Related Topics {#related-topics .skip}

- [[parse | Parse]].
- [[parser-implementations | Parser Implementations]].
- [[parsing | Parsing]].
- [[string_parsing | String Parsing]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[arguments: review](https://github.com/john-cd/rust_howto/issues/233)

[Parsing arguments in Rust with no dependencies][blog~parsing-arguments-rust-no-dep]↗.

</div>
