<!-- TODO Reviewed ??-??-2026 -->

# Generate FFI Bindings to Node.js Code

{{#include node.incl.md}}

Interfacing with Node.js from Rust is typically done using Node-API (N-API), which is a stable ABI for Node.js addons.

## `napi` {#napi}

[![napi][c~napi~docs~badge]][c~napi~docs] [![napi~crates.io][c~napi~crates.io~badge]][c~napi~crates.io] [![napi~repo][c~napi~repo~badge]][c~napi~repo] [![napi~lib.rs][c~napi~lib.rs~badge]][c~napi~lib.rs]{{hi:napi}}{{hi:Ffi}}{{hi:Node}}{{hi:NodeJS}}{{hi:N-api}}{{hi:napi}}

[`napi`][c~napi~docs]↗{{hi:napi}} provides N-API bindings for `Node.js`.

Use the [napi-rs v3 scaffold][ffi~napi-start] for a native addon project:

```bash
npm install --global @napi-rs/cli
napi new
```

Choose a package name and targets in the prompts, then enter the generated directory, install its npm dependencies, and run its build script (`npm run build`, which invokes `napi build`). The scaffold configures `napi`, `napi-derive`, the native library target, and JavaScript/TypeScript entry points together.

In the generated `src/lib.rs`, import `napi_derive::napi` and mark exported functions with `#[napi]`. For example:

```rust
use napi_derive::napi;

/// Returns a greeting for the JavaScript caller.
#[napi]
pub fn hello(name: String) -> String {
    format!("Hello, {name}!")
}
```

`napi build` compiles the generated Cargo `cdylib` and writes a Node-loadable `.node` library. After building a CommonJS package, its generated loader can be called from a local smoke test:

```js
const assert = require('node:assert/strict');
const { hello } = require('./index.js');
assert.equal(hello('Ada'), 'Hello, Ada!');
```

This book's included example tests the plain Rust sum, point conversion, greeting, and async functions. Compile its N-API exports with `--features node`; this is not the generated addon package. Copying `#[napi]` into an executable alone does not create an npm-loadable library.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/examples/node/napi.rs:example}}
```

## `neon` {#neon}

[![neon~website][c~neon~website~badge]][c~neon~website] [![neon][c~neon~docs~badge]][c~neon~docs] [![neon~crates.io][c~neon~crates.io~badge]][c~neon~crates.io] [![neon~repo][c~neon~repo~badge]][c~neon~repo] [![neon~lib.rs][c~neon~lib.rs~badge]][c~neon~lib.rs]{{hi:neon}}

[`neon`][c~neon~docs]↗{{hi:neon}} is a safe abstraction layer for Node.js.

Neon uses a `FunctionContext` to access JavaScript values and `#[neon::main]` to register exports in a native module. Its [project guide][ffi~neon-start] starts with Node.js and Rust; use the generated package's npm build script and load its `.node` artifact through the package entry point. Choose between Neon and napi-rs according to their API and packaging needs; this chapter makes no comparative performance claim.

The book example's ordinary Rust test checks the host-independent greeting. To test the native export, generate and build a Neon package, then add a JavaScript test that loads the generated addon. Keep the scaffold's generated `build` script and add a test script that builds the module before Node's built-in test runner:

```bash
npm init neon@latest neon-example
cd neon-example
npm install
```

In the generated `package.json`, set the test script while preserving its existing build command:

```json
{
  "scripts": {
    "test": "npm run build && node --test"
  }
}
```

For example, save this as `test/addon.test.cjs` after adapting the generated Rust `hello` export to return the greeting from this example:

```js
const assert = require('node:assert/strict');
const test = require('node:test');
const { hello } = require('..');

test('the Neon addon returns its greeting', () => {
  assert.equal(hello(), 'hello from neon!');
});
```

Run the test with:

```bash
npm test
```

The test above exercises the compiled Node addon. The Cargo test only calls the plain Rust function and must not invoke `#[neon::main]` outside Node.js.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/examples/node/neon.rs:example}}
```

## Communication/Serialization {#communication-serialization .skip}

- [`serde`][c~serde~docs]↗{{hi:serde}}: Often used for serializing and deserializing data passed between Rust and Node.js.
- [`serde_json`][c~serde_json~docs]↗{{hi:serde_json}}: If you're exchanging [JSON][p~json] data.

For ordinary calls, napi-rs and Neon convert runtime values directly. Use typed arrays or buffers for bulk bytes, with explicit ownership/lifetime rules, rather than converting a large binary payload to JSON. Use `Result` at the wrapper boundary to return a JavaScript error when validation fails.

## Build Tools {#build-tools .skip}

- Use [`cargo`][c~cargo~docs]↗{{hi:cargo}} for building the Rust library.
- Use `npm` or `yarn` for managing the Node.js side and integrating with the Rust addon.

For distribution, use napi-rs's [native package release workflow][ffi~napi-release]. Build and test binaries for each supported operating system, architecture, and ABI/libc combination; publish those artifacts with the generated loader. Node-API stability does not make one compiled binary portable across all targets. Test loading from the packed npm package on each target, including a clean machine without Rust installed.

## Key Concepts and Workflow {#key-concepts-and-workflow .skip}

- N-API: You'll write Rust code that interacts with the N-API to create Node.js objects, call JavaScript functions, and handle data.
- Addons: You'll create a Node.js addon that exposes your Rust functions to JavaScript.
- Data Marshaling: You'll need to convert data between Rust and JavaScript types. [`neon`][c~neon~docs]↗{{hi:neon}} helps with this.
- Event Loop: You'll need to be mindful of Node.js's event loop and ensure that your Rust code interacts with it correctly (e.g., using [asynchronous][p~asynchronous] operations when necessary).

### Export Classes and Asynchronous Work {#node-async}

A napi-rs class marks its struct and exported `impl` with `#[napi]`, with `#[napi(constructor)]` for construction; see the [class guide][ffi~napi-class]. Keep resource ownership in the Rust instance and define what happens when JavaScript releases it.

For CPU work, implement `Task` and return [`AsyncTask`][ffi~napi-task] so computation runs in libuv's worker pool and resolves a Promise. Avoid touching JavaScript objects during `compute`; convert the owned result in `resolve`. For asynchronous IO, [`#[napi] async fn`][ffi~napi-async] integrates with Tokio when its runtime feature is enabled. An async function still blocks its worker if it performs long synchronous work, so move that work to a bounded blocking pool.

### Choose WebAssembly When Appropriate {#node-wasm}

Use [WebAssembly](../wasm/index.md) when sharing a portable computation with a browser is more useful than direct native integration. The [JavaScript interop chapter](../wasm/interfacing_with_javascript.md) covers that boundary. Native addons remain useful for native dependencies and OS resources; WebAssembly still needs an explicit host interface for such capabilities.

From `later/crates`, run `cargo test --locked -p development_tools_ffi --features node --example node_napi` and `cargo test --locked -p development_tools_ffi --features node --example node_neon` to check the pure-Rust paths. The N-API and Neon examples are separate Cargo targets because each addon exports its own Node.js module entry point. These Rust tests do not load the Node addon. Build the matching `cdylib` package with `napi build` or the Neon scaffold's npm script, then run its JavaScript smoke test against the generated loader.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
