<!-- TODO Reviewed ??-??-2026. -->

# JSON Parsing

{{#include json.incl.md}}

[`serde_json`][c~serde_json~docs]↗{{hi:serde_json}} is a common choice for JSON parsing. [`simd-json`][c~simd-json~docs]↗{{hi:simd-json}} is optimized for performance.

Use `serde_json` for the usual Serde-based typed or dynamic JSON data model. Choose `json5` only when the input intentionally uses JSON5 syntax, or `simd-json` when profiling shows JSON parsing is a bottleneck and its mutable input and feature requirements fit the application.

## Parse JSON with `serde_json` {#serde-json}

{{#badges serde_json parser-implementations="Parser implementations" no-std="No standard library" encoding=Encoding}}{{hi:Json}}{{hi:Serde}}{{hi:Serialization}}

[`serde_json`][c~serde_json~docs]↗{{hi:serde_json}} serializes and deserializes JSON, including values mapped to Rust structs and enums.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/json/serde_json.rs:example}}
```

## Parse JSON with `json5` {#json5}

{{#badges json5}}{{hi:Serde}}{{hi:Parse}}{{hi:json5}}{{hi:Parser}}{{hi:Json}}

[`json5`][c~json5~docs]↗{{hi:json5}} is a Rust JSON5 serializer and deserializer which speaks [`serde`][c~serde~docs]↗{{hi:serde}}.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/json/json5.rs:example}}
```

## Parse JSON with `simd-json` {#simd-json}

{{#badges simd-json}}

[`simd-json`][c~simd-json~docs]↗{{hi:simd-json}} is a high-performance JSON parser based on a port of [`simdjson`][simdjson~repo]↗{{hi:simdjson}}.

Test this Cargo-backed example from `bk/crates` with `cargo test --locked -p parser_implementations --example json`.

```rust,editable,noplayground
{{#include ../../../crates/cats/parser_implementations/examples/json/simd_json.rs:example}}
```

## Locate a JSON Deserialization Error {#json-error-path}

Use `serde_path_to_error` with the JSON deserializer to distinguish the field path from the original Serde error. In this example, `servers[1].port` identifies the second server’s port. `inner()` retains the cause, including Serde’s line and column when available. For a different problem, see [reporting unknown fields](../encoding/serde.md#serde-ignored).

```rust,ignore,noplayground
{{#include ../../../crates/cats/parser_implementations/examples/json_error_path.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Serde](../encoding/serde.md) demonstrates mapping structured data into application types.
- [Choose Borrowed or Owned JSON Fields](../encoding/serde.md#borrowed-json) compares parsing from a byte slice and a reader.
- [Parsing Utilities](../parsing/parsing.md) covers custom grammars and parser construction.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
