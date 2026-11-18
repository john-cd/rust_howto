<!-- TODO Reviewed ??-??-2026. -->

# Scripting and Rust

Scripting in Rust has two distinct forms: running Rust source as a lightweight script, and embedding another language in a Rust application. Choose based on startup time, deployment, host integration, and the trust level of the code being run.

## Run Rust Source as a Script

Cargo's single-file script support uses frontmatter and is currently an unstable nightly feature. Its command-line form is `cargo +nightly -Zscript script.rs`; check the [Cargo unstable feature documentation][cargo-script-nightly] before relying on it. [`rust-script`][c~rust-script~repo]↗{{hi:rust-script}} is another script runner.

Save the example as `config_path.rs`. The `---cargo` block declares the script's dependency, and Clap parses `--config` as a path. The script prints the selected path. The first invocation resolves and builds the dependencies. Run without `--config` to use the default, `settings.toml`.

```text
{{#include examples/config_path.rs}}
```

```sh
cargo +nightly -Zscript config_path.rs --config local.toml
```

The command prints `local.toml`. Use a regular Cargo package when you need a checked-in lockfile and a stable-toolchain workflow.

## Embed a Scripting Language

| Language | Crate | Typing | Sandboxing and embedding notes |
| ------------------------ | ----------------------------------------------------------------------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| Rhai | [`rhai`][c~rhai~docs]↗{{hi:rhai}} | Dynamic | Start with the [embedded evaluator](rhai.md#rust-scripting) and its example of limiting script work. |
| Lua | [`mlua`][c~mlua~docs]↗{{hi:mlua}} | Dynamic | Bindings for Lua and Luau; select a runtime and features explicitly. See [[lua | Lua]]. |
| Rune | [`rune`][c~rune~docs]↗{{hi:rune}} | Dynamic | Embeddable scripting language with host functions and modules. |
| Koto | [`koto`][c~koto~docs]↗{{hi:koto}} | Dynamic | Small language designed for embedding. |
| Roto | [`roto`][c~roto~docs]↗{{hi:roto}} | Static | Domain-specific scripting for network routing applications. |
| JavaScript | [`boa`][c~boa~docs]↗{{hi:boa}}, [`deno_core`][c~deno_core~docs]↗{{hi:deno_core}} | Dynamic | Choose an engine or embed Deno's runtime core; host integration and permissions are your responsibility. |
| Python | [`pyo3`][c~pyo3~docs]↗{{hi:pyo3}} | Dynamic | Embeds or interoperates with Python; see [[python | Python]]. |
| Other JavaScript engines | [`quickjs-rs`][c~quickjs-rs~docs]↗{{hi:quickjs-rs}}, [`v8`][c~v8~docs]↗{{hi:v8}} | Dynamic | Bindings to QuickJS or Google's V8; check native build requirements and the maintenance state of the binding. |
| Python bindings | [`cpython`][c~cpython~docs]↗{{hi:cpython}} | Dynamic | Older CPython bindings; compare with PyO3 for new integrations. |
| WebAssembly runtimes | [`wasmtime`][c~wasmtime~docs]↗{{hi:wasmtime}}, [`wasmi`][c~wasmi~docs]↗{{hi:wasmi}} | Varies | Embed a WebAssembly runtime or interpreter when scripts can be compiled to WASM; sandboxing still depends on the host configuration. |

Define the script's permissions when you choose an engine. Make host callbacks
small and bound the work they perform separately from interpreter operations.
Use an explicit import policy and enable filesystem or network access only
for tasks that need it. For stronger isolation, run the interpreter under
separate process permissions and resource limits.

## Build a Language

For custom languages, see [[compilers | Compilers]], [[text-processing | Text Processing]], [[parsing | Parsing]], and [[parser-implementations | Parser Implementations]]. The [[rhai-ecosystem | Rhai ecosystem]] and [[external-process-management | External process management]] examples cover related workflows.

For parsing building blocks, see [`logos`][c~logos~docs]↗{{hi:logos}} for lexing, [`nom`][c~nom~docs]↗{{hi:nom}} and [`chumsky`][c~chumsky~docs]↗{{hi:chumsky}} for parser combinators, and [`pest`][c~pest~docs]↗{{hi:pest}} or [`lalrpop`][c~lalrpop~docs]↗{{hi:lalrpop}} for grammar-driven parsers. Procedural macro tooling such as [`quote`][c~quote~docs]↗{{hi:quote}} can help generate Rust syntax trees. The [`object`][c~object~docs]↗{{hi:object}} crate can read object and executable files and write object files or selected executable formats.

For a broader survey of languages that compile to WebAssembly, see [Awesome WebAssembly Languages][awesome-wasm-langs-typescript~repo]↗.

## Related Topics

- [[development-tools_ffi | Development Tools: FFI]].
- [[wasm | WebAssembly]] and [[wasm_standalone_runtimes | Standalone WebAssembly runtimes]].
- [[task-automation | Task Automation]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

[cargo-script-nightly]: https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#script
