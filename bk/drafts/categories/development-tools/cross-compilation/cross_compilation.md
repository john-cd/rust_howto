<!-- TODO Reviewed ??-??-2026 -->

# Cross-compilation

{{#include cross_compilation.incl.md}}

| Recipe                                                                                   | What it covers                                                          |
| ---------------------------------------------------------------------------------------- | ----------------------------------------------------------------------- |
| [Choose and install a target][ex~cross-compilation~target-and-toolchain]                 | Target triples, Rust target libraries, native linkers, and sysroots.    |
| [Build for Linux targets][ex~cross-compilation~linux-targets]                            | Static musl Linux and AArch64 GNU/Linux.                                |
| [Build for Windows GNU][ex~cross-compilation~windows-gnu]                                | MinGW linker setup and a Windows GNU build.                             |
| [Build for WebAssembly][ex~cross-compilation~webassembly]                                | Browser or embedding-host WebAssembly and WASI Preview 2 components.    |
| [Handle native dependencies and platform code][ex~cross-compilation~native-dependencies] | Cross C/C++ toolchains, scoped Rust flags, and conditional compilation. |
| [Use cross-compilation plugins][ex~cross-compilation~cross-compilation]                  | Docker or Podman based `cross` and Zig based `cargo-zigbuild`.          |

## Choose and install a target {#target-and-toolchain}

Cargo builds for the host by default. To produce a binary for another platform, select a target triple with `--target` and install that target's Rust standard library. A triple identifies the architecture, vendor, operating system, and ABI; it does not install a native linker, sysroot, SDK, or runtime.

```bash
rustup show
rustup target add aarch64-unknown-linux-gnu
```

Install the target's native toolchain as well when the target needs a linker or system libraries. Cargo can select the linker per target in `.cargo/config.toml`:

```toml
# .cargo/config.toml
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```

## Build for Linux targets {#linux-targets}

For a musl-linked x86-64 Linux binary, install the Rust target and a matching musl cross linker such as `x86_64-linux-musl-gcc`. The command name varies by operating system and toolchain package.

```toml
# .cargo/config.toml
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"
```

```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

For 64-bit ARM Linux, install the AArch64 GNU cross compiler and its target sysroot, then build with the target triple:

```bash
rustup target add aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
```

## Build for Windows GNU {#windows-gnu}

The Windows GNU target uses the MinGW-w64 toolchain. Install the Rust target and a cross compiler that provides the target's headers and libraries:

```toml
# .cargo/config.toml
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
```

```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

The linker executable may have a different name on Windows hosts. Use the compiler supplied by the installed MinGW-w64 toolchain.

## Build for WebAssembly {#webassembly}

`wasm32-unknown-unknown` produces a WebAssembly module without a built-in operating system interface. The browser, embedding application, or other runtime supplies any host functions the program needs.

```bash
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
```

`wasm32-wasip2` targets the WASI Preview 2 component model. Use a runtime or component host that supports WASI Preview 2; it is a different execution environment from browser WebAssembly.

```bash
rustup target add wasm32-wasip2
cargo build --release --target wasm32-wasip2
```

## Handle native dependencies and platform code {#native-dependencies}

Crates with C or C++ build scripts need a cross compiler, target headers, and any required target libraries and sysroot. The [`cc`][c~cc~docs]↗{{hi:cc}} crate honors target-aware compiler and archiver settings; make sure those settings point to the same target selected for Cargo. A Rust target installation alone cannot supply native dependencies.

Use Cargo's target-specific `rustflags` for options that apply to one target. A global `RUSTFLAGS` value is passed to rustc invocations for multiple targets and can accidentally affect host build scripts or procedural macros:

```toml
# .cargo/config.toml
[target.aarch64-unknown-linux-gnu]
rustflags = ["-C", "target-cpu=generic"]
```

When implementation details differ by platform, use conditional compilation such as `#[cfg(target_os = "linux")]` or `#[cfg(target_arch = "aarch64")]`. Compile and, where possible, run tests for every supported target; a successful cross build does not verify runtime behavior.

## Use cross-compilation plugins {#cross-compilation}

[![cross][c~cross~docs~badge]][c~cross~docs]{{hi:cross}} [![cross~repo][c~cross~repo~badge]][c~cross~repo] [![cat~compilers][cat~compilers~badge]][cat~compilers]{{hi:Compilers}}

[`cross`][c~cross~docs]↗{{hi:cross}} uses Docker or Podman to provide target toolchains and has a Cargo-like CLI. [`cargo-zigbuild`][c~cargo-zigbuild~docs]↗{{hi:cargo-zigbuild}} uses Zig to simplify Linux cross-linking and can select a minimum glibc version. Both tools reduce some manual toolchain setup; choose the target, confirm dependency support, and validate the runtime separately.

```bash
cargo install cross --locked
cross build --target aarch64-unknown-linux-gnu
cargo install --locked cargo-zigbuild
cargo zigbuild --target x86_64-unknown-linux-gnu.2.17
```

For target setup, use Cargo's built-in support in this chapter. For additional Cargo plugins, see [cross-compilation with Cargo tools](../../development-tools_cargo-plugins/cross_compiling.md).

## Related Topics {#related-topics .skip}

- [[autocfg | Autocfg]].
- [[build_time_tooling | Build Time Tooling]].
- [[development-tools_build-utils | Development Tools: Build Utils]].
- [[hardware-support | Hardware Support]].

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}
