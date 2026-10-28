<!-- TODO Reviewed ??-??-2026 -->

# Python Extension

This crate contains code examples for the **Python Extension** category on crates.io. It demonstrates integration with key ecosystem crates including `pyo3`.

## Examples Covered

- **Core Examples**: Code modules in `src/` or `tests/` demonstrating category functionality.

## Usage

To check or run examples in this crate:

```console
cargo check --package rust_howto_python_module --examples
cargo test --package rust_howto_python_module
```

Code examples correspond to chapters in `bk/src/categories/python_extension` or related book sections.

## Build and import the Rust Python extension

From this directory, create and activate a virtual environment, then build and install the extension into it with Maturin:

```console
python -m venv .venv
```

Activate the environment with `.\.venv\Scripts\Activate.ps1` in PowerShell or `source .venv/bin/activate` on macOS and Linux. Then run:

```console
python -m pip install "maturin==1.15.0"
maturin develop
python python/use_rust_from_python.py
```

The final command imports `rust_howto_python_module` and calls `add(10, 20)`. Maturin sets PyO3's extension-module build configuration while building the `cdylib`.
