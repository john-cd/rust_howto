<!-- TODO Reviewed ??-??-2026. -->

# TOML

{{#include toml.incl.md}}

[`toml`][c~toml~docs]↗{{hi:toml}} is the primary crate.

Use `toml` with Serde to map data to Rust types, or parse into `toml::Value` when the structure is not known in advance. Use `toml_edit` when an editor must preserve comments and formatting. The smaller `basic-toml` example provides a minimal alternative. For serialization choices shared across formats, see [Serde](../encoding/serde.md).

## `toml` {#toml}

{{#badges toml config=Configuration encoding=Encoding parser-implementations="Parser implementations" parsing="Parsing tools"}}{{hi:Encoding}}{{hi:toml}}

[`toml`][c~toml~docs]↗{{hi:toml}} encodes and decodes TOML. It implements Serde's `Serialize` and `Deserialize` traits so Rust structs can be read from and written to TOML data.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/toml/toml.rs:example}}
```

## `toml_edit` {#toml-edit}

{{#badges toml_edit config=Configuration encoding=Encoding parser-implementations="Parser implementations" parsing="Parsing tools"}}{{hi:Encoding}}{{hi:Toml}}

[`toml_edit`][c~toml_edit~docs]↗{{hi:toml_edit}} is a format-preserving TOML parser.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/toml/toml_edit.rs:example}}
```

## `basic-toml` {#basic-toml}

{{#badges basic-toml config=Configuration parser-implementations="Parser implementations" encoding=Encoding}}{{hi:Toml}}{{hi:Serde}}

[`basic-toml`][c~basic-toml~docs]↗{{hi:basic-toml}} is a minimal TOML library with few dependencies

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/toml/basic_toml.rs:example}}
```

## Analyze and Format TOML with Taplo {#taplo}

[![taplo~website][c~taplo~website~badge]][c~taplo~website] {{#badges taplo}} {{hi:Parser}} {{hi:Toml}} {{hi:Linter}} {{hi:Formatter}} {{#categories parser-implementations="Parser implementations"}}

[`Taplo`][c~taplo~website]↗{{hi:Taplo}} is a TOML parser, analyzer, formatter, and language server. Use its editor tooling to validate and format configuration files; use the Rust libraries above when TOML parsing is part of an application.

## Related Topics {#related-topics .skip}

- [Serde](../encoding/serde.md) shows how TOML fits with other serialized formats.
- [Parsing Utilities](../parsing/parsing.md) covers building parsers for custom grammars.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
