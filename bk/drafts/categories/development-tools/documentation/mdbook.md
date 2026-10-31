<!-- TODO Reviewed ??-??-2026 -->

# `mdBook`

{{#include mdbook.incl.md}}

## Create and Build a Book {#mdbook}

[![mdbook~repo][c~mdbook~repo~badge]][c~mdbook~repo]{{hi:mdbook}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]

[mdBook][c~mdbook~repo]↗ turns Markdown source into a searchable, linked book. A project normally keeps its configuration in `book.toml`, its chapter files under `src/`, and its navigation in `src/SUMMARY.md`.

```bash
cargo install --locked mdbook
```

Install a prebuilt release instead if you do not want to build mdBook from source. To create a starter project, run `mdbook init my-book` and then change into that directory.

```bash
mdbook serve --open
```

`mdbook serve` watches the source tree, rebuilds the book, and serves a local preview. Use `mdbook build` for a one-time build; by default, the generated site is written to `book/`.

The `SUMMARY.md` file defines chapter order and hierarchy. `book.toml` configures the source and output directories, build behavior, preprocessors, and renderers. See the [mdBook installation guide][book~mdbook~installation]↗ and [creating a book guide][book~mdbook~creating]↗ for setup and source conventions.

## Organize Chapters in `SUMMARY.md` {#summary-file}

Every chapter must appear in `src/SUMMARY.md`; its list order and indentation define the book's navigation hierarchy. A section heading groups chapters without linking to a page:

```markdown
# Summary

[Introduction](README.md)

# Guide

- [Create a book](guide/creating.md)
- [Add chapters](guide/chapters.md)
  - [Reuse content](guide/includes.md)
```

Keep chapter paths relative to the summary file and add new chapters there as part of the same change. See the [mdBook summary format][book~mdbook~summary]↗.

## Include Shared Content and Anchored Examples {#includes-and-anchors}

Use `{{#{{#include}} path/to/file.md}}` to share Markdown and `{{#{{#include}} path/to/example.rs:example}}` to insert a named region from a source file. Include paths are relative to the Markdown file containing the directive. In the source file, mark the region with matching anchors, for example `// ANCHOR: example` and `// ANCHOR_END: example` in Rust. This keeps the example in the book synchronized with compilable code.

```rust
{{#{{#include}} path/to/example.rs:example}}
```

mdBook also supports numeric line ranges and `rustdoc_include`, which hides setup code initially while keeping it available to Rustdoc. See [including files][book~mdbook~including-files]↗ and [Rust code block attributes][book~mdbook~code-block-attributes]↗.

## Test Code in Your `mdBook` {#termbook}

Run `mdbook test` to compile Rust code blocks with Rustdoc. Mark intentionally non-compiling examples with attributes such as `ignore` or `compile_fail`; use `--library-path` when the snippets need dependencies already built in a Cargo workspace. For executable shell examples, `termbook` runs command blocks and captures their output, but its published crate depends on mdBook 0.3.x and is not suitable for the mdBook 0.5.4 version pinned by this repository's CI.

[`mdbook-keeper`][c~mdbook-keeper~repo]↗ adds support for external crates and running documentation checks during a build. Its current source depends on mdBook 0.4.x, so verify it against your selected mdBook release before adopting it. This repository uses Cargo tests for its examples rather than either legacy tool. See the [mdBook test command][book~mdbook~test]↗.

## Extend mdBook with Preprocessors and Renderers {#mdbook-plugins}

Preprocessors transform a book's content before rendering; renderers turn the processed book into an output format. Configure each under `[preprocessor.<name>]` or `[output.<name>]` in `book.toml`. mdBook discovers these tools as `mdbook-<name>` executables on `PATH`. The repository pins mdBook 0.5.4 in `.devcontainer/.env` and configures `mdbook-private` and its own `mdbook-scrub` in `bk/book.toml`.

The `mdbook-hide` example below is an older third-party preprocessor whose source depends on mdBook 0.4.35; it is not configured or verified against the 0.5.4 CI pin. Treat it as an example to evaluate, not a supported dependency.

### Remove Private Draft Content with `mdbook-scrub` {#mdbook-scrub}

This repository's `mdbook-private` handles private chapters and sections. Its `mdbook-scrub` removes hidden HTML sections and includes of hidden chapters, then expands custom directives in a second stage. The ordering in `bk/book.toml` is:

```toml
[preprocessor.scrub]
command = "../bin/mdbook-scrub"
after = ["private"]
before = ["links"]

[preprocessor.directives]
command = "../bin/mdbook-scrub --stage directives"
after = ["links"]
```

The repository builds this preprocessor and renders the book in CI. Reuse it only if its specific hidden-section and directive behavior matches your book; other projects can write a small process-based preprocessor or choose a maintained plugin.

## Let Readers Execute Sample Code in the Rust Playground {#playground}

[![Rust by example - Playground][book~rust-by-example~playground~badge]][book~rust-by-example~playground] [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}

[Playground (Rust by example)][book~rust-by-example~playground]↗{{hi:Rust by example}}.

## Other Third-Party Plugin References {#augment-mdbook-with-plugins .skip}

[`mdBook`][c~mdbook~docs]↗{{hi:mdBook}} has a [third-party plugin list][c~mdbook~third-party-plugins~wiki]↗. The entries below are catalog pointers; this repository does not configure them, so its CI does not establish their compatibility with mdBook 0.5.4. Check their current release and mdBook dependency before use.

### Check Links with `mdbook-linkcheck` {#mdbook-linkcheck}

{{#badges mdbook-linkcheck}}

[`mdbook-linkcheck`][c~mdbook-linkcheck~docs]↗{{hi:mdbook-linkcheck}} is a backend for [`mdbook`][c~mdbook~docs]↗{{hi:mdbook}}, which will check your links for you.

### Hide Entire Chapters with `mdbook-private` {#mdbook-private}

{{#badges mdbook-private}}

[`mdbook-private`][c~mdbook-private~docs]↗{{hi:mdbook-private}} is a [`mdbook`][c~mdbook~docs]↗{{hi:mdbook}} pre-processor that controls visibility of private chapters and sections within them.

### Hide Pages with `mdbook-hide` {#mdbook-hide}

{{#badges mdbook-hide development-tools=}}

[`mdbook-hide`][c~mdbook-hide~docs]↗{{hi:mdbook-hide}} is a pre-processor for [`mdbook`][c~mdbook~docs]↗{{hi:mdbook}} that adds support for hidden chapters.

```bash
cargo install mdbook-hide
```

Its configuration from the project's README is:

```toml
[preprocessor.hide]
hide = true
```

Add `<!--hidden-->` to a chapter to mark it for removal when `hide = true`. The documented configuration is specific to this plugin; it does not control mdBook's built-in handling of hidden code lines.

## Build and Test a Book in CI {#cd-ci}

Run the same book checks in continuous integration that contributors use locally: install the pinned mdBook version, build the source, and run `mdbook test` when the book contains Rust code blocks. For example:

```yaml
steps:
  - name: Install mdBook
    run: cargo install --locked --version 0.5.4 mdbook
  - name: Build and test the book
    run: |
      mdbook build
      mdbook test
```

This repository pins version 0.5.4 in `.devcontainer/.env`; its Windows workflow reads that value before installing mdBook. For deployment, publish the generated `book/` directory using the hosting provider's current instructions. See the [mdBook continuous integration guide][book~mdbook~ci]↗ and [GitHub Pages documentation][github~pages~docs]↗.

## Add a Table of Contents to Each Page {#add-a-table-of-contents-to-each-page .skip}

### `mdbook-toc` {#mdbook-toc}

{{#badges mdbook-toc}}

[`mdbook-toc`][c~mdbook-toc~docs]↗{{hi:mdbook-toc}} is a pre-processor for [`mdbook`][c~mdbook~docs]↗{{hi:mdbook}} to add inline Table of Contents support.

### `mdbook-theme` {#mdbook-theme}

{{#badges mdbook-theme}}{{hi:Ace}}{{hi:Book}}{{hi:Markdown}}{{hi:Rustbook}}{{hi:Theme}}

[`mdbook-theme`][c~mdbook-theme~docs]↗{{hi:mdbook-theme}} is a pre-processor and a backend to config theme for mdbook, especially creating a pagetoc on the right and setting full color themes from the official ace editor.

### `mdbook-pagetoc` {#mdbook-pagetoc}

{{#badges mdbook-pagetoc}}{{hi:Toc}}{{hi:Table}}{{hi:Pagetoc}}{{hi:Mdbook}}{{hi:Contents}}

[`mdbook-pagetoc`][c~mdbook-pagetoc~docs]↗{{hi:mdbook-pagetoc}} is a [`mdbook`][c~mdbook~docs]↗{{hi:mdbook}} plugin that provides a table of contents for each page.

## Create Pages from a Template {#create-pages-from-a-template .skip}

### Create Pages from a Template with `mdbook-tera` {#mdbook-tera}

{{#badges mdbook-tera command-line-utilities="Command line utilities" template-engine="Template engine"}}{{hi:Pre-processor}}{{hi:Tera}}{{hi:Mdbook}}

[`mdbook-tera`][c~mdbook-tera~docs]↗{{hi:mdbook-tera}} is a Tera pre-processor for mdBook.

### Replace Text in Chapters with `yapp` {#yapp}

{{#badges mdbook-yapp template-engine="Template engine" text-processing="Text processing"}}{{hi:Text}}{{hi:Pre-processor}}{{hi:Mdbook}}{{hi:Replace}}{{hi:Pattern}}

[`yapp`][c~yapp~docs]↗{{hi:yapp}} is a [`mdbook`][c~mdbook~docs]↗{{hi:mdbook}} pre-processor that simply replaces text in chapters. Phrases to be replaced with specified content are defined in plain-text configuration file.

### Create a Journal with `mdbook-journal` {#mdbook-journal}

[![mdbook-journal~website][c~mdbook-journal~website~badge]][c~mdbook-journal~website] [![mdbook-journal][c~mdbook-journal~docs~badge]][c~mdbook-journal~docs] [![mdbook-journal~crates.io][c~mdbook-journal~crates.io~badge]][c~mdbook-journal~crates.io] [![mdbook-journal~repo][c~mdbook-journal~repo~badge]][c~mdbook-journal~repo] [![mdbook-journal~lib.rs][c~mdbook-journal~lib.rs~badge]][c~mdbook-journal~lib.rs]{{hi:mdbook-journal}}{{hi:Book}}{{hi:Gitbook}}{{hi:Markdown}}{{hi:Mdbook}}{{hi:Plugin}} [![cat~command-line-utilities][cat~command-line-utilities~badge]][cat~command-line-utilities]{{hi:Command line utilities}} [![cat~template-engine][cat~template-engine~badge]][cat~template-engine]{{hi:Template engine}}

[`mdbook-journal`][c~mdbook-journal~docs]↗{{hi:mdbook-journal}} is a journal plugin for [`mdBook`][c~mdbook~docs]↗{{hi:mdBook}}.

## Other Plugins {#other .skip}

### Preprocess your Book via Arbitrary Commands with `mdbook-cmdrun` {#mdbook-cmdrun}

{{#badges mdbook-cmdrun}}{{hi:Mdbook}}{{hi:Pre-processor}}{{hi:Runcmd}}

[`mdbook-cmdrun`][c~mdbook-cmdrun~docs]↗{{hi:mdbook-cmdrun}} is a [`mdbook`][c~mdbook~docs]↗{{hi:mdbook}} pre-processor to run arbitrary commands.

## Alternatives {#alternatives .skip}

Use `rustdoc` and `cargo doc` when the primary output is API documentation generated from Rust source. Use mdBook for a navigable guide, tutorial, or book. If you want to reuse book chapters for presentations, [`mdslides`][mdslides~repo]↗ converts Markdown chapters into reveal.js slides; it is a separate command-line tool, not an mdBook preprocessor:

```console
cargo install --locked mdslides
mdslides --mdbook-path . --template ./template.html --output-dir ./slides
```

## Related Topics {#related-topics .skip}

For API reference, see [Rustdoc][book~rustdoc]↗. For more documentation tooling, see [Document Rust Code][p~code-documentation] and the [mdBook third-party plugins list][c~mdbook~third-party-plugins~wiki]↗; verify plugin maintenance and compatibility before adding a dependency.

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}
