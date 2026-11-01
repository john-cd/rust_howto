<!-- TODO Reviewed ??-??-2026 -->

# Python Extension

This crate contains code examples for the **Python Extension** category on crates.io. It demonstrates integration with key ecosystem crates including `pyo3`.

## Examples Covered

- **Core Examples**: Code modules in `src/` or `tests/` demonstrating category functionality.

## Usage

To check or run the Rust examples in this crate:

```console
cargo check --package rust_howto_python_module --examples
cargo test --package rust_howto_python_module
```

## Build and import the Rust Python extension

From the `rust_howto` repository root, use the project Justfile to sync the locked environment, build the extension, and run the example:

```console
just --justfile later/crates/cats/development_tools_ffi/python_extension/justfile env
just --justfile later/crates/cats/development_tools_ffi/python_extension/justfile sync
just --justfile later/crates/cats/development_tools_ffi/python_extension/justfile uv run maturin develop
just --justfile later/crates/cats/development_tools_ffi/python_extension/justfile uv run python python/use_rust_from_python.py
```

Install and register the repository hooks once with `just --justfile later/crates/cats/development_tools_ffi/python_extension/justfile uv tool install prek` and `just --justfile later/crates/cats/development_tools_ffi/python_extension/justfile hooks-install`, then run them with `just --justfile later/crates/cats/development_tools_ffi/python_extension/justfile hooks`.

The final command imports `rust_howto_python_module` and calls `add(10, 20)`. Maturin sets PyO3's extension-module build configuration while building the `cdylib`.
