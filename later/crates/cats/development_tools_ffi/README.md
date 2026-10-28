<!-- TODO Reviewed ??-??-2026 -->

# Development Tools FFI

This crate contains code examples for the **Development Tools FFI** category on crates.io. It demonstrates integration with key ecosystem crates including `anyhow`, `clap`, `cxx`, `flutter_rust_bridge`, `jni`, `mlua`.

## Platform Prerequisites

The repository's `.devcontainer` image and install scripts provide the native SDKs
for these examples. On Debian, they install the default JDK, Ruby headers, Python
3 with virtual-environment support, and the pinned Maturin release. The Windows
script installs Eclipse Temurin JDK 21, Python 3.13, RubyInstaller with DevKit
3.3, and the same Maturin version.

- **Java/JNI:** Rust code can compile without a JDK. Running the Java caller
  requires `java` and `javac` on `PATH`; set `JAVA_HOME` to the JDK root when
  another tool needs an explicit JDK path. The Windows setup script configures
  it when the Temurin installer does not.
- **Ruby:** `ruby-dev` supplies headers on Debian; RubyInstaller with DevKit
  supplies Ruby, headers, and its C toolchain on Windows. This crate currently
  gates Magnus and Rutie dependencies/examples to non-Windows targets, so Windows
  runs only the platform-independent greeting example.
- **Python:** The embedded PyO3 example supports Linux and Windows; the Rust
  module example remains Linux-only. Windows embedded runs need `PYTHONHOME`
  set to Python's base installation so CPython can find its standard library.
  The separate `python_extension` workspace builds an importable module with
  Python, an active virtual environment, and Maturin. The installed Maturin
  version is pinned in `.devcontainer/.env`.

See the Java, Ruby, and Python chapters for the commands to compile and run each
language-side example.

## Examples Covered

- **C** (`examples/c/`): Demonstrates concepts like `bindgen`, `cbindgen`.
- **Cpp** (`examples/cpp/`): Demonstrates concepts like `cxx`.
- **Erlang Elixir** (`examples/erlang_elixir/`): Demonstrates concepts like `rustler`.
- **Flutter** (`examples/flutter/`): Demonstrates concepts like `flutter_rust_bridge`.
- **Java** (`examples/java/`): Demonstrates concepts like `jni`.
- **Lua** (`examples/lua/`): Demonstrates concepts like `mlua`, `mlua_scripting`, `mlua_userdata`, `module`.
- **Node** (`examples/node/`): Demonstrates concepts like `napi`, `neon`.
- **Objc** (`examples/objc/`): Demonstrates concepts like `objc2`.
- **Python** (`examples/python/`): Demonstrates concepts like `pyo3`, `use_rust_from_python`.
- **Ruby** (`examples/ruby/`): Demonstrates concepts like `greeting`, `magnus`, `rutie`.
- **Standalone Examples**: `uniffi`.

## Usage

To check or run examples in this crate:

```console
cargo check --package development_tools_ffi --examples
cargo test --package development_tools_ffi
```

Code examples correspond to chapters in `bk/src/categories/development_tools_ffi` or related book sections.
