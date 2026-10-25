<!-- TODO Reviewed ??-??-2026 -->

# Performance

{{#include performance.incl.md}}

| Topic                     | Cargo tool                              | Best fit                                                                                                       |
| ------------------------- | --------------------------------------- | -------------------------------------------------------------------------------------------------------------- |
| Choose a profile template | [`cargo-wizard`][c~cargo-wizard~repo]   | Compare starting points for compile time, runtime speed, or binary size. Review its edits before keeping them. |
| Unify workspace features  | [`cargo-hakari`][c~cargo-hakari~repo]   | Reduce duplicate dependency builds in a large workspace with a generated `workspace-hack` crate.               |
| Find runtime hot spots    | [`cargo flamegraph`][c~flamegraph~repo] | Sample a representative workload and visualize where it spends CPU time.                                       |
| Run tests efficiently     | [`cargo-nextest`][c~cargo-nextest~repo] | Schedule already compiled tests with a dedicated runner; this does not shorten compilation itself.             |
| Reuse Docker build layers | [`cargo-chef`][c~cargo-chef~repo]       | Cache dependency compilation between container image builds.                                                   |

## Choose a starting profile with `cargo-wizard` {#cargo-wizard}

[![cargo-wizard][c~cargo-wizard~docs~badge]][c~cargo-wizard~docs] [![cargo-wizard~crates.io][c~cargo-wizard~crates.io~badge]][c~cargo-wizard~crates.io] [![cargo-wizard~repo][c~cargo-wizard~repo~badge]][c~cargo-wizard~repo] [![cargo-wizard~lib.rs][c~cargo-wizard~lib.rs~badge]][c~cargo-wizard~lib.rs]{{hi:cargo-wizard}}{{hi:Profile}}{{hi:Cargo}}{{hi:Template}} [![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]{{hi:Cargo plugins}}

[`cargo wizard`][c~cargo-wizard~repo]↗{{hi:cargo-wizard}} is a [`cargo`][c~cargo~docs]↗{{hi:cargo}} subcommand that interactively selects or applies profile and configuration templates. Its `fast-compile`, `fast-runtime`, and `min-size` templates adjust Cargo profiles and, in some cases, `.cargo/config.toml` flags. Run `cargo wizard` to review the interactive choices or `cargo wizard apply fast-compile dev` to apply a template to a profile.

The templates are starting points, not universal settings. They can change optimization, debug information, linker selection, and other build behavior; inspect the resulting diff and benchmark it for the target workload. Some options require nightly Rust.

## Unify workspace dependency features with `cargo-hakari` {#cargo-hakari}

[![cargo-hakari][c~cargo-hakari~docs~badge]][c~cargo-hakari~docs] [![cargo-hakari~crates.io][c~cargo-hakari~crates.io~badge]][c~cargo-hakari~crates.io] [![cargo-hakari~repo][c~cargo-hakari~repo~badge]][c~cargo-hakari~repo] [![cargo-hakari~lib.rs][c~cargo-hakari~lib.rs~badge]][c~cargo-hakari~lib.rs]{{hi:cargo-hakari}}{{hi:Cargo}}{{hi:Dependencies}}{{hi:Feature-unification}}{{hi:Guppy}}{{hi:Workspace-hack}} [![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]{{hi:Cargo plugins}}

[`cargo-hakari`][c~cargo-hakari~repo]↗{{hi:cargo-hakari}} manages a generated `workspace-hack` package that unifies dependency features across workspace members. When several crates compile the same dependency with different feature sets, Cargo may compile multiple variants. A workspace-hack can make those feature sets consistent and reduce duplicate dependency work in large workspaces.

Initialize and configure the workspace-hack package with `cargo hakari init`. Review proposed changes with `cargo hakari generate --diff`, then update the generated package with `cargo hakari generate` and add its dependency to workspace members with `cargo hakari manage-deps`. Check the result with `cargo hakari verify`. Keep the generated dependency features synchronized in CI so changes do not reintroduce duplicate builds.

Hakari can reduce repeated dependency compilation, but the gain depends on the workspace's feature graph and build workload. It does not remove the initial cost of compiling dependencies or make every crate faster.

## Profile runtime hot spots with `cargo flamegraph` {#cargo-flamegraph}

[`cargo flamegraph`][c~flamegraph~repo]↗ samples a running program and renders a flame graph. Use the matching profiler for your platform, build with symbols, and profile a representative workload; a flame graph shows where samples accumulated, not a benchmark result by itself. For installation and examples, see [cargo flamegraph in the profiling guide](../development-tools_profiling/benchmarking.md#cargo-flamegraph).

## Run compiled tests with `cargo-nextest` {#cargo-nextest}

[`cargo-nextest`][c~cargo-nextest~repo]↗ is a test runner that schedules Rust test binaries and reports each test independently. It can shorten the test-run portion of the development loop in workspaces with many tests. The test code still has to compile first, and doctests use Cargo's separate doctest runner. See [the test runner guide](../development-tools_testing/test_runners.md#cargo-nextest) for installation and invocation.

## Cache dependency builds in Docker with `cargo-chef` {#cargo-chef}

[`cargo-chef`][c~cargo-chef~repo]↗ prepares a dependency recipe from a Cargo workspace and compiles that recipe in a Docker build layer. This lets Docker reuse compiled dependencies when application source changes without changing the manifests or lockfile. It is useful for container image builds; it does not replace local Cargo caching or accelerate a build when the dependency layer itself changes.

In a Dockerfile, `cargo chef prepare --recipe-path recipe.json` produces the recipe, and `cargo chef cook --release --recipe-path recipe.json` builds its dependency layer before copying and compiling the application source.

## Related Topics {#related-topics .skip}

- [[benchmarking | Benchmarking]] for runtime profiling and benchmarks.
- [[reduce_compilation_duration | Reduce Compilation Duration]] for compiler and Cargo build settings.
- [[cross_compilation | Cross-compilation]] for target toolchains.
- [[test_runners | Test Runners]] for test execution details.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
