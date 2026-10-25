<!-- TODO Reviewed ??-??-2026 -->

# Reduce Compilation Duration

{{#include reduce_compilation_duration.incl.md}}

Rust compile times can dominate the edit-build-test loop. Measure the build you care about first, then change one setting at a time. Build performance depends on the target, features, dependencies, compiler version, and whether the artifacts are warm.

## Measure Build Times {#build-time}

[![cargo~website][c~cargo~website~badge]][c~cargo~website] [![cat~compilers][cat~compilers~badge]][cat~compilers]{{hi:Compilers}}

```bash
cargo check
```

[`cargo check`][book~cargo~check]↗ type-checks a package without generating an executable or linking it, so it is useful when the next step does not need a runnable binary. For full builds, use Cargo's timing report:

```bash
cargo build --timings
```

Cargo writes an HTML report under `target/cargo-timings`. Compare clean builds with clean builds and incremental builds with incremental builds, using the same target and feature set. A configuration that helps one workload may slow another. For linker-specific measurements, see [Faster Linking](faster_linking.md).

You may also use [`hyperfine`][c~hyperfine~docs]↗{{hi:hyperfine}}. See [[benchmarking | Benchmarking]].

## Incremental Compilation {#incremental-compilation}

The `dev` profile enables incremental compilation by default. Cargo reuses artifacts after source changes, but changes to dependencies, features, build scripts, compiler settings, or broad shared modules can invalidate substantial work. Keep frequently changed code separate from stable code where that makes the dependency structure clearer.

Incremental compilation has storage and build-time trade-offs. Keep it enabled for repeated local builds, and measure before disabling it in continuous integration. A CI job that starts with a warm build cache may benefit from incremental artifacts; a clean one-shot job may not.

For reusable compiler artifacts across clean builds, [`sccache`][c~sccache~docs]↗ can act as `RUSTC_WRAPPER`. Install it with `cargo install sccache --locked`, then set the wrapper when building:

```bash
RUSTC_WRAPPER=sccache cargo build
```

In Windows PowerShell, set `$env:RUSTC_WRAPPER = "sccache"` before running Cargo. Cache hits require matching compiler inputs and settings, and sccache does not replace Cargo's local target directory. Check its cache statistics with `sccache --show-stats` when a build does not get the expected hits.

## Project Structure and Dependencies {#project-structure-and-dependencies .skip}

Every dependency and enabled feature adds code that may need to be resolved, compiled, and linked. Inspect the dependency graph and feature selection before replacing crates or changing workspace structure:

```bash
cargo tree -e features
cargo tree --duplicates
```

Remove unused dependencies and disable unneeded optional features when their behavior is not required. [`cargo-udeps`][c~cargo-udeps~repo]↗ can help find unused dependencies, but it requires nightly Rust and its results still need manual review. Keep shared, stable code in separate crates only when the ownership boundary is useful; splitting crates adds build and maintenance overhead of its own.

```bash
cargo install cargo-udeps --locked
cargo +nightly udeps
```

## Compiler-Level Optimizations {#compiler-level-optimizations .skip}

Compiler settings can trade build time for executable size, runtime speed, or debugging detail. Development builds usually favor low compile time. Avoid LTO during iteration unless you are measuring the release artifact; full LTO can substantially increase build time. Thin LTO may cost less, but is still generally a release-build choice. Increasing codegen units can expose more parallel work, though the defaults are often reasonable.

Profile-guided optimization is a runtime optimization workflow, not a way to shorten the edit-build loop. Use it for release performance work when measurements justify the additional builds.

## Optimize Development Profile {#optimization-levels}

[![cargo~website][c~cargo~website~badge]][c~cargo~website] [![cat~compilers][cat~compilers~badge]][cat~compilers]{{hi:Compilers}}

Place profile settings in the workspace root `Cargo.toml` or a developer-specific Cargo configuration. This example reduces debug information and applies a small optimization level to dependencies; measure the effect on your project and target:

```toml
[profile.dev]
debug = "line-tables-only"
split-debuginfo = "unpacked"

# Try this only when faster dependency runtime offsets the slower optimized build.
[profile.dev.package."*"]
opt-level = 1
```

Cargo defaults to `opt-level = 0` for development code and dependencies. Optimizing dependencies can reduce runtime in tests or build scripts, but usually increases their compile time. `split-debuginfo` defaults vary by platform, so use a value supported by the target and debugger. For other debuginfo trade-offs, see the [Cargo profiles reference][book~cargo~profiles]↗.

## Use Dynamic Linking {#dynamic-linking}

[![cargo-add-dynamic][c~cargo-add-dynamic~docs~badge]][c~cargo-add-dynamic~docs] [![cargo-add-dynamic~crates.io][c~cargo-add-dynamic~crates.io~badge]][c~cargo-add-dynamic~crates.io] [![cargo-add-dynamic~repo][c~cargo-add-dynamic~repo~badge]][c~cargo-add-dynamic~repo] [![cargo-add-dynamic~lib.rs][c~cargo-add-dynamic~lib.rs~badge]][c~cargo-add-dynamic~lib.rs]{{hi:cargo-add-dynamic}}{{hi:Cargo}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}} [![cat~compilers][cat~compilers~badge]][cat~compilers]{{hi:Compilers}}

Dynamic linking can let multiple local binaries share dependencies, but it adds deployment and loader-path considerations and is not a general replacement for Cargo's default static Rust dependency linking. Treat it as a local development optimization and verify the resulting binaries in the environments where they will run.

```bash
cargo install cargo-add-dynamic
cargo add-dynamic polars --features csv-file,lazy,list,describe,rows,fmt,strings,temporal
cargo build
```

See also: [Speeding up incremental Rust compilation with dylibs][blog~speeding-up-incremental-rust-compilation]↗.

## Experimental Nightly Features {#nightly-features}

These compiler options are unstable, require nightly Rust, and may change. Keep them in a local configuration or a separate toolchain file until the project has verified that they work for its targets and dependencies.

### Cranelift code generation

The Cranelift backend can generate development code faster than LLVM on supported targets, at the cost of less optimized runtime code and incomplete feature support. Install the nightly component, enable Cargo's unstable backend option, then select it for the `dev` profile:

```bash
rustup component add rustc-codegen-cranelift-preview --toolchain nightly
cargo +nightly build -Z codegen-backend
```

```toml
# .cargo/config.toml
[unstable]
codegen-backend = true

[profile.dev]
codegen-backend = "cranelift"
```

### Parallel compiler frontend

The nightly `-Z threads` option enables parallel frontend work. Start with a modest thread count and measure CPU and memory use; more threads do not guarantee a faster build.

```toml
# .cargo/config.toml
[build]
rustflags = ["-Z", "threads=8"]
```

Run this configuration with `cargo +nightly build`. Do not add `-Z` flags to a stable toolchain's shared configuration.

## Build Machine Hardware Considerations {#build-machine-hardware-considerations .skip}

Build your code with the right machine:

- A multi-core CPU significantly speeds up compilation, especially when using multiple codegen units.
- Sufficient RAM is essential, especially when using LTO or compiling large projects.
- A fast SSD can greatly reduce I/O bottlenecks during compilation.

Consider using remote build servers or a separate build machine for large projects.

## References and Related Topics {#references-and-related-topics .skip}

### References {#references}

- [Cargo build performance guide][book~cargo~build-performance]↗.
- [Cargo profile reference][book~cargo~profiles]↗.
- [Eight solutions for troubleshooting your Rust build times][blog~rust-build-times]↗.
- [How I improved my Rust compile times by seventy-five percent][blog~how-i-improved-my-rust-compile-times-by-seventy-five-percent]↗.
- [Rust compilation time][blog~rust-compilation-time]↗.

### Related Topics {#related-topics}

- [[benchmarking | Benchmarking]].
- [[build_utils | Build Utils]].
- [[development-tools | Development Tools]].
- [[development-tools_build-utils | Development Tools: Build Utils]].
- [[development-tools_cargo-plugins | Development Tools: Cargo Plugins]].
- [[development-tools_profiling | Development Tools: Profiling]].
- [[faster_linking | Faster Linking]].
- [[performance | Performance]].

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}
