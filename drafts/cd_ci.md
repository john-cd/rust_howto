# Continuous Deployment / Continuous Integration

[Continuous Integration (cargo book)][book-cargo-continuous-integration]⮳

## GitHub Actions

### Installation of development tools

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

{{hi:install-action}}[`install-action`][install-action]⮳ is a GitHub Action for installing development tools (mainly from GitHub Releases).

```yaml
- uses: taiki-e/install-action@v2
  with:
    tool: cargo-binstall,just,mdbook,mdbook-lintcheck
```

### Compilation caching

{{hi:rust_cache}}[`rust_cache`][c-rust_cache-github]⮳

`Swatinem/rust-cache` is the current recommended cache action, which handles properly optimizing cache effectiveness for a cargo build in CI. That action also automatically sets `CARGO_INCREMENTAL=0` for users of the action.

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

    # A cache key that is used instead of the automatic `job`-based key,
    # and is stable over multiple jobs.
    # default: empty
    shared-key: ""

    # An additional cache key that is added alongside the automatic `job`-based
    # cache key and can be used to further differentiate jobs.
    # default: empty
    key: ""

    # A whitespace separated list of env-var *prefixes* who's value contributes
    # to the environment cache key.
    # The env-vars are matched by *prefix*, so the default `RUST` var will
    # match all of `RUSTC`, `RUSTUP_*`, `RUSTFLAGS`, `RUSTDOC_*`, etc.
    # default: "CARGO CC CFLAGS CXX CMAKE RUST"
    env-vars: ""

    # The cargo workspaces and target directory configuration.
    # These entries are separated by newlines and have the form
    # `$workspace -> $target`. The `$target` part is treated as a directory
    # relative to the `$workspace` and defaults to "target" if not explicitly given.
    # default: ". -> target"
    workspaces: ""

    # Additional non workspace directories to be cached, separated by newlines.
    cache-directories: ""

    # Determines whether workspace `target` directories are cached.
    # If `false`, only the cargo registry will be cached.
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

## See also

[Optimizing CI/CD pipelines][blog-optimizing-ci-cd-pipelines]⮳

<https://docs.github.com/en/actions/creating-actions/creating-a-docker-container-action>⮳
<https://github.com/marketplace/actions/upload-a-build-artifact>⮳

{{hi:docker-cache}}[`docker-cache`][docker-cache-github]⮳

[Cached Docker images][cached-docker-images]⮳

[Docker GitHub Action][docker-gitHub-action]⮳

<https://docs.docker.com/build/cache/backends/>⮳

<https://docs.docker.com/build/ci/github-actions/cache/>⮳

<https://stackoverflow.com/questions/61491484/how-to-cache-docker-compose-build-inside-github-action>⮳

{{#include ../refs/link-refs.md}}
