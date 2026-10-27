<!-- TODO Reviewed ??-??-2026 -->

# GitHub Actions

{{#include github_actions.incl.md}}

GitHub Actions runs workflows stored in `.github/workflows/*.yml`. A workflow selects events and defines jobs that execute on runners. Steps within a job share a checkout and installed tools; separate jobs start with separate environments. See [CI/CD](cd_ci.md) for the path from validation to release.

## A Minimal Rust CI Workflow {#rust-ci}

Save this complete workflow as `.github/workflows/ci.yml` in a crate or workspace with `Cargo.toml` and a committed `Cargo.lock` at the repository root:

```yaml
name: Rust CI
on: [push, pull_request]

permissions:
  contents: read

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      # actions/checkout v7.0.1
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1
        with:
          persist-credentials: false
      # dtolnay/rust-toolchain: commit from master; select Rust separately.
      - uses: dtolnay/rust-toolchain@683b588c487a69d01f6c6ddf40f036ff682cc2c8
        with:
          toolchain: stable
          components: clippy, rustfmt
      # Swatinem/rust-cache v2.9.3
      - uses: Swatinem/rust-cache@d1527c8f67ef9af357290475d4af4b0fab8f12e0
        with:
          save-if: ${{ github.event_name == 'push' }}
      - name: Check formatting
        run: cargo fmt --all -- --check
      - name: Run Clippy
        run: cargo clippy --workspace --all-targets --locked -- -D warnings
      - name: Run tests and doctests
        run: cargo test --workspace --locked
```

[`dtolnay/rust-toolchain`][rust-toolchain~repo]↗ installs Rust through `rustup` and adds the requested components. Its concise `dtolnay/rust-toolchain@stable` form selects stable too; the example instead pins the action implementation and selects the compiler through `toolchain: stable`. Select a numeric compiler version for reproducibility. Use the project's formatter version, including a separate pinned nightly if it requires nightly-only Rustfmt options.

`cargo fmt` checks style without rewriting files. Clippy treats warnings as errors and checks examples and test targets as well as libraries and binaries. `cargo test --workspace` also runs doctests; it does not imply every feature combination is tested. Add feature-specific jobs for supported configurations. `--locked` prevents lockfile changes; projects without a committed lockfile must choose an explicit dependency-resolution policy instead.

For a nested workspace, set `defaults.run.working-directory` in the job and configure the cache's `workspaces` input. A `run` working directory does not change where an action operates. This book's [Windows workflow][rust-howto~windows-workflow]↗ runs Cargo separately in its independent `bk/crates` and `tools` workspaces.

## Test Across Operating Systems and Toolchains {#test-matrix}

This separate workflow tests stable and beta on Linux, macOS, and Windows. `fail-fast: false` lets other matrix entries finish when one configuration fails:

```yaml
name: Rust compatibility
on: [push, pull_request]

permissions:
  contents: read

jobs:
  test:
    name: Test (${{ matrix.os }}, ${{ matrix.toolchain }})
    runs-on: ${{ matrix.os }}
    strategy:
      fail-fast: false
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        toolchain: [stable, beta]
    steps:
      # actions/checkout v7.0.1
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1
        with:
          persist-credentials: false
      # dtolnay/rust-toolchain: commit from master
      - uses: dtolnay/rust-toolchain@683b588c487a69d01f6c6ddf40f036ff682cc2c8
        with:
          toolchain: ${{ matrix.toolchain }}
      # Swatinem/rust-cache v2.9.3
      - uses: Swatinem/rust-cache@d1527c8f67ef9af357290475d4af4b0fab8f12e0
        with:
          key: ${{ matrix.toolchain }}
          save-if: ${{ github.event_name == 'push' }}
      - run: cargo test --workspace --locked
```

Add the project's declared minimum supported Rust version (MSRV) as a quoted numeric toolchain value. Its lockfile format and dependencies must also work with that compiler. Add nightly for a supported configuration or a deliberate early-warning check. Decide whether beta and nightly failures block merges; every entry in this example is required.

Hosted runner labels select operating systems; they do not install project-specific native libraries. Install these explicitly or limit jobs to supported platforms. Keep formatting in one job to avoid repeating it across the matrix. See the [matrix documentation][github-actions~matrix~docs]↗ for `include`, `exclude`, and concurrency limits.

## Install Development Tools {#installation-of-development-tools}

[`taiki-e/install-action`][install_action~repo]↗{{hi:install-action}} installs utilities primarily from prebuilt releases, avoiding compilation on every fresh runner. The `tool` input accepts comma-separated names and `name@version` pins. Pin tools independently of the action revision when repeatability matters; consult its supported platforms and fallback behavior.

The following complete workflow assumes a committed `Cargo.lock` and a reviewed `deny.toml`. Create the latter locally with `cargo deny init`, set the project's license and dependency policies, and commit it before enabling this job:

```yaml
name: Rust tests and supply chain
on: [push, pull_request]

permissions:
  contents: read

jobs:
  audit:
    runs-on: ubuntu-latest
    steps:
      # actions/checkout v7.0.1
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1
        with:
          persist-credentials: false
      # dtolnay/rust-toolchain: commit from master
      - uses: dtolnay/rust-toolchain@683b588c487a69d01f6c6ddf40f036ff682cc2c8
        with:
          toolchain: stable
      # taiki-e/install-action v2.87.30
      - uses: taiki-e/install-action@a4d3f3f761dcd95cce1b72cb61cb6d1963438177
        with:
          tool: cargo-nextest,cargo-deny,cargo-audit,zizmor
      - name: Run tests with Nextest
        run: cargo nextest run --workspace --locked
      - name: Run doctests separately
        run: cargo test --workspace --doc --locked
      - name: Check dependency policy
        run: cargo deny --locked check
      - name: Check the lockfile for advisories
        run: cargo audit
      - name: Audit workflows without GitHub API access
        run: zizmor --offline .github/workflows
```

[`cargo-nextest`][nextest~running~docs]↗ runs test binaries with its own scheduler and reports. Keep the separate doctest command: stable Nextest does not run doctests.

## Cache Your Compilation {#compilation-caching}

[`Swatinem/rust-cache`][rust-cache~action~repo]↗{{hi:rust-cache}} caches Cargo downloads and build artifacts. Install the compiler before restoring the cache so its version contributes to the key. The action also accounts for manifests and lockfiles and disables incremental compilation for its CI cache strategy.

Use defaults for a root workspace. For a nested workspace, set `workspaces` to a mapping such as `bk/crates -> ../../target` when Cargo writes to a shared root `target` directory. Match any `CARGO_TARGET_DIR` override, and distinguish feature or build configurations with additional keys where needed. The [action's input reference][rust-cache~action~repo]↗ describes the options.

Cache misses must still yield correct builds. These examples save caches only on pushes; pull requests can restore existing entries. Keep secrets out of caches. Use [workflow artifacts][github-actions~workflow-artifacts~docs]↗ for reports and release files that must be retained or passed between jobs.

## Security and Supply-Chain Checks {#supply-chain-checks}

| Tool                               | What it checks                                                                          | Preparation                                                                           |
| ---------------------------------- | --------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------- |
| [`cargo-deny`][cargo-deny~guide]↗  | Dependency advisories, licenses, duplicate or banned dependencies, and allowed sources. | Review and commit `deny.toml`; keep advisory data current.                            |
| [`cargo-audit`][cargo-audit~repo]↗ | Resolved dependencies in `Cargo.lock` against RustSec advisories.                       | Commit the lockfile and review reports and exceptions.                                |
| [`zizmor`][zizmor~usage~docs]↗     | Security properties of workflow and action definitions.                                 | Run against `.github/workflows`; offline mode omits checks requiring GitHub API data. |

`cargo-deny` and `cargo-audit` overlap on advisories; decide whether both reports serve your policy. A clean scan means no matching known advisory was found, not that every dependency has been audited. Consider scheduled scans, since advisory databases change without a source commit.

The examples pin actions to full commit SHAs. Tags such as `@v2` and channels such as `@stable` can move; a full SHA fixes the action code being executed. For `dtolnay/rust-toolchain`, choose a commit in its `master` history and supply `toolchain` explicitly. Review upstream changes when updating pins. Pinning an installer does not pin downloaded tools, and `toolchain: stable` still advances the compiler. GitHub's [secure-use reference][github-actions~secure-use~docs]↗ explains these trust boundaries.

Give test jobs only `contents: read`, disable checkout credential persistence when pushes are unnecessary, and keep publishing credentials in separately controlled jobs. Do not execute untrusted pull request code in a privileged `pull_request_target` job. Pass untrusted event fields through environment variables instead of interpolating them directly into shell scripts.

## Automate GitHub from Rust {#automate-github-from-rust}

[`octocrab`][octocrab~api~docs]↗ is an asynchronous GitHub API client for Rust. Use it for a bot or integration that reads issues, manages pull requests, or queries Actions runs when workflow YAML alone is insufficient. Provide credentials through the runtime environment or a GitHub App installation token with narrow permissions. Handle API errors, pagination, and rate limits explicitly.

## Useful GitHub Actions {#useful-github-actions}

| Task                                            | Action                                           |
| ----------------------------------------------- | ------------------------------------------------ |
| Check out source                                | [`actions/checkout`][actions-checkout~repo]↗     |
| Install a compiler and components               | [`dtolnay/rust-toolchain`][rust-toolchain~repo]↗ |
| Reuse Cargo downloads and compilation artifacts | [`Swatinem/rust-cache`][rust-cache~action~repo]↗ |
| Install command-line tools                      | [`taiki-e/install-action`][install_action~repo]↗ |

## Related Topics {#related-topics .skip}

- [CI/CD](cd_ci.md) for validation, artifact promotion, and deployment boundaries.
- [Release Automation](release_automation.md) for packaging and publication.
- [`gh-workflow`][c~gh-workflow~docs]↗, a typed GitHub Actions workflow generator in Rust.

{{#include refs.incl.md}}
{{#include ../../../../bk/src/refs/link-refs.md}}
