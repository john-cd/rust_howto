# Generate FFI bindings to Erlang / Elixir code

{{#include erlang_elixir.incl.md}}

## `rustler` {#rustler}

[![rustler][c-rustler-badge]][c-rustler] [![rustler-crates.io][c-rustler-crates.io-badge]][c-rustler-crates.io] [![rustler-github][c-rustler-github-badge]][c-rustler-github] [![rustler-lib.rs][c-rustler-lib.rs-badge]][c-rustler-lib.rs]{{hi:rustler}}

[`rustler`][c-rustler]⮳{{hi:rustler}} provides safe Rust wrappers for creating Erlang NIF functions.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/tests/erlang_elixir/rustler.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[P2 write](https://github.com/john-cd/rust_howto/issues/1070)
</div>
