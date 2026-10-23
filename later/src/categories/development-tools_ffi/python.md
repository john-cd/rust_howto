<!-- TODO Reviewed ??-??-2026 -->
# Python Interop

{{#include python.incl.md}}

## Calling Rust from Python {#calling-rust-from-python}

- As detailed below, [`pyo3`][c~pyo3~docs]↗{{hi:pyo3}} is a popular crate for exposing Rust functions and modules to Python (and for embedding Python in Rust). This is the most common approach.
- While primarily a C tool, [`cffi`][c~cffi~docs]↗{{hi:cffi}} can be used in conjunction with Rust and Python to create bindings. Less common than [`pyo3`][c~pyo3~docs]↗{{hi:pyo3}} for this direction.

## Calling Python from Rust {#calling-python-from-rust}

- [`cpython`][c~cpython~docs]↗{{hi:cpython}} provides direct bindings to the CPython interpreter. Lower-level, but very powerful.
- [`pyo3`][c~pyo3~docs]↗{{hi:pyo3}} is higher-level and easier to use than [`cpython`][c~cpython~docs]↗{{hi:cpython}} for many use cases.

## Embed Python in a Rust Program {#pyo3}

[![pyo3][c~pyo3~docs~badge]][c~pyo3~docs] [![pyo3~crates.io][c~pyo3~crates.io~badge]][c~pyo3~crates.io] [![pyo3~repo][c~pyo3~repo~badge]][c~pyo3~repo] [![pyo3~lib.rs][c~pyo3~lib.rs~badge]][c~pyo3~lib.rs]{{hi:pyo3}}{{hi:Cpython}}{{hi:Ffi}}{{hi:pyo3}}{{hi:Python}} [![cat~api-bindings][cat~api-bindings~badge]][cat~api-bindings]{{hi:API bindings}} [![cat~development-tools::ffi][cat~development-tools::ffi~badge]][cat~development-tools::ffi]{{hi:FFI}}

[`pyo3`][c~pyo3~docs]↗{{hi:pyo3}} is a Rust crate that enables seamless integration between Rust and Python. It allows developers to write Python extensions in Rust, leveraging Rust's performance and safety benefits, or to embed Python within Rust applications. Pyo3 provides tools for managing Python objects, handling data exchange, and defining Python classes and modules in Rust.

[`pyo3`][c~pyo3~website]↗ supports both calling python code from Rust and exposing Rust code to Python.

This example is an embedded Python executable. It is enabled on Linux, where the `development_tools_ffi` crate declares its PyO3 dependency. It initializes the interpreter explicitly because the crate does not enable PyO3's `auto-initialize` feature.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/examples/python/pyo3.rs:example}}
```

## Define a Rust Module for Embedded Python {#use-rust-from-python}

This example registers a Rust-defined module in the embedded interpreter's `sys.modules`, then imports it from Python code. It demonstrates module and function registration inside a Rust executable; the next section shows how to build a native extension that a separate Python process can import.

```rust,noplayground
{{#include ../../../crates/cats/development_tools_ffi/examples/python/use_rust_from_python.rs:example}}
```

### Build and import a Python extension module {#build-python-extension}

The example above embeds Python in a Rust executable. To build a native Python module that an ordinary Python script can import, use the separate PyO3 `cdylib` package and Maturin:

```rust,noplayground
{{#include ../../../crates/cats/development_tools_ffi/python_extension/src/lib.rs}}
```

From `later/crates/cats/development_tools_ffi/python_extension`, activate a Python virtual environment and run:

```console
python -m pip install "maturin>=1.9.4,<2.0"
maturin develop
python python/use_rust_from_python.py
```

Maturin builds the extension for the active Python environment. The script imports `rust_howto_python_module` and prints the result of calling its Rust `add` function.

```python
{{#include ../../../crates/cats/development_tools_ffi/python_extension/python/use_rust_from_python.py}}
```

## Data Exchange Between Rust and Python {#data-exchange-between-rust-and-python}

While not Python-specific, [`serde`][c~serde~docs]↗{{hi:serde}}'s serialization capabilities allow for efficiently exchanging data between Rust and Python. You'd likely use [`serde_json`][c~serde_json~docs]↗{{hi:serde_json}} or another format in conjunction with your chosen Python binding crate.

See also:

- [[complex_encoding | Complex Encoding]].
- [[encoding | Encoding]].
- [[json | JSON]].
- [[serde | Serde]].

## Numerical Data {#numerical-data}

If you're working with numerical data, integrating with [`numpy`][c~numpy~docs]↗{{hi:numpy}} is often essential. Crates like [`ndarray`][c~ndarray~docs]↗{{hi:ndarray}} in Rust can also facilitate this. See [[mathematics | Mathematics]].

## Build Tools {#build-tools .skip}

- [`maturin`][c~maturin~docs]↗{{hi:maturin}} is a popular tool for building and distributing Python packages that include Rust extensions. It handles the complexities of compilation and linking.
- [`setuptools-rust`][pypi~setuptools-rust]{{hi:setuptools-rust}} is another option for integrating Rust builds into Python's setuptools.

See [[cargo | Cargo]] for Cargo's build workflow.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[python_interop: write](https://github.com/john-cd/rust_howto/issues/210)

- [How to use Rust with Python, and Python with Rust][blog~how-to-use-rust-with-python-and-python-with-rust]↗.

</div>
