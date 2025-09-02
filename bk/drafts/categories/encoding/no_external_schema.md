# Non-self-describing, no External Schema File

{{#include no_external_schema.incl.md}}

## `postcard` {#postcard}

[![postcard][c~postcard~docs~badge]][c~postcard~docs] [![postcard~crates.io][c~postcard~crates.io~badge]][c~postcard~crates.io] [![postcard~repo][c~postcard~repo~badge]][c~postcard~repo] [![postcard~lib.rs][c~postcard~lib.rs~badge]][c~postcard~lib.rs]{{hi:postcard}}{{hi:Framing}}{{hi:Serde}}{{hi:Cobs}} [![cat~embedded][cat~embedded~badge]][cat~embedded]{{hi:Embedded development}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`postcard`][c~postcard~docs]↗{{hi:postcard}} is a [`#![no_std]`][book~rust-reference~no_std] and [`serde`][c~serde~docs]↗{{hi:serde}}-compatible message library for Rust.

[`#![no_std]`][book~rust-reference~no_std]-focused [`serde`][c~serde~docs]↗{{hi:serde}} serializer/deserializer, aimed at constrained environments.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/no_external_schema_file/postcard.rs:example}}
```

## `rkyv` {#rkyv}

[![rkyv][c~rkyv~docs~badge]][c~rkyv~docs] [![rkyv~crates.io][c~rkyv~crates.io~badge]][c~rkyv~crates.io] [![rkyv~repo][c~rkyv~repo~badge]][c~rkyv~repo] [![rkyv~lib.rs][c~rkyv~lib.rs~badge]][c~rkyv~lib.rs]{{hi:rkyv}}{{hi:Serialization}}{{hi:Zero-copy}}{{hi:Archive}}{{hi:No_std}}{{hi:rkyv}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~no-std::no-alloc][cat~no-std::no-alloc~badge]][cat~no-std::no-alloc]{{hi:No dynamic allocation}}

[`rkyv`][c~rkyv~docs]↗{{hi:rkyv}} is a fast zero-copy deserialization framework that allows arbitrary field types and safe zero-copy mutation.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/no_external_schema_file/rkyv.rs:example}}
```

## Related Topics {#related-topics .skip}

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1078)
</div>
