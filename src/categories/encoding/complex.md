# Structured Data

{{#include complex.incl.md}}

## Serialize and {{i:deserialize}} unstructured {{i:JSON}}

[![serde_json][serde_json-badge]][serde_json]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

The [`serde_json`][serde_json] crate provides a [`from_str`][serde_json::from_str] function to {{i:parse}} a `&str` of JSON.

Unstructured JSON can be parsed into a universal [`serde_json::Value`][serde_json::Value] type that is able to represent any valid JSON data.

The example below shows a `&str` of JSON being parsed. The expected value is declared using the [`json!`][serde_json::json] macro.

```rust,editable
{{#include ../../../deps/tests/json.rs}}
```

## Deserialize a {{i:TOML}} configuration file

[![toml][toml-badge]][toml]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

Parse some TOML into a universal `toml::Value` that is able to represent any valid TOML data.

```rust,editable
{{#include ../../../deps/tests/toml.rs}}
```

Parse TOML into your own structs using [`Serde`][serde].

```rust,editable
{{#include ../../../deps/tests/toml1.rs}}
```

## Read and write integers in {{i:little-endian}} {{i:byte order}}

[![byteorder][byteorder-badge]][byteorder]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

`byteorder` can reverse the significant bytes of structured data. This may be necessary when receiving information over the network, such that bytes received are from another system.

```rust,editable
{{#include ../../../deps/tests/endian-byte.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
