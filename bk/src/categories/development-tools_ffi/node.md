# Generate FFI bindings to Node.js code

{{#include node.incl.md}}

## `napi` {#napi}

[![napi][c-napi-badge]][c-napi] [![napi-crates.io][c-napi-crates.io-badge]][c-napi-crates.io] [![napi-github][c-napi-github-badge]][c-napi-github] [![napi-lib.rs][c-napi-lib.rs-badge]][c-napi-lib.rs]{{hi:napi}}{{hi:Ffi}}{{hi:Node}}{{hi:Nodejs}}{{hi:N-api}}{{hi:napi}}

N-API bindings for Node.js.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/tests/node/napi.rs:example}}
```

## `neon` {#neon}

[![neon-website][c-neon-website-badge]][c-neon-website] [![neon][c-neon-badge]][c-neon] [![neon-crates.io][c-neon-crates.io-badge]][c-neon-crates.io] [![neon-github][c-neon-github-badge]][c-neon-github] [![neon-lib.rs][c-neon-lib.rs-badge]][c-neon-lib.rs]{{hi:neon}}

A safe abstraction layer for Node.js.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/tests/node/neon.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[P2 write](https://github.com/john-cd/rust_howto/issues/1074)
</div>
