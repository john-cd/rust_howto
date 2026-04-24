# rust_howto Copilot Instructions

## Build, test, and lint

This repository is a monorepo with several independent Rust workspaces. Run commands from the matching workspace instead of assuming the repo root is a Cargo workspace.

| Area | Working directory | Build / lint / test | Run one test |
| --- | --- | --- | --- |
| Main book examples | `bk\` | `just ba`, `just ca`, `just nta` | `just ntt <keyword>` |
| Book rendering | `bk\` | `just bb` to build the book, `just s` to serve it, `just q` for a faster preview path on Unix | n/a |
| Future/staging crates | `later\` | `just ba`, `just ca`, `just nta` | `cargo test -p <crate> <test_name>` from `later\crates` |
| Helper binaries | `tools\` | `just ba`, `just ca`, `just nta` | `cargo test -p <crate> <test_name>` |
| Additional long-form examples | `xmpl\` | `just ba`, `just ca`, `just nta` | `cargo test -p <crate> <test_name>` |
| Scratch workspace | `playground\` | `just ba`, `just ca`, `just nta` | `cargo test -p <crate> <test_name>` |
| Placeholder published crate | `publish\` | `just ba`, `just ca`, `just nta` | `cargo test <test_name>` |

Additional repo-specific commands:

- `just all <cmd>` at the repo root fans out to the main subprojects.
- `just release` in `tools\` builds the helper binaries and copies them into the repo-level `bin\` directory.
- `cargo +nightly fmt --all` is the formatting path used by the workspace `just` recipes.

## High-level architecture

This repo is primarily an mdBook publishing repo with Rust workspaces attached to it.

- `bk\` is the canonical published book. `bk\src\SUMMARY.md` defines the book structure, and `bk\book.toml` wires the mdBook preprocessors and outputs.
- `bk\crates\` is a separate Cargo workspace containing the Rust examples embedded in the book. Crates are grouped either by book section (`language`, `standard_library`, `code_organization`, etc.) or by crates.io category under `bk\crates\cats\...`.
- `tools\` is a separate Rust workspace for repo-specific helper binaries. `mdbook-scrub` is built there and consumed by `bk\book.toml` through the repo-level `bin\` directory. `tool_lib` is the shared library used by the tool crates.
- `mdbook-utils\` is a companion CLI/library project used to manage links, reference definitions, code block extraction/includes, and sitemap generation for large mdBook source trees.
- `later\src\` and `later\crates\` mirror the book/example structure for work-in-progress or future content that is not yet part of the main published book.
- `xmpl\` holds standalone examples that are too large to embed directly in the book, `playground\` is for exploratory code, and `publish\` is the small placeholder crate published to crates.io so the book is discoverable there.

Each workspace has its own `.cargo\config.toml`, but they all redirect builds into the repo-level `target\...` tree instead of using per-workspace `target` folders.

## Key conventions

- Book content is include-heavy. Many chapter files are thin wrappers that include a sibling `*.incl.md` file plus shared reference-definition files. Preserve that structure instead of inlining generated tables or link refs.
- When adding or moving book content under `bk\src\`, also update `bk\src\SUMMARY.md`; the book structure is driven from there.
- Markdown links are often centralized through `refs.incl.md` or `refs\link-refs.md` instead of inline URLs.
- Embedded Rust examples usually live under `bk\crates\<section-or-category>\examples\<chapter>\`. A chapter-level `main.rs` pulls recipe files in via `mod ...;`.
- Example files normally expose only the book-visible snippet between `// ANCHOR: example` and `// ANCHOR_END: example`, then add a hidden `#[test] fn test() { main(); }` so the snippet is continuously exercised by workspace tests.
- The `bk\justfile` is the main contributor entry point. It delegates to `bk\scripts\...\mod.just` modules for code, book, links, refs, examples, indices, and related maintenance tasks; prefer those recipes over inventing new ad hoc commands.
- On Windows, the repo's `justfile`s are already configured to use PowerShell (`pwsh.exe`). Prefer `just` recipes when available instead of assuming Bash-only command sequences.
