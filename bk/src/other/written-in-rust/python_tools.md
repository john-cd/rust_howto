<!-- TODO Reviewed ??-??-2026. -->

# Python Tools Written in Rust

Rust powers Python interpreters, code-quality tools, environment managers, and tools for shipping native extensions.

{{#include python_tools.incl.md}}

## RustPython {#rustpython}

{{#badges rustpython}}

[RustPython][rustpython~repo]↗{{hi:RustPython}} is a Python interpreter that can run inside a Rust application or in the browser through WebAssembly. The repository documents which CPython features it supports.

## PyOxidizer {#pyoxidizer}

[PyOxidizer][c~pyoxidizer~repo]↗{{hi:PyOxidizer}} packages a Python application, its interpreter, and its resources together. Its [latest tagged release][pyoxidizer~releases~repo]↗, 0.24.0, dates to 2022.

## Ruff {#ruff}

[Ruff][c~ruff~docs]↗{{hi:Ruff}} lints and formats Python code. Use `ruff check` for diagnostics and `ruff format` to apply formatting.

```bash
ruff check .
ruff format .
```

See the [Ruff linter documentation][ruff~linter~website]↗ for rule configuration and the [formatter documentation][ruff~formatter~website]↗ for formatting options.

## ty {#ty}

[ty][ty~repo]↗ checks Python types and provides an editor language server. Run `ty check` to check a project from the terminal.

## uv {#uv}

[`uv`][uv~docs~website]↗{{hi:uv}} manages Python projects, dependencies, virtual environments, and Python versions.

For example, these commands create a project, add a dependency, and run a Python command in its environment:

```bash
uv init my-project
cd my-project
uv add requests
uv run python -c "print('hello')"
```

## Pixi {#pixi}

[Pixi][pixi~repo]↗ manages project environments and tasks. It can install dependencies from Conda channels and PyPI in the same environment.

## Build a Python Wheel for a Rust Extension {#maturin}

[Maturin][maturin~website]↗ builds wheels for Rust extensions. PyO3
defines the extension interface, while Maturin handles the Python package
build. Follow the existing [PyO3 and Maturin recipe](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/development-tools_ffi/python.md)
to build and import the extension with the intended Python interpreter.

## Related Topics {#related-topics .skip}

- [Development tools](development_tools.md) covers editors, build systems, and general-purpose command-line tools.
- [Scripting and Rust](../scripting/index.md) covers embedding scripting languages in Rust applications.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
