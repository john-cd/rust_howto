<!-- TODO Reviewed ??-??-2026 -->

# Cross-compilation Tools

{{#include cross_compilation.incl.md}}

|  |  |
|---|---|
| Target triple | A target triple names the CPU architecture, vendor, operating system, and ABI, for example `aarch64-unknown-linux-gnu`. Install Rust's standard library for the target with `rustup target add`. |
| Linker and sysroot | For targets that need a native linker or system libraries, install a compatible cross toolchain and set a target-specific linker in `.cargo/config.toml`. |
| C and C++ dependencies | Build scripts using [`cc`][c~cc~docs]↗{{hi:cc}} need a cross C compiler and any target sysroot or libraries. Configure the compiler and archiver for the target. |
| Platform-specific code | Use conditional compilation [attributes][p~attributes] such as [`cfg`][book~rust-reference~conditional-compilation]↗{{hi:cfg}}, and test on the target when possible. |
| Higher-level tooling | Use [`cross`][c~cross~docs]↗{{hi:cross}} or [`cargo-zigbuild`][c~cargo-zigbuild~docs]↗{{hi:cargo-zigbuild}} to provide or simplify parts of the cross toolchain. |

## Cross-compile with `cargo` {#cross-compile-with-cargo}

Cargo can cross-compile a Rust crate when the target's Rust standard library, linker, and system libraries are available. A target triple identifies the output platform; it does not install those external tools.

```sh
# Install the Rust standard library for the target.
rustup target add aarch64-unknown-linux-gnu

# Build the current package for that target.
cargo build --target aarch64-unknown-linux-gnu
```

Add a target-specific linker when the host's default linker cannot produce that target's binaries:

```toml
# .cargo/config.toml
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"

[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
```

Install the matching cross compiler and target system libraries on the host. `RUSTFLAGS` can pass target-specific rustc options, but prefer Cargo's target-specific configuration when possible so flags do not unintentionally affect host build scripts or other targets. Crates with C or C++ build scripts also need a target-aware compiler; [`cc`][c~cc~docs]↗{{hi:cc}} honors target-specific compiler environment variables.

Common targets include `x86_64-unknown-linux-musl` for a musl-linked Linux executable, `aarch64-unknown-linux-gnu` for 64-bit ARM Linux, and `x86_64-pc-windows-gnu` for Windows GNU. The `wasm32-unknown-unknown` target is useful for WebAssembly modules, while `wasm32-wasip2` targets the WASI Preview 2 environment; each has a different runtime and system interface.

For example, after installing an x86_64 musl cross compiler, configure and build the musl target:

```toml
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"
```

```sh
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

## Cross-compile for Multiple Target OSes and Architectures {#cross-compilation}

[![cross][c~cross~docs~badge]][c~cross~docs]{{hi:cross}} [![cross~repo][c~cross~repo~badge]][c~cross~repo] [![cat~compilers][cat~compilers~badge]][cat~compilers]{{hi:Compilers}}

For a Cargo-compatible workflow, [`cross`][c~cross~docs]↗{{hi:cross}} uses Docker or Podman to provide target toolchains. [`cargo-zigbuild`][c~cargo-zigbuild~docs]↗{{hi:cargo-zigbuild}} uses Zig to simplify Linux cross-linking and supports selecting a glibc minimum version. These tools can reduce manual linker and sysroot setup, but they do not remove the need to check your dependencies and runtime target.

[`cross`][c~cross~repo]↗{{hi:cross}} simplify cross-compilation. It builds your Rust project for different target operating systems and architectures from a single development environment. It requires [`rustup`][rustup~website]↗{{hi:rustup}} and [`Docker`][docker~website]↗{{hi:docker}} or [`Podman`][podman~website]↗{{hi:podman}}.

```sh
cargo install cross --locked

# Optionally, if `cargo-binstall` is present, install the pre-built binary:
cargo binstall cross
```

[`cross`][c~cross~docs]↗{{hi:cross}} has the exact same CLI as [`cargo`][c~cargo~docs]↗{{hi:cargo}} but relies on [`docker`][docker~website]↗{{hi:docker}} or [`podman`][podman~website]↗{{hi:podman}}.

```sh
cross build --target aarch64-unknown-linux-gnu

cross test --target mips64-unknown-linux-gnuabi64

cross run --target aarch64-unknown-linux-gnu

cargo install --locked cargo-zigbuild
cargo zigbuild --target x86_64-unknown-linux-gnu.2.17
```

[![cross~wiki][c~cross~wiki~badge]][c~cross~wiki]

Use `#[cfg(target_os = "...")]` or related conditional-compilation attributes when implementation details differ by target. A target build alone does not prove the program works at runtime; run target-native tests or use an emulator or container where available.

For target setup, use Cargo's built-in support in this chapter. For additional Cargo plugins, see [cross-compilation with Cargo tools](../../development-tools_cargo-plugins/cross_compiling.md).

## Related Topics {#related-topics .skip}

- [[autocfg | Autocfg]].
- [[build_time_tooling | Build Time Tooling]].
- [[development-tools_build-utils | Development Tools: Build Utils]].
- [[hardware-support | Hardware Support]].

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}
