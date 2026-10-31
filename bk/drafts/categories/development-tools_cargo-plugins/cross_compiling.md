<!-- TODO Reviewed ??-??-2026 -->

# Cross-compilation

{{#include cross_compiling.incl.md}}

Cross-compiling builds an executable for a target different from the host. Start by choosing a Rust target triple and checking whether your dependencies need a target linker, system libraries, or a sysroot. A successful build does not prove that the executable works on the target; run it there or use a suitable emulator when possible.

## Install a Rust Target {#install-a-rust-target}

Install the target's Rust standard library with [`rustup`][rustup~documentation]↗{{hi:rustup}}, then ask [Cargo][p~cargo] to build for that triple:

```bash
rustup target add aarch64-unknown-linux-gnu
cargo build --target aarch64-unknown-linux-gnu
```

This works directly when the host has a compatible linker and the target's required native libraries. For cross C/C++ dependencies, install a target toolchain and configure its linker in `.cargo/config.toml`:

```toml
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```

## Use `cross` for Containerized Builds {#cross}

[![cross][c~cross~docs~badge]][c~cross~docs] [![cross~crates.io][c~cross~crates.io~badge]][c~cross~crates.io] [![cross~repo][c~cross~repo~badge]][c~cross~repo]{{hi:cross}}

[`cross`][c~cross~docs]↗{{hi:cross}} wraps Cargo in a container with a target toolchain and libraries. It uses Docker or Podman, so install and start one of those container engines first. The upstream [`cross-rs/cross`][c~cross~repo] repository documents supported targets and custom images.

```bash
cargo install cross --git https://github.com/cross-rs/cross --locked
cross build --target aarch64-unknown-linux-gnu
```

## Cross-compile Using `zig` as the Linker {#zig}

{{#badges cargo-zigbuild}}{{hi:Cargo}}{{hi:Zig}}

[`cargo-zigbuild`][c~cargo-zigbuild~docs]↗{{hi:cargo-zigbuild}} uses Zig as a linker for targets that normally need a platform-specific linker or sysroot. Its upstream [`rust-cross/cargo-zigbuild`][c~cargo-zigbuild~repo] repository documents the supported targets and Zig requirements.

```bash
cargo install --locked cargo-zigbuild
cargo zigbuild --target aarch64-unknown-linux-gnu
```

Both `cross` and `cargo-zigbuild` are maintained in their upstream GitHub repositories; check the repository releases and crates.io before choosing a version for a new build environment.

## Related Topics {#related-topics .skip}

For the Cargo target model, Rust standard-library components, and more linker configuration, see [cross-compilation with Cargo](../development-tools/cross-compilation/cross_compilation.md). Related topics include [build tools][p~code-building], [CI][p~testing], [embedded][p~embedded], [WebAssembly][webassembly~website] and [cross-platform development][p~cross-platform].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
