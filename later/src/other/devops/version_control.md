<!-- TODO Reviewed ??-??-2026 -->

# Version Control

{{#include version_control.incl.md}}

Rust projects use version control (typically [`git`][git~website]↗{{hi:git}}) like any other software. `git` tracks changes to your code, enables collaboration, and allows reverting to previous versions. Cargo's [`Cargo.lock`][c~cargo~cargo.lock]↗{{hi:Cargo.lock}} file specifically ensures reproducible builds by pinning exact dependency versions, which can also be version controlled.

The following describes tools that make version control workflows more reliable and easier to maintain for Rust projects.

## Check Your Code Before Committing {#check-your-code-before-committing}

Git hooks can run quick, deterministic checks before a commit or push. They help catch mistakes early, but they are a convenience for each local clone: hooks can be skipped or absent, so continuous integration remains the authoritative shared check. Keep hooks short and document their commands so contributors can also run them directly.

## Choose a Hook Manager {#choose-a-hook-manager}

[`pre-commit`][pre-commit.com~website]↗ is a Python framework with a broad ecosystem of reusable hooks and configuration in `.pre-commit-config.yaml`. Install it with your preferred Python tool and register the hooks with `pre-commit install`; use `pre-commit run --all-files` to run them across the repository.

[`prek`][prek~repo]↗ is a Rust implementation of the pre-commit workflow. It can use existing `.pre-commit-config.yaml` files and also supports a native `prek.toml` format. The repository includes both formats; choose one configuration as the source of truth for a project rather than maintaining duplicate hook lists.

[`lefthook`][lefthook~repo]↗ is a language-agnostic hook manager with a YAML configuration and support for parallel jobs and staged-file selection. It is an option when a project needs more control over job execution than a simple list of pre-commit hooks.

## Run Rust Checks with pre-commit {#pre-commit-rust-checks}

This example uses local hooks to run workspace-wide formatting and Clippy checks when Rust files change. Those commands can be expensive for a large workspace; narrow them to the affected package or move slower checks to a pre-push hook or CI as appropriate.

```yaml
{{#include examples/pre-commit-config.yaml}}
```

For this configuration, install hooks with `pre-commit install`, then run `pre-commit run --all-files` to check the initial baseline. `prek install` and `prek run --all-files` provide corresponding commands when using `prek`.

## Cargo-Integrated Hook Managers {#cargo-integrated-hook-managers}

[`cargo-husky`][c~cargo-husky~docs]↗ installs Git hooks from a Cargo build script when the development dependency is built. Its default setup creates a `pre-push` hook that runs `cargo test`; features can add `pre-commit`, `cargo fmt`, and Clippy checks. Because the hook installation is coupled to Cargo builds, review the generated scripts and ensure they do not replace hooks managed by another tool.

[`rusty-hook`][rusty-hook~repo]↗ is another Rust-focused manager. Its README describes it as functional but still in beta, with hooks configured in `.rusty-hook.toml`; check the current repository and crate compatibility before adopting it.

For most projects, format with `cargo fmt --all -- --check`, lint with `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and run tests at a cadence that fits their cost. Keep the same required checks in CI, even when a hook runs them locally.

## Git Libraries and Rust Tools {#git-libraries-and-rust-tools}

Use a library when an application needs to inspect or manipulate repositories directly: [`git2`][git2-rs~repo]↗ provides Rust bindings to the C `libgit2` library, while [`gix`][c~gix~docs]↗ is the entry point to the Gitoxide project's pure-Rust Git implementation. Compare their supported features and platform/build requirements for the operations your application needs.

Rust also has standalone Git tools: [`gitoxide`][gitoxide~repo]↗ provides Git command-line tools, [`delta`][delta~repo]↗ is a syntax-highlighting pager for Git and diff output, and [`gitui`][gitui~repo]↗ is a terminal interface for Git repositories. These are developer tools, not Git hook managers.

## See Also {#see-also .skip}

- [Rust CI Tooling][rust-ci-tooling~website]↗ for CI, Clippy, and commit checks.
- [`pre-commit` hook collection][pre-commit.com~hooks~website]↗.
- [Code Formatting and Linting][p~formatting] for details about `rustfmt` and Clippy.

## Related Topics {#related-topics .skip}

- [[development-tools_testing | Development Tools Testing]].
- [[development-tools_cargo-plugins | Development Tools Cargo Plugins]].
- [[building | Building]].
- [[versioning | Versioning]].
  {{#include refs.incl.md}}
  {{#include ../../refs/link-refs.md}}
