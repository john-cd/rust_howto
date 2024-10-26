# Build Time Tooling

Utilities for build scripts and other build time steps.

This section covers "build-time" tooling, or code that is run prior to compiling a crate's source code. Conventionally, build-time code lives in a `build.rs` file and is commonly referred to as a "build script". Common use cases include rust code generation and compilation of bundled C/C++/asm code. See crates.io's [documentation on the matter][book-cargo-build-script]⮳ for more information.

{{#include index.incl.md}}

## Compile and link statically to a bundled C library

[![cc][c-cc-badge]][c-cc]{{hi:cc}}  [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]  [![cat-development-tools::build-utils][cat-development-tools::build-utils-badge]][cat-development-tools::build-utils]

To accommodate scenarios where additional C, C++, or assembly is required in a project, the [`cc`][c-cc]{{hi:cc}}⮳ crate offers a simple api for compiling bundled C/C++/asm code into static libraries (`.a`) that can be statically linked to by [`rustc`][rustc].

The following example has some bundled C code (`src/hello.c`) that will be used from rust. Before compiling rust source code, the "build" file (`build.rs`) specified in `Cargo.toml` runs. Using the [cc][c-cc]{{hi:cc}}⮳ crate, a static library file will be produced (in this case, `libhello.a`, see [`cc::Build::compile`][c-cc::Build::compile]{{hi:cc::Build::compile}}⮳) which can then be used from rust by declaring the external function signatures in an [`compile`][c-cc::Build::compile]⮳, which can then be used from rust by declaring the external function signatures in an [extern block][book-rust-reference-extern-blocks]⮳ block.

Since the bundled C is very simple, only a single source file needs to be passed to [`cc::Build`][c-cc::Build]{{hi:cc::Build}}⮳. For more complex build requirements, [`cc::Build`][c-cc::Build]{{hi:cc::Build}}⮳ offers a full suite of builder methods for specifying [`cc::Build::include`][c-cc::Build::include]{{hi:cc::Build::include}}⮳ paths and extra compiler [`cc::Build::flag`][c-cc::Build::flag]{{hi:cc::Build::flag}}s⮳.

### `Cargo.toml`

```toml
[package]
...
build = "build.rs"

[build-dependencies]
cc = "1"

[dependencies]
error-chain = "0.11"
```

### `build.rs`

```rust,no_run
{{#include ../../../deps/tests/cc-bundled-static.rs}}
```

### `src/hello.c`

```c
#include <stdio.h>

void hello() {
    printf("Hello from C!\n");
}

void greet(const char* name) {
    printf("Hello, %s!\n", name);
}
```

### `src/main.rs`

```rust,ignore
{{#include ../../../deps/tests/cc-bundled-static1.rs}}
```

## Compile and link statically to a bundled C++ library

[![cc][c-cc-badge]][c-cc]{{hi:cc}}  [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]

Linking a bundled C++ library is very similar to linking a bundled C library. The two core differences when compiling and statically linking a bundled C++ library are specifying a C++ compiler via the builder method [`cc::Build::cpp`][c-cc::Build::cpp]{{hi:cc::Build::cpp}}⮳ and preventing name mangling by the C++ compiler by adding the `extern "C"` section at the top of our C++ source file.

### `Cargo.toml` (static C++)

```toml
[package]
...
build = "build.rs"

[build-dependencies]
cc = "1"
```

### `build.rs` (static C++)

```rust,no_run
{{#include ../../../deps/tests/cc-bundled-cpp.rs}}
```

### `src/foo.cpp` (static C++)

```cpp
extern "C" {
    int multiply(int x, int y);
}

int multiply(int x, int y) {
    return x*y;
}
```

### `src/main.rs` (static C++)

```rust,ignore
{{#include ../../../deps/tests/cc-bundled-cpp1.rs}}
```

## Compile a C library while setting custom defines

[![cc][c-cc-badge]][c-cc]{{hi:cc}}  [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]

It is simple to build bundled C code with custom defines using [`cc::Build::define`][c-cc::Build::define]{{hi:cc::Build::define}}⮳
The method takes an [`std::option::Option`][c-std::option::Option]{{hi:std::option::Option}}⮳ value, so it is possible to create defines such as `#define APP_NAME "foo"`
as well as `#define WELCOME` (pass [`std::option::Option::None`][c-std::option::Option::None]{{hi:std::option::Option::None}}⮳ as the value for a value-less define). This example builds
a bundled C file with dynamic defines set in `build.rs` and prints "`Welcome to foo - version 1.0.2`"
when run. Cargo sets some [environment variables][book-cargo-env]⮳ which may be useful for some custom defines.

### `Cargo.toml` (custom defines)

```toml
[package]
...
version = "1.0.2"
build = "build.rs"

[build-dependencies]
cc = "1"
```

### `build.rs` (custom defines)

```rust,no_run
{{#include ../../../deps/tests/cc-defines.rs}}
```

### `src/foo.c` (custom defines)

```c
#include <stdio.h>

void print_app_info() {
#ifdef WELCOME
    printf("Welcome to ");
#endif
    printf("%s - version %s\n", APP_NAME, VERSION);
}
```

### `src/main.rs` (custom defines)

```rust,ignore
{{#include ../../../deps/tests/cc-defines1.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
