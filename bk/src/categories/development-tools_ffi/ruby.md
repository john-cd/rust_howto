# Generate FFI bindings to Ruby code

{{#include ruby.incl.md}}

## Magnus {#magnus}

[![magnus][c-magnus-badge]][c-magnus] [![magnus-crates.io][c-magnus-crates.io-badge]][c-magnus-crates.io] [![magnus-github][c-magnus-github-badge]][c-magnus-github] [![magnus-lib.rs][c-magnus-lib.rs-badge]][c-magnus-lib.rs]{{hi:magnus}}{{hi:Ruby}}{{hi:Extension}}{{hi:Gem}}{{hi:Rubygem}} [![cat-api-bindings][cat-api-bindings-badge]][cat-api-bindings]{{hi:API bindings}} [![cat-development-tools::ffi][cat-development-tools::ffi-badge]][cat-development-tools::ffi]{{hi:FFI}}

High level Ruby bindings. Write Ruby extension gems in Rust, or call Ruby code from a Rust binary.

```rust,editable
{{#include ../../../crates/ex/cats/development_tools_ffi/tests/ruby/magnus.rs:example}}
```

## Rutie {#rutie}

[![rutie][c-rutie-badge]][c-rutie] [![rutie-crates.io][c-rutie-crates.io-badge]][c-rutie-crates.io] [![rutie-github][c-rutie-github-badge]][c-rutie-github] [![rutie-lib.rs][c-rutie-lib.rs-badge]][c-rutie-lib.rs]{{hi:rutie}}{{hi:Ruby}}{{hi:Cruby}}{{hi:rutie}}

The tie between Ruby and Rust.

```rust,editable
{{#include ../../../crates/ex/cats/development_tools_ffi/tests/ruby/rutie.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[P2 write](https://github.com/john-cd/rust_howto/issues/1076)
</div>
