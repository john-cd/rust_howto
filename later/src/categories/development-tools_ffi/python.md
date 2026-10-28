<!-- TODO Reviewed ??-??-2026 -->

# Python Interop

{{#include python.incl.md}}

## Call Python from Rust {#calling-python-from-rust}

[![pyo3][c~pyo3~docs~badge]][c~pyo3~docs] [![pyo3~crates.io][c~pyo3~crates.io~badge]][c~pyo3~crates.io] [![pyo3~repo][c~pyo3~repo~badge]][c~pyo3~repo] [![pyo3~lib.rs][c~pyo3~lib.rs~badge]][c~pyo3~lib.rs]{{hi:pyo3}}{{hi:Cpython}}{{hi:Ffi}}{{hi:pyo3}}{{hi:Python}} [![cat~api-bindings][cat~api-bindings~badge]][cat~api-bindings]{{hi:API bindings}} [![cat~development-tools::ffi][cat~development-tools::ffi~badge]][cat~development-tools::ffi]{{hi:FFI}}

[`pyo3`][c~pyo3~docs]↗{{hi:pyo3}} is a Rust crate that enables seamless integration between Rust and Python. It allows developers to write Python extensions in Rust, leveraging Rust's performance and safety benefits, or to embed Python within Rust applications. Pyo3 provides tools for managing Python objects, handling data exchange, and defining Python classes and modules in Rust.

PyO3 lets a Rust program call Python code through the interpreter. In current PyO3, `Python::attach` provides the interpreter token used by Python APIs; older examples may call the renamed `Python::with_gil` method. Python objects tied to that scope use the `Bound<'py, T>` API, while `Py<T>` owns a reference beyond the scope.

The embedded PyO3 example is enabled on Linux and Windows and requires a
compatible Python installation and development files. The Rust-defined module
example remains Linux-only. The separate extension package has its own Python
and Maturin setup below.

This example is an embedded Python executable. On Windows, set `PYTHONHOME` to
the base Python installation before running the executable or its test so
CPython can locate the standard library:

```powershell
$env:PYTHONHOME = (python -c "import sys; print(sys.base_prefix)").Trim()
cargo run --locked -p development_tools_ffi --example python -- pyo3
```

The example initializes the interpreter explicitly because the crate does not
enable PyO3's `auto-initialize` feature. On Windows, its test is ignored during
the default test run; from `later/crates`, run it manually with the environment
above:

```console
cargo test --locked -p development_tools_ffi --example python test_pyo3_example_entry_point -- --ignored --test-threads=1
```

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/examples/python/pyo3.rs:example}}
```

### Define a Rust Module for Embedded Python {#use-rust-from-python}

This example registers a Rust-defined module in the embedded interpreter's `sys.modules`, then imports it from Python code. It demonstrates module and function registration inside a Rust executable; the next section shows how to build a native extension that a separate Python process can import.

```rust,noplayground
{{#include ../../../crates/cats/development_tools_ffi/examples/python/use_rust_from_python.rs:example}}
```

## Write a Python Extension in Rust {#calling-rust-from-python}

An embedded module is available only to the Rust process that registers it. To expose Rust code as a native extension that an ordinary Python process can import, define a `#[pymodule]` and build the separate PyO3 `cdylib` package with Maturin:

```rust,noplayground
{{#include ../../../crates/cats/development_tools_ffi/python_extension/src/lib.rs}}
```

From `later/crates/cats/development_tools_ffi/python_extension`, create and
activate a Python virtual environment, then install the pinned Maturin version
and build the extension:

```console
python -m venv .venv
```

Activate the environment with `source .venv/bin/activate` on Linux or `.venv\\Scripts\\Activate.ps1` in PowerShell, then run:

```console
python -m pip install "maturin==1.15.0"
maturin develop
python python/use_rust_from_python.py
```

Maturin builds the extension for the active Python environment. The script imports `rust_howto_python_module` and prints the result of calling its Rust `add` function.

An ignored integration test builds the extension and runs the same Python script
in a separate process. From `later/crates`, run it only after activating a
virtual environment with Python and Maturin available:

```console
cargo test --locked -p development_tools_ffi --example python builds_and_imports_the_python_extension -- --ignored --test-threads=1
```

Do not set `PYTHONHOME` for this extension test; its active virtual environment
selects Python for Maturin.

```python
{{#include ../../../crates/cats/development_tools_ffi/python_extension/python/use_rust_from_python.py}}
```

## Data Exchange Between Rust and Python {#data-exchange-between-rust-and-python}

PyO3 converts common Rust collections to Python containers: `Vec<T>` maps to a Python `list`, and `HashMap<String, T>` maps to a Python `dict` when `T` supports the corresponding conversion. Use `FromPyObject` to extract values from Python and `IntoPyObject` to create Python objects. A `#[pyclass]` exposes a Rust-owned type when a plain list or dictionary does not express the domain model. For interchange with other languages or processes, serialize explicitly with [`serde`][c~serde~docs]↗{{hi:serde}} and a format such as [`serde_json`][c~serde_json~docs]↗{{hi:serde_json}}.

See also:

- [[complex_encoding | Complex Encoding]].
- [[encoding | Encoding]].
- [[json | JSON]].
- [[serde | Serde]].

## Numerical Data {#numerical-data}

The [`numpy`][c~numpy~docs]↗{{hi:numpy}} crate provides `PyReadonlyArray1` and related types for borrowing NumPy arrays. Convert the borrowed array to an `ndarray` view with `as_array()` to read its elements without copying; keep the Python array alive for the lifetime of that view. Use an owned `ndarray` array when Rust needs to retain or mutate the data independently. See [[mathematics | Mathematics]].

For DataFrame-oriented workloads, see the [Polars plugin documentation](https://docs.pola.rs/user-guide/plugins/)↗; plugins extend Polars operations and use a different integration path from PyO3 extension modules.

## Build Tools {#build-tools .skip}

- [`maturin`][c~maturin~docs]↗{{hi:maturin}} is a popular tool for building and distributing Python packages that include Rust extensions. It handles the complexities of compilation and linking.
- [`setuptools-rust`][pypi~setuptools-rust]{{hi:setuptools-rust}} is another option for integrating Rust builds into Python's setuptools.

See [[cargo | Cargo]] for Cargo's build workflow.

Learn more: [How to use Rust with Python, and Python with Rust][blog~how-to-use-rust-with-python-and-python-with-rust]↗.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
