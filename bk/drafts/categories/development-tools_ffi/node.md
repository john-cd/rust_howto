# Generate FFI Bindings to Node.js Code

{{#include node.incl.md}}

## `napi` {#napi}

[![napi][c-napi-badge]][c-napi] [![napi-crates.io][c-napi-crates.io-badge]][c-napi-crates.io] [![napi-github][c-napi-github-badge]][c-napi-github] [![napi-lib.rs][c-napi-lib.rs-badge]][c-napi-lib.rs]{{hi:napi}}{{hi:Ffi}}{{hi:Node}}{{hi:NodeJS}}{{hi:N-api}}{{hi:napi}}

[`napi`][c-napi]⮳{{hi:napi}} provides N-API bindings for `Node.js`.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/tests/node/napi.rs:example}}
```

## `neon` {#neon}

[![neon-website][c-neon-website-badge]][c-neon-website] [![neon][c-neon-badge]][c-neon] [![neon-crates.io][c-neon-crates.io-badge]][c-neon-crates.io] [![neon-github][c-neon-github-badge]][c-neon-github] [![neon-lib.rs][c-neon-lib.rs-badge]][c-neon-lib.rs]{{hi:neon}}

[`neon`][c-neon]⮳{{hi:neon}} is a safe abstraction layer for Node.js.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/tests/node/neon.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1074)

Interfacing with Node.js from Rust is typically done using Node-API (N-API), which is a stable ABI for Node.js addons.

## Node-API Bindings {#skip}

[`neon`][c-neon]⮳{{hi:neon}}: This crate provides a safe and convenient way to write Node.js addons in Rust using N-API. It handles a lot of the boilerplate and memory management. This is the strongly recommended approach.
Other (Less Common) Options:

`nodejs-sys`: Provides lower-level bindings to the Node.js API, but [`neon`][c-neon]⮳{{hi:neon}} is generally preferred for ease of use and safety.

## Communication/Serialization {#skip}

[`serde`][c-serde]⮳{{hi:serde}}: (Often used for serializing and deserializing data passed between Rust and Node.js. Although N-API provides mechanisms for passing data, using [serde][p-serde] can be helpful for more complex data structures.)
serde_json: (If you're exchanging [JSON][p-json] data.)

## Build Tools {#skip}

[`cargo`][c-cargo]⮳{{hi:cargo}} for building the Rust library.
`npm` or `yarn` for managing the Node.js side and integrating with the Rust addon.)
`node-gyp` often used for building native Node.js addons, but [`neon`][c-neon]⮳{{hi:neon}} simplifies this process.)

## Key Concepts and Workflow {#skip}

N-API: You'll write Rust code that interacts with the N-API to create Node.js objects, call JavaScript functions, and handle data.
Addons: You'll create a Node.js addon that exposes your Rust functions to JavaScript.
Data Marshaling: You'll need to convert data between Rust and JavaScript types. [`neon`][c-neon]⮳{{hi:neon}} helps with this.
Event Loop: You'll need to be mindful of Node.js's event loop and ensure that your Rust code interacts with it correctly (e.g., using [asynchronous][p-asynchronous] operations when necessary).

[`neon`][c-neon]⮳{{hi:neon}} is the most important crate here. It significantly simplifies the process of creating Node.js addons in Rust. It's highly recommended to use it unless you have very specific low-level N-API requirements. It abstracts away much of the complexity of manual N-API interaction.
</div>
