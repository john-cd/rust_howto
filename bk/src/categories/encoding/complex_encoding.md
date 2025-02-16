# Structured Data

{{#include complex_encoding.incl.md}}

## Serialize and deserialize unstructured JSON {#serde-json}

[![serde_json][c-serde_json-badge]][c-serde_json]{{hi:serde_json}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}}{{hi:Deserialization}}{{hi:JSON}}

The [`serde_json`][c-serde_json]{{hi:serde_json}}⮳ crate provides a [`serde_json::from_str`][c-serde_json::from_str]{{hi:serde_json::from_str}}⮳ function to parse{{hi:Parsing}} a `&str` of JSON.

Unstructured [JSON][p-json] can be parsed into a universal [`serde_json::Value`][c-serde_json::Value]{{hi:serde_json::Value}}⮳ type that is able to represent any valid JSON data.

The example below shows a `&str` of [JSON][p-json] being parsed. The expected value is declared using the [`serde_json::json`][c-serde_json::json]{{hi:serde_json::json}}⮳ macro.

```rust,editable
{{#include ../../../crates/cats/encoding/tests/serde/json.rs:example}}
```

## Deserialize a TOML configuration file {#toml}

[![toml][c-toml-badge]][c-toml]{{hi:toml}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}}{{hi:TOML}}

TOML is a simple, ergonomic, and readable [configuration][p-configuration] format that is often used by Rust's tooling - for example `cargo`.
The following parses some TOML into a universal `toml::Value` that is able to represent any valid [TOML][p-toml] data.

```rust,editable
{{#include ../../../crates/cats/encoding/tests/toml/toml.rs:example}}
```

Parse [TOML][p-toml] into your own [structs][p-structs] using [`serde`][c-serde]{{hi:serde}}⮳.

```rust,editable
{{#include ../../../crates/cats/encoding/tests/toml/toml1.rs:example}}
```

## Read and write integers in little-endian byte order {#byteorder}

[![byteorder][c-byteorder-badge]][c-byteorder] [![byteorder-crates.io][c-byteorder-crates.io-badge]][c-byteorder-crates.io] [![byteorder-github][c-byteorder-github-badge]][c-byteorder-github] [![byteorder-lib.rs][c-byteorder-lib.rs-badge]][c-byteorder-lib.rs]{{hi:byteorder}}{{hi:Little-endian}}{{hi:Big-endian}}{{hi:Endian}}{{hi:Byte}}{{hi:Binary}}{{hi:Encoding}}{{hi:Byte order}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}} [![cat-parsing][cat-parsing-badge]][cat-parsing]{{hi:Parsing tools}}

[`byteorder`][c-byteorder]⮳ is a library for reading/writing numbers in big-endian and little-endian. It can reverse the significant bytes of structured data. This may be necessary when receiving information over the network, when bytes received are from another system.

```rust,editable
{{#include ../../../crates/cats/encoding/tests/endian_byte.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[complex: clean up toml (P1)](https://github.com/john-cd/rust_howto/issues/350)

</div>
