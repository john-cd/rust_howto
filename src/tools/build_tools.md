# Build Time Tooling

This section covers "build-time" tooling, or code that is run prior to compiling a crate's source code.
Conventionally, build-time code lives in a **build.rs** file and is commonly referred to as a "build script".
Common use cases include rust code generation and compilation of bundled C/C++/asm code.
See crates.io's [documentation on the matter][build-script-docs] for more information.

## Compile and link statically to a bundled C library

[![cc-badge]][cc] [![cat-development-tools-badge]][cat-development-tools]

To accommodate scenarios where additional C, C++, or assembly is required in a project, the [**cc**][cc] crate
offers a simple api for compiling bundled C/C++/asm code into static libraries (**.a**) that can be statically linked to by **rustc**.

The following example has some bundled C code (**src/hello.c**) that will be used from rust.
Before compiling rust source code, the "build" file (**build.rs**) specified in **Cargo.toml** runs.
Using the [**cc**][cc] crate, a static library file will be produced (in this case, **libhello.a**, see
`[compile` docs][cc-build-compile]) which can then be used from rust by declaring the external function signatures in an `extern` block.

Since the bundled C is very simple, only a single source file needs to be passed to `[cc::Build]` cc-build].
For more complex build requirements, `[cc::Build]` cc-build] offers a full suite of builder methods for specifying
`[include]` cc-build-include] paths and extra compiler `[flag]` cc-build-flag]s.

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

```rust,editable,no_run
{#include ../../deps/examples/cc-bundled-static.rs}
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

```rust,editable,ignore
{#include ../../deps/examples/cc-bundled-static2.rs}
```

## Compile and link statically to a bundled C++ library

[![cc-badge]][cc] [![cat-development-tools-badge]][cat-development-tools]

Linking a bundled C++ library is very similar to linking a bundled C library. The two core differences when compiling and statically linking a bundled C++ library are specifying a C++ compiler via the builder method `[cpp(true)]` cc-build-cpp] and preventing name mangling by the C++ compiler by adding the `extern "C"` section at the top of our C++ source file.

### `Cargo.toml`

```toml
[package]
...
build = "build.rs"

[build-dependencies]
cc = "1"
```

### `build.rs`

```rust,editable,no_run
{#include ../../deps/examples/cc-bundled-cpp.rs}
```

### `src/foo.cpp`

```cpp
extern "C" {
    int multiply(int x, int y);
}

int multiply(int x, int y) {
    return x*y;
}
```

### `src/main.rs`

```rust,editable,ignore
{#include ../../deps/examples/cc-bundled-cpp2.rs}
```

[cc-build-cpp]: https://docs.rs/cc/*/cc/struct.Build.html#method.cpp

## Compile a C library while setting custom defines

[![cc-badge]][cc] [![cat-development-tools-badge]][cat-development-tools]

It is simple to build bundled C code with custom defines using `[cc::Build::define]`
The method takes an `[Option]` value, so it is possible to create defines such as `#define APP_NAME "foo"`
as well as `#define WELCOME` (pass `None` as the value for a value-less define). This example builds
a bundled C file with dynamic defines set in `build.rs` and prints "**Welcome to foo - version 1.0.2**"
when run. Cargo sets some [environment variables][cargo-env] which may be useful for some custom defines.

### `Cargo.toml`

```toml
[package]
...
version = "1.0.2"
build = "build.rs"

[build-dependencies]
cc = "1"
```

### `build.rs`

```rust,editable,no_run
{#include ../../deps/examples/cc-defines.rs}
```

### `src/foo.c`

```c
#include <stdio.h>

void print_app_info() {
#ifdef WELCOME
    printf("Welcome to ");
#endif
    printf("%s - version %s\n", APP_NAME, VERSION);
}
```

### `src/main.rs`

```rust,editable,ignore
{#include ../../deps/examples/cc-defines2.rs}
```

[cc::Build::define]: https://docs.rs/cc/*/cc/struct.Build.html#method.define
[Option]: https://doc.rust-lang.org/std/option/enum.Option.html
[cc-build-compile]: https://docs.rs/cc/*/cc/struct.Build.html#method.compile
[cc-build-flag]: https://docs.rs/cc/*/cc/struct.Build.html#method.flag
[cc-build-include]: https://docs.rs/cc/*/cc/struct.Build.html#method.include
[cc-build]: https://docs.rs/cc/*/cc/struct.Build.html
[cargo-env]: https://doc.rust-lang.org/cargo/reference/environment-variables.html
[build-script-docs]: http://doc.crates.io/build-script.html
{{#include ../refs/link-refs.md}}
