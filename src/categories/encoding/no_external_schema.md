# Non-self-describing, no external schema file

{{#include no_external_schema.incl.md}}

## `postcard`

`no_std`-focused `serde` serializer/deserializer, aimed at constrained environments.

```rust,editable
{{#include ../../../crates/ex/cats/encoding/tests/no_external_schema_file/postcard.rs:example}}
```

## `rkyv`

Fast zero-copy deserialization framework that allows arbitrary field types and safe zero-copy mutation.

```rust,editable
{{#include ../../../crates/ex/cats/encoding/tests/no_external_schema_file/rkyv.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[P1 write](https://github.com/john-cd/rust_howto/issues/1078)
</div>
