# GitHub Actions workflows

| File | Trigger | Purpose |
|---|---|---|
| `build_deploy_book.yml` | push / PR to `develop`, manual, `workflow_call` | Linux, in Docker. PRs: build, test, lint and render the book. Pushes: build and push the CI image, run tests in two partitions, build the book, upload the Pages artifact. A manual run with `deploy_to_gh_pages` also deploys to GitHub Pages. |
| `build_book_code_and_book_on_windows.yml` | push / PR to `develop`, manual | Native Windows build, lint and test of `bk/crates` and `tools/`, then renders the book. |
| `build_tools.yml` | push / PR to `develop`, manual | Builds and tests the `tools/` workspace in the `local-tool-builder` Docker stage. |
| `test-postgres-example.yml` | PR / push touching `bk/crates/cats/database` | Runs the PostgreSQL example against a service container. |
| `clean_cache.yml` | manual | Deletes the repository's GitHub Actions caches. |

To deploy the book, run **Build the book code and deploy the book to GitHub Pages (Linux)** manually with
`deploy_to_gh_pages` enabled. (The former `deploy_book.yaml` wrapper is gone: it duplicated that input and its
called job lacked the Pages permissions.)

## Shared composite actions (`../actions`)

| Action | Role |
|---|---|
| `load-pinned-versions` | Exports `RUST_VERSION`, `MDBOOK_*`, `CARGO_NEXTEST_VERSION`, `JUST_VERSION` from `.devcontainer/.env`, the single source of truth shared with the Docker image. |
| `setup-rust` | Installs the pinned stable toolchain and the pinned nightly used only for `rustfmt`, then restores the Cargo cache. Identical in `book_tooling` and `mdbook-utils`. |
| `set-cache-scope` | Sets `GHA_CACHE_SCOPE`, the Docker layer cache scope used by `.devcontainer/compose-ci.yaml`. |
| `docker-login` | Docker Hub login; the only place that pins `docker/login-action`. |

## Conventions

- **Pinning.** Third-party actions are pinned to a full commit SHA with the version in a comment.
  Dependabot (`github-actions`, including `.github/actions/*`) proposes updates. Do not use floating tags.
- **Tools.** Prebuilt binaries come from `taiki-e/install-action`; the unpinned `cargo-binstall` bootstrap script is gone.
  `mdbook-indexing` and `mdbook-utils` are installed with `cargo install --locked` and cached with `~/.cargo/bin`.
- **Shell.** `run` steps use `bash`, including on Windows.
- **Compiler wrapper.** `.cargo/config.toml` enables `sccache`; CI sets `RUSTC_WRAPPER=""` and relies on `rust-cache`.
- **Permissions.** `contents: read` by default; only the Pages deploy job gets `pages: write` and `id-token: write`.
- **Toolchain bumps.** Update `.devcontainer/.env` (`RUST_VERSION`) and, for the nightly, the `+nightly-...`
  occurrences and the default in `../actions/setup-rust/action.yml`.

## Checking workflows locally

```sh
pip install actionlint-py && actionlint
```
