<!-- TODO Reviewed ??-??-2026 -->

# Structured Data

{{#include complex_encoding.incl.md}}

These examples focus on converting serialized data to and from application values. For library-specific parser comparisons, including format-preserving TOML editing, see [TOML parsing](../parser-implementations/toml.md).

## Serialize and Deserialize Unstructured JSON {#serde-json}

[![serde_json][c~serde_json~docs~badge]][c~serde_json~docs]{{hi:serde_json}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}}{{hi:Deserialization}}{{hi:JSON}}

The [`serde_json`][c~serde_json~docs]↗{{hi:serde_json}} crate provides a [`serde_json::from_str`][c~serde_json::from_str~docs]↗{{hi:serde_json::from_str}} function to parse{{hi:Parsing}} a `&str` of JSON.

Unstructured [JSON][p~json] can be parsed into a universal [`serde_json::Value`][c~serde_json::Value~docs]↗{{hi:serde_json::Value}} type that is able to represent any valid JSON data.

The example below shows a [`&str`][primitive~str]↗{{hi:&str}} of [JSON][p~json] being parsed. The expected value is declared using the [`serde_json::json`][c~serde_json::json~docs]↗{{hi:serde_json::json}} macro.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/serde/json.rs:example}}
```

## Deserialize a TOML Configuration File {#toml}

[![toml][c~toml~docs~badge]][c~toml~docs]{{hi:toml}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}}{{hi:TOML}}

TOML is a simple, ergonomic, and readable [configuration][p~configuration] format that is often used by Rust's tooling - for example [`cargo`][c~cargo~docs]↗{{hi:cargo}}.
The following parses some TOML into a universal [`toml::Value`][c~toml::Value~docs]↗{{hi:toml::Value}} that is able to represent any valid [TOML][p~toml] data.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/toml_encoding/toml.rs:example}}
```

Parse [TOML][p~toml] into your own [structs][p~structs] using [`serde`][c~serde~docs]↗{{hi:serde}}.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/toml_encoding/toml1.rs:example}}
```

## Read and Write Integers in Little-endian Byte Order {#byteorder}

[![byteorder][c~byteorder~docs~badge]][c~byteorder~docs] [![byteorder~crates.io][c~byteorder~crates.io~badge]][c~byteorder~crates.io] [![byteorder~repo][c~byteorder~repo~badge]][c~byteorder~repo] [![byteorder~lib.rs][c~byteorder~lib.rs~badge]][c~byteorder~lib.rs]{{hi:byteorder}}{{hi:Little-endian}}{{hi:Big-endian}}{{hi:Endian}}{{hi:Byte}}{{hi:Binary}}{{hi:Encoding}}{{hi:Byte order}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~parsing][cat~parsing~badge]][cat~parsing]{{hi:Parsing tools}}

[`byteorder`][c~byteorder~docs]↗ is a library for reading/writing numbers in big-endian and little-endian. It can reverse the significant bytes of structured data. This may be necessary when receiving information over the network, when bytes received are from another system.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/endian_byte.rs:example}}
```

## Related Topics {#related-topics .skip}

- [TOML parsing](../parser-implementations/toml.md) compares Rust TOML libraries and format-preserving editing.
- [JSON parsing](../parser-implementations/json.md) covers parser-specific JSON options.
- [Binary Encoders](_binary_encoders.md) covers converting structured data to binary formats.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
