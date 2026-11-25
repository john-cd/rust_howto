<!-- TODO Reviewed ??-??-2026. -->

# GitHub Actions workflows

| File                                      | Trigger                                                                                       | Purpose                                                                                                                                                                                                                                         |
| ----------------------------------------- | --------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `build_deploy_book.yml`                   | push / PR to `develop`, manual, `workflow_call`                                               | Linux, in Docker. PRs: render the book and validate changed code scopes. Pushes: build and push the CI image, compile/lint/test once when required, render the book without repeating code validation, upload the Pages artifact. A manual run with `deploy_to_gh_pages` also deploys to GitHub Pages. |
| `build_book_code_and_book_on_windows.yml` | push / PR to `develop`, manual                                                                | Native Windows build, lint and test of `bk/crates` and `tools/`, then renders the book.                                                                                                                                                         |
| `build_book_code_on_macos.yml`            | push / PR to `develop` touching `bk`, CI configs, scripts, Cargo files, manual                | Native macOS build, lint, format check, dependency audit, and test execution for `bk/crates`.                                                                                                                                                   |
| `build_tools.yml`                         | push / PR to `develop`, manual                                                                | Builds and tests the `tools/` workspace in the `local-tool-builder` Docker stage.                                                                                                                                                               |
| `build_independent_workspaces.yml`        | push / PR to `develop` touching `xmpl`, `publish`, `scripts`, `.cargo`, manual                | Linux matrix over the independent `xmpl` and `publish` workspaces: format, Clippy, tests and doctests, each from its own workspace directory.                                                                                                   |
| `check_staging_workspace.yml`             | weekly, manual                                                                                | Advisory `cargo check` of the staging workspace `later/crates`; isolated and `continue-on-error`, never a required check.                                                                                                                       |
| `audit_dependencies.yml`                  | push / PR to `develop` touching maintained manifests, locks, or deny policies, weekly, manual | Audits every maintained Cargo dependency root with `cargo deny`; immutable Cargo fixtures under `bk/scripts` are excluded.                                                                                                                      |
| `native_platform_examples.yml`            | push / PR to `develop` touching `later` OS APIs or FFI, manual                                | Advisory lint and test runs for native platform examples: macOS APIs and Objective-C integration on macOS, and FreeBSD APIs in a VM.                                                                                                            |
| `test-allocator-examples.yml`             | push / PR to `develop` touching `bk/crates/cats/memory_management`, manual                    | Tests custom allocator selection (jemalloc and mimalloc) and verifies conflicting allocator feature detection on Linux.                                                                                                                         |
| `test-postgres-example.yml`               | PR / push touching `bk/crates/cats/database`                                                  | Runs the PostgreSQL example against a service container.                                                                                                                                                                                        |
| `clean_cache.yml`                         | manual                                                                                        | Deletes the repository's GitHub Actions caches.                                                                                                                                                                                                 |

To deploy the book, run **Build the book code and deploy the book to GitHub Pages (Linux)** manually with
`deploy_to_gh_pages` enabled.

## Shared composite actions (`../actions`)

| Action                 | Role                                                                                                                                                                                  |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `load-pinned-versions` | Exports `RUST_VERSION`, `MDBOOK_*`, `CARGO_NEXTEST_VERSION`, `JUST_VERSION` from `.devcontainer/.env`, the single source of truth shared with the Docker image.                       |
| `setup-rust`           | Installs the pinned stable/nightly toolchains and sccache, then restores Cargo artifact caches for the configured target/build paths. Book Tooling has a related helper; mdbook-utils owns inline setup in its Rust CI. |
| `set-cache-scope`      | Sets `GHA_CACHE_SCOPE`, the Docker layer cache scope used by `.devcontainer/compose.ci.yaml`.                                                                                         |
| `docker-login`         | Docker Hub login; the only place that pins `docker/login-action`.                                                                                                                     |

## Conventions

- **Pinning.** Third-party actions are pinned to a full commit SHA with the version in a comment.
  Dependabot (`github-actions`, including `.github/actions/*`) proposes updates. Do not use floating tags.
- **Tools.** Prebuilt binaries come from `taiki-e/install-action`; the unpinned `cargo-binstall` bootstrap script is gone.
  `mdbook-indexing` and `mdbook-utils` are installed with `cargo install --locked` and cached with `~/.cargo/bin`.
- **Shell.** `run` steps use `bash`, including on Windows.
- **Compiler cache.** `setup-rust` installs pinned sccache before Rust commands. Rust-cache uses the same target/build paths as Cargo. Its key hashes Cargo manifests, lockfiles, Cargo configs, stable and formatter toolchains, the pinned `.devcontainer/.env` versions, local action and workflow inputs, and all Just recipes that select build commands, targets, features, or profiles. `CARGO_INCREMENTAL=0` keeps sccache-compatible builds. Cache writes are limited to pushes on the default branch. Jobs without the composite action disable the wrapper only in the job that lacks sccache.
- **Permissions.** `contents: read` by default; only the Pages deploy job gets `pages: write` and `id-token: write`.
- **Toolchain bumps.** Update `.devcontainer/.env` (`RUST_VERSION`) and `rustfmt-toolchain.toml`. Workflows and Just
  recipes read the nightly formatter pin from that file.

## Checking workflows locally

```sh
pip install actionlint-py && actionlint
```

## Book Tooling submodule

Jobs that build or audit `tools/` initialize only `book_tooling/` at the committed
gitlink using `checkout-book-tooling`. Cargo uses path dependencies from that
checkout, and Docker copies the same sources into its build context. CI never
uses `--remote` to advance the submodule.

The submodule is public, so CI can initialize it without a repository secret.
`BOOK_TOOLING_TOKEN` remains an optional input for authenticated access. For
local Docker builds, initialize the submodule with your Git credentials first.

## CI cost and local checks

Pushes containing only recognized prose changes skip Linux Rust validation; PRs still render
and spellcheck the book. Native Windows/macOS code workflows follow Rust/configuration paths.
Unknown files, fixtures, manifests, toolchains and workflow changes retain full validation.
The final Linux rendering job uses `VALIDATE_CODE=false`; this skips workspace audits, formatting,
compilation, Clippy and tests already handled by the validation job. Book-specific audits and
preprocessors still run. Runtime `/code/target` and `/code/build` are bind-mounted to the host
cache; the final job restores the same cache identity. Each CI image is tagged with its source commit
so parallel validations cannot pull another run's source snapshot. Validation cancels superseded runs,
while explicitly requested publication keeps the protected Pages concurrency group.

Trusted Windows static checks and optional tools-workspace compilation can be run through
[local_runners' manual workflow](https://github.com/john-cd/local_runners/actions/workflows/rust-repositories.yml).
It resolves the current `develop` SHA, runs only when dispatched by the owner, and serializes
the three Rust repositories on J-RAZER. Fork PRs remain on GitHub-hosted runners. Full local
checks require 15 GiB free on D: and use two Cargo workers; static checks remain available.

<!-- workflow-operations-reference:start -->

<!-- TODO Reviewed ??-??-2026. -->

## Workflow operations reference

The workflow YAML files own execution behavior. Their inline comments explain the purpose and
the boundaries that must survive future refactoring. This reference records the current job map;
repository-level Actions enable/disable settings remain independent of these tracked definitions.

### Changes in this review

- audit_dependencies.yml: Bound cargo-deny to 120 minutes.
- audit_dependencies.yml: Bound cargo-deny-other to 120 minutes.
- audit_dependencies.yml: Bound check-trigger-paths to 120 minutes.
- build_book_code_and_book_on_windows.yml: Bound build-book-code-on-windows to 120 minutes.
- build_tools.yml: Bound build to 120 minutes.
- clean_cache.yml: Added branch-scoped concurrency with obsolete-run cancellation.
- clean_cache.yml: Bound cleanup to 30 minutes.
- test-allocator-examples.yml: Added branch-scoped concurrency with obsolete-run cancellation.
- test-allocator-examples.yml: Bound allocator-examples to 120 minutes.
- test-postgres-example.yml: Added branch-scoped concurrency with obsolete-run cancellation.
- test-postgres-example.yml: Bound postgres-example to 120 minutes.

### [Audit dependencies](audit_dependencies.yml)

Events: `push, pull_request, schedule, workflow_dispatch`. Branch/path filters and manual defaults are defined in the linked file.

| Job | Runs on / reuse | Dependencies | Guard | Main checks or side effects |
| --- | --- | --- | --- | --- |
| check-trigger-paths | "ubuntu-24.04" | [] | Successful dependencies (default) | actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1; Check out Book Tooling; Set up Rust; cargo run --manifest-path tools/Cargo.toml --locked -p repo-audit -- sync-lints --check; cargo run --manifest-path tools/Cargo.toml --locked -p repo-audit -- audit-workflow-paths; cargo test --manifest-path tools/Cargo.toml --locked -p repo-audit |
| cargo-deny | "ubuntu-24.04" | [] | Successful dependencies (default) | actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1; EmbarkStudios/cargo-deny-action@3c6349835b2b7b196a839186cb8b78e02f7b5f25 |
| cargo-deny-other | "ubuntu-24.04" | [] | Successful dependencies (default) | actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1; Check out Book Tooling; EmbarkStudios/cargo-deny-action@3c6349835b2b7b196a839186cb8b78e02f7b5f25 |

### [Build book code (and book) on Windows](build_book_code_and_book_on_windows.yml)

Events: `push, pull_request, workflow_dispatch`. Branch/path filters and manual defaults are defined in the linked file.

| Job | Runs on / reuse | Dependencies | Guard | Main checks or side effects |
| --- | --- | --- | --- | --- |
| build-book-code-on-windows | "windows-2025" | [] | Successful dependencies (default) | Code checkout (clones the Git repository); Check the mdbook-utils pin against the submodule; Check out Book Tooling; Load pinned tool versions; Install OpenSSL for Vcpkg; Set up Rust (pinned stable + nightly rustfmt, cached); Install `protoc`, `jq`, NASM and Ninja (choco); Protoc version (debug); Check minimum supported Rust; Install mdBook, mdBook linkcheck, Nextest, Just and cargo-deny (prebuilt binaries); Install mdBook indexing from its locked source; Install `mdbook-utils`; Audit dependencies with cargo-deny (#51); Format; Fetch; Build; Clippy; Run tests and fail on process leaks; Test examples requiring Rust 1.100; Build / release the tools, incl. mdbook-scrub; Format, lint, build, and test helper workspace; Build the book |

### [Build book code on macOS](build_book_code_on_macos.yml)

Events: `push, pull_request, workflow_dispatch`. Branch/path filters and manual defaults are defined in the linked file.

| Job | Runs on / reuse | Dependencies | Guard | Main checks or side effects |
| --- | --- | --- | --- | --- |
| build-book-code-on-macos | "macos-15" | [] | Successful dependencies (default) | Code checkout; Check out Book Tooling; Load pinned tool versions; Set up Rust toolchains and cache; Install Protocol Buffers compiler; Install Nextest and cargo-deny; Audit dependencies with cargo-deny (#51); Format; Fetch; Build; Clippy; Run tests and fail on process leaks |

### [Build the book code and deploy the book to GitHub Pages (Linux)](build_deploy_book.yml)

Events: `workflow_call, push, pull_request, workflow_dispatch`. Branch/path filters and manual defaults are defined in the linked file.

| Job | Runs on / reuse | Dependencies | Guard | Main checks or side effects |
| --- | --- | --- | --- | --- |
| changes | "ubuntu-latest" | [] | Successful dependencies (default) | actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1; dorny/paths-filter@0e4a8c6effa4802afeda77dc8d303f8176d7dfad; Check CI entrypoint modes without compilation |
| validate-pr | "ubuntu-latest" | "changes" | ${{ github.event_name == 'pull_request' }} | Code checkout; Check out Book Tooling; Reject repository links to the empty main branch; Set build cache scope; Set up Docker Buildx; Build, test, lint, and render through Compose |
| build-image | "ubuntu-latest" | [] | ${{ github.event_name != 'pull_request' }} | Free Disk Space (Ubuntu); Code checkout (clones the Git repository); Check out Book Tooling; Set build cache scope; Login to Docker Hub; Set up Docker Buildx; Build, cache (type=gha), and push to DockerHub |
| test | "ubuntu-latest" | ["changes", "build-image"] | ${{ github.event_name != 'pull_request' && (github.event_name != 'push' \|\| needs.changes.outputs.code == 'true') }} | Code checkout; Set build cache scope; Install compiler cache; Rust Cache; Login to Docker Hub; Check portable GPU examples; Build, lint, and run all tests once; Restore cache ownership |
| build-book | "ubuntu-latest" | ["changes", "build-image", "test"] | ${{ always() && !cancelled() && github.event_name != 'pull_request' && needs.build-image.result == 'success' && (needs.test.result == 'success' \|\| needs.test.result == 'skipped') }} | Code checkout; Set build cache scope; Install compiler cache; Rust Cache; Login to Docker Hub; Build the book; Restore cache ownership; Upload artifacts |
| deploy | "ubuntu-latest" | "build-book" | ${{ github.event_name != 'pull_request' && inputs.deploy_to_gh_pages }} | Deploy to GitHub Pages |

### [Build independent workspaces on Linux](build_independent_workspaces.yml)

Events: `push, pull_request, workflow_dispatch`. Branch/path filters and manual defaults are defined in the linked file.

| Job | Runs on / reuse | Dependencies | Guard | Main checks or side effects |
| --- | --- | --- | --- | --- |
| independent-workspace | "ubuntu-24.04" | [] | Successful dependencies (default) | Code checkout; Load pinned tool versions; Set up Rust toolchains and cache; Check minimum supported Rust; Format; Fetch; Clippy; Tests; Doctests |

### [Build tools on Linux](build_tools.yml)

Events: `push, pull_request, workflow_dispatch`. Branch/path filters and manual defaults are defined in the linked file.

| Job | Runs on / reuse | Dependencies | Guard | Main checks or side effects |
| --- | --- | --- | --- | --- |
| build | "ubuntu-latest" | [] | Successful dependencies (default) | Code checkout (clones the Git repository); Check out Book Tooling; Set up Docker Buildx; Build local-tool-builder image; Run tools tests inside container |

### [Check staging workspace (advisory)](check_staging_workspace.yml)

Events: `schedule, workflow_dispatch`. Branch/path filters and manual defaults are defined in the linked file.

| Job | Runs on / reuse | Dependencies | Guard | Main checks or side effects |
| --- | --- | --- | --- | --- |
| staging-check | "ubuntu-24.04" | [] | Successful dependencies (default) | Code checkout; Load pinned tool versions; Set up Rust toolchains and cache; Check |

### [Cleanup GitHub Action caches](clean_cache.yml)

Events: `workflow_dispatch`. Branch/path filters and manual defaults are defined in the linked file.

| Job | Runs on / reuse | Dependencies | Guard | Main checks or side effects |
| --- | --- | --- | --- | --- |
| cleanup | "ubuntu-latest" | [] | Successful dependencies (default) | Cleanup |

### [Check native platform examples (advisory)](native_platform_examples.yml)

Events: `push, pull_request, workflow_dispatch`. Branch/path filters and manual defaults are defined in the linked file.

| Job | Runs on / reuse | Dependencies | Guard | Main checks or side effects |
| --- | --- | --- | --- | --- |
| macos | "macos-15" | [] | Successful dependencies (default) | Check out source; Load pinned tool versions; Set up Rust; Lint native macOS APIs; Test native macOS APIs; Lint Objective-C integration; Test Objective-C integration |
| freebsd | "ubuntu-24.04" | [] | Successful dependencies (default) | Check out source; Load pinned tool versions; Lint and test inside FreeBSD |

### [Test allocator examples](test-allocator-examples.yml)

Events: `pull_request, push, workflow_dispatch`. Branch/path filters and manual defaults are defined in the linked file.

| Job | Runs on / reuse | Dependencies | Guard | Main checks or side effects |
| --- | --- | --- | --- | --- |
| allocator-examples | "ubuntu-24.04" | [] | Successful dependencies (default) | Checkout; Load pinned tool versions; Set up Rust; Test jemalloc on Linux; Test allocator feature choices |

### [Test PostgreSQL book example](test-postgres-example.yml)

Events: `pull_request, push, workflow_dispatch`. Branch/path filters and manual defaults are defined in the linked file.

| Job | Runs on / reuse | Dependencies | Guard | Main checks or side effects |
| --- | --- | --- | --- | --- |
| postgres-example | "ubuntu-24.04" | [] | Successful dependencies (default) | Checkout; Load pinned tool versions; Set up Rust; Run the PostgreSQL CRUD and connection teardown test |

### Caches, permissions and failures

- Keep package downloads and compiler outputs separate. Match cache keys to compiler, OS, dependency locks, build profiles and features; a cache is an optimization, not a prerequisite for correctness.
- Matrix jobs start on separate machines. Build/test commands in one job reuse local outputs; moving them to separate jobs requires an explicit compatible artifact handoff.
- Read-only validation and live publication have different lifecycles. Preserve existing credentials, event/ref conditions, environments and non-cancelling deployment groups.
- Timeouts bound hung or unexpectedly expensive runs. Diagnose a timeout before increasing it; never convert a failed check into an unconditional success.
- A queued/disabled/billing-blocked run is not a passed check. Record hosted results separately from local workflow parsing/format validation.

### Local validation

```sh
actionlint
git diff --check
```

Use the repository formatter for edited YAML/Markdown. Run the commands named by the affected
jobs when their executable behavior changes; platform/container/credential-dependent checks
must be reported at the scope actually executed. Do not run publishing commands merely to test
a comment or validation-only change.

<!-- workflow-operations-reference:end -->
