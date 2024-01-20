## Compile and link statically to a bundled C++ library

[![cc-badge]][cc] [![cat-development-tools-badge]][cat-development-tools]

Linking a bundled C++ library is very similar to linking a bundled C library. The two core differences when compiling and statically linking a bundled C++ library are specifying a C++ compiler via the builder method [`cpp(true)`][cc-build-cpp] and preventing name mangling by the C++ compiler by adding the `extern "C"` section at the top of our C++ source file.

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
{#include ../../../deps/examples/cc-bundled-cpp.rs}
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
{#include ../../../deps/examples/cc-bundled-cpp2.rs}
```

[cc-build-cpp]: https://docs.rs/cc/*/cc/struct.Build.html#method.cpp
