# Repository configuration

Reviewed 2026-11-01. This is the consistency plan and reference for `rust_howto`.

<!-- TODO Reviewed ??-??-2026 -->

Keep this file, README, contributor guidance, and configuration comments synchronized.

## Consistency plan

1. Share LF/UTF-8 editing defaults and language formatting conventions.
2. Pin nightly Rustfmt to `nightly-2026-11-11` and align lint baselines while preserving stricter rules, domain limits, and teaching examples.
3. Enumerate maintained dependency roots and use predictable monthly grouped updates.
4. Isolate checkout outputs, align dependency compiler flags, and make CI caches configuration-aware.
5. Validate format/lint and representative builds; measure cold/warm time and output size before
   promoting further optimization changes. Keep platform and integration blockers visible in TODO.

## File ownership

| Files                                       | Convention                                                                                                                                                                   |
| ------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `.editorconfig`, `.gitattributes`           | UTF-8 and LF, four-space Rust/Python, two-space configuration; preserve Markdown hard breaks and binary attributes.                                                          |
| `.gitignore`, `.dockerignore`, `.aiexclude` | Exclude outputs, compiler caches, environments, and worktrees; keep maintained configuration and lockfiles reviewable. Ignore files are not a security boundary.             |
| `AGENTS.md`, `CONTRIBUTING.md`, `README.md` | Keep repository-specific commands, ownership, and examples; link this reference.                                                                                             |
| `Cargo.toml`, `clippy.toml`, `rustfmt.toml` | Workspace lint inheritance and a common baseline; preserve explicit exceptions and package compatibility requirements.                                                       |
| `pyproject.toml`, `uv.lock`                 | Ruff uses 100 columns and LF; explicit default E4/E7/E9/F lint selection. Keep project Python pins and locked resolution. Quality tools use a separate locked `lint` group.  |
| `prek.toml`                                 | Keep the existing repository hook set and pinned hook revisions. Scope language tools to their files; the Rust formatter runs only when Rust source is staged.               |
| `deny.toml`, `nextest.toml`                 | Keep dependency-license exceptions, test filters, and resource/time limits local. Nextest normally discovers `.config/nextest.toml`; a Cargo profile is selected separately. |
| `.github/`, `.devcontainer/`, `Dockerfile*` | Preserve deployment/platform behavior; align package coverage and cache inputs. Keep compiler/Python pins compatible with manifests and lockfiles.                           |

## Dependency updates

Dependabot runs monthly on Monday at 6:00 a.m. Pacific with grouped minor/patch updates. These are
the maintained roots; workspace members share their root manifest/lockfile. Immutable parser,
test-book, and malformed-manifest fixtures are excluded intentionally. Revisit this table and
`.github/dependabot.yml` whenever a manifest, Dockerfile, chart, or local action is added.
Python uv workspace members share the root lock; they do not need overlapping update entries.

| Ecosystem        | Roots                                                                                                                                                                                                                                                                                                                                                                                                                     |
| ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `cargo`          | `/bk/crates`, `/bk/crates/cats/database/examples/query_builders_orms/toasty_validation`, `/bk/crates/cats/memory_management/allocator_selection`, `/later/crates`, `/later/crates/cats/development_tools_ffi/python_extension`, `/later/crates/cats/embedded`, `/later/crates/cats/gui/vizia`, `/later/crates/cats/wasm/examples/wasm_standalone_runtimes/components/guest`, `/playground`, `/publish`, `/tools`, `/xmpl` |
| `uv`             | `/later/crates/cats/development_tools_ffi/python_extension`                                                                                                                                                                                                                                                                                                                                                               |
| `docker`         | `/.devcontainer`                                                                                                                                                                                                                                                                                                                                                                                                          |
| `docker-compose` | `/.devcontainer`                                                                                                                                                                                                                                                                                                                                                                                                          |
| `devcontainers`  | `/`                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `github-actions` | `/`, `/.github/actions/docker-login`, `/.github/actions/load-pinned-versions`, `/.github/actions/set-cache-scope`, `/.github/actions/setup-rust`                                                                                                                                                                                                                                                                          |
| `gitsubmodule`   | `/`                                                                                                                                                                                                                                                                                                                                                                                                                       |

GitHub Actions uses `/` for workflows and explicit directories for nested composite actions.
Dependabot can update supported toolchain files, container images/features, charts, and packages;
it does not update arbitrary version strings in shell scripts or custom formatter-pin files.

## CI caches

Rust-cache entries include toolchain/environment inputs plus workflow, job, architecture, Cargo
manifest/lockfile/configuration, and workflow-file hashes. The workflow hash captures target,
feature, and profile choices made by the build commands. Save Rust artifact caches only on pushes to the default branch;
pull requests restore caches. CI sets matching `CODE_*` and `CARGO_*` paths under `.ci/target/<repo>/main` and
`.ci/build/<repo>/main`. Rust-cache workspace maps and its intermediate-artifact directory use
the same paths as Cargo. Specialized jobs may choose separate outputs when their cache mappings
use those same paths. `setup-rust` installs pinned sccache before Rust commands, and incremental
compilation stays disabled with both sccache and GitHub artifact caches.
An artifact cache does not make Rust incremental compilation compatible with sccache.
Python workflows use setup-uv caching keyed by `uv.lock`, and commands retain `--locked`.
Container builds retain their existing cargo-chef/BuildKit strategy; when cooking dependencies,
use the same compiler, features, profile, target, flags, and build paths as the application build.

## Cargo profiles and output paths

`.cargo/config.toml` owns shared development profiles. Child Cargo configs keep only workspace
environment/target requirements and package tuning for independent dependency graphs. Cargo
config discovery starts at the command's working directory; run from the owning workspace,
especially when that workspace has a child config. Config values override manifest profiles,
and environment/command-line overrides take precedence over config values.

| Profile                                | Workspace code                                           | Dependencies and build tools                                                                                                              |
| -------------------------------------- | -------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| `dev` / `test` | Level 0, no symbols, incremental disabled | Level 1 by default, selected compute-heavy dependencies at level 3, no symbols. |
| `debug` | Level 2 symbols for workspace crates | Same optimized dependency settings as `dev`, with dependency symbols disabled. |
| `ci`                                   | Inherits `dev`, incremental disabled                     | Same flags as `dev` for dependency cache reuse. Use release jobs for performance validation.                                              |
| `tools` | Level 2 for repository tools | Same dependency settings as `dev`. |
| `release` / other specialized profiles | Preserve existing shipping/profiling choices             | Preserve existing shipping/profiling choices.                                                                                             |

`test`, `ci`, `debug`, and `tools` inherit `dev`. Cargo checks an exact package override first, then
the wildcard package override, then the build override. Independent `bk/crates`, `later/crates`,
and `tools` workspaces keep exact level-3 overrides for selected math, matrix, concurrency,
allocator, and compile-time dependencies. These overrides use no dependency debug symbols across
profiles so optimized dependency artifacts remain reusable. Higher optimization can increase cold
compile time, so keep named exceptions focused on measured workloads. Matching profile flags
improves cache eligibility; compiler versions, target/features, source paths, native tools, and
rustflags also affect hits.

An effective unit-graph spot check on 2026-11-02 used Rust 1.99.1 for builds and
`nightly-2026-11-11` for Cargo's unstable graph output, on `x86_64-pc-windows-msvc`. From `tools/`,
the command below was run for each listed profile:

```powershell
foreach ($profile in @('dev', 'test', 'debug', 'ci', 'tools')) {
    cargo +nightly-2026-11-11 -Z unstable-options build --unit-graph --locked --offline -p mdbook-scrub --profile $profile
}
```

Each graph contained 133 units and 3 roots. For `dev`, `test`, and `ci`, the 3 workspace units had
level 0, line tables, LTO off, 256 codegen units, and incremental off; 13 named dependency units
had level 3 and no debug info; the remaining 117 units used size optimization and no debug info.
Build scripts and procedural macros may report no codegen-unit setting. `tools` had the same
dependency flags and level-2 workspace units. All 133 `debugging` units had level 0, full symbols,
and incremental off. This validates one package graph in `tools`; other package graphs and hosted
platforms still need their own checks.

For `mdbook-scrub` with the `ci` profile, an isolated Windows measurement used Rust 1.99.1,
sccache 0.19.0, fresh target/build roots, and a dedicated local sccache server/cache. The cold build
took 116.13 seconds, wrote 21,867,284 bytes, and had 102 cache misses with no hits. After cleaning
only that target's `ci` profile outputs, two same-path warm replays took 97.99 and 46.84 seconds;
each had 90 cache hits and 12 misses among 102 cacheable compiler calls, with the same output size.
The isolated cache reported no cache errors or timeouts. Another run with a different target path
had no hits. The warm timing variance and other concurrent machine builds make these single-package
measurements unsuitable for claiming a general performance improvement.

Run `just build-setup` once per checkout. It creates ignored `target`, `build`, and `.sccache`
links and refuses to replace existing files, directories, or links to another slot. PowerShell 7
is required by `scripts/cargo-host.ps1`. Windows uses junctions and NTFS intermediates; Unix
uses native cache roots and symlinks. Existing Rust How-To setup recipes also provide native
shell setup. Do not copy platform-specific links between Windows and Linux checkouts.

The existing `just cargo <subcommand> ...` runner sets absolute `CODE_TARGET_ROOT/<repo>/<slot>`
and `CODE_BUILD_ROOT/<repo>/<slot>` paths. Direct Cargo and rust-analyzer use the checkout-local
`target`/`build` paths, which resolve through those links. With
`rust-analyzer.cargo.targetDir = null`, rust-analyzer follows Cargo metadata's target directory
rather than allocating another target tree. Inherited `CARGO_*` environment variables can override
these settings; launch the editor without another checkout's output variables.

On Windows slot `a` (2026-11-09), `rust-analyzer analysis-stats --disable-build-scripts .`
completed from `tools/` (32 crates, 12,783 lines of code, and four failed MIR bodies, reported as
0%). Cargo metadata for that workspace reported
`D:\code\rust_howto\.worktrees\a\target`, a junction to `D:\target\rust_howto\a`. The run did not
create `tools/target` or `tools/build`; build scripts were disabled, so it emitted no compile
artifacts.
Multiple independent workspaces share a checkout's output slot and Cargo locking. On Windows,
slot `b` setup ran twice successfully; its `target`, `build`, and `.sccache` junctions resolved to
the configured central paths. A mismatched target override was rejected without changing the
junction. The build root was confirmed as NTFS (the target root is ReFS, which is allowed). The
direct Cargo `ci` build above wrote into those central paths. With elevated process access, the Just
base-directory probe and `build-setup` passed, and evaluation confirmed that `CODE_TARGET_ROOT` and
`CODE_BUILD_ROOT` overrides select the requested slot paths. `just cache-setup` correctly refused
to restart sccache while other Cargo/rustc processes were active; retry only after those builds end.
The default sandbox blocks the Git Bash child processes used by Just, so these runner checks require
elevated process access in this environment. The rust-analyzer target-dir configuration was
inspected, but an actual rust-analyzer invocation was not tested.

The level-3 build override applies to build scripts, procedural macros, and their dependencies
only when no package override matches. The wildcard package override matches non-workspace
dependencies first, including external build dependencies. Confirm effective unit-graph flags
before describing a dependency group as specially optimized. sccache caches eligible library
compilations, not linking of binaries, dynamic libraries, or procedural macros. Keep the shared
45 GiB cache and stable per-slot directories. Inspect running Cargo/rustc processes before
restarting sccache. Clean only demonstrated stale/corrupt content, never by recursively deleting a
checkout link or another active slot.

Select profiles with `just cargo build --profile debug` or `--profile tools`; ordinary
build/test uses `dev`/`test`. Select a Cargo profile in nextest with `--cargo-profile ci`, independently
of nextest's test-runner `--profile ci`.

## Linkers

| Platform         | Policy                                                                                                                                                                                                                                        |
| ---------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Windows MSVC     | Rust How-To keeps the MSVC toolchain default; an earlier `rust-lld` comparison showed no gain. Preserve stack flags. MSVC libraries/SDK are still required. The invoked linker was captured on 2026-11-09; see the validation evidence below. |
| x86-64 GNU/Linux | Rust 1.90+ already defaults to LLD. Existing mold users retain mold and its installation steps. For AArch64/other triples, explicitly install/configure the appropriate linker before switching.                                              |
| macOS            | Use the modern Apple linker shipped with Xcode 15+. sold's maintainer now recommends Apple's linker.                                                                                                                                          |
| Experiments      | Compare mold and Wild on representative native Linux binaries. Wild lists Linux/Wasm support and future Mach-O/Windows/incremental-linking work; do not assume universal drop-in support.                                                     |

Changing linker flags invalidates compiler cache keys. Benchmark cold and warm links with native
dependencies, debug symbols, and release/LTO settings before adopting a new default.

## Validation evidence

The `tools` unit graph was inspected with Cargo `1.99.1` and `nightly-2026-11-11` on Windows slot
`b`, using `cargo +nightly-2026-11-11 -Z unstable-options build --unit-graph --locked --offline
--manifest-path tools/Cargo.toml -p mdbook-scrub --profile <dev|test|ci|tools|debugging>`. The
`dev`, `test`, and `ci` workspace units use level 0, line tables, and 256 codegen units; their
dependency units use size optimization, no debug info, and 16 codegen units. `tools` workspace
units use level 2 with line tables and 256 codegen units. `debugging` units use level 0, full
symbols, and disabled incremental compilation. For this package graph, 13 named dependency units
receive level 3 before the wildcard override selects size optimization for other dependencies.
The full package-specific lists differ by workspace and are recorded in their local Cargo
configuration files.

On Windows slot `b`, `scripts/setup-build.ps1` reused the expected target/build/cache junctions
and refused an alternate `CODE_TARGET_ROOT` because the existing target junction points elsewhere.
`scripts/cargo-host.ps1 -BuildRoot D:/code_build -LinkOutputs` accepted the NTFS build volume and
existing checkout links. With elevated process access, the Just base-directory probe, `build-setup`,
and custom `CODE_TARGET_ROOT`/`CODE_BUILD_ROOT` evaluation passed. `just deny` passed all 13 configured
Cargo dependency-policy roots after the `pulldown-cmark` security update and scoped zip exceptions.
`just all ca` passed the Windows Clippy matrix for the initialized workspaces; the optional
`mdbook-utils` checkout was skipped because its submodule is uninitialized. The repository-wide
`dprint check` also passes. A native Ubuntu WSL smoke link with Rust 1.99.1 on
`x86_64-unknown-linux-gnu` invoked GCC's LLD 23.1.1; this confirms the compiler default, not the
repository's full Linux workspace matrix. The Apple linker and macOS workspace checks remain
unverified because no macOS runner is available. Warm-cache timings varied substantially, so no
cross-platform performance improvement is claimed from these measurements.

On Windows slot `a`, `just cargo rustc --locked --package mdbook-scrub --bin mdbook-scrub --
--print link-args` selected Visual Studio 2022 Build Tools MSVC 14.44.35207's
`C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\14.44.35207\bin\HostX64\x64\link.exe`, not `rust-lld`.

The 2026-11-09 issue rerun initialized `mdbook-utils` at submodule revision
`821b2cfc1fe2b0d19ca63559826b8c3f412d25bb` and included it in `just all fa` and `just all ca`;
focused `just mdbook-utils fa`, `just mdbook-utils ca`, and `just mdbook-utils deny` all passed. In
slot `a`, the full `just all fa` and `just all ca` runs also failed in `bk/crates` rustfmt with
Windows filename-length error 206. On integrated `develop`, `just all ca` passed formatting but
Clippy could not spawn rustc for `web-sys` in `bk/crates`; it also reported `mem_forget` in the
Rustler example and `exhaustive_enums` in the UniFFI example under `later/crates`. The full root
`just deny` run passed its first three roots after the primary checkout's staged `actix-http`
upgrade to 3.20.3, then stopped in `toasty_validation` on RUSTSEC-2026-0459 for `libsqlite3-sys`
0.37.0. The existing [#2116](https://github.com/john-cd/rust_howto/issues/2116) tracks this
dependency compatibility blocker, so the root loop did not reach `mdbook-utils`; its focused deny
check passed. The top-level `deny` recipe now includes the optional submodule and skips it when it
is not initialized.

## References

- [Cargo profiles](https://doc.rust-lang.org/cargo/reference/profiles.html) and
  [configuration precedence](https://doc.rust-lang.org/cargo/reference/config.html).
- [Rust 1.90 release notes: LLD default on x86-64 Linux](https://blog.rust-lang.org/2025/09/18/Rust-1.90.0/).
- [sccache Rust limitations](https://github.com/mozilla/sccache/blob/main/docs/Rust.md).
- [Dependabot options](https://docs.github.com/en/code-security/reference/supply-chain-security/dependabot-options-reference).
- [Rust 1.90 LLD default](https://blog.rust-lang.org/2025/09/01/rust-lld-on-1.90.0-stable/),
  [LLD platforms](https://lld.llvm.org/), [mold](https://github.com/rui314/mold),
  [sold recommendation](https://github.com/bluewhalesystems/sold), and [Wild support](https://github.com/wild-linker/wild).
- [rust-analyzer target directory](https://rust-analyzer.github.io/book/configuration.html#rust-analyzercargotargetdir).
