<!-- TODO Reviewed ??-??-2026 -->

A transpiler translates source code into source code while retaining much of its structure. In Rust projects, translation tools are often used to migrate legacy code or generate language bindings; they do not replace compiler checks, tests, or human review. In particular, C-to-Rust output can preserve the original program while still containing extensive `unsafe` code.

| Tool                              | Direction                            | Status and role                                                                             |
| --------------------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------- |
| [C2Rust][c2rust~repo]             | C99 to Rust                          | Maintained upstream; translates C-like code to unsafe Rust as a migration starting point.   |
| [bindgen][bindgen~repo]           | C/C++ headers to Rust                | Maintained Rust project; generates Rust FFI declarations for calling native libraries.      |
| [cbindgen][cbindgen~repo]         | Rust public C API to C/C++ headers   | Mozilla project; generates headers for Rust libraries exposing a C ABI.                     |
| [wasm-bindgen][wasm-bindgen~repo] | Rust and JavaScript/WebAssembly glue | Maintained project; generates bindings that connect Rust WebAssembly modules to JavaScript. |

See [[development-tools_ffi | Development Tools: FFI]] for native interfaces and [[wasm | WebAssembly]] for compiling and running Rust modules in a web environment.

| Recipe                                              | Crates                | Categories                          |
| --------------------------------------------------- | --------------------- | ----------------------------------- |
| [C to Rust Translation][ex~transcompilation~c2rust] | [C2Rust][c2rust~repo] | {{#categories compilers=Compilers}} |
