<!-- TODO Reviewed ??-??-2026 -->

# Code Formatting and Linting

{{#include code_formatting_linting.incl.md}}

Run formatting and lint checks locally and in continuous integration. Formatting makes source consistent; linting points out suspicious or hard-to-maintain code. Review diagnostics before changing code or suppressing a warning.

## Format Rust Code with `rustfmt` {#rustfmt}

[![rustfmt~repo][c~rustfmt~repo~badge]][c~rustfmt~repo] [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}} [![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]{{hi:Cargo plugins}}

`rustfmt` is the standard Rust formatter, available through the `rustfmt` Rustup component. Use `cargo fmt --all` to format a workspace and `cargo fmt --all -- --check` in CI to fail when formatting is needed. Commit the workspace's `rustfmt.toml` or `rustfmt.toml` configuration when the team needs consistent non-default settings.

## Find Common Mistakes with Clippy {#cargo-clippy}

{{#badges clippy development-tools="Development tools" development-tools::cargo-plugins="Cargo plugins"}}

`cargo clippy` runs the Clippy lints for the selected packages and targets. For a workspace-wide CI check, use `cargo clippy --workspace --all-targets --all-features -- -D warnings` when those feature combinations are supported on the CI host. If the project supports only a subset of targets or features on that host, list the combinations that CI can actually build.

Clippy diagnostics include suggestions, not automatic requirements. Prefer the clearest code that fits the project's declared Rust version. Fix warnings where appropriate; use a narrow `#[allow(clippy::lint_name)]` with a reason when a lint does not fit a particular item.

`rust-analyzer` provides editor diagnostics while you work. Keep the command-line CI check as the shared gate so local editor settings do not determine whether a change passes.

## Apply Compiler Suggestions with `cargo fix` {#cargo-fix}

[`cargo fix`][cargo-fix~docs]↗ applies machine-applicable compiler suggestions to source code. Run it on a clean or otherwise understood working tree, review the resulting diff, and then run formatting, checks, and tests. It does not replace Clippy or code review.

## Check Spelling and Documentation Links {#spelling-and-links}

[`cargo-spellcheck`][c~cargo-spellcheck~docs]↗{{hi:cargo-spellcheck}} scans Rust source and documentation comments for likely spelling errors. Review suggestions because crate names, identifiers, and technical terms may be intentional.

[`cargo-deadlinks`][c~cargo-deadlinks~docs]↗{{hi:cargo-deadlinks}} checks links in generated Rust documentation. Generate the documentation first, then run the link checker against the project's documentation output; confirm tool options against its current help for the installed version.

For Markdown in this book, use the repository's Markdown lint and link-check recipes instead of adding an unrelated Rust crate. See [[version_control | Version Control and Git Hooks]] for running existing checks automatically before a commit.

## Related Topics {#related-topics .skip}

- [[building | Building]].
- [[testing | Testing]].
- [[documentation | Documentation]].
- [[version_control | Version Control and Git Hooks]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
