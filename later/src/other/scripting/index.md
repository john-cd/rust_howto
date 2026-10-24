<!-- TODO Reviewed ??-??-2026 -->
# Scripting and Rust

Scripting in Rust has two distinct forms: running Rust source as a lightweight script, and embedding another language in a Rust application. Choose based on startup time, deployment, host integration, and the trust level of the code being run.

## Run Rust Source as a Script

Cargo's single-file script support uses frontmatter and is currently an unstable nightly feature. Its command-line form is `cargo +nightly -Zscript script.rs`; check the [Cargo unstable feature documentation][cargo-script-nightly] before relying on it. [`rust-script`][c~rust-script~repo]↗{{hi:rust-script}} is another script runner.

```rust
---
[dependencies]
anyhow = "1"
---

fn main() -> anyhow::Result<()> {
    println!("Hello from one Rust source file");
    Ok(())
}
```

## Embed a Scripting Language

| Language | Crate | Typing | Sandboxing and embedding notes |
|---|---|---|---|
| Rhai | [`rhai`][c~rhai~docs]↗{{hi:rhai}} | Dynamic | Rust-oriented syntax; configure operation and data limits for untrusted scripts. |
| Lua | [`mlua`][c~mlua~docs]↗{{hi:mlua}} | Dynamic | Bindings for Lua and Luau; select a runtime and features explicitly. See [[lua | Lua]]. |
| Rune | [`rune`][c~rune~docs]↗{{hi:rune}} | Dynamic | Embeddable scripting language with host functions and modules. |
| Koto | [`koto`][c~koto~docs]↗{{hi:koto}} | Dynamic | Small language designed for embedding. |
| Roto | [`roto`][c~roto~docs]↗{{hi:roto}} | Static | Domain-specific scripting for network routing applications. |
| JavaScript | [`boa`][c~boa~docs]↗{{hi:boa}}, [`deno_core`][c~deno_core~docs]↗{{hi:deno_core}} | Dynamic | Choose an engine or embed Deno's runtime core; host integration and permissions are your responsibility. |
| Python | [`pyo3`][c~pyo3~docs]↗{{hi:pyo3}} | Dynamic | Embeds or interoperates with Python; see [[python | Python]]. |

An embedded interpreter is not automatically a security boundary. Restrict exposed host functions, filesystem and network access, execution time, and memory according to the threat model.

## Build a Language

For custom languages, see [[compilers | Compilers]], [[text-processing | Text Processing]], [[parsing | Parsing]], and [[parser-implementations | Parser Implementations]]. The [[rhai-ecosystem | Rhai ecosystem]] and [[external-process-management | External process management]] examples cover related workflows.

## Related Topics

- [[development-tools_ffi | Development Tools: FFI]].
- [[wasm | WebAssembly]] and [[wasm_standalone_runtimes | Standalone WebAssembly runtimes]].
- [[task-automation | Task Automation]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

[cargo-script-nightly]: https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#script

<div class="hidden">
[content audit: category index content removed by recent commits; review and restore](https://github.com/john-cd/rust_howto/issues/1903)
</div>
