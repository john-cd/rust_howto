<!-- TODO Reviewed ??-??-2026 -->

# Encoding Without an External Schema File

{{#include no_external_schema.incl.md}}

## `postcard` {#postcard}

[![postcard][c~postcard~docs~badge]][c~postcard~docs] [![postcard~crates.io][c~postcard~crates.io~badge]][c~postcard~crates.io] [![postcard~repo][c~postcard~repo~badge]][c~postcard~repo] [![postcard~lib.rs][c~postcard~lib.rs~badge]][c~postcard~lib.rs]{{hi:postcard}}{{hi:Framing}}{{hi:Serde}}{{hi:Cobs}} [![cat~embedded][cat~embedded~badge]][cat~embedded]{{hi:Embedded development}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`postcard`][c~postcard~docs]↗{{hi:postcard}} is a [`#![no_std]`][book~rust-reference~no_std] and [`serde`][c~serde~docs]↗{{hi:serde}}-compatible message library for Rust. It targets constrained environments and provides a stable wire format; its `heapless` and `alloc` features let an application choose how to buffer encoded messages.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/no_external_schema_file/postcard.rs:example}}
```

## `rkyv` {#rkyv}

[![rkyv][c~rkyv~docs~badge]][c~rkyv~docs] [![rkyv~crates.io][c~rkyv~crates.io~badge]][c~rkyv~crates.io] [![rkyv~repo][c~rkyv~repo~badge]][c~rkyv~repo] [![rkyv~lib.rs][c~rkyv~lib.rs~badge]][c~rkyv~lib.rs]{{hi:rkyv}}{{hi:Serialization}}{{hi:Zero-copy}}{{hi:Archive}}{{hi:No_std}}{{hi:rkyv}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~no-std::no-alloc][cat~no-std::no-alloc~badge]][cat~no-std::no-alloc]{{hi:No dynamic allocation}}

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
