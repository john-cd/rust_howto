<!-- TODO Reviewed ??-??-2026 -->

# Build Caches

{{#include build_cache.incl.md}}

## Use `sccache` {#sccache}

{{#badges sccache command-line-utilities="Command line utilities" development-tools::build-utils="Build Utils"}}{{hi:Ccache}}

Cargo creates build output that can be reused between invocations, but cache paths and wrapper settings determine what is retained. Cargo's `target` directory holds final build outputs; the separate `build` directory stores intermediate artifacts. A compiler cache can reuse compatible compiler outputs across workspaces or machines.

[`sccache`][c~sccache~repo]↗ is a compiler wrapper that reuses compatible compilation results. Configure Cargo to call it for Rust compilations:

```toml
[build]
rustc-wrapper = "sccache"
```

Set `SCCACHE_DIR` to choose its local cache directory. For remote caches, configure the backend and credentials described by the [sccache documentation][c~sccache~repo]↗. `SCCACHE_BASEDIRS` can normalize absolute source paths across checkouts. Rust incremental compilation can prevent rustc outputs from being cached, so consider setting `CARGO_INCREMENTAL=0` when relying on sccache.

## Choose Cargo Output Directories {#target-and-build-directories}

Cargo uses `CARGO_TARGET_DIR` (or `build.target-dir` in configuration) for final build outputs and `CARGO_BUILD_BUILD_DIR` (or `build.build-dir`) for intermediate build artifacts. Keeping these paths stable lets later builds reuse their results; separate directories can also isolate toolchains or checkouts when needed.

```toml
[build]
target-dir = "target"
build-dir = "build"
```

See the [Cargo build cache reference][book~cargo~build-cache]↗ for defaults and configuration details.

## Cache Cargo Builds in GitHub Actions {#cache-in-ci}

For GitHub Actions workflows, [`Swatinem/rust-cache`][rust-cache~repo]↗ caches Cargo registry data and build output between runs. Pin the action to a reviewed version or commit according to your workflow's dependency policy. A hosted cache is useful for CI; `sccache` is a separate compiler-output cache and can also use remote storage.

## Related Topics {#related-topics .skip}

- [[cargo | Cargo]].
- [[caching | Caching]].
- [[cd_ci | Continuous Delivery and Integration]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
