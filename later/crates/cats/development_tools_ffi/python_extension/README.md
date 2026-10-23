# Build and import the Rust Python extension

From this directory, create and activate a virtual environment, then build and install the extension into it with Maturin:

```console
python -m venv .venv
```

Activate the environment with `.\.venv\Scripts\Activate.ps1` in PowerShell or `source .venv/bin/activate` on macOS and Linux. Then run:

```console
python -m pip install "maturin>=1.9.4,<2.0"
maturin develop
python python/use_rust_from_python.py
```

The final command imports `rust_howto_python_module` and calls `add(10, 20)`. Maturin sets PyO3's extension-module build configuration while building the `cdylib`.