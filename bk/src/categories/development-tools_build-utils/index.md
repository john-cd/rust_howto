# Build Utils

[![cat-development-tools::build-utils][cat-development-tools::build-utils-badge]][cat-development-tools::build-utils]{{hi:Development tools::Build Utils}}

## Build-time tooling

Utilities for build scripts and other build time steps.

[`cc`][c-cc]⮳{{hi:cc}} for compiling C/C++ code within build scripts,
[`pkg-config`][c-pkg_config]⮳{{hi:pkg-config}} for finding system libraries

{{#include build_time_tooling.incl.md}}

## Build Caches

{{#include build_cache.incl.md}}

## Related Topics

| | |
|---|---|
| Benchmarking | `cargo bench` (also part of the build/verification process) |
| Code Generation | Often done with procedural macros or build scripts |
| Cross-Compilation Tools | [`cross`][c-cross]⮳{{hi:cross}} simplifies cross-compilation using Docker |
| Dependency Management | [`cargo`][c-cargo]⮳{{hi:cargo}} though primarily a package manager, it also handles build dependencies |
| Documentation Generation | `cargo doc` though primarily for documentation, it's part of the build process |
| Packaging | [`cargo-deb`][c-cargo_deb]⮳{{hi:cargo-deb}}, [`cargo-rpm`][c-cargo_rpm]⮳{{hi:cargo-rpm}}, `create-dmg` for creating distribution packages |
| Task Runners | [`xtask`][c-xtask]⮳{{hi:xtask}} for managing complex build tasks, often used for CI/CD, testing, etc. |
| Testing | `cargo test` |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
<div class="hidden">
[development-tools_build-utils/index: add](https://github.com/john-cd/rust_howto/issues/306)
</div>
