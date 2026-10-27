<!-- TODO Reviewed ??-??-2026 -->

# Book scripts

Run the book recipes from `bk/`; `bk/justfile` is the single entry point and imports the modules below `bk/scripts/`. The supported Just version is pinned as `JUST_VERSION` in `.devcontainer/.env` (currently `1.59.0`). On Windows all `rust_howto`-owned recipe shells use Git Bash. Recipes that need native PowerShell helpers invoke `pwsh.exe -File` explicitly; each imported module sets its own shell because justfile settings do not inherit across modules.

Use `just` to list root recipes and modules, then `just <module>` to list a module's recipes. Recipe arguments that can contain spaces are quoted with Just's `quote()` helper, and path-sensitive PowerShell commands use `-LiteralPath`. Just returns the exit status of the final command in a recipe; platform-specific commands are marked with `[unix]` or `[windows]` in the owning `mod.just` file.

## Recipe support

| Module                                                                                        | Windows support                                                                                                                                                                           | Unix support                                                   |
| --------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------- |
| `anchors`, `chapters`, `crates`, `examples`, `includes`, `titles`, `toc`, `urls`, `wikilinks` | Their listed processing recipes have Windows implementations. Several use Git Bash; see each module's `mod.just`.                                                                         | Their listed processing recipes.                               |
| `book`                                                                                        | Full build, `refdefs-generate`, `refdefs-check`, category/alphabetical index generation, static-file copy, sitemap, `serve`, and `quick`. `quick` runs its Bash preview through Git Bash. | Full build and serve recipes, plus the faster `quick` preview. |
| `code`                                                                                        | Format, build, Clippy, nextest, doctests, package run, and temp cleanup. `heavy_test` and `heavytestall` are Unix-only.                                                                   | Same, including heavy tests that start external services.      |
| `deps`                                                                                        | Cargo update, crate information, and cargo-deny checks.                                                                                                                                   | Same.                                                          |
| `docker`                                                                                      | `push_dev` only; it requires Docker credentials and publishes an image.                                                                                                                   | Docker image push and service-list recipes.                    |
| `docs`                                                                                        | The `doc` and `docall` preview recipes return an explicit Unix-only error.                                                                                                                | Build and serve package or workspace documentation.            |
| `gh`                                                                                          | All listed recipes run through Git Bash and use the GitHub CLI.                                                                                                                           | All listed recipes.                                            |
| `indices`                                                                                     | `crate_indices` and `crate_page` use the built helper binary.                                                                                                                             | Those recipes and the Bash-based index/reference generators.   |
| `links`                                                                                       | Link insertion, cross-linking, URL suggestions, inline-link conversion, and reference lookup run through Git Bash.                                                                        | Same.                                                          |
| `precommit`                                                                                   | Fix, setup, Git Town configuration, and the native Python installer.                                                                                                                      | Fix, setup, Git Town configuration, and the Unix installer.    |
| `recipes`                                                                                     | Recipe table fixer runs with Python.                                                                                                                                                      | Recipe table fixer runs with Python.                           |
| `refdefs`                                                                                     | Sort reference files with the shared Bash helper. Other collection and validation scripts are Unix-only.                                                                                  | Sort, check, collect, and relabel recipes.                     |
| `spelling`                                                                                    | `spell` runs the pinned Typos configuration over book, draft, and future Markdown. The optional Aspell check and dictionary recipes run through Git Bash.                                 | Same Typos and Aspell recipes.                                 |
| `utils`                                                                                       | `mdbook-utils` and `templ` helper commands.                                                                                                                                               | Same.                                                          |

The full book build and normal `serve` recipe ensure the pinned `mdbook-indexing` preprocessor is installed. `quick` is an intentionally reduced preview that skips preprocessors and other full-build checks. Git Bash is the shell for Windows recipes.

The `links insert_category_links` recipe previews category links in published `src/` pages without changing files. Review the diff, then run `links insert_category_links_apply` to write the changes. Categories with a published book page use a local `[p~category-...]` reference; other categories keep their crates.io link. Pass `--include-drafts` to the script to preview or apply the same behavior to draft pages. The preprocessor-directive variant is tracked in issue [#248](https://github.com/john-cd/rust_howto/issues/248).

## Just version and module behavior

Use the pinned `just` version shown in `.devcontainer/.env`; the current supported baseline is `1.59.0`. The development-container setup scripts install the pinned version. To inspect the commands exposed by the root and an individual module, run these from the repository root:

```powershell
just --version
just --justfile bk/justfile --list --unsorted
just --justfile bk/scripts/links/mod.just --list --unsorted
```

An included module has its own settings and default recipe directory. Set the module's `windows-shell` explicitly when Windows needs a different shell; the root setting is not inherited. Recipes without a platform attribute are shared, `[windows]` selects the Windows implementation, and `[unix]` recipes are not available on Windows. Keep repository paths explicit when a recipe runs from a module directory.

## Contributor commands

From `bk/`, run `just ba`, `just ca`, and `just nta` for the example workspace build, Clippy, and tests. Run `just bb` to build the book and `just s` to serve it. Use `just q` for a faster Unix-only preview. A focused example test is `just ntt <keyword>` from `bk/`; direct Cargo tests run from `bk/crates/`.

[docker~website]: https://docker.com
[just~website]: https://just.systems/man/en
