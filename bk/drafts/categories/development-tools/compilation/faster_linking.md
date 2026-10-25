<!-- TODO Reviewed ??-??-2026 -->

# Faster Linking {#faster-linking}

{{#include faster_linking.incl.md}}

[![cat~compilers][cat~compilers~badge]][cat~compilers]{{hi:Compilers}}

Linking can be a significant part of a Rust build, especially after an incremental compile. Measure before changing linkers, and compare the full build rather than link time alone. The [compilation duration guide](reduce_compilation_duration.md) covers build timing and other ways to shorten iteration.

## Rust's Default Linux Linker {#rust-lld}

Starting with stable Rust 1.90, `rustc` uses `rust-lld` by default for the `x86_64-unknown-linux-gnu` target ([announcement][blog~rust-lld~1.90]↗). This default applies to that Linux x86-64 GNU target; it does not select the linker for macOS, Windows, or every other Linux target. Most projects on the covered target need no configuration. If a project encounters an incompatibility, opt out for that target with:

```toml
# .cargo/config.toml
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "linker-features=-lld"]
```

## Use LLD with Clang {#lld-linker}

LLVM's [`lld`][lld~website]↗{{hi:lld}} has ELF, COFF, and Mach-O variants for supported Linux, Windows, and Apple targets. Install the matching linker and Clang driver for the target. For a Linux ELF target that does not use Rust's default `rust-lld`, configure Cargo to pass the LLD selection through Clang:

```toml
# .cargo/config.toml
[target.aarch64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
```

Use a linker and target triple supported by the host toolchain. On Windows, use the LLVM linker support appropriate to the MSVC or GNU target; the Linux `-fuse-ld` example does not apply unchanged.

## Use mold on Linux {#mold-linker}

[`mold`][mold~repo]↗{{hi:mold}} is a fast ELF linker for Linux. Install it and Clang with your system package manager, then select its path in Cargo:

```toml
# .cargo/config.toml
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=/usr/bin/mold"]
```

Adjust the path for your installation. `mold` does not target macOS or Windows.

## Try Wild on Linux {#wild-linker}

[`wild`][wild~repo]↗{{hi:wild}} is an actively developed linker focused on fast iterative builds. It targets Linux, with support centered on x86-64, AArch64, RISC-V, and LoongArch; check its current support matrix for the exact architecture and binary features you need. Treat it as an additional toolchain component rather than a drop-in choice for every project.

```sh
cargo install --locked wild-linker
```

```toml
# .cargo/config.toml
[target.x86_64-unknown-linux-gnu]
linker = "wild"
```

## Use the Apple Linker on macOS {#apple-linker}

Xcode 15 introduced Apple's newer linker (`ld-prime`), selected by Xcode for supported macOS, iOS, and other Apple platform binaries ([Xcode 15 release notes][apple~xcode-15~release-notes]↗). Rust projects using Apple targets normally need no linker override: install the matching Xcode or Command Line Tools and let Clang use its selected linker. If an older project needs the classic linker for compatibility, pass Apple's `-ld_classic` option:

```toml
# .cargo/config.toml
[target.aarch64-apple-darwin]
rustflags = ["-C", "link-arg=-Wl,-ld_classic"]
```

The [`zld`][zld~repo]↗ project is archived. [`sold`][sold~repo]↗ is archived and its maintainers point users to Apple's Xcode linker. The standalone [`mold-macho`][mold-macho~repo]↗ repository was archived after its macOS work moved into the main `mold` project; check the upstream project for its current macOS support instead of treating that archived repository as a separate maintained linker.

## Verify the selected linker {#verify-linker}

Build with verbose Cargo output to inspect the linker command and confirm that the configured linker is actually selected:

```sh
cargo build -vv
```

When comparing linkers, use the same clean or incremental build scenario and compiler settings, and record the complete build time. Different linkers can produce different diagnostics or binaries, so run the project's tests and inspect the result on the target platform.

## References {#references .skip}

- [Enable Fast Compiles (Bevy)][c~bevy~enable-fast-compiles]↗{{hi:bevy}}.

## Related Topics {#related-topics .skip}

- [[development-tools | Development Tools]].
- [[development-tools_build-utils | Development Tools: Build Utils]].
- [[development-tools_cargo-plugins | Development Tools: Cargo Plugins]].
- [[development-tools_debugging | Development Tools" Debugging]].
- [[development-tools_ffi | Development Tools: FFI]].
- [[development-tools_procedural-macro-helpers | Development Tools Procedural Macro Helpers]].
- [[development-tools_testing | Development Tools Testing]].
- [[performance | Performance]].

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}
