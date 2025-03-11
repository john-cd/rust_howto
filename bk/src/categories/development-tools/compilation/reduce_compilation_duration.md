# Reduce compilation duration

{{#include reduce_compilation_duration.incl.md}}

Rust compile times{{hi:Compile times}} can be long. Reducing Rust compilation [duration][p-duration] involves several strategies, targeting both the compiler and project structure. Here's a breakdown:

## Project Structure and Dependencies {#skip}

- _Reduce the number of dependencies_. Analyze your `Cargo.toml` and eliminate unused or redundant dependencies. Consider alternatives if a dependency is excessively large or slow to compile.
- For multi-crate projects, use a _workspace_ to share dependencies and enable workspace-level optimizations. This can reduce redundant compilation.
- Avoid Unnecessary Recompilation: Be mindful of changes that trigger recompilation. For example, changing build scripts or modifying `Cargo.toml` can invalidate the cache.
- Use feature flags to conditionally compile code. This allows you to exclude unnecessary code during development, reducing compilation time.
- Precompiled Dependencies: For large, infrequently changing dependencies, consider using precompiled versions if available.
- Code Organization: Organizing your code into smaller, independent [modules][p-modules] can improve incremental compilation by reducing the scope of changes.

## Compiler-Level Optimizations {#skip}

- Build Profiles: Use the `dev` profile for development, which prioritizes fast compilation over optimizations. Switch to the `release` profile only for final builds.
- Ensure incremental compilation is enabled (the default). It reuses previously compiled code, significantly speeding up subsequent builds after changes. Avoid actions that invalidate the cache, such as changing dependencies or build scripts unnecessarily.
- Increasing the number of codegen units (`rustc` flag: `-C codegen-units=N`) allows the compiler to parallelize code generation. Experiment to find the optimal value; too many can hinder Link-Time Optimization (LTO).
- Tune Link-Time Optimization (LTO). While LTO can improve runtime [performance][p-performance], it increases compile time, especially "fat" LTO.
Consider using "thin" LTO (`-C lto=thin`) for a faster, though less aggressive, approach. For debug builds, disable LTO entirely (`-C lto=no`).
- Profile-Guided Optimization (PGO): PGO can improve runtime performance but requires additional compilation and profiling steps, thus increasing overall build time. Use PGO only for release builds where runtime [performance][p-performance] is critical, and not for general development.
- Ccache: Using ccache can dramatically speed up compilation by [caching][p-caching] compiled objects. It's particularly effective when recompiling similar code across multiple projects or branches.

## Build Machine Hardware Considerations {#skip}

- CPU Cores: A multi-core CPU significantly speeds up compilation, especially when using multiple codegen units.
- RAM: Sufficient RAM is essential, especially when using LTO or compiling large projects.
- SSD: A fast SSD can greatly reduce I/O bottlenecks during compilation.

## Tools {#skip}

- [`cargo-bloat`][c-cargo_bloat]⮳{{hi:cargo-bloat}} can help you identify large dependencies [contributing][p-contributing] to compile times.
- `cargo-graph` can visualize your dependency graph, making it easier to identify potential issues.

Profiling: [cargo][p-cargo] flamegraph, perf (system profiler)
Code Optimization: (Often done without specific crates, focusing on algorithmic efficiency, data structures, and avoiding unnecessary allocations/copies)
Dependency Management: (Minimize dependencies, use [cargo][p-cargo] tree to analyze)
Link-Time Optimization (LTO): (Controlled via Cargo.toml)
Incremental Compilation: (Leverage [Cargo][p-cargo]'s [caching][p-caching], be mindful of changes that invalidate the cache)
Build Profiles: (Optimize for release builds with appropriate flags in Cargo.toml)
Compiler Flags: (Experiment with compiler flags, but be careful and measure improvements)
Code Generation: (Avoid excessive monomorphization, consider techniques like dynamic dispatch where applicable)

## Measure build times {#build-time}

[![cargo-website][c-cargo-website-badge]][c-cargo-website] [![cat-compilers][cat-compilers-badge]][cat-compilers]{{hi:Compilers}}

```sh
time cargo build
```

```sh
cargo build --timings
```

## Optimize compilation levels {#optimization-levels}

[![cargo-website][c-cargo-website-badge]][c-cargo-website] [![cat-compilers][cat-compilers-badge]][cat-compilers]{{hi:Compilers}}

In `config.toml`

```toml
# Enable a small amount of optimization in debug mode
[profile.dev]
opt-level = 1

# Enable high optimizations for dependencies, but not for our code:
[profile.dev.package."*"]
opt-level = 3
```

## Use dynamic linking {#dynamic-linking}

[![cargo-add-dynamic][c-cargo_add_dynamic-badge]][c-cargo_add_dynamic] [![cargo-add-dynamic-crates.io][c-cargo_add_dynamic-crates.io-badge]][c-cargo_add_dynamic-crates.io] [![cargo-add-dynamic-github][c-cargo_add_dynamic-github-badge]][c-cargo_add_dynamic-github] [![cargo-add-dynamic-lib.rs][c-cargo_add_dynamic-lib.rs-badge]][c-cargo_add_dynamic-lib.rs]{{hi:cargo-add-dynamic}}{{hi:Cargo}} [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}} [![cat-compilers][cat-compilers-badge]][cat-compilers]{{hi:Compilers}}

```sh
cargo install cargo-add-dynamic
cargo add-dynamic polars --features csv-file,lazy,list,describe,rows,fmt,strings,temporal
cargo build
```

[Speeding up incremental Rust compilation with dylibs][blog-speeding-up-incremental-rust-compilation]⮳.

## Compile incrementally {#incremental-compilation}

[![cat-compilers][cat-compilers-badge]][cat-compilers]{{hi:Compilers}}

From-scratch builds with incremental compilation{{hi:Incremental compilation}} enabled adds about 15–20% overhead compared to disabled. The initial build needs to write out more intermediate state in order for later incremental builds to take advantage of it. In a CI{{hi:CI}} situation, it would be extremely unusual for there to be a later incremental build within the same job. The jobs are not making changes to source code and rebuilding. However, workflows that cache the target directory across runs might be benefiting from incremental compilation.

## References {#skip}

- [Eight solutions for troubleshooting your Rust build times][blog-rust-build-times]⮳.
- [How I improved my Rust compile times by seventy-five percent][blog-how-i-improved-my-rust-compile-times-by-seventy-five-percent]⮳.
- [Rust compilation time][blog-rust-compilation-time]⮳.

## Related Topics {#skip}

- [[benchmarking | Benchmarking]].
- [[build_utils | Build Utils]].
- [[development-tools | Development Tools]].
- [[development-tools_build-utils | Development Tools: Build Utils]].
- [[development-tools_cargo-plugins | Development Tools: Cargo Plugins]].
- [[development-tools_profiling | Development Tools: Profiling]].
- [[faster_linking | Faster Linking]].
- [[performance | Performance]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/245)

- [`Cargo`][c-cargo]⮳{{hi:Cargo}}'s Incremental Compilation: This is the primary mechanism for incremental compilation in Rust. It's built into [Cargo][p-cargo] and generally "just works" automatically. See [[cargo | Cargo]].

- Understanding Incremental Compilation: Knowing how [Cargo][p-cargo]'s incremental compiler works ([caching][p-caching], invalidation) is key to maximizing its benefits. Changes to dependencies, function signatures, or certain code structures can invalidate the cache.

- Minimizing Cache Invalidation: This is the main focus when trying to improve incremental compilation. Strategies include:

- Keeping dependencies stable.
Structuring code to minimize changes that trigger recompilation (e.g., separating interface and implementation).
Being mindful of how [generics][p-generics] and macros can affect recompilation.
Build Profiles: Release builds can sometimes behave differently with incremental compilation due to optimizations.

`cargo tree` is also a useful tool for dependency analysis.

- Debugging Incremental Compilation: Sometimes, unexpected recompilation can occur. [Cargo][p-cargo]'s verbose output can help diagnose these situations. There aren't specific crates for this, but careful observation of compiler output is key.

</div>
