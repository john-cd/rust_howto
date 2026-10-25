<!-- TODO Reviewed ??-??-2026 -->

# Build Utils

[![cat~development-tools::build-utils][cat~development-tools::build-utils~badge]][cat~development-tools::build-utils]{{hi:Development tools::Build Utils}}

Build utilities run before or alongside compilation to prepare native libraries, generate build-time data, detect compiler capabilities, and reuse build artifacts. The guides below cover the recipes in this category.

## Build-time Tools

Cargo runs a package's `build.rs` on the host before compiling the package for its target. Use a build script for work such as compiling bundled C or C++ code, discovering system libraries, generating source files, or emitting target-specific configuration. The [[build_time_tooling | Build-time Tooling]] chapter covers these workflows, including [`cc`][c~cc~docs]↗{{hi:cc}} and [`pkg-config`][c~pkg-config~docs]↗{{hi:pkg-config}}.

{{#include build_time_tooling.incl.md}}

## Build Caches

Reuse build outputs across local builds and CI to reduce repeated compilation. The [[build_cache | Build Cache]] chapter discusses `target` and intermediate directories, shared compiler caches, and CI caching.

{{#include build_cache.incl.md}}

## Compiler Feature Detection

Use compile-time checks when a build depends on compiler or target capabilities. The [[autocfg | autocfg]] chapter shows how to detect supported Rust features from a build script.

{{#include autocfg.incl.md}}

## Related Topics {#related-topics .skip}

- [Cross-compiling][ex~development-tools_cargo-plugins~cross-compiling]↗.
- [[cargo | Cargo]].
- [[development-tools_cargo-plugins | Cargo Plugins]].
- [[code_formatting_linting | Code Formatting and Linting]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
