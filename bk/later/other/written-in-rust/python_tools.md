# Tools for Python Written in Rust

{{#include python_tools.incl.md}}

## `rustpython` {#rustpython}

[![rustpython][c-rustpython-badge]][c-rustpython]{{hi:rustpython}}
[![rustpython-crates.io][c-rustpython-crates.io-badge]][c-rustpython-crates.io]
[![rustpython-github][c-rustpython-github-badge]][c-rustpython-github]
[![rustpython-lib.rs][c-rustpython-lib.rs-badge]][c-rustpython-lib.rs]

[`RustPython`][rustpython-github]{{hi:RustPython}}⮳ is an open-source Python Interpreter written in Rust.

RustPython supports Python 3 (CPython >= 3.11.0). RustPython can be embedded into Rust programs to use Python as a [scripting][p-scripting] language for your application, or it can be compiled to WebAssembly in order to run Python in the browser.

```bash
cargo install --git https://github.com/RustPython/RustPython rustpython
# or
wapm install rustpython
# or
conda install rustpython -c conda-forge
```

## `pyOxidizer` {#pyoxidizer}

[![pyOxidizer][c-pyoxidizer-badge]][c-pyoxidizer-github]{{hi:pyOxidizer}}⮳.

[`pyOxidizer`][c-pyoxidizer]⮳{{hi:pyOxidizer}} is a utility for producing binaries that embed Python.

[`pyOxidizer`][c-pyoxidizer]⮳{{hi:pyOxidizer}} is capable of producing a single file executable - with a copy of Python and all its dependencies statically linked and all resources (like `.pyc` files) embedded in the executable. You can copy a single executable file to another machine and run a Python application contained within.

[`pyOxidizer`][c-pyoxidizer]⮳{{hi:pyOxidizer}} can also be used to (a) easily add a Python interpreter to any Rust project; (b) add Rust to Python.

## `Ruff` {#ruff}

[Ruff][c-ruff]{{hi:ruff}}⮳.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

## `uv` {#uv}

"uv" is a relatively new and fast Python package installer and resolver, designed as a drop-in replacement for `pip`, `pip-tools`, `pipx`, `poetry`, `pyenv`, `twine`, `virtualenv`, and more. It's gaining popularity for its speed and efficiency.

Key features include:

- Installing and managing Python versions.
- Running and installing tools published as Python packages.
- Installing packages from `PyPI`.
- Resolving dependencies and generating lock files (like `pip-compile`).
- Working with virtual environments.
- Supports [`Cargo`][c-cargo]⮳{{hi:Cargo}}-style workspaces for scalable projects.

<div class="hidden">
[python_tools: write](https://github.com/john-cd/rust_howto/issues/617)
</div>
