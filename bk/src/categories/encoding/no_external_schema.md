<!-- TODO Reviewed ??-??-2026. -->

# Binary Encoders Without External Schemas

These approaches use Rust types (usually described with [`serde`][ex~encoding~serde]) as the mapping for encoding and decoding values. They do not require a separate schema file or generated types. That does not make every payload self-describing: formats differ in how much type information and field names they put on the wire, and a concrete Rust type still supplies application-level meaning.

Here, _self-describing_ means the format carries enough type information to decode a value without first choosing a Rust struct. It does not carry application semantics or replace versioned type definitions. MessagePack structs are encoded as positional arrays by `rmp_serde::to_vec`; use `rmp_serde::to_vec_named` to encode field names as a map.

These formats contrast with [schema-first binary encoders][p~binary-encoders], which define a separate contract such as a `.proto`, `.fbs`, or `.capnp` file and generate types from it.

{{#include no_external_schema.incl.md}}

## `bincode` {#bincode}

{{#badges bincode encoding=Encoding network-programming="Network programming"}}{{hi:Deserialize}}{{hi:Serialize}}{{hi:Binary}}{{hi:Encode}}{{hi:Decode}}

[`bincode`][c~bincode~docs]↗{{hi:bincode}} encodes and decodes Rust values as compact binary data. The receiver must use a compatible type definition and configuration; the byte stream does not carry field names or an independent schema.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/binary_encoders/bincode.rs:example}}
```

## MessagePack with `rmp-serde` {#rmp-serde}

{{#badges rmp-serde encoding=Encoding}}{{hi:MessagePack}}{{hi:Serialization}}{{hi:Serde}}

[`rmp-serde`][c~rmp-serde~docs]↗{{hi:rmp-serde}} connects MessagePack with [`serde`][c~serde~docs]↗{{hi:serde}}. This example uses `to_vec_named` to encode struct field names as a map. `to_vec` uses a compact positional array; both formats require the matching Rust type when decoding into `MyData`.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/binary_encoders/rmp_serde.rs:example}}
```

## `postcard` {#postcard}

{{#badges postcard embedded="Embedded development" no-std="No standard library"}}{{hi:Framing}}{{hi:Serde}}{{hi:Cobs}}

[`postcard`][c~postcard~docs]↗{{hi:postcard}} is a [`#![no_std]`][book~rust-reference~no_std] and [`serde`][c~serde~docs]↗{{hi:serde}}-compatible message library for Rust. It targets constrained environments and provides a stable wire format; its `heapless` and `alloc` features let an application choose how to buffer encoded messages.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/no_external_schema_file/postcard.rs:example}}
```

## CBOR with `ciborium` {#ciborium}

{{#badges ciborium data-structures="Data structures" no-std="No standard library" embedded="Embedded development" encoding=Encoding parsing="Parsing tools"}}{{hi:CBOR}}{{hi:Serde}}

Concise Binary Object Representation (CBOR) is a binary serialization format with a standardized data model. [`ciborium`][c~ciborium~docs]↗{{hi:ciborium}} implements CBOR with [`serde`][c~serde~docs]↗{{hi:serde}}.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/binary_encoders/ciborium.rs:example}}
```

## `rkyv` {#rkyv}

{{#badges rkyv encoding=Encoding no-std="No standard library" no-std::no-alloc="No dynamic allocation"}}{{hi:Serialization}}{{hi:Zero-copy}}{{hi:Archive}}{{hi:No_std}}{{hi:rkyv}}

[`rkyv`][c~rkyv~docs]↗{{hi:rkyv}} is a fast zero-copy deserialization framework that allows arbitrary field types and safe zero-copy mutation. Unlike the Serde formats compared above, it stores a type-specific archive layout for direct access; the types on both sides therefore form part of the compatibility contract.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/no_external_schema_file/rkyv.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Serialization with Serde][ex~encoding~serde].
- [Binary encoders with an external schema](binary_encoders.md).
- [Typecasts and memory layout](typecasts.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  This chapter compares formats that use application types without a separate schema file, including their wire-level type information and `no_std` support. The `rkyv` example remains as a distinct type-specific archive approach.
</div>
