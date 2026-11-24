<!-- TODO Reviewed ??-??-2026. -->

# Book Editing and Example Code Development

{{#include book_editing_and_example_code_development.incl.md}}

Run `just` from `bk/` to list the available book and example recipes. This directory contains the published book and its runnable Rust examples.

Use `just s` to serve a preview at [http://localhost:3000][localhost:3000]↗. Use `just q` for the faster preview path on Unix, and `just bb` to build the book.

## Edit a chapter

Address the reader directly when giving instructions, for example, “Run `cargo test`.” When explaining behavior in a chapter or a code comment, name the actor, for example, “The parser returns an error.” Reserve first person for an identifiable speaker, such as a maintainer describing a decision or a user quoted in an example.

Add or edit Markdown under `bk/src/`. The table of contents in `bk/src/SUMMARY.md` controls navigation, so add an entry whenever you add or move a published chapter. Many pages include a sibling `*.incl.md` file and shared reference definitions; preserve those fragments instead of copying their contents into generated pages.

The keyboard-accessible **Suggest an edit** button in each page's toolbar opens its Markdown source on the `develop` branch. If the source is an include-only wrapper, edit its referenced `*.incl.md` file or shared reference file under `bk/src/`; the wrapper usually only connects those fragments.

## Add or update an example

Runnable examples embedded in the book live under `bk/crates/<section-or-category>/examples/<chapter>/`. Keep the book-visible snippet between `// ANCHOR: example` and `// ANCHOR_END: example`. In a chapter, write an include directive by joining `{{` + `#include path/to/example.rs:example` + `}}` with no spaces.

The hidden part of an example normally provides `pub fn run() { main(); }` and a `#[cfg(test)] mod tests` containing `#[test] fn test_main() { main(); }`, so workspace tests exercise it. A very short snippet can live directly in Markdown, but it will not be formatted, linted, or tested by the Rust workspace.

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

For the lint settings behind the example checks, see [Configure Clippy for a Workspace](../../code-organization/clippy.md#repository-policy).

## Rust Playground

A `rust` code block can show a play button that runs the code in the [Rust Playground][rust-playground~website]↗{{hi:Rust Playground}}. Add the `mdbook-runnable` attribute to keep the button when `ignore` is set. The Playground supports a limited set of libraries; use `noplayground` when an example cannot run there.

Test rendered Run buttons with `cargo run --manifest-path tools/Cargo.toml --locked -p book-audit -- --repo-root . playground bk/book/html --book-root bk` from the repository root. The check compares visible Rust imports with the pinned package IDs in `bk/master/playground-crates.txt` and with the resolved book workspace dependencies. For an example that needs a local dependency or a feature the Playground lacks, mark its fence `noplayground` and document the owning Cargo command and any setup requirements.

To refresh the fixture, fetch [the Playground crate list](https://play.rust-lang.org/meta/crates). Extract each `crates[].id`, replace hyphens with underscores, and write the sorted unique names one per line. Review changes before rerunning the rendered check. Confirm that a supported example still runs through the Playground and that any newly unavailable example has an equivalent local path.

## Related Topics {#related-topics .skip}

- [[dev_container_and_docker | Dev Container and Docker]].
- [[documentation_generation | Documentation Generation]].
- [[repository_structure | Repository Structure]].
- [[topics_of_interest | Topics of Interest]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
