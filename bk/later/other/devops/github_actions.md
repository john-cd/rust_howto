# GitHub Actions

{{#include github_actions.incl.md}}

GitHub Action for installing [development tools][p~development-tools] (mainly from GitHub Releases).

## Install Development Tools {#installation-of-development-tools}

```yaml
    - name: Install cargo check tools
      run: |
        cargo install --locked cargo-deny || true
        cargo install --locked cargo-outdated || true
        cargo install --locked cargo-udeps || true
        cargo install --locked cargo-audit || true
        cargo install --locked cargo-pants || true
    - name: Check dependencies
      run: |
        cargo deny check
        cargo outdated --exit-code 1
        cargo udeps
        rm -rf ~/.cargo/advisory-db
        cargo audit
        cargo pants
```

[`install-action`][install_action~github]{{hi:install-action}}⮳ is a GitHub Action for installing development tools (mainly from GitHub Releases).

```yaml
- uses: taiki-e/install-action@v2
  with:
    tool: cargo-binstall,just,mdbook,mdbook-lintcheck
```

## Cache Your Compilation {#compilation-caching}

[`rust-cache`][c~rust_cache~github]{{hi:rust-cache}}⮳.

[![rust-cache][c~rust_cache~docs~badge]][c~rust_cache~docs]{{hi:rust-cache}}
[![rust-cache~crates.io][c~rust_cache~crates.io~badge]][c~rust_cache~crates.io]
[![rust-cache~github][c~rust_cache~github~badge]][c~rust_cache~github]
[![rust-cache~lib.rs][c~rust_cache~lib.rs~badge]][c~rust_cache~lib.rs]

`Swatinem/rust-cache` is the current recommended cache action, which handles properly optimizing cache effectiveness for a [cargo][p~cargo] build in CI. That action also automatically sets `CARGO_INCREMENTAL=0` for users of the action.

Example `.github/workflows/<name>.yml`

```yaml
- uses: actions/checkout@v4

# Selecting a toolchain either by action or manual `rustup` calls. That should happen before the plugin, as the cache uses the current rustc version as its cache key
- run: rustup toolchain install stable --profile minimal

- uses: Swatinem/rust-cache@v2
  with:
    # The prefix cache key, this can be changed to start a new cache manually.
    # default: "v0-rust"
    prefix-key: ""

    # A cache key that is used instead of the automatic [`job`][c~job~docs]⮳{{hi:job}}-based key,
    # and is stable over multiple jobs.
    # Default: empty
    shared-key: ""

    # An additional cache key that is added alongside the automatic [`job`][c~job~docs]⮳{{hi:job}}-based
    # cache key and can be used to further differentiate jobs.
    # Default: empty
    key: ""

    # A whitespace separated list of env-var *prefixes* who's value contributes
    # to the environment cache key.
    # The env-vars are matched by *prefix*, so the default `RUST` var will
    # match all of `RUSTC`, `RUSTUP_*`, `RUSTFLAGS`, `RUSTDOC_*`, etc.
    # default: "CARGO CC CFLAGS CXX CMAKE RUST"
    env-vars: ""

    # The [`cargo`][c~cargo~docs]⮳{{hi:cargo}} workspaces and target directory configuration.
    # These entries are separated by newlines and have the form
    # `$workspace -> $target`. The `$target` part is treated as a directory
    # relative to the `$workspace` and defaults to "target" if not explicitly given.
    # default: ". -> target"
    workspaces: ""

    # Additional non workspace directories to be cached, separated by newlines.
    cache-directories: ""

    # Determines whether the workspace `target` directories are cached.
    # If `false`, only the [`cargo`][c~cargo~docs]⮳{{hi:cargo}} registry will be cached.
    # default: "true"
    cache-targets: ""

    # Determines if the cache should be saved even when the workflow has failed.
    # default: "false"
    cache-on-failure: ""

    # Determines which crates are cached.
    # If `true` all crates will be cached, otherwise only dependent crates will be cached.
    # Useful if additional crates are used for CI tooling.
    # default: "false"
    cache-all-crates: ""

    # Determiners whether the cache should be saved.
    # If `false`, the cache is only restored.
    # Useful for jobs where the matrix is additive e.g. additional Cargo features,
    # or when only runs from `master` should be saved to the cache.
    # default: "true"
    save-if: ""
    # To only cache runs from `master`:
    save-if: ${{ github.ref == 'refs/heads/master' }}

    # Specifies what to use as the backend providing cache
    # Can be set to either "github" or "buildjet"
    # default: "github"
    cache-provider: ""
```

## Useful GitHub Actions {#skip}

FIXME

- `rust-toolchain`: Github action to install Rust components via rustup
- [`rust-cache`][c~rust_cache~docs]⮳{{hi:rust-cache}}: Github action to cache compilation artifacts and speed up subsequent runs.
- `install-action`: GitHub Action for installing [development tools][p~development-tools] (mainly from GitHub Releases).

## See Also {#skip}

[![gh-workflow][c~gh_workflow~docs~badge]][c~gh_workflow~docs] [![gh-workflow~crates.io][c~gh_workflow~crates.io~badge]][c~gh_workflow~crates.io] [![gh-workflow~github][c~gh_workflow~github~badge]][c~gh_workflow~github] [![gh-workflow~lib.rs][c~gh_workflow~lib.rs~badge]][c~gh_workflow~lib.rs]{{hi:gh-workflow}}{{hi:Actions}}{{hi:Generator}}{{hi:Github}}{{hi:Workflow}}

A type-safe GitHub Actions workflow generator: [`gh-workflow`][c~gh_workflow~docs].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[github_actions - see blessed.rs](https://github.com/john-cd/rust_howto/issues/600)
write / review in depth

rust-toolchain (github action): Github action to install Rust components via rustup
rust-cache (github action): Github action to cache compilation artifacts and speed up subsequent runs.
install-action (github action): GitHub Action for installing [development tools][p~development-tools] (mainly from GitHub Releases).

- [Octocrab](https://lib.rs/crates/octocrab)

</div>
