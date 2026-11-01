# Repository configuration

Reviewed 2026-11-01. This is the consistency plan and reference for `rust_howto`.
Keep this file, README, contributor guidance, and configuration comments synchronized.

## Consistency plan

1. Share LF/UTF-8 editing defaults and language formatting conventions.
2. Pin nightly Rustfmt to `nightly-2026-10-22` and align lint baselines while preserving stricter rules, domain limits, and teaching examples.
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
| `pyproject.toml`, `uv.lock`                 | Ruff uses 100 columns and LF; explicit default E4/E7/E9/F lint selection. Keep project Python pins and locked resolution.                                                    |
| `prek.toml`                                 | Keep the existing repository hook set and pinned hook revisions. Run the configured formatter and linter rather than duplicating policy in hooks.                            |
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

Rust-cache entries include toolchain/environment inputs plus workflow, job, architecture, and
Cargo-config/lockfile hashes. Save Rust artifact caches only on pushes to the default branch;
pull requests restore caches. CI sets matching `CODE_*` and `CARGO_*` paths under `.ci/target/<repo>/main` and
`.ci/build/<repo>/main`. Rust-cache workspace maps use those same absolute paths,
including intermediate artifacts, so direct Cargo and task runners reuse one output tree.
Keep specialized coverage, target, feature, and release jobs isolated. sccache is pinned where
installed, and incremental compilation stays disabled, including when using GitHub caches.
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

| Profile                                | Workspace code                                           | Dependencies and build tools                                                                                                                 |
| -------------------------------------- | -------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| `dev` / `test`                         | `opt-level = 0`, line tables, LTO off, 256 codegen units | `opt-level = "s"`, no debug info, 16 units; selected hashing, collections, math, regex, parallelism, allocator and macro crates use level 3. |
| `debugging`                            | Level 0 and full symbols                                 | Level 0 and full symbols, including build scripts/procedural macros; intentionally uses a separate cache variant.                            |
| `ci`                                   | Inherits `dev`, incremental disabled                     | Same flags as `dev` for dependency cache reuse. Use release jobs for performance validation.                                                 |
| `tools`                                | Level 2 for repeatedly executed repository tools         | Same dependency flags as `dev`; build overrides request level 3 with no debug info; named package overrides take precedence.                                              |
| `release` / other specialized profiles | Preserve existing shipping/profiling choices             | Preserve existing shipping/profiling choices.                                                                                                |

`test` inherits `dev`. Package overrides take precedence over build overrides; macro parsing
crates are tuned explicitly. Optimization can increase cold compile time, and size optimization
is not guaranteed to make every crate smaller. Matching profile flags improves cache eligibility;
compiler versions, target/features, source paths, native tools, and rustflags also affect hits.
The `debugging` profile intentionally rebuilds dependencies to make them debuggable.

Run `just build-setup` once per checkout. It creates ignored `target`, `build`, and `.sccache`
links and refuses to replace existing files, directories, or links to another slot. PowerShell 7
is required by `scripts/cargo-host.ps1`. Windows uses junctions and NTFS intermediates; Unix
uses native cache roots and symlinks. Existing Rust How-To setup recipes also provide native
shell setup. Do not copy platform-specific links between Windows and Linux checkouts.

The existing `just cargo <subcommand> ...` runner sets absolute `CODE_TARGET_ROOT/<repo>/<slot>`
and `CODE_BUILD_ROOT/<repo>/<slot>` paths. Direct Cargo and rust-analyzer inherit checkout-local
`target`/`build` config paths, which resolve through those links. `cargo.targetDir = null` prevents
rust-analyzer from allocating an additional target tree. Inherited `CARGO_*` environment variables
can override these settings; launch the editor without another checkout's output variables.
Multiple independent workspaces share a checkout's output slot and Cargo locking.

Build scripts/macros are optimized because they execute during compilation. Cargo gives
package overrides precedence over build overrides, so unspecified external build dependencies
use the size-optimized wildcard and explicitly tuned macro parsing crates use level 3. sccache caches
eligible library compilations, not linking of binaries, dynamic libraries, or procedural macros.
Keep the shared 35 GiB cache and stable per-slot directories. Inspect running Cargo/rustc
processes before restarting sccache. Clean only demonstrated stale/corrupt content, never by
recursively deleting a checkout link or another active slot.

Select profiles with `just cargo build --profile debugging` or `--profile tools`; ordinary
build/test uses `dev`/`test`. Select a Cargo profile in nextest with `--cargo-profile ci`, independently
of nextest's test-runner `--profile ci`.

## Linkers

| Platform         | Policy                                                                                                                                                                                           |
| ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Windows MSVC     | Rust-bundled `rust-lld.exe` in the maintained repos; preserve stack flags. Rust How-To retains its measured default-linker exception. MSVC libraries/SDK are still required.                     |
| x86-64 GNU/Linux | Rust 1.90+ already defaults to LLD. Existing mold users retain mold and its installation steps. For AArch64/other triples, explicitly install/configure the appropriate linker before switching. |
| macOS            | Use the modern Apple linker shipped with Xcode 15+. sold's maintainer now recommends Apple's linker.                                                                                             |
| Experiments      | Compare mold and Wild on representative native Linux binaries. Wild lists Linux/Wasm support and future Mach-O/Windows/incremental-linking work; do not assume universal drop-in support.        |

Changing linker flags invalidates compiler cache keys. Benchmark cold and warm links with native
dependencies, debug symbols, and release/LTO settings before adopting a new default.

## Validation evidence

Validation results and remaining prerequisites are recorded here before the topic is committed.
No cross-platform performance improvement is claimed from configuration inspection alone.

## References

- [Cargo profiles](https://doc.rust-lang.org/cargo/reference/profiles.html) and
  [configuration precedence](https://doc.rust-lang.org/cargo/reference/config.html).
- [sccache Rust limitations](https://github.com/mozilla/sccache/blob/main/docs/Rust.md).
- [Dependabot options](https://docs.github.com/en/code-security/reference/supply-chain-security/dependabot-options-reference).
- [Rust 1.90 LLD default](https://blog.rust-lang.org/2025/09/01/rust-lld-on-1.90.0-stable/),
  [LLD platforms](https://lld.llvm.org/), [mold](https://github.com/rui314/mold),
  [sold recommendation](https://github.com/bluewhalesystems/sold), and [Wild support](https://github.com/wild-linker/wild).
- [rust-analyzer target directory](https://rust-analyzer.github.io/book/configuration.html#rust-analyzercargotargetdir).
