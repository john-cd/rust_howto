# Version Control

{{#include version_control.incl.md}}

Rust projects use version control (typically [`git`][git~website]↗{{hi:git}}) like any other software. `git` tracks changes to your code, enables collaboration, and allows reverting to previous versions. Cargo's [`Cargo.lock`][c~cargo~cargo.lock]↗{{hi:Cargo.lock}} file specifically ensures reproducible builds by pinning exact dependency versions, which can also be version controlled.

The following describes tools that make FIXME.

## Check Your Code Before Committing it {#check-your-code-before-committing}

[![cargo-husky][c~cargo-husky~docs~badge]][c~cargo-husky~docs]{{hi:cargo-husky}}
[![cargo-husky~crates.io][c~cargo-husky~crates.io~badge]][c~cargo-husky~crates.io]
[![cargo-husky~repo][c~cargo-husky~repo~badge]][c~cargo-husky~repo]
[![cargo-husky~lib.rs][c~cargo-husky~lib.rs~badge]][c~cargo-husky~lib.rs]
[![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}

Git hook scripts are useful for automatically identifying simple issues, such as missing semicolons, trailing whitespace, poor [[code_formatting_linting | formatting]] of the code or [configuration][p~configuration] files, when committing in [`git`][git~website]↗{{hi:git}}, prior to submission to code review or start of a CI workflow.

[cargo-husky][c~cargo-husky~repo]↗{{hi:cargo-husky}} setup Git hooks automatically for [`cargo`][c~cargo~docs]↗{{hi:cargo}} projects with 🐶.

Add the [`cargo-husky`][c~cargo-husky~docs]↗{{hi:cargo-husky}} crate to the `[dev-dependencies]` section of your project's [`Cargo.toml`][book~cargo~cargo-toml]↗{{hi:Cargo.toml}}.

```toml
[dev-dependencies]
cargo-husky = "1"
```

Then run tests in your project directory.

```sh
cargo test
```

## `pre-commit` {#pre-commit}

[`pre-commit`][pre-commit.com~website]↗ is a Python framework for managing and maintaining multi-language pre-commit hooks.

- [`pre-commit` hooks][pre-commit.com~hooks~website].

`pre-commit` is written in Python. Include the following into your [`Dockerfile`][dockerfile~website]↗{{hi:Dockerfile}} or run the commands by hand to install `pre-commit`:

```sh
# Install `python3`, `pipx`, `pre-commit` (Ubuntu & friends)
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get install -y python3 pipx \
    && pipx install pre-commit \
    && pipx ensurepath
```

Verify that it is properly installed:

```sh
pre-commit --version
```

Add a file called `.pre-commit-config.yaml`{{hi:.pre-commit-config.yaml}} to the root of your project. Use `pre-commit sample-config` for a template.
Edit it to configure your preferred hooks.

```sh
# Set up the git hook scripts
pre-commit install

# It's usually a good idea to run the hooks against all of the files when adding new hooks
# (`pre-commit` will only run on the changed files during git hooks)
pre-commit run --all-files
```

## Useful Git Hooks for Rust {#skip}

FIXME.

- [`cargo fmt`][book~cargo~cargo-fmt]↗{{hi:cargo fmt}}.
- [`cargo check`][book~cargo~cargo-check]↗{{hi:cargo check}}, clippy, test...
- spell checks.

## Related Topics

- [[development-tools_testing | Development Tools Testing]].
- [[development-tools_cargo-plugins | Development Tools Cargo Plugins]].
  - [[building | Building]].
  - [[code_formatting_linting | Code Formatting & Linting]].
  - [[versioning | Versioning]].

## See Also

- [Rust CI Tooling][rust-ci-tooling~website]↗: Clippy, commitlint, pre-commit.
- [`pre-commit-rust`][pre-commit-rust~repo].
- [A pre-commit hook for `commitlint`][commitlint-pre-commit-hook~repo]↗.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[git_hooks: write](https://github.com/john-cd/rust_howto/issues/602)
dedupe with code_formatting_linting.

- [overcommit][overcommit~repo]↗: A fully configurable and extendable Git hook manager.

</div>
