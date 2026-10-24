<!-- TODO Reviewed ??-??-2026 -->

# Build Time Tooling

{{#include build_time_tooling.incl.md}}

This section covers "build-time" tooling, or code that is run prior to compiling a crate's source code. Conventionally, build-time code lives in a `build.rs`{{hi:build.rs}} file and is commonly referred to as a "build script". Common use cases include rust code generation and compilation of bundled C/C++/asm code. See [`crates.io`][crates.io~website]{{hi:crates.io}}'s [documentation on the matter][book~cargo~build-script]↗ for more information.

## Embed Build Information {#build-info}

The [`build-info`][c~build-info~docs]↗ and [`build-info-build`][c~build-info-build~docs]↗ crates separate build-time collection from the runtime API. A build script records the package, Git revision, build timestamp, compiler version, and selected dependency metadata. The generated information is embedded in the binary, so the application can report which build is running without shipping the build-time collector.

In `Cargo.toml`:

```toml
{{#include ../../../crates/cats/development_tools_build_utils/Cargo.toml:build-info}}
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

To accommodate scenarios where additional C, C++, or [assembly][p~assembly] is required in a project, the [`cc`][c~cc~docs]↗{{hi:cc}} crate offers a simple api for compiling bundled C/C++/asm code into static libraries (`.a`) that can be statically linked to by [`rustc`][book~rustc]↗.

The following example has some bundled C code (`src/hello.c`) that will be used from rust. Before compiling rust source code, the "build" file ([`build.rs`][book~cargo~build-script]↗{{hi:build.rs}}) specified in [`Cargo.toml`][book~cargo~cargo-toml]↗{{hi:Cargo.toml}} runs. Using the [cc][c~cc~docs]↗{{hi:cc}} crate, a static library file will be produced (in this case, `libhello.a`, see [`cc::Build::compile`][c~cc::Build::compile~docs]↗{{hi:cc::Build::compile}}) which can then be used from rust by declaring the external function signatures in an [`compile`][c~cc::Build::compile~docs]↗, which can then be used from rust by declaring the external function signatures in an [`extern` block][book~rust-reference~extern-blocks]↗ block.

Since the bundled C is very simple, only a single source file needs to be passed to [`cc::Build`][c~cc::Build~docs]↗{{hi:cc::Build}}. For more complex build requirements, [`cc::Build`][c~cc::Build~docs]↗{{hi:cc::Build}} offers a full suite of builder methods for specifying [`cc::Build::include`][c~cc::Build::include~docs]↗{{hi:cc::Build::include}} paths and extra compiler [`cc::Build::flag`][c~cc::Build::flag~docs]↗{{hi:cc::Build::flag}}.

### `Cargo.toml` {#cargo-toml .skip}

[`Cargo.toml`][book~cargo~cargo-toml]↗{{hi:Cargo.toml}}

```toml
[package]
...
build = "build.rs"

[build-dependencies]
cc = "1"

[dependencies]
error-chain = "0.11"
```

### `build.rs` {#build-rs .skip}

`build.rs`

```rust,editable
{{#include ../../../crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_static.rs:example}}
```

### `src/hello.c` {#hello-c .skip}

```c
#include <stdio.h>

void hello() {
    printf("Hello from C!\n");
}

void greet(const char* name) {
    printf("Hello, %s!\n", name);
}
```

### `src/main.rs` {#main-rs .skip}

```rust,editable
{{#include ../../../crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_static1.rs:example}}
```

## Compile and Link Statically to a Bundled C++ Library {#cpp}

[![cc][c~cc~docs~badge]][c~cc~docs]{{hi:cc}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}

[`cc`][c~cc~docs]↗{{hi:cc}}

Linking a bundled C++ library is very similar to linking a bundled C library. The two core differences when compiling and statically linking a bundled C++ library are specifying a C++ compiler via the builder method [`cc::Build::cpp`][c~cc::Build::cpp~docs]↗{{hi:cc::Build::cpp}} and preventing name mangling by the C++ compiler by adding the `extern "C"` section at the top of our C++ source file.

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
{{#include ../../../crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_cpp.rs:example}}
```

### `src/foo.cpp` (static C++) {#foo-cpp .skip}

```cpp
extern "C" {
    int multiply(int x, int y);
}

int multiply(int x, int y) {
    return x*y;
}
```

### `src/main.rs` (static C++) {#main-rs .skip}

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
{{#include ../../../crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_defines.rs:example}}
```

<div class="hidden">
This revision turns the commented build commands into the package's active build script, adds a real C implementation that uses the three preprocessor definitions, and tests that Rust reads the compiled values. The build script derives `VERSION` from the Cargo package metadata so the documentation does not promise a stale hard-coded version.
</div>

## Related Topics {#related-topics .skip}

- [Cargo build scripts][book~cargo~build-script].
- [Compiler feature detection with `autocfg`][ex~development-tools_build-utils~autocfg].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/921)

The new recipe collects Git, timestamp, compiler, and dependency metadata at build time and reads it from the binary at runtime.
</div>
