<!-- TODO Reviewed ??-??-2026 -->
The WebAssembly target and the host runtime must agree on the module's imports and supported features.

- For browser applications and JavaScript hosts, `wasm32-unknown-unknown` is commonly used with JavaScript bindings. It imports no operating-system functions, so some standard-library operations such as filesystem access are unavailable.
- For WASI programs, choose `wasm32-wasip1` or `wasm32-wasip2` to match the runtime. Preview 1 uses the `wasi_snapshot_preview1` API; preview 2 uses the component model. File and other host access still depends on the runtime's implementation and granted capabilities.
- For a custom WebAssembly host, build for the ABI and imports it provides. A minimal `wasm32-unknown-unknown` module does not gain host services automatically.

See the Rust target documentation for [`wasm32-unknown-unknown`][rustc~wasm32-unknown-unknown~website], [`wasm32-wasip1`][rustc~wasm32-wasip1~website], and [`wasm32-wasip2`][rustc~wasm32-wasip2~website].
