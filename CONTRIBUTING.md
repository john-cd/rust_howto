<!-- TODO Reviewed ??-??-2026 -->
# Contributing

Thanks for helping improve the Rust How-to book and its companion tools.

## Start here

Read the [contributing guide in the book](https://john-cd.com/rust_howto/appendices/contributing/index.html) for contribution types, writing style, repository structure, development setup, and guidance for chapters and examples. It also links to the project's Code of Conduct and contribution license notice.

Before starting a larger change, search the [existing issues](https://github.com/john-cd/rust_howto/issues). For a bug or feature, open or update an issue with the expected behavior and a reproducible example. Keep pull requests focused, link the issue, and list the checks you ran.

## Choose the workspace

The repository root is not one Cargo workspace. Run commands from the workspace you change. `bk/` contains the published book and its runnable examples. `later/`, `tools/`, `xmpl/`, `playground/`, and `publish/` are independent workspaces. The companion `mdbook-utils/` project has its own contributor instructions.

For changes to the published book examples, run these commands from `bk/`:

```text
just fa
just ba
just ca
just nta
```

Use `just ntt <keyword>` for a focused test. Run the matching `just` recipes from the root of any other affected workspace.

## Change and preview the book

`bk/src/SUMMARY.md` controls chapter navigation. Add an entry when you add or move a chapter. Preserve shared include and reference fragments, and put runnable Rust examples under `bk/crates/<section-or-category>/examples/<chapter>/`.

After changing book content, run `just bb` from `bk/` to build the book. In the pull request, note any service, GUI, or platform-specific checks that could not run in your environment.
