# Structured Data

{{#include complex.incl.md}}

## Serialize and deserialize unstructured JSON

[![serde_json][c-serde_json-badge]][c-serde_json]{{hi:serde_json}}  [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}}{{hi:Deserialization}}{{hi:JSON}}

The [`serde_json`][c-serde_json]{{hi:serde_json}}⮳ crate provides a [`serde_json::from_str`][c-serde_json::from_str]{{hi:serde_json::from_str}}⮳ function to parse{{hi:Parsing}} a `&str` of JSON.

Unstructured JSON can be parsed into a universal [`serde_json::Value`][c-serde_json::Value]{{hi:serde_json::Value}}⮳ type that is able to represent any valid JSON data.

The example below shows a `&str` of JSON being parsed. The expected value is declared using the [`serde_json::json`][c-serde_json::json]{{hi:serde_json::json}}⮳ macro.

```rust
{{#include ../../../deps/tests/json.rs}}
```

## Deserialize a TOML configuration file

[![toml][c-toml-badge]][c-toml]{{hi:toml}}  [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}}{{hi:TOML}}

Parse some TOML into a universal `toml::Value` that is able to represent any valid TOML data.

```rust
{{#include ../../../deps/tests/toml.rs}}
```

Parse TOML into your own structs using [`serde`][c-serde]{{hi:serde}}⮳.

```rust
{{#include ../../../deps/tests/toml1.rs}}
```

## Read and write integers in little-endian byte order

[![byteorder][c-byteorder-badge]][c-byteorder]{{hi:byteorder}}  [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}}{{hi:Little endian}}{{hi:Byte order}}

[`byteorder`][c-byteorder]⮳ can reverse the significant bytes of structured data. This may be necessary when receiving information over the network, such that bytes received are from another system.

```rust
{{#include ../../../deps/tests/endian_byte.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
