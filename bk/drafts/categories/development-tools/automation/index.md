<!-- TODO Reviewed ??-??-2026 -->

# Repository Automation

[![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Automation}}

Checks and chores that are specific to a repository, such as "every crate opts in to the workspace lints" or "which files change the most", are often written as shell scripts. Writing them in Rust instead gives them types, tests, cross-platform behavior and the same toolchain as the project.

## `xtask` Project Checks

{{#include xtask.incl.md}}

## Git History

{{#include git.incl.md}}

## Related Topics {#related-topics .skip}

- [[xtask | `xtask` Project Checks]]
- [[git | Git History with `gix`]]
- [[rust_source | Analyze and Generate Rust Code]] describes the `syn` and `cargo_metadata` building blocks.

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}
