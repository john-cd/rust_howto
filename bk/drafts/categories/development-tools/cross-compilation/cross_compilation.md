# Cross-compilation Tools

{{#include cross_compilation.incl.md}}

|  |  |
|---|---|
| Build Systems | Use [cargo][p~cargo], [`xtask`]( ){{hi: }} (for managing complex builds) |
| Target Specification | Handled by [cargo][p~cargo] through target triples |
| C/C++ Dependencies | Often a source of complexity; [`cc`][c~cc~docs]↗{{hi:cc}} crate can help |
| Platform-Specific Code | Use conditional compilation [attributes][p~attributes] like [`cfg`][book~rust-reference~conditional-compilation]↗{{hi:cfg}} to manage platform-specific code |
| Foreign Function Interface (FFI) | Use [`std::ffi`](https://doc.rust-lang.org/std/ffi/index.html)↗{{hi:std::ffi}} (for interacting with C code, which might require cross-compilation) |

## Cross-compile with `cargo` {#cross-compile-with-cargo}

## Cross-compile for Multiple Target OSes and Architectures {#cross-compilation}

[![cross][c~cross~docs~badge]][c~cross~docs]{{hi:cross}} [![cross~github][c~cross~github~badge]][c~cross~github] [![cat~compilers][cat~compilers~badge]][cat~compilers]{{hi:Compilers}}

Cross-Compilation Tools include [`cross`][c~cross~docs]↗{{hi:cross}}, a popular tool that uses Docker to simplify cross-compilation.

[`cross`][c~cross~github]{{hi:cross}}↗ simplify cross-compilation. It builds your Rust project for different target operating systems and architectures from a single development environment. It requires [`rustup`][rustup~website]{{hi:rustup}}↗ and [`Docker`][docker~website]{{hi:docker}}↗ or [`Podman`][podman~website]{{hi:podman}}↗.

```sh
cargo install cross --git https://github.com/cross-rs/cross

# Optionally, if you have `cargo-binstall`, you can install the pre-built binary
cargo binstall cross
```

[`cross`][c~cross~docs]{{hi:cross}}↗ has the exact same CLI as [`cargo`][c~cargo~docs]{{hi:cargo}}↗ but relies on [`docker`][docker~website]{{hi:docker}}↗ or [`podman`][podman~website]{{hi:podman}}↗.

```sh
cross build --target aarch64-unknown-linux-gnu

cross test --target mips64-unknown-linux-gnuabi64

cross run --target aarch64-unknown-linux-gnu
```

[![cross~wiki][c~cross~wiki~badge]][c~cross~wiki]

Examples: [![cross~toml-example][c~cross~toml-example~badge]][c~cross~toml-example]

## Related Topics {#related-topics}

- [[autocfg | Autocfg]].
- [[build_time_tooling | Build Time Tooling]].
- [[development-tools_build-utils | Development Tools: Build Utils]].
- [[hardware-support | Hardware Support]].

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">
[cross_compilation: expand](https://github.com/john-cd/rust_howto/issues/240)
</div>
