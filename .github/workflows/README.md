# GitHub Actions workflows

| File                                      | Trigger                                                                                      | Purpose                                                                                                                                                                                                                                         |
| ----------------------------------------- | -------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `build_deploy_book.yml`                   | push / PR to `develop`, manual, `workflow_call`                                              | Linux, in Docker. PRs: build, test, lint and render the book. Pushes: build and push the CI image, run tests in two partitions, build the book, upload the Pages artifact. A manual run with `deploy_to_gh_pages` also deploys to GitHub Pages. |
| `build_book_code_and_book_on_windows.yml` | push / PR to `develop`, manual                                                               | Native Windows build, lint and test of `bk/crates` and `tools/`, then renders the book.                                                                                                                                                         |
| `build_book_code_on_macos.yml`            | push / PR to `develop` touching `bk`, CI configs, scripts, Cargo files, manual               | Native macOS build, lint, format check, dependency audit, and test execution for `bk/crates`.                                                                                                                                                  |
| `build_tools.yml`                         | push / PR to `develop`, manual                                                               | Builds and tests the `tools/` workspace in the `local-tool-builder` Docker stage.                                                                                                                                                               |
| `build_independent_workspaces.yml`        | push / PR to `develop` touching `xmpl`, `playground`, `publish`, `scripts`, `.cargo`, manual | Linux matrix over the independent `xmpl`, `playground` and `publish` workspaces: format, Clippy, tests and doctests, each from its own workspace directory.                                                                                     |
| `check_staging_workspace.yml`             | weekly, manual                                                                               | Advisory `cargo check` of the staging workspace `later/crates`; isolated and `continue-on-error`, never a required check.                                                                                                                       |
| `audit_dependencies.yml`                  | push / PR to `develop` touching manifests, locks, or deny policies, weekly, manual           | Audits dependencies with `cargo deny` across `bk/crates`, `later/crates`, `tools`, and `xmpl`.                                                                                                                                                  |
| `native_platform_examples.yml`            | push / PR to `develop` touching `later` OS APIs or FFI, manual                                | Advisory lint and test runs for native platform examples: macOS APIs and Objective-C integration on macOS, and FreeBSD APIs in a VM.                                                                                                           |
| `test-allocator-examples.yml`              | push / PR to `develop` touching `bk/crates/cats/memory_management`, manual                   | Tests custom allocator selection (jemalloc and mimalloc) and verifies conflicting allocator feature detection on Linux.                                                                                                                        |
| `test-postgres-example.yml`               | PR / push touching `bk/crates/cats/database`                                                 | Runs the PostgreSQL example against a service container.                                                                                                                                                                                        |
| `clean_cache.yml`                         | manual                                                                                       | Deletes the repository's GitHub Actions caches.                                                                                                                                                                                                 |

To deploy the book, run **Build the book code and deploy the book to GitHub Pages (Linux)** manually with
`deploy_to_gh_pages` enabled.

## Shared composite actions (`../actions`)

| Action                 | Role                                                                                                                                                                |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `load-pinned-versions` | Exports `RUST_VERSION`, `MDBOOK_*`, `CARGO_NEXTEST_VERSION`, `JUST_VERSION` from `.devcontainer/.env`, the single source of truth shared with the Docker image.     |
| `setup-rust`           | Installs the pinned stable toolchain and the pinned nightly used only for `rustfmt`, then restores the Cargo cache. Identical in `book_tooling` and `mdbook-utils`. |
| `set-cache-scope`      | Sets `GHA_CACHE_SCOPE`, the Docker layer cache scope used by `.devcontainer/compose-ci.yaml`.                                                                       |
| `docker-login`         | Docker Hub login; the only place that pins `docker/login-action`.                                                                                                   |

## Conventions

- **Pinning.** Third-party actions are pinned to a full commit SHA with the version in a comment.
  Dependabot (`github-actions`, including `.github/actions/*`) proposes updates. Do not use floating tags.
- **Tools.** Prebuilt binaries come from `taiki-e/install-action`; the unpinned `cargo-binstall` bootstrap script is gone.
  `mdbook-indexing` and `mdbook-utils` are installed with `cargo install --locked` and cached with `~/.cargo/bin`.
- **Shell.** `run` steps use `bash`, including on Windows.
- **Compiler wrapper.** `.cargo/config.toml` enables `sccache`; CI sets `RUSTC_WRAPPER=""` and relies on `rust-cache`.
- **Permissions.** `contents: read` by default; only the Pages deploy job gets `pages: write` and `id-token: write`.
- **Toolchain bumps.** Update `.devcontainer/.env` (`RUST_VERSION`) and `rustfmt-toolchain.toml`. Workflows and Just
  recipes read the nightly formatter pin from that file.

## Checking workflows locally

```sh
pip install actionlint-py && actionlint
```
