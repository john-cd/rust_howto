<!-- TODO Reviewed ??-??-2026. -->

# Development Tools - Build Utils

[![cat~development-tools::build-utils][cat~development-tools::build-utils~badge]][cat~development-tools::build-utils]{{hi:Development tools::Build Utils}}

Build utilities run before or alongside compilation to prepare native libraries, generate source or resources, detect compiler capabilities, and reuse build artifacts. Keep build scripts deterministic, declare their inputs, and write generated files to Cargo's `OUT_DIR`.

## Build-time Tools

Cargo runs a package's `build.rs` on the host before compiling the package for its target. Use a build script to compile bundled C or C++ code, discover system libraries, generate source files, or emit target-specific configuration. Cargo documents build-script [inputs, outputs, and directives][book~cargo~build-script]↗.

The examples that compile bundled native code require a host C compiler; the C++ recipe also requires a C++ compiler. Cross-compiling requires a compatible target toolchain and linker configuration.

{{#include build_time_tooling.incl.md}}

## Build Caches

Cargo reuses compatible outputs in its target and build directories. A compiler cache such as `sccache` can reuse compilation results across builds, and CI actions can cache registry data and build outputs between workflow runs. Cache hits depend on stable inputs, toolchain, and cache configuration.

{{#include build_cache.incl.md}}

## Compiler Feature Detection

Use compile-time probes when an implementation depends on an API that the active compiler accepts. `autocfg` runs small compiler probes from a build script and emits custom `cfg` settings for Rust code.

{{#include autocfg.incl.md}}

## Related Topics {#related-topics .skip}

- [Diagnose Rust Build Time](../development-tools/build_time_diagnosis.md) for compile-time bottlenecks.
- [Development Tools - Profiling](../development-tools_profiling/index.md) for runtime measurements.
- [Cargo](https://doc.rust-lang.org/cargo/) for package and build-script configuration.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
