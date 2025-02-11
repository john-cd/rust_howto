# Non-self-describing, no external schema file

{{#include no_external_schema.incl.md}}

## `postcard` {#postcard}

[![postcard][c-postcard-badge]][c-postcard] [![postcard-crates.io][c-postcard-crates.io-badge]][c-postcard-crates.io] [![postcard-github][c-postcard-github-badge]][c-postcard-github] [![postcard-lib.rs][c-postcard-lib.rs-badge]][c-postcard-lib.rs]{{hi:postcard}}{{hi:Framing}}{{hi:Serde}}{{hi:Cobs}} [![cat-embedded][cat-embedded-badge]][cat-embedded]{{hi:Embedded development}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

A no_std + serde compatible message library for Rust.

`no_std`-focused `serde` serializer/deserializer, aimed at constrained environments.

```rust,editable
{{#include ../../../crates/cats/encoding/tests/no_external_schema_file/postcard.rs:example}}
```

## `rkyv` {#rkyv}

[![rkyv][c-rkyv-badge]][c-rkyv] [![rkyv-crates.io][c-rkyv-crates.io-badge]][c-rkyv-crates.io] [![rkyv-github][c-rkyv-github-badge]][c-rkyv-github] [![rkyv-lib.rs][c-rkyv-lib.rs-badge]][c-rkyv-lib.rs]{{hi:rkyv}}{{hi:Serialization}}{{hi:Zero-copy}}{{hi:Archive}}{{hi:No_std}}{{hi:rkyv}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}} [![cat-no-std::no-alloc][cat-no-std::no-alloc-badge]][cat-no-std::no-alloc]{{hi:No dynamic allocation}}

Fast zero-copy deserialization framework that allows arbitrary field types and safe zero-copy mutation.

```rust,editable
{{#include ../../../crates/cats/encoding/tests/no_external_schema_file/rkyv.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[P1 write](https://github.com/john-cd/rust_howto/issues/1078)
</div>
