# Book Editing and Example Code Development

{{#include book_editing_and_example_code_development.incl.md}}

Run `just` from `bk/` to list the available book and example recipes. This directory contains the published book and its runnable Rust examples.

Use `just s` to serve a preview at [http://localhost:3000][localhost:3000]↗. Use `just q` for the faster preview path on Unix, and `just bb` to build the book.

## Edit a chapter

Add or edit Markdown under `bk/src/`. The table of contents in `bk/src/SUMMARY.md` controls navigation, so add an entry whenever you add or move a published chapter. Many pages include a sibling `*.incl.md` file and shared reference definitions; preserve those fragments instead of copying their contents into generated pages.

## Add or update an example

Runnable examples embedded in the book live under `bk/crates/<section-or-category>/examples/<chapter>/`. Keep the book-visible snippet between `// ANCHOR: example` and `// ANCHOR_END: example`, and include that anchor from the chapter:

```text
{{#include path/to/example.rs:example}}
```

The hidden part of an example normally provides `pub fn run() { main(); }` and a `#[test] fn test() { main(); }` so workspace tests exercise it. A very short snippet can live directly in Markdown, but it will not be formatted, linted, or tested by the Rust workspace.

For a complex standalone example that does not fit the embedded-example layout, use the separate `xmpl/` workspace and link to it from the relevant chapter.

## Validate your changes

Run these commands from `bk/` for changes to the published examples:

```text
just fa
just ba
just ca
just nta
```

Use `just ntt <keyword>` for a focused test. For one Cargo test, run `cargo test -p <crate> <test_name>` from `bk/crates/`. After changing book content, use `just bb` to build it. In your pull request, list the checks you ran and any service, GUI, or platform-specific checks that could not run locally. Pull requests validate changes without publishing the book.

## Rust Playground

A `rust` code block can show a play button that runs the code in the [Rust Playground][rust-playground~website]↗{{hi:Rust Playground}}. Add the `mdbook-runnable` attribute to keep the button when `ignore` is set. The Playground supports a limited set of libraries; use `noplayground` when an example cannot run there.

## Related Topics {#related-topics .skip}

- [[dev_container_and_docker | Dev Container and Docker]].
- [[documentation_generation | Documentation Generation]].
- [[repository_structure | Repository Structure]].
- [[topics_of_interest | Topics of Interest]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[development_editing: review](https://github.com/john-cd/rust_howto/issues/523)
</div>
