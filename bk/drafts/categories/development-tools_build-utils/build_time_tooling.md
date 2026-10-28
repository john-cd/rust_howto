<!-- TODO Reviewed ??-??-2026 -->

# Build Time Tooling

{{#include build_time_tooling.incl.md}}

Cargo compiles and runs a `build.rs`{{hi:build.rs}} program on the host before compiling the package for its target. Build scripts are useful for work that must happen during a build, such as compiling bundled native code, generating Rust source or resources, or emitting target-specific configuration. Cargo documents their [inputs, outputs, and directives][book~cargo~build-script]↗. Keep build scripts small and write generated files only to `OUT_DIR`.

## Embed Build Information {#build-info}

The [`build-info`][c~build-info~docs]↗ and [`build-info-build`][c~build-info-build~docs]↗ crates separate build-time collection from the runtime API. A build script records package, Git, timestamp, compiler, and dependency metadata. The runtime example reads that metadata from the binary.

Declare the runtime and build-time crates in their respective manifest sections:

```toml
{{#include ../../../crates/cats/development_tools_build_utils/Cargo.toml:build-info}}
```

```toml
[build-dependencies]
build-info-build = "0.0.47"
```

In `build.rs`:

```rust,editable
{{#include ../../../crates/cats/development_tools_build_utils/build.rs:build-info}}
```

In `examples/build_info/build_info.rs`:

```rust,editable
{{#include ../../../crates/cats/development_tools_build_utils/examples/build_info/build_info.rs:example}}
```

## Compile and Link Statically to a Bundled C Library {#cc}

[![cc][c~cc~docs~badge]][c~cc~docs]{{hi:cc}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}} [![cat~development-tools::build-utils][cat~development-tools::build-utils~badge]][cat~development-tools::build-utils]{{hi:Build utils}}

Use [`cc`][c~cc~docs]↗{{hi:cc}} as a build dependency to compile bundled C source and link the resulting static library into Rust targets. Its `Build::compile` method emits Cargo's native link metadata, including `cargo::rustc-link-lib`. Cargo supplies target-aware compiler settings to `cc`.

The build script below compiles `hello.c` into a static library. Rust code declares the C function in an [`extern` block][book~rust-reference~extern-blocks]↗ and calls it through the C ABI. The C example is compiled by the package's normal test build, so its behavior is checked with the Rust example.

Since the bundled C is very simple, only a single source file needs to be passed to [`cc::Build`][c~cc::Build~docs]↗{{hi:cc::Build}}. For more complex build requirements, [`cc::Build`][c~cc::Build~docs]↗{{hi:cc::Build}} offers a full suite of builder methods for specifying [`cc::Build::include`][c~cc::Build::include~docs]↗{{hi:cc::Build::include}} paths and extra compiler [`cc::Build::flag`][c~cc::Build::flag~docs]↗{{hi:cc::Build::flag}}.

### `Cargo.toml` {#cargo-toml .skip}

[`Cargo.toml`][book~cargo~cargo-toml]↗{{hi:Cargo.toml}}

```toml
[package]
build = "build.rs"

[build-dependencies]
cc = "1"
```

### `build.rs` {#build-rs .skip}

`build.rs`

```rust,editable
{{#include ../../../crates/cats/development_tools_build_utils/build.rs:C}}
```

### `examples/build_time_tooling/hello.c` {#hello-c .skip}

```c
{{#include ../../../crates/cats/development_tools_build_utils/examples/build_time_tooling/hello.c}}
```

### `examples/build_time_tooling/cc_bundled_static1.rs` {#c-main-rs .skip}

```rust,editable
{{#include ../../../crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_static1.rs:example}}
```

## Compile and Link Statically to a Bundled C++ Library {#cpp}

[![cc][c~cc~docs~badge]][c~cc~docs]{{hi:cc}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}

Linking a bundled C++ library follows the same pattern as C. Select C++ with [`cc::Build::cpp`][c~cc::Build::cpp~docs]↗{{hi:cc::Build::cpp}} and export functions with `extern "C"` so the Rust declaration and C++ symbol use the same unmangled name.

### `Cargo.toml` (static C++) {#cargo-toml .skip}

```toml
[package]
...
build = "build.rs"

[build-dependencies]
cc = "1"
```

### `build.rs` (static C++) {#build-rs-static-cpp .skip}

```rust,editable
{{#include ../../../crates/cats/development_tools_build_utils/build.rs:CPP}}
```

### `examples/build_time_tooling/foo.cpp` {#foo-cpp .skip}

```cpp
{{#include ../../../crates/cats/development_tools_build_utils/examples/build_time_tooling/foo.cpp}}
```

### `examples/build_time_tooling/cc_bundled_cpp1.rs` (static C++) {#cpp-main-rs .skip}

```rust,editable
{{#include ../../../crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_cpp1.rs:example}}
```

## Compile a C Library While Setting Custom Defines {#cc-custom-defines}

[![cc][c~cc~docs~badge]][c~cc~docs]{{hi:cc}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}

[`cc`][c~cc~docs]↗{{hi:cc}}

It is simple to build bundled C code with custom defines using [`cc::Build::define`][c~cc::Build::define~docs]↗{{hi:cc::Build::define}}.
The method accepts an optional value, so it can define macros such as `APP_NAME "foo"` or a value-less macro such as `WELCOME` (pass `None` for the latter). This example sets the name, the package version from `CARGO_PKG_VERSION`, and `WELCOME` in [`build.rs`][book~cargo~build-script]↗{{hi:build.rs}}. The Rust code calls functions in the compiled C library and prints those values. [Cargo][p~cargo] sets package [environment variables][book~cargo~env]↗ that build scripts can use for custom defines.

### `Cargo.toml` (custom defines) {#cargo-toml-custom-defines .skip}

```toml
[package]
...
version = "1.0.2"
build = "build.rs"

[build-dependencies]
cc = "1"
```

### `build.rs` (custom defines) {#build-rs-custom-defines .skip}

```rust,editable
{{#include ../../../crates/cats/development_tools_build_utils/build.rs:C_DEFINES}}
```

### `examples/build_time_tooling/cc_defines.c` {#cc-defines-c .skip}

```c
{{#include ../../../crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_defines.c:C_DEFINES}}
```

### Rust code that calls the C functions {#rust-cc-defines .skip}

```rust,editable
{{#include ../../../crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_defines1.rs:example}}
```

## Generate Rust Source in `OUT_DIR` {#out-dir-codegen}

Code generation can turn a checked-in schema, a source file, or other build input into Rust code. Write the generated file to Cargo's [`OUT_DIR`][book~cargo~build-script]↗, then include it from the crate with `include!`. Cargo preserves `OUT_DIR` between builds, so the script must write the complete output it needs instead of assuming that directory is empty.

```rust,editable
{{#include ../../../crates/cats/development_tools_build_utils/build.rs:out-dir-codegen}}
```

```rust,editable
{{#include ../../../crates/cats/development_tools_build_utils/examples/build_time_tooling/out_dir_codegen.rs:example}}
```

## Rebuild Inputs and Target Configuration {#rerun-if}

Emit `cargo::rerun-if-changed=PATH` for files or directories that affect generated output, and `cargo::rerun-if-env-changed=NAME` for relevant environment variables such as `PROTOC`. If no `rerun-if` instruction is emitted, Cargo conservatively scans the package for changes. Cargo itself tracks the build script and values referenced by `env!` in Rust source.

Build scripts run on the host, while `TARGET` and `CARGO_CFG_TARGET_*` describe the platform being built. Do not use `cfg!()` or `#[cfg]` in `build.rs` to choose target-specific behavior: those check the host. Read Cargo's `CARGO_CFG_TARGET_OS`, `CARGO_CFG_TARGET_ARCH`, or another `CARGO_CFG_*` value instead. See [cross-compilation][ex~development-tools_cargo-plugins~cross-compiling] for target linker setup.

## Related Topics {#related-topics .skip}

- [Foreign Function Interfaces (FFI)][ex~links~ffi] and [cross-compilation][ex~development-tools_cargo-plugins~cross-compiling].
- [Compiler feature detection with `autocfg`][ex~development-tools_build-utils~autocfg].
- [`winres`][c~winres~docs]↗{{hi:winres}} generates Windows executable resources. [`protoc-bin-vendored`][c~protoc-bin-vendored~docs]↗{{hi:protoc-bin-vendored}} supplies a host `protoc` binary for Protocol Buffers code generation. Add either as a build dependency only when the package needs it.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
