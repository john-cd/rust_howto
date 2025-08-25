# Reduce Compilation Duration

{{#include reduce_compilation_duration.incl.md}}

Rust compile times{{hi:Compile times}} can be long. Reducing Rust compilation [duration][p~duration] involves several strategies, targeting both the compiler and project structure.

| Methods | Description |
|---|---|
| Incremental Compilation | Leverage [Cargo][p~cargo]'s [caching][p~caching], be mindful of changes that invalidate the cache. |
| Dependency Management | Use [`cargo tree`][book~cargo~cargo-tree]↗{{hi:cargo tree}} to analyze dependencies. [`cargo-bloat`][c~cargo-bloat~docs]↗{{hi:cargo-bloat}} can help identify large dependencies contributing to compile times. |
| Compiler Flags | Experiment with compiler flags, but be careful and measure the impact. |
| Build Profiles | Optimize release builds with appropriate flags in `config.toml`. |
| Link-Time Optimization (LTO) | Controlled via [`Cargo.toml`][book~cargo~cargo-toml]↗{{hi:Cargo.toml}} and `config.toml`. |
| Profiling | [cargo][p~cargo] flamegraph, [`perf`][c~perf~docs]↗{{hi:perf}} (system profiler) |
| Code Structure | Avoid excessive monomorphization, consider techniques like dynamic dispatch where applicable. |

## Measure Build Times {#build-time}

[![cargo~website][c~cargo~website~badge]][c~cargo~website] [![cat~compilers][cat~compilers~badge]][cat~compilers]{{hi:Compilers}}

```sh
time cargo build
```

```sh
cargo build --timings
```

You may also use [`hyperfine`][c~hyperfine~docs]↗{{hi:hyperfine}}. See [[benchmarking | Benchmarking]].

## Incremental Compilation {#incremental-compilation}

Incremental compilation in Rust is built into [Cargo][p~cargo] and [`rustc`][book~rustc]↗{{hi:rustc}}, and generally "just works" automatically. It reuses previously compiled code, significantly speeding up subsequent builds after changes.

Keeping in mind how the incremental compiler works is key to maximizing its benefits. Changes to dependencies or function signatures can invalidate the cache. Strategies to minimizing cache invalidation include:

- Keeping dependencies stable,
- Structuring code to minimize changes that trigger recompilation (see below),
- Avoid actions that invalidate the cache, such as changing dependencies (modifying [`Cargo.toml`][book~cargo~cargo-toml]↗{{hi:Cargo.toml}}) or build scripts unnecessarily.
- Being mindful of how [generics][p~generics] and macros can affect recompilation.

From-scratch builds with incremental compilation{{hi:Incremental compilation}} enabled adds about 15-20% overhead compared to disabled. The initial build needs to write out more intermediate state in order for later incremental builds to take advantage of it. In a CI{{hi:CI}} situation, it would be unusual for there to be a later incremental build within the same job. Thus consider disabling incremental compilation in that context.However, CI workflows that cache the target directory across runs may be benefiting from incremental compilation.

## Project Structure and Dependencies {#skip}

- _Reduce the number of dependencies_. Analyze your [`Cargo.toml`][book~cargo~cargo-toml]↗{{hi:Cargo.toml}} and eliminate unused or redundant dependencies. Consider alternatives if a dependency is excessively large or slow to compile.
- For multi-crate projects, use a _workspace_ to share dependencies and enable workspace-level optimizations. This can reduce redundant compilation.
- Use feature flags to conditionally compile your code and dependencies. They can exclude unnecessary code during development, reducing compilation time.
- For large, infrequently changing dependencies, consider using precompiled versions if available.
- Organizing your code into smaller, independent [modules][p~modules] and crates can improve incremental compilation by reducing the scope of changes.

[`cargo tree`][book~cargo~cargo-tree]↗{{hi:cargo tree}} is a useful tool for dependency analysis.

## Compiler-Level Optimizations {#skip}

- Use the [`dev`][book~cargo~dev-profile]↗{{hi:dev}} profile for development, which prioritizes fast compilation over optimizations. Switch to the [`release`][book~cargo~release-profile]↗{{hi:release}} profile only for final builds.
- Increasing the number of codegen units ([`rustc`][book~rustc~docs]↗{{hi:book~rustc}} flag: `-C codegen-units=N`) allows the compiler to parallelize code generation. Experiment to find the optimal value; too many can hinder Link-Time Optimization (LTO).
- Tune Link-Time Optimization (LTO). While LTO can improve runtime [performance][p~performance], it increases compile time, especially "fat" LTO.
Consider using "thin" LTO (`-C lto=thin`) for a faster, though less aggressive, approach. For debug builds, disable LTO entirely ([`-C lto=no`][book~cargo~profiles-lto]↗{{hi:LTO}}).
- Profile-Guided Optimization (PGO): PGO can improve runtime performance but requires additional compilation and profiling steps, thus increasing overall build time. Use PGO only for release builds where runtime [performance][p~performance] is critical, and not for general development.
- Using [`sccache`][c~sccache~docs]↗{{hi:sccache}} can dramatically speed up compilation by [caching][p~caching] compiled objects. It's particularly effective when recompiling similar code across multiple projects or branches.

## Optimize Compilation Levels {#optimization-levels}

[![cargo~website][c~cargo~website~badge]][c~cargo~website] [![cat~compilers][cat~compilers~badge]][cat~compilers]{{hi:Compilers}}

In `config.toml`

```toml
# Enable a small amount of optimization in debug mode
[profile.dev]
opt-level = 1

# Enable high optimizations for dependencies, but not for our code:
[profile.dev.package."*"]
opt-level = 3
```

## Use Dynamic Linking {#dynamic-linking}

[![cargo-add-dynamic][c~cargo-add-dynamic~docs~badge]][c~cargo-add-dynamic~docs] [![cargo-add-dynamic~crates.io][c~cargo-add-dynamic~crates.io~badge]][c~cargo-add-dynamic~crates.io] [![cargo-add-dynamic~repo][c~cargo-add-dynamic~repo~badge]][c~cargo-add-dynamic~repo] [![cargo-add-dynamic~lib.rs][c~cargo-add-dynamic~lib.rs~badge]][c~cargo-add-dynamic~lib.rs]{{hi:cargo-add-dynamic}}{{hi:Cargo}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}} [![cat~compilers][cat~compilers~badge]][cat~compilers]{{hi:Compilers}}

```sh
cargo install cargo-add-dynamic
cargo add-dynamic polars --features csv-file,lazy,list,describe,rows,fmt,strings,temporal
cargo build
```

See also: [Speeding up incremental Rust compilation with dylibs][blog~speeding-up-incremental-rust-compilation]↗.

## Build Machine Hardware Considerations {#skip}

Build your code with the right machine:

- A multi-core CPU significantly speeds up compilation, especially when using multiple codegen units.
- Sufficient RAM is essential, especially when using LTO or compiling large projects.
- A fast SSD can greatly reduce I/O bottlenecks during compilation.

Consider using remote build servers or a separate build machine for large projects.

## References {#references}

- [Eight solutions for troubleshooting your Rust build times][blog~rust-build-times]↗.
- [How I improved my Rust compile times by seventy-five percent][blog~how-i-improved-my-rust-compile-times-by-seventy-five-percent]↗.
- [Rust compilation time][blog~rust-compilation-time]↗.

## Related Topics {#related-topics}

- [[benchmarking | Benchmarking]].
- [[build_utils | Build Utils]].
- [[development-tools | Development Tools]].
- [[development-tools_build-utils | Development Tools: Build Utils]].
- [[development-tools_cargo-plugins | Development Tools: Cargo Plugins]].
- [[development-tools_profiling | Development Tools: Profiling]].
- [[faster_linking | Faster Linking]].
- [[performance | Performance]].

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">
[write / expand / align table and text](https://github.com/john-cd/rust_howto/issues/245)
</div>
