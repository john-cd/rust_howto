<!-- TODO Reviewed ??-??-2026 -->

# Curation Record

This inventory was reviewed for [issue #1720](https://github.com/john-cd/rust_howto/issues/1720) on October 28, 2026. The unavailable Chrome export is tracked separately in [#2054](https://github.com/john-cd/rust_howto/issues/2054), as requested by the owner.

## Integrated Material

| Original notes                                                                                                            | Disposition                                                                                                                                                                       |
| ------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| VS Code debugger, sccache background, Firefox WebDriver, Pake                                                             | Already integrated on October 22 into development-environment, dev-container, and example-code chapters.                                                                          |
| Foundations and its two articles; bye; rust-shed; reactive_graph; reactive_stores; optics; want; Samoyed                  | Added contextual links to [Example Code](../src/links/example_code.md#focused-library-implementations-to-study). Samoyed's repository redirect was resolved to `espiria/samoyed`. |
| Playground dependency list; Cargo unstable features; Coroutine; Rustonomicon representations                              | Added to [Playground and Experimental APIs](../src/links/example_code.md#explore-the-playground-and-experimental-apis), with stable/unstable and platform distinctions.           |
| Hands-On Microservices with Rust, especially Chapter 8's chat service                                                     | Added to [Learning Rust](../src/links/learning_rust.md#learn-from-historical-application-examples) as historical 2018 source, with an upgrade caveat.                             |
| crates.io February 2025 update; March 2025 language-interop essay; Rust 2024 review; startup infrastructure retrospective | Added to [Dated Ecosystem Discussions](../src/links/blogs_and_forums.md#dated-ecosystem-discussions). These are dated context rather than current API claims.                     |
| Talent Plan                                                                                                               | Already linked in [Learning Rust](../src/links/learning_rust.md); no duplicate added.                                                                                             |
| cargo-semver-checks                                                                                                       | Already covered in [Maintaining Crates](categories/development-tools_cargo-plugins/maintaining.md); its 2025 GSoC background article was added there.                             |

## Entries Not Promoted

These are explicit editorial dispositions, not an unfinished curation queue:

| Original entry                                                                       | Reason                                                                                                                   |
| ------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------ |
| ForeverVM (`forevervm.com`)                                                          | A language-independent hosted execution service; the notes supplied no Rust integration or recipe.                       |
| GitHub Actions as a public database (`wesql.io/blog/use-github-actions-as-database`) | An infrastructure experiment, without a reproducible Rust example or a supported database workflow for this book.        |
| Fractalide (`github.com/fractalide/fractalide`)                                      | The note supplies a project slogan but no concrete Rust learning task; not added as an unexplained link.                 |
| Sonatype secure-Rust press release                                                   | Product announcement and marketing, rather than a Rust API or implementation guide.                                      |
| Six Rust programming mistakes article                                                | Generic historical advice; current language and error-handling chapters provide the maintained teaching context.         |
| Rust as an AI contender article                                                      | General industry commentary without a concrete Rust technique to teach.                                                  |
| Two identical Zed `unsupportedversion` notes (upstream issue #14126)                 | Duplicate editor-incident pointers without a reproducible Rust recipe; troubleshooting belongs with the upstream report. |
| `hello-world.rs` satire                                                              | Intentionally humorous dependency inflation; not a practical recipe.                                                     |
| Empty "To Sort" section                                                              | Contained no source material.                                                                                            |

## VS Code Bookmark Review

The checked-in `.vscode/bookmarks.json` contains 105 locations in 59 files, all with empty labels. They are navigation positions in existing book/source files, not a separate set of external bookmarks or written instructions. The referenced content was reviewed; no additional prose was supplied by these entries. The bookmark file is preserved.

Three pointers are stale: a line beyond the current trigonometry chapter, a companion-project container override absent from this checkout, and the old `later/src_crates/cats/science/Cargo.toml` path (the workspace is now under `later/crates`). They do not identify missing text to import. Human review markers and source TODOs at bookmarked locations remain governed by their existing issues.
