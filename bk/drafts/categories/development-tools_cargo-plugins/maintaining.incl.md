<!-- TODO Reviewed ??-??-2026 -->

Use maintenance plugins when Cargo's built-in release, feature-testing, and cache commands do not cover the workflow. Review the tool's supported Rust versions, platforms, license, and permissions before adopting it.

| Recipe                                                                                                 | Crates                         | Categories                                                       |
| ------------------------------------------------------------------------------------------------------ | ------------------------------ | ---------------------------------------------------------------- |
| [Check Public API Changes for SemVer Breakage][ex~development-tools_cargo-plugins~cargo-semver-checks] | {{#crate cargo-semver-checks}} | {{#categories development-tools::cargo-plugins="Cargo plugins"}} |
| [Inspect and Reclaim the Cargo Cache][ex~development-tools_cargo-plugins~cargo-cache]                  | {{#crate cargo-cache}}         | {{#categories development-tools::cargo-plugins="Cargo plugins"}} |
| [Inspect Macro Expansion with `cargo-expand`][ex~development-tools_cargo-plugins~cargo-expand]         | {{#crate cargo-expand}}        | {{#categories development-tools::cargo-plugins="Cargo plugins"}} |
| [Test Feature Combinations with `cargo-hack`][ex~development-tools_cargo-plugins~cargo-hack]           | {{#crate cargo-hack}}          | {{#categories development-tools::cargo-plugins="Cargo plugins"}} |
