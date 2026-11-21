<!-- TODO Reviewed ??-??-2026. -->

# Book scripts

Run the book recipes from `bk/`; `bk/justfile` is the single entry point and imports the modules below `bk/scripts/`. The supported Just version is pinned as `JUST_VERSION` in `.devcontainer/.env` (currently `1.59.0`). On Windows all `rust_howto`-owned recipe shells use Git Bash. Recipes that need native PowerShell helpers invoke `pwsh.exe -File` explicitly; each imported module sets its own shell because justfile settings do not inherit across modules.

Use `just` to list root recipes and modules, then `just <module>` to list a module's recipes. Recipe arguments that can contain spaces are quoted with Just's `quote()` helper, and path-sensitive PowerShell commands use `-LiteralPath`. Just returns the exit status of the final command in a recipe; platform-specific commands are marked with `[unix]` or `[windows]` in the owning `mod.just` file.

## Recipe support

| Module | Windows support | Unix support |
| --- | --- | --- |
| `anchors`, `chapters`, `crates`, `examples`, `titles`, `toc`, `urls`, `wikilinks` | Their listed processing recipes have Windows implementations. Several use Git Bash; see each module's `mod.just`. | Their listed processing recipes. |
| `includes` | Read-only local-reference and subchapter include audits run through the Rust `example-integrity` command. | Same Rust audits. |
| `book` | Full build and rendered-content audits, `refdefs-generate`, `refdefs-check`, category/alphabetical index generation, static-file copy, sitemap, `serve`, and `quick`. Audits cover rendered links, the word index, Playground imports, visible TODOs, and the generated search index. | Full build and rendered-content audits, serve recipes, plus the faster `quick` preview. `future` also checks staged preview links. |
| `code` | Format, build, Clippy, nextest, doctests, package run, and temp cleanup. `heavy_test` and `heavytestall` are Unix-only. | Same, including heavy tests that start external services. |
| `deps` | Cargo update, crate information, and cargo-deny checks. | Same. |
| `docker` | `push_dev` only; it requires Docker credentials and publishes an image. | Docker image push and service-list recipes. |
| `docs` | The `doc` and `docall` preview recipes return an explicit Unix-only error. | Build and serve package or workspace documentation. |
| `gh` | All listed recipes run through Git Bash and use the GitHub CLI. | All listed recipes. |
| `indices` | `crate_indices`, `crate_page`, and example-index generation use Rust binaries. | The same Rust commands. |
| `links` | Link insertion, cross-linking, URL suggestions, inline-link conversion, and reference lookup run through Git Bash. | Same. |
| `hooks` | Run and install prek hooks, configure Git Town, and install prek on Windows. | Run and install prek hooks, configure Git Town, and install prek. |
| `recipes` | Recipe table fixer runs through the Rust `recipe-tables` command. | Recipe table fixer runs through the Rust `recipe-tables` command. |
| `refdefs` | Sort reference files with the shared Bash helper. Other collection and validation scripts are Unix-only. | Sort, check, collect, and relabel recipes. |
| `spelling` | `spell` runs the pinned Typos configuration over book, draft, and future Markdown. The optional Aspell check and dictionary recipes run through Git Bash. | Same Typos and Aspell recipes. |
| `utils` | `mdbook-utils` and `templ` helper commands. | Same. |

The full book build and normal `serve` recipe ensure the pinned `mdbook-indexing` preprocessor is installed. `quick` is an intentionally reduced preview that skips preprocessors and other full-build checks. Git Bash is the shell for Windows recipes.

The `links insert_category_links` recipe previews category links in published `src/` pages without changing files. Review the diff, then run `links insert_category_links_apply` to write the changes. Categories with a published book page use a local `[p~category-...]` reference; other categories keep their crates.io link. The optional `--include-drafts` flag remains supported for external legacy book roots; this repository no longer has a `bk/drafts` tree. The preprocessor-directive variant is tracked in issue [#248](https://github.com/john-cd/rust_howto/issues/248).

## Rust command parity review

The candidate Rust commands cover related work, but each has a behavior gap that affects its existing Just recipe. These scripts remain the authoritative entry points until the listed difference is addressed.

| Existing recipe | Rust candidate | Behavior that still differs |
| --- | --- | --- |
| `toc list_missing_chapters` | `mdbook-utils markdown identify-files-not-in-summary` | The Rust command includes `.incl.md` and `*refs.md` files and prints paths; the recipe excludes those files and emits ready-to-paste Markdown links. |
| `urls list_duplicated` | `mdbook-utils links duplicate-links` | The recipe counts raw destinations only in `src/refs/*.md`; the Rust command scans parsed links across a Markdown root and writes a Markdown report. |
| `refdefs check` | `book_tool refdefs-audit` | The Rust audit reports include-scoped provenance and unused definitions, but it does not report uses with no definition, which the recipe includes. |
| `links replace_inline_links` | `book_tool inline-links` | The recipe previews by default, writes only with `--write`, uses stable URL-hash labels in sibling `refs.incl.md` files, and skips inline code. The Rust command writes in place and stores numeric definitions in the same file. |
| `urls check` / `check_advisory` | `book_tool link-check` | The recipe checks Markdown, TOML, and YAML, classifies failures, and supports advisory exit status. The Rust command checks HTTP(S) links in Markdown and has no advisory mode. |
| `book refdefs-generate` / `refdefs-check` | `mdbook-utils refdefs generate` | The Python wrapper already calls the Rust generator, then normalizes Cargo package labels, preserves curated destinations, merges or checks the output, and verifies the pinned tool revision. |

The recipes above retain their existing implementations because direct substitution would change their output or omit checks.

## Just version and module behavior

Use the pinned `just` version shown in `.devcontainer/.env`; the current supported baseline is `1.59.0`. The development-container setup scripts install the pinned version. To inspect the commands exposed by the root and an individual module, run these from the repository root:

```powershell
just --version
just --justfile bk/justfile --list --unsorted
just --justfile bk/scripts/links/mod.just --list --unsorted
```

An included module has its own settings and default recipe directory. Set the module's `windows-shell` explicitly when Windows needs a different shell; the root setting is not inherited. Recipes without a platform attribute are shared, `[windows]` selects the Windows implementation, and `[unix]` recipes are not available on Windows. Keep repository paths explicit when a recipe runs from a module directory.

## Contributor commands

From `bk/`, run `just ba`, `just lc`, and `just t` for the example workspace build, Clippy, and tests. Run `just bb` to build the book and `just s` to serve it. Use `just q` for a faster Unix-only preview. A focused example test is `just ntt <keyword>` from `bk/`; direct Cargo tests run from `bk/crates/`.

[docker~website]: https://docker.com
[just~website]: https://just.systems/man/en
