<!-- TODO Reviewed ??-??-2026 -->

# INI Parsing

{{#include ini.incl.md}}

## INI Syntax and Parser Choice {#ini}

INI is a family of configuration syntaxes rather than one fully standardized format. Files commonly use section headers such as `[server]` and key-value lines such as `port = 8080`, but case sensitivity, comments, duplicate keys, escaping, and multiline values vary between readers. Choose a parser that matches the files your application must read; test it against representative files from their actual producer.

When you control the format, consider whether a more explicit format such as TOML is a better fit. For an existing INI file, also decide how to handle missing keys, invalid typed values, and configuration overrides. A parser cannot choose those application policies for you.

## Parse INI with `configparser` {#configparser}

[![configparser][c~configparser~docs~badge]][c~configparser~docs] [![configparser~crates.io][c~configparser~crates.io~badge]][c~configparser~crates.io] [![configparser~repo][c~configparser~repo~badge]][c~configparser~repo]{{hi:configparser}}{{hi:INI}}{{hi:Configuration}}

[`configparser`][c~configparser~docs]↗{{hi:configparser}} is a pure-Rust parser with no dependencies. Its `Ini` type reads from strings, files, or readers and stores values as strings. Convenience getters such as `getint`, `getuint`, `getfloat`, and `getbool` parse common types and return errors for invalid values. The default constructor treats section and key names case-insensitively; use `Ini::new_cs` when the input format requires case-sensitive names.

`read` and `load` replace the current contents. Use `read_and_append` or `load_and_append` when later configuration should overlay earlier values. The parser exposes settings for comment characters, delimiters, and other syntax choices, so configure them to match the producer rather than assuming all INI dialects behave alike.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/ini/configparser_ini.rs:example}}
```

## Parse INI with `rust-ini` {#rust-ini}

[![rust-ini][c~rust-ini~docs~badge]][c~rust-ini~docs] [![rust-ini~crates.io][c~rust-ini~crates.io~badge]][c~rust-ini~crates.io] [![rust-ini~repo][c~rust-ini~repo~badge]][c~rust-ini~repo] [![rust-ini~lib.rs][c~rust-ini~lib.rs~badge]][c~rust-ini~lib.rs]{{hi:rust-ini}}{{hi:INI}}{{hi:Configuration}}

[`rust-ini`][c~rust-ini~docs]↗{{hi:rust-ini}} provides an `Ini` value with methods to load, inspect, modify, and write sections and properties. Its crate name in Rust code is `ini`, so it is distinct from the [`ini`][c~ini~docs] macro crate described below.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/ini/rust_ini.rs:example}}
```

## Use the `ini` macro crate for simple inputs {#ini-macro}

The separate [`ini`][c~ini~docs] crate offers macros for loading files, parsing strings, merging multiple files, and writing their nested map representation. It is built on `configparser` and is convenient when macro-based access fits the application. The default macros panic on parse or I/O errors; prefix them with `safe` to receive a `Result` instead. For explicit error handling, typed getters, or parser configuration, use `configparser` directly.

## Related Topics {#related-topics .skip}

- [Configuration files][ex~parser-implementations~configuration]
- [TOML parsing][ex~parser-implementations~basic-toml]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
