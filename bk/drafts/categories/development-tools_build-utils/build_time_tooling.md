# Build Time Tooling

{{#include build_time_tooling.incl.md}}

This section covers "build-time" tooling, or code that is run prior to compiling a crate's source code. Conventionally, build-time code lives in a `build.rs`{{hi:build.rs}} file and is commonly referred to as a "build script". Common use cases include rust code generation and compilation of bundled C/C++/asm code. See [`crates.io`][crates.io~website]{{hi:crates.io}}'s [documentation on the matter][book~cargo~build-script]↗ for more information.

## Compile and Link Statically to a Bundled C Library {#cc}

[![cc][c~cc~docs~badge]][c~cc~docs]{{hi:cc}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}} [![cat~development-tools::build-utils][cat~development-tools::build-utils~badge]][cat~development-tools::build-utils]{{hi:Build utils}}

To accommodate scenarios where additional C, C++, or [assembly][p~assembly] is required in a project, the [`cc`][c~cc~docs]{{hi:cc}}↗ crate offers a simple api for compiling bundled C/C++/asm code into static libraries (`.a`) that can be statically linked to by [`rustc`][book~rustc].

The following example has some bundled C code (`src/hello.c`) that will be used from rust. Before compiling rust source code, the "build" file ([`build.rs`](https://doc.rust-lang.org/cargo/reference/build-scripts.html)↗{{hi:build.rs}}) specified in [`Cargo.toml`][book~cargo~cargo-toml]↗{{hi:Cargo.toml}} runs. Using the [cc][c~cc~docs]{{hi:cc}}↗ crate, a static library file will be produced (in this case, `libhello.a`, see [`cc::Build::compile`][c~cc::Build::compile~docs]{{hi:cc::Build::compile}}↗) which can then be used from rust by declaring the external function signatures in an [`compile`][c~cc::Build::compile~docs]↗, which can then be used from rust by declaring the external function signatures in an [`extern` block][book~rust-reference~extern-blocks]↗ block.

Since the bundled C is very simple, only a single source file needs to be passed to [`cc::Build`][c~cc::Build~docs]{{hi:cc::Build}}↗. For more complex build requirements, [`cc::Build`][c~cc::Build~docs]{{hi:cc::Build}}↗ offers a full suite of builder methods for specifying [`cc::Build::include`][c~cc::Build::include~docs]{{hi:cc::Build::include}}↗ paths and extra compiler [`cc::Build::flag`][c~cc::Build::flag~docs]{{hi:cc::Build::flag}}s↗.

### `Cargo.toml` {#skip1}

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

### `build.rs` {#skip2}

`build.rs`

```rust,editable
{{#include ../../../crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_static.rs:example}}
```

### `src/hello.c` {#skip3}

```c
#include <stdio.h>

void hello() {
    printf("Hello from C!\n");
}

void greet(const char* name) {
    printf("Hello, %s!\n", name);
}
```

### `src/main.rs` {#skip4}

```rust,editable
{{#include ../../../crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_static1.rs:example}}
```

## Compile and Link Statically to a Bundled C++ Library {#cpp}

[![cc][c~cc~docs~badge]][c~cc~docs]{{hi:cc}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}

[`cc`][c~cc~docs]↗{{hi:cc}}

Linking a bundled C++ library is very similar to linking a bundled C library. The two core differences when compiling and statically linking a bundled C++ library are specifying a C++ compiler via the builder method [`cc::Build::cpp`][c~cc::Build::cpp~docs]{{hi:cc::Build::cpp}}↗ and preventing name mangling by the C++ compiler by adding the `extern "C"` section at the top of our C++ source file.

### `Cargo.toml` (static C++) {#skip5}

```toml
[package]
...
build = "build.rs"

[build-dependencies]
cc = "1"
```

### `build.rs` (static C++) {#skip6}

```rust,editable
{{#include ../../../crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_cpp.rs:example}}
```

### `src/foo.cpp` (static C++) {#skip7}

```cpp
extern "C" {
    int multiply(int x, int y);
}

int multiply(int x, int y) {
    return x*y;
}
```

### `src/main.rs` (static C++) {#skip8}

```rust,editable
{{#include ../../../crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_bundled_cpp1.rs:example}}
```

## Compile a C Library While Setting Custom Defines {#cc-custom-defines}

[![cc][c~cc~docs~badge]][c~cc~docs]{{hi:cc}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}

[`cc`][c~cc~docs]↗{{hi:cc}}

It is simple to build bundled C code with custom defines using [`cc::Build::define`][c~cc::Build::define~docs]{{hi:cc::Build::define}}↗.
The method takes an [`std::option::Option`][c~std::option::Option~docs]{{hi:std::option::Option}}↗ value, so it is possible to create defines such as `#define APP_NAME "foo"`
as well as `#define WELCOME` (pass [`std::option::Option::None`][c~std::option::Option::None~docs]{{hi:std::option::Option::None}}↗ as the value for a value-less define). This example builds
a bundled C file with dynamic defines set in `build.rs` and prints "`Welcome to foo - version 1.0.2`"
when run. [Cargo][p~cargo] sets some [environment variables][book~cargo~env]↗ which may be useful for some custom defines.

### `Cargo.toml` (custom defines) {#skip9}

```toml
[package]
...
version = "1.0.2"
build = "build.rs"

[build-dependencies]
cc = "1"
```

### `build.rs` (custom defines) {#skip10}

```rust,editable
{{#include ../../../crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_defines.rs:example}}
```

### `src/foo.c` (custom defines) {#skip11}

```c
#include <stdio.h>

void print_app_info() {
#ifdef WELCOME
    printf("Welcome to ");
#endif
    printf("%s - version %s\n", APP_NAME, VERSION);
}
```

### `src/main.rs` (custom defines) {#skip12}

```rust,editable
{{#include ../../../crates/cats/development_tools_build_utils/examples/build_time_tooling/cc_defines1.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/921)
</div>
