# Scripting and Rust

Scripting involves writing small programs, often in high-level, interpreted languages, to automate tasks, manipulate data, or control software workflows. It is widely used for system administration, task automation, web development, and integrating various software components efficiently.

Crates like [`rhai`][c~rhai~docs]↗{{hi:rhai}} enable embedding scripting capabilities into Rust applications, allowing efficient automation and customization while leveraging Rust's strong type system and concurrency features.

## Common Use Cases for Scripting

- Game Engines: Scripting game logic, AI, and events.
- Embedded Systems: Configuring and controlling devices.
- Application Plugins: Extending application functionality.
- Automation Tools: Automating repetitive tasks.
- Configuration Files: Using Rhai as a more powerful configuration language.
- Data Processing: Transforming and manipulating data.

## Language Embedding

You may integrating an existing scripting language within a Rust application. Common choices include:

- Rhai.
- Lua.
- JavaScript.
- Python.

### Rhai

Rhai is a lightweight scripting language similar to Rust that integrates well with Rust, making it ideal for embedding scripting capabilities into Rust applications. Its features include dynamic typing, Rust function registration, modules, and sandboxing.

{{#include rhai.incl.md}}

### Lua

The Lua scripting language has a robust, widely used, mature ecosystem. Use `rlua` or [`mlua`][c~mlua~docs]↗{{hi:mlua}} to interact with or embed Lua.
See [[lua | Lua]].

## JavaScript

Embedding a JavaScript runtime gives you full JavaScript language support:

- `v8`: bindings to Google's V8 engine.
- `deno_core`: The core runtime of Deno, a modern JavaScript/TypeScript runtime built with V8, Rust, and Tokio.
- `quickjs-rs`: bindings to the QuickJS engine.

See also [[node | Node]].

## Python

You may also integrate Python into Rust applications via:

- [`cpython`][c~cpython~docs]↗{{hi:cpython}} (older bindings).
- [`pyo3`][c~pyo3~docs]↗{{hi:pyo3}}.

See [[python | Python]].

## WebAssembly

Alternatively, you may embed a WebAssembly runtime like [`wasmtime`][c~wasmtime~docs]↗{{hi:wasmtime}}, or a WebAssembly interpreter (e.g. [`wasmi`][c~wasmi~docs]↗{{hi:wasmi}}), into your Rust application.

WASM offers portable bytecode and sandboxed execution. Many languages (C, C++, Rust, Go, C#...) compile to or have their VMs in WASM. A list is found in [Awesome WebAssembly Languages](https://github.com/appcypher/awesome-wasm-langs#typescript)↗.

See [[wasm | WASM]] and [[wasm_standalone_runtimes | WASM Standalone Runtimes]].

## Building Custom Scripting Languages

To build your own custom scripting language, consider using the following crates:

- Lexical Analysis: [`logos`][c~logos~docs]↗{{hi:logos}}, a fast and efficient lexer generator.
- Parser Generators: [`pest`][c~pest~docs]↗{{hi:pest}}, a Parsing Expression Grammar (PEG) parser, or [`lalrpop`][c~lalrpop~docs]↗{{hi:lalrpop}}, a LALR(1) parser generator.
- Parser Combinators: [`nom`][c~nom~docs]↗{{hi:nom}} or [`chumsky`][c~chumsky~docs]↗{{hi:chumsky}}.
See [[text-processing | Text Processing]], [[parsing | Parsing]], and for examples, [[parser-implementations | Parser Implementations]] and [[programming_languages | Programming Languages]].

Abstract Syntax Tree (AST) manipulation is usually custom-built. [`syn`][c~syn~docs]↗{{hi:syn}}, a library for parsing Rust code, and [`quote`][c~quote~docs]↗{{hi:quote}} may be good examples - see [[development-tools_procedural-macro-helpers | Development Tools: Procedural Macro Helpers]].

If your scripting language compiles to an executable, use the `object` crate. It supports reading relocatable object files and executable files, and writing COFF/ELF/Mach-O/XCOFF relocatable object files and ELF/PE executable files.

- Handle errors with [`thiserror`][c~thiserror~docs]↗{{hi:thiserror}}, [`anyhow`][c~anyhow~docs]↗{{hi:anyhow}}, and `codespan-reporting`, a library for generating human-readable error messages with source code snippets. See [[error_handling | Error Handling]], [[error_customization | Error Customization]].

## Related Topics

- [[development-tools_ffi | Development Tools: FFI]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/991)

- [rust-script | Run Rust files and expressions as scripts without any setup or compilation step.](https://rust-script.org/)
- [rust-script: Run Rust files and expressions as scripts without any setup or compilation step.](https://github.com/fornwall/rust-script)
- [shell2batch: Coverts simple basic shell scripts to windows batch scripts.](https://github.com/sagiegurari/shell2batch)
- [cargo-scripter](https://lib.rs/crates/cargo-scripter)
- [rust-script | Run Rust files and expressions as scripts without any setup or compilation step.](https://rust-script.org/?ref=niccoloforlini.com)

[Pre-RFC: `cargo-script` for everyone - tools and infrastructure / cargo - Rust Internals](https://internals.rust-lang.org/t/pre-rfc-cargo-script-for-everyone/18639)

</div>
