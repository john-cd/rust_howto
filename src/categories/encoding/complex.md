# Structured Data

{{#include complex.incl.md}}

## Serialize and deserialize unstructured JSON

[![serde_json][c-serde_json-badge]][c-serde_json]  [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:deserialize}}{{hi:JSON}}

The {{hi:serde_json}}[`serde_json`][c-serde_json]⮳ crate provides a {{hi:from_str}}[`from_str`][c-serde_json::from_str]⮳ function to {{i:parse}} a `&str` of JSON.

Unstructured JSON can be parsed into a universal {{hi:serde_json::Value}}[`serde_json::Value`][c-serde_json::Value]⮳ type that is able to represent any valid JSON data.

The example below shows a `&str` of JSON being parsed. The expected value is declared using the {{hi:json!}}[`json!`][c-serde_json::json]⮳ macro.

```rust,editable
{{#include ../../../deps/tests/json.rs}}
```

## Deserialize a TOML configuration file

[![toml][c-toml-badge]][c-toml]  [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:TOML}}

Parse some TOML into a universal `toml::Value` that is able to represent any valid TOML data.

```rust,editable
{{#include ../../../deps/tests/toml.rs}}
```

Parse TOML into your own structs using {{hi:Serde}}[`Serde`][c-serde]⮳.

```rust,editable
{{#include ../../../deps/tests/toml1.rs}}
```

## Read and write integers in little-endian byte order

[![byteorder][c-byteorder-badge]][c-byteorder]  [![cat-encoding][cat-encoding-badge]][cat-encoding] {{hi:little-endian}}{{hi:byte order}}

{{hi:byteorder}}[`byteorder`][c-byteorder]⮳ can reverse the significant bytes of structured data. This may be necessary when receiving information over the network, such that bytes received are from another system.

```rust,editable
{{#include ../../../deps/tests/endian-byte.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
