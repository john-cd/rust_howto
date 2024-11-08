# Git hook scripts

Git hook scripts are useful for automatically identifying simple issues, such as missing semicolons, trailing whitespace, poor formatting of the code or configuration files, when commiting in `git`, prior to submission to code review or start of a CI workflow.

## Check your code before committing it

[![cargo-husky][c-cargo_husky-badge]][c-cargo_husky]{{hi:cargo-husky}}
[![cargo-husky-crates.io][c-cargo_husky-crates.io-badge]][c-cargo_husky-crates.io]
[![cargo-husky-github][c-cargo_husky-github-badge]][c-cargo_husky-github]
[![cargo-husky-lib.rs][c-cargo_husky-lib.rs-badge]][c-cargo_husky-lib.rs]
[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}}

[cargo-husky][c-cargo_husky-github]{{hi:cargo-husky}}⮳ setup Git hooks automatically for cargo projects with 🐶

Add the `cargo-husky` crate to the `[dev-dependencies]` section of your project's `Cargo.toml`.

```toml
[dev-dependencies]
cargo-husky = "1"
```

Then run tests in your project directory.

```sh
cargo test
```

## pre-commit

[`pre-commit`][pre-commit.com-website]⮳ is a Python framework for managing and maintaining multi-language pre-commit hooks.

[`pre-commit` hooks][pre-commit.com-hooks-website]

### Installation

`pre-commit` is written in Python. Include the following into your `Dockerfile` or run the commands by hand to install `pre-commit`:

```sh
## Install python3, pipx, pre-commit (Ubuntu & friends)
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get install -y python3 pipx \
    && pipx install pre-commit \
    && pipx ensurepath
```

- Verify that it is properly installed:

```sh
pre-commit --version
```

- Add a file called `.pre-commit-config.yaml` to the root of your project. Use `pre-commit sample-config` for a template.
- Edit it to configure your preferred hooks.

```sh
# Set up the git hook scripts
pre-commit install

# It's usually a good idea to run the hooks against all of the files when adding new hooks
# (pre-commit will only run on the changed files during git hooks)
pre-commit run --all-files
```

## Useful links

https://rodneylab.com/rust-ci-tooling/

https://github.com/doublify/pre-commit-rust

https://github.com/alessandrojcm/commitlint-pre-commit-hook

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO write
identify useful hooks for Rust
- cargo fmt
- cargo check, clippy, test...
- spell checks
add a sample config
</div>