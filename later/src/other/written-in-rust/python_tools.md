<!-- TODO Reviewed ??-??-2026 -->

# Python Tools Written in Rust

Rust is used to build tools across the Python workflow: an interpreter, an application packager, a linter and formatter, and a project and environment manager. These tools still work with Python programs; they do not translate Python source code into Rust.

{{#include python_tools.incl.md}}

## `rustpython` {#rustpython}

[![rustpython][c~rustpython~docs~badge]][c~rustpython~docs]{{hi:rustpython}}
[![rustpython~crates.io][c~rustpython~crates.io~badge]][c~rustpython~crates.io]
[![rustpython~repo][c~rustpython~repo~badge]][c~rustpython~repo]
[![rustpython~lib.rs][c~rustpython~lib.rs~badge]][c~rustpython~lib.rs]

[`RustPython`][rustpython~repo]↗{{hi:RustPython}} is a Python interpreter implemented in Rust. It is a separate implementation of the language, rather than a way to compile arbitrary Python programs to native Rust.

The project is still in development and does not claim full compatibility with CPython. Its repository documents embedding and WebAssembly use cases; check its current compatibility and platform notes before depending on a feature.

For current build and installation instructions, see the [RustPython repository][rustpython~repo]↗.

## `pyOxidizer` {#pyoxidizer}

[![pyOxidizer][c~pyoxidizer~docs~badge]][c~pyoxidizer~repo]↗{{hi:pyOxidizer}}.

[`PyOxidizer`][c~pyoxidizer~repo]↗{{hi:PyOxidizer}} is a tool for packaging Python applications with an embedded interpreter and resources. It can also embed Python in a Rust application. Packaging and embedding approaches are sensitive to the Python version and target platform, so verify those details against the project's documentation before adopting it.

The [upstream releases page][pyoxidizer~releases~repo]↗ currently lists version 0.24.0, released in 2022. Check project activity and compatibility before choosing it for a new application.

## `Ruff` {#ruff}

[`Ruff`][c~ruff~docs]↗{{hi:Ruff}} is a Python linter and formatter implemented in Rust. It can replace several common linting tools in one fast command-line tool.

```bash
ruff check .
ruff format .
```

See the [Ruff linter documentation][ruff~linter~website]↗ for rule configuration and the [formatter documentation][ruff~formatter~website]↗ for formatting options.

## `uv` {#uv}

[`uv`][uv~repo]↗{{hi:uv}} is a Python version, project, dependency, and environment manager. It can create projects, resolve and install dependencies, manage virtual environments and Python versions, and run project commands. Its command surface covers workflows that otherwise use several tools; check the [uv documentation][uv~docs~website]↗ for details and supported behavior.

For example, these commands create a project, add a dependency, and run a Python command in its environment:

```bash
uv init my-project
cd my-project
uv add requests
uv run python -c "print('hello')"
```

For Python extensions written in Rust, see [[python | Python FFI and extensions]] for the separate interoperability workflow.

## Related Topics {#related-topics .skip}

- [[development_tools | Development Tools]].
- [[scripting | Scripting]].
- [[cross-platform | Cross-platform Development]].
- [[programming_languages | Programming Languages]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[python_tools: write](https://github.com/john-cd/rust_howto/issues/617)
[content audit: Written-in-Rust tool lists trimmed by recent commits; review and restore removed entries](https://github.com/john-cd/rust_howto/issues/1902)
</div>
