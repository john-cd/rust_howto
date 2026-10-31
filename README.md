<!-- TODO Reviewed ??-??-2026 -->

# The Rust How-To Book

[![Build and deploy (Linux)](https://github.com/john-cd/rust_howto/actions/workflows/build_deploy_book.yml/badge.svg)](https://github.com/john-cd/rust_howto/actions/workflows/build_deploy_book.yml)[![(Windows)](https://github.com/john-cd/rust_howto/actions/workflows/build_book_code_and_book_on_windows.yml/badge.svg)](https://github.com/john-cd/rust_howto/actions/workflows/build_book_code_and_book_on_windows.yml)

**A documentation summary and a compendium of recipes for the Rust language and ecosystem.**
**Everything you need for day-to-day Rust coding, all in one place.**

Point your browser to [`https://john-cd.com/rust_howto`][rust-howto~website]↗ to read the [Rust How-to][rust-howto~website]↗ book.

## Work In Progress

This book is going through _heavy edits_. Pardon the dust.

## Contributing

Contributions are most welcome! Please review the [contributing][rust-howto~contributing~website]↗ section of the book and peruse the book's [GitHub repo][rust-howto~repo]↗.

## Build outputs and worktrees

The primary checkout stays on `develop`. Use `.worktrees/a` and `.worktrees/b` for new auxiliary checkouts; the existing `.worktrees/c` is a detached legacy checkout. Build artifacts and Cargo intermediate files stay outside the repository: Windows uses `D:/target/rust_howto/<slot>` and `C:/code_build/rust_howto/<slot>`; Linux uses `$XDG_CACHE_HOME/rust_howto/target/<slot>` and `$XDG_CACHE_HOME/rust_howto/build/<slot>` when set, or the matching paths under `~/.cache` otherwise.

Run `just build-setup` once in each checkout. Use `just cargo <subcommand> ...` to run Cargo with the checkout's paths. Workspace just recipes disable sccache and enable incremental compilation; direct Cargo uses sccache with incremental compilation disabled. The shared sccache cache defaults to 20 GiB, and `just cache-setup` refreshes base directories while no Rust build is running.

## Companion Tools

Unpublished chapters have separate authoring previews. From `bk/`, run `just book drafts`
or `just book future` after building the helper binaries with `just release` from `tools/`.
These commands use `bk/drafts/book.toml` and `later/book.toml`, respectively, and write
HTML under `bk/book/`. They expand includes and custom directives without adding draft
chapters to the published book. Review build diagnostics as well as the exit status:
mdBook can report a missing include while continuing to render a chapter.

The [`mdbook-utils`][c~mdbook-utils~crates.io]↗ utility ([(github)][c~mdbook-utils~repo]↗ [(docs.rs)][c~mdbook-utils~docs]↗ [(user guide)][c~mdbook-utils~user-guide]↗) helps manage links and code blocks in [`mdbook`][c~mdbook~docs]↗ source directories. It is the companion tool for this book.

There are also a number of utilities and pre-processors in the [`tools`][rust-howto~tools~repo]↗ folder of this repo.

## Legal

Please refer to the LICENSE file.

[c~mdbook~docs]: https://rust-lang.github.io/mdBook
[c~mdbook-utils~repo]: https://github.com/john-cd/mdbook-utils
[c~mdbook-utils~docs]: https://docs.rs/mdbook-utils/latest/mdbook_utils
[c~mdbook-utils~crates.io]: https://crates.io/crates/mdbook-utils
[c~mdbook-utils~user-guide]: https://john-cd.github.io/mdbook-utils
[rust-howto~website]: https://john-cd.com/rust_howto
[rust-howto~contributing~website]: https://john-cd.com/rust_howto/appendices/contributing/index.html
[rust-howto~tools~repo]: https://github.com/john-cd/rust_howto/tree/develop/tools
[rust-howto~repo]: https://github.com/john-cd/rust_howto
## Central build outputs and worktrees

- Keep auxiliary Git worktrees under .worktrees/ and name them with one lowercase letter from a through j. Only use slots a and b for new tasks.
- Preserve the pre-existing `.worktrees/c` checkout as a legacy exception; do not create additional slots beyond a or b. Its `just` paths route outputs under slot c while it remains registered.
- Windows: target outputs go to D:/target/rust_howto/slot and intermediate build outputs go to C:/code_build/rust_howto/slot. C: is NTFS; D: is ReFS.
- Linux/macOS: target outputs go to the XDG cache folder (or ~/.cache) under cargo-target/rust_howto/<slot>; intermediate outputs go under cargo-build/rust_howto/<slot>.
- Run ad hoc commands as just cargo <subcommand> [args...]; existing cargo-make tasks also receive the configured paths.
- The workspace justfiles use no compiler wrapper with incremental compilation enabled; direct Cargo uses sccache with incremental compilation disabled by `.cargo/config.toml`. Both paths keep Cargo outputs in the external directories above. Prefer the justfiles for ad hoc Cargo commands.
- SCCACHE_BASEDIRS adds the common repository root, which covers the main checkout and every auxiliary worktree, while preserving inherited project bases.
