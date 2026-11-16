# Plan to promote Rust How-To chapters from `later` to `bk`

## First migrations

Start with **WebSockets**, then **number formatting**, **image processing**, and
**`no_std`**. These cover distinct reader tasks and already have local Rust
examples and tests. The proposed destinations are in the published book. Where a
neighboring chapter is still under `bk/drafts` , add an entry in
`bk/src/SUMMARY.md` that works on its own.

### 1. WebSockets — lead with the Tokio echo recipe

Promote the [WebSockets chapter][source-1] and its category introduction under
Code Examples by Category. Readers get a complete client/server round trip and
guidance on connection lifecycle, authentication, and limits. The published book
has no WebSocket chapter. The [HTTP drafts](bk/drafts/SUMMARY.md) provide future
cross-links.

- Move `tokio_tungstenite.rs` with its one-connection echo server and ephemeral
  loopback test. Keep `async_tungstenite.rs` as a secondary recipe if the
  chapter explains what readers gain from the alternative adapter. Both
  currently use Tokio.
- The blocking `tungstenite.rs` example defaults to a public echo service and
  has an ignored network test. Keep it staged until it runs against loopback by
  default, or publish it later with the external dependency clearly marked as
  optional.
- Adjust the example runner, related topics, and category introduction to match
  the recipes that actually move. Verify the async examples complete a text
  round trip and terminate cleanly.

### 2. Number formatting — publish a small, broadly useful chapter

Promote [Number Formatting][source-2] under Code Examples by Category, with a
link from the published [Strings chapter][source-3] . Lead with standard
formatting and locale grouping in `standard_and_localized.rs` , then display
units in `humansize_example.rs` . Keep `itoa.rs` and `ryu.rs` as short
specialized recipes for numeric serialization.

- Move the `number_formatting` example target and its dependencies. Check
  grouping, decimal formatting, byte units, and numeric conversions against the
  existing assertions.
- Leave identifier case conversion with [String Manipulation][source-4] , which
  already has case conversion recipes. Remove that section and target from the
  promoted number-formatting package.

### 3. Image processing — publish the headless recipes first

Promote [Images](later/src/categories/multimedia_images/images.md) and a
shortened category introduction. Lead with `images.rs` for generating a PNG,
`resize.rs` for resizing and cropping, and `convert.rs` for changing formats.
This gives readers a workflow they can run on CI or a machine without a
graphical desktop.

- The `images` target already includes `blur.rs` , `exif_metadata.rs` , and
  `optimize_png.rs` . Keep blur if the chapter remains focused. Add EXIF and PNG
  optimization when the published commands have reproducible sample inputs and
  documented outputs.
- Verify generated files using temporary paths, including decoded dimensions and
  output format. Update references to color handling and pixel buffers according
  to which pages ship. Keep rendering and GUI dependencies outside the first
  move.

### 4. `no_std` libraries — publish the feature boundary

Promote the library-focused sections of
[No `std`](later/src/categories/no-std/no_std.md) as a chapter near
[Dependencies](bk/src/code-organization/dependencies.md) . Move
`src/checksum.rs` , the library entry point, and the `no_std` host example,
which together show how one library can support both configurations.

- Explain that the host example uses `std` for printing and the library builds
  with default features disabled. Retitle “Core-only example” to distinguish
  these roles.
- Require a host library check with `--no-default-features` . Run the documented
  `thumbv7em-none-eabihf` check before promoting the bare-metal instructions.
  Keep panic-handler and board setup material staged until the embedded chapter
  has a validated target workflow.

## Next candidates

| Candidate                                                                       | Scope and condition for promotion                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| ------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Procedural macro tools][source-5]                                              | Add a companion to the published [Macros chapter](bk/src/language/macros.md). Lead with `derive_expansion.rs`, `cargo expand`, and the diagnostic-testing workflow. The book already teaches derive, attribute, and function-like macros plus `proc-macro2`. Verify that any `trybuild` or `macrotest` instructions point to a runnable fixture before presenting them as recipes. Check dependency resolution without importing the staging workspace's local patches. |
| [Wasm basics](later/src/categories/wasm/wasm_basics.md)                         | Start with `inspect_wasm_module.rs` and the shared `wasm.rs` example. Promote build and JavaScript interop instructions once a documented Wasm target build and browser run work. Keep Wasmtime, Wasmer, components, Leptos, and Yew out of the first package.                                                                                                                                                                                                          |
| [Tabular data](later/src/other/data-processing/dataframes.md)                   | Start with the Polars filter, projection, aggregation, and join recipe. Give it a focused package with optional dependencies as needed. Verify the feature-enabled build and expected rows before promoting it. Link to the existing [CSV draft](bk/drafts/categories/encoding/csv.md) when it is published. Evaluate Arrow and DataFusion separately because they bring their own dependency and fixture requirements.                                                 |
| [Rhai](later/src/other/scripting/rhai.md) and [architecture patterns][source-6] | Consider a compact Rhai chapter using `rhai.rs` and `rhai_types_and_limits.rs`, or one applied design recipe using `state_machine.rs`. The current pages are plausible follow-ups. Their examples and chapter fit need a dedicated review.                                                                                                                                                                                                                              |

Defer the broader GUI, graphics, device, native OS, and cloud batches until
their examples have evidence on the platform or service readers will use. The
[repository tracker](TODO.md) records native GUI and hardware validation under
issue #2213, macOS and FreeBSD under issue #2214, and a separate devcontainer
dependency for the PyO3 example under issue #2211.

## Migration and acceptance

1. Use one focused issue per chapter or companion page. Check for an existing
   issue before opening one. Move the chosen chapter, its Markdown includes and
   references, and the corresponding Cargo targets together. Preserve human
   review markers and existing heading anchors.
2. Add the entry to `bk/src/SUMMARY.md` . Resolve every sibling include,
   reference definition, and related-topic link against the published book.
   Check that `later/src/SUMMARY.md` and any staged pages still resolve after
   the move. Remove duplicate ownership once both books render.
3. Resolve dependencies in `bk/crates` , review its lockfile diff, and run
   locked checks and tests for the moved packages, including the chapter's
   normal example commands. Run formatting, Clippy, Markdown lint,
   `git diff --check` , and the book render. Spot-check rendered snippets,
   example index entries, links, and navigation. Record the exact commands and
   outcomes on the issue.
4. Get the author review noted in [the tracker](TODO.md) before publication.
   Keep optional network, browser, or hardware checks explicit so a local Rust
   test pass cannot stand in for them.

## Review evidence

The staging inventory contained 186 Markdown pages excluding the summary and
include fragments. A scan found 121 files with Rust includes and found a target
file and named anchor markers for each matched anchored Rust include.
Publication fit was assessed against the two `bk` tables of contents and the
relevant published or draft chapters. The recommendation is based on reader
usefulness, overlap, and how much of each chapter can be verified locally.

The locked, offline Windows test run passed 30 tests across
`web_programming_websocket`, `value_formatting`, `multimedia_images`, and
`no_std`. The public WebSocket echo test was ignored by design. A separate
locked host check of the `no_std` library with `--no-default-features` passed.
The published-book render and destination workspace checks belong to the
migration. The bare-metal target and follow-up candidates remain unverified.

[source-1]: later/src/categories/web-programming_websocket/websocket.md
[source-2]: later/src/categories/value-formatting/number_formatting.md
[source-3]: bk/src/categories/data-structures/strings.md
[source-4]: bk/drafts/categories/text-processing/string_manipulation.md
[source-5]: later/src/categories/development-tools_procedural-macro-helpers/macro_tools.md
[source-6]: later/src/other/architecture/architectural_patterns.md
