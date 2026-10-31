<!-- TODO Reviewed ??-??-2026 -->

# Encoding Without an External Schema File

{{#include no_external_schema.incl.md}}

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

## `rkyv` {#rkyv}

{{#badges rkyv encoding=Encoding no-std="No standard library" no-std::no-alloc="No dynamic allocation"}}{{hi:Serialization}}{{hi:Zero-copy}}{{hi:Archive}}{{hi:No_std}}{{hi:rkyv}}

[`rkyv`][c~rkyv~docs]↗{{hi:rkyv}} is a fast zero-copy deserialization framework that allows arbitrary field types and safe zero-copy mutation. Unlike the Serde formats compared above, it stores a type-specific archive layout for direct access; the types on both sides therefore form part of the compatibility contract.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/no_external_schema_file/rkyv.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Serialization with Serde][ex~encoding~serde].
- [Binary encoders with an external schema][p~binary-encoders].
- [Typecasts and memory layout][p~typecasts].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1078)

This chapter compares formats that use application types without a separate schema file, including their wire-level type information and `no_std` support. The `rkyv` example remains as a distinct type-specific archive approach.

</div>
