<!-- TODO Reviewed ??-??-2026 -->

# JSON Parsing

{{#include json.incl.md}}

[`serde_json`][c~serde_json~docs]↗{{hi:serde_json}} is the most commonly used crate for JSON Parsing. [`simd-json`][c~simd-json~docs]↗{{hi:simd-json}} is optimized for performance.

Use `serde_json` for the usual Serde-based typed or dynamic JSON data model. Choose `json5` only when the input intentionally uses JSON5 syntax, or `simd-json` when profiling shows JSON parsing is a bottleneck and its mutable input and feature requirements fit the application.

## Parse JSON with `serde_json` {#serde-json}

{{#badges serde_json parser-implementations="Parser implementations" no-std="No standard library" encoding=Encoding}}{{hi:Json}}{{hi:Serde}}{{hi:Serialization}}

[`serde_json`][c~serde_json~docs]↗{{hi:serde_json}} offers a JSON serialization file format.

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

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/json/simd_json.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Structured Data](../encoding/complex_encoding.md) demonstrates deserializing JSON and TOML into values and application types.
- [Parsing Utilities](../parsing/parsing.md) covers custom grammars and parser construction.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
