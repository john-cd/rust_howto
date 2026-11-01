<!-- TODO Reviewed ??-??-2026 -->

# Version Control

{{#include version_control.incl.md}}

Rust projects use version control (typically [`git`][git~website]↗{{hi:git}}) like any other software. `git` tracks changes to your code, enables collaboration, and allows reverting to previous versions. Cargo's [`Cargo.lock`][c~cargo~cargo.lock]↗{{hi:Cargo.lock}} file specifically ensures reproducible builds by pinning exact dependency versions, which can also be version controlled.

The following describes tools that make version control workflows more reliable and easier to maintain for Rust projects.

## Check Your Code Before Committing {#check-your-code-before-committing}

Git hooks can run quick, deterministic checks before a commit or push. They help catch mistakes early, but they are a convenience for each local clone: hooks can be skipped or absent, so continuous integration remains the authoritative shared check. Keep hooks short and document their commands so contributors can also run them directly.

## Use prek for Git Hooks {#use-prek-for-git-hooks}

[`prek`][prek~repo]↗ is a Rust-based Git hook manager with a native `prek.toml` configuration. Keep the hook list in that file, install the hooks with `prek install`, and run the full set with `prek run --all-files`.

For Rust workspaces, use local hooks to run commands from the workspace root. This keeps hook behavior explicit and lets contributors invoke the same commands directly.

## Configure Rust Checks {#configure-rust-hook-checks}

This example runs workspace-wide formatting and Clippy checks when Rust files change. Those commands can be expensive for a large workspace; narrow them to the affected package or move slower checks to CI as appropriate.

```toml
{{#include examples/prek.toml}}
```

Install the hooks with `prek install`. Run `prek run` on staged files or `prek run --all-files` for the initial repository-wide check.

For most projects, format with `cargo fmt --all -- --check`, lint with `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and run tests at a cadence that fits their cost. Keep the same required checks in CI, even when a hook runs them locally.

## Git Libraries and Rust Tools {#git-libraries-and-rust-tools}

Use a library when an application needs to inspect or manipulate repositories directly: [`git2`][git2-rs~repo]↗ provides Rust bindings to the C `libgit2` library, while [`gix`][c~gix~docs]↗ is the entry point to the Gitoxide project's pure-Rust Git implementation. Compare their supported features and platform/build requirements for the operations your application needs.

Rust also has standalone Git tools: [`gitoxide`][gitoxide~repo]↗ provides Git command-line tools, [`delta`][delta~repo]↗ is a syntax-highlighting pager for Git and diff output, and [`gitui`][gitui~repo]↗ is a terminal interface for Git repositories. These are developer tools, not Git hook managers.

## See Also {#see-also .skip}

- [Rust CI Tooling][rust-ci-tooling~website]↗ for CI, Clippy, and commit checks.
- [`prek`][prek~repo]↗ for Git hook configuration and execution.
- [Code Formatting and Linting][p~formatting] for details about `rustfmt` and Clippy.

## Related Topics {#related-topics .skip}

- [[development-tools_testing | Development Tools Testing]].
- [[development-tools_cargo-plugins | Development Tools Cargo Plugins]].
- [[building | Building]].
- [[versioning | Versioning]].
  {{#include refs.incl.md}}
  {{#include ../../refs/link-refs.md}}
