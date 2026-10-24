<!-- TODO Reviewed ??-??-2026 -->
# Book scripts

Run the book recipes from `bk/`; `bk/justfile` is the single entry point and imports the modules below `bk/scripts/`. The supported Just version is pinned as `JUST_VERSION` in `.devcontainer/.env`. On Windows the modules run recipes with PowerShell 7 (`pwsh.exe`) without loading a user profile.

Use `just` to list root recipes and modules, then `just <module>` to list a module's recipes. Recipe arguments that can contain spaces are quoted with Just's `quote()` helper, and path-sensitive PowerShell commands use `-LiteralPath`. Just returns the exit status of the final command in a recipe; platform-specific commands are marked with `[unix]` or `[windows]` in the owning `mod.just` file.

## Recipe support

| Module | Windows support | Unix support |
| --- | --- | --- |
| `anchors`, `chapters`, `crates`, `examples`, `includes`, `titles`, `toc`, `urls`, `wikilinks` | Listing only; processing recipes are gated as Unix-only Bash/tooling commands. | All listed processing recipes. |
| `book` | Full build, `refdefs-generate`, `refdefs-check`, category/alphabetical index generation, static-file copy, sitemap, and `serve`. `quick` prints an unsupported-platform error. | Full build and serve recipes, plus the faster `quick` preview. |
| `code` | Format, build, Clippy, nextest, doctests, package run, and temp cleanup. Heavy tests are gated as Unix-only. | Same, including heavy tests that start external services. |
| `deps` | Cargo update, crate information, and cargo-deny checks. | Same. |
| `docker` | `push_dev` only; it requires Docker credentials and publishes an image. | Docker image push and service-list recipes. |
| `docs` | Listing only; documentation preview uses Unix shell/server commands. | Build and serve package or workspace documentation. |
| `gh` | `login` and `issue` use the GitHub CLI. Bulk issue creation and refresh are gated as Unix-only. | All listed recipes. |
| `indices` | `crate_indices` and `crate_page` use the built helper binary. | Those recipes and the Bash-based index/reference generators. |
| `links` | `get` returns an explicit unsupported-platform error; link scanning and editing recipes are gated as Unix-only. | Link insertion, cross-linking, URL suggestions, inline-link conversion, and reference lookup. |
| `precommit` | Fix, setup, Git Town configuration, and the native Python installer. | Fix, setup, Git Town configuration, and the Unix installer. |
| `recipes` | Recipe table fixer runs with Python. | Recipe table fixer runs with Python. |
| `refdefs` | Sort reference files with PowerShell. Other collection and validation scripts are Unix-only. | Sort, check, collect, and relabel recipes. |
| `spelling` | `spell` runs the pinned Typos configuration over book, draft, and future Markdown. `spell-aspell` and `aspell-add-word` retain the optional Unix-only Aspell workflow. | Same Typos check, plus the optional Aspell check and dictionary update recipes. |
| `utils` | `mdbook-utils` and `templ` helper commands. | Same. |

The full book build and normal `serve` recipe ensure the pinned `mdbook-indexing` preprocessor is installed. `quick` is an intentionally reduced Unix preview that skips preprocessors and other full-build checks.

## Contributor commands

From `bk/`, run `just ba`, `just ca`, and `just nta` for the example workspace build, Clippy, and tests. Run `just bb` to build the book and `just s` to serve it. Use `just q` for a faster Unix-only preview. A focused example test is `just ntt <keyword>` from `bk/`; direct Cargo tests run from `bk/crates/`.

[docker~website]: https://docker.com
[just~website]: https://just.systems/man/en
