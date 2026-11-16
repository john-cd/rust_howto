<!-- TODO Reviewed ??-??-2026. -->

<!-- TODO Reviewed 10-25-2026 -->

# Task tracking

Track implementation, acceptance criteria, and validation evidence in
[GitHub Issues][issues]. Recheck the live issue before starting work.

## Book review, November 16, 2026

Reviewed the committed `develop` revision [`31b76b925`][baseline], which
matched `origin/develop`. The inventory covers 133 published, 245 draft, and
184 future table-of-contents entries. Spot checks focused on foundational
Rust semantics, the first promoted recipes, and the next migration candidates.

- [#2273][links] · The full local link-check build passed on `75c31d963`.
  The release check now covers all published HTML pages, including the root
  and legacy introduction URLs. It passes on the current render and detects
  unresolved references and broken links in the original audit render.
- [#2274][index] · Verified all word-index fragments in the current render.
  The release check in `465832a57` catches all 14 failures from the old render.
  The check is integrated on `develop`.
- **P2** [#2275][run] · Marked five recipe pages for local Cargo runs in
  `bca582811` and verified their rendered controls. The image, WebSocket,
  and fixed-capacity tests passed. The host checksum ran, and the Polars
  example passed `cargo check`. The remaining published recipes still need
  an execution-requirements audit.
- [#2276][owners] · Corrected `Clone`, `Copy`, reborrowing, and `Box`
  guarantees in `d31ce9992`. All 17 ownership example tests and strict
  Clippy passed.
- [#2277][life] · Corrected common lifetime selection, nested references,
  and static bounds in `d6ed36b28`. All 22 lifetime example tests and
  strict Clippy passed. The new negative example produced `E0597`.
- **P2** [#2269][i2269] · The future No Allocation index still includes a file
  removed by the published migration. Fix and rerender the preview before
  closing.
- **P2** [#2279][ranges] · The draft HTTP range recipe can combine chunks from
  different versions and leave a partial destination.

The published and draft HTML builds completed with mdBook 0.5.4. The
published link checker reported 224 potential incomplete links, a mix of
reader-facing defects and literal syntax examples. The future render reported
a missing include even though the chapter remained in its table of contents.
Rust 1.100 Windows probes confirmed shared allocation after `Arc::clone`,
boxed-slice pointer metadata, a valid mutable reborrow, and a safe mutation
through `&mut str`. Locked offline tests passed for WebSockets,
No Allocation, and Unicode (four tests). Full workspace tests and external
links were outside this review.

## Draft migration plan

1. **Finish the landed promotions.** WebSockets [#2250][i2250], Number
   Formatting [#2251][i2251], headless Images [#2252][i2252], and the host
   `no_std` library [#2253][i2253] are already in `develop`. Their
   issues remain open. No Allocation [#2269][i2269] is also landed and open. Verify
   published commands, navigation, and remaining staging pages on the
   integrated revision. The Polars DataFrames pilot [#2266][i2266] is landed
   and closed.
2. **Let the active draft migration finish.** The checkout has in-progress
   moves for caching, tar and compression, map recipes, date and time,
   encoding, parsing, and template engines. Recheck the migration branch and
   live issues before claiming work in those areas. Review the integrated
   published and draft tables of contents and example index before starting
   follow-on edits.
3. **Publish reader workflows from existing drafts.** Start with Tokio
   task ownership and cancellation [#2257][i2257], then a loopback HTTP
   client [#2280][http] and the Axum update endpoint [#2259][i2259]. Promote
   bounded retries and backpressure [#2260][i2260] with its own service fixture.
   In parallel, promote Unicode text [#2278][unicode] after the encoding
   navigation settles. Link it from Strings and preserve the distinction
   between grapheme count, byte length, and terminal columns.
4. **Promote focused material from `later`.** Take macro diagnostics
   [#2281][macro] next. For Wasm, use the
   [Wasm basics draft][wasm] and existing WIT guest/host work [#2105][i2105]
   to pick a single end-to-end workflow. Validate its target, build command,
   and host or browser behavior before adding it to the book. For deployment,
   start with local Lambda invocation [#2267][i2267]. Retain broader GUI,
   hardware, native OS, and service material in staging until the relevant
   checks in the blocker list are available.

For each promotion, move the chosen prose and references with the runnable
example, update both affected tables of contents and published indices, and
check any links left in staging. Run the documented command and targeted
locked tests on the minimum supported Rust version where applicable. Render
the published book and affected preview, inspect warnings and the visible
snippets, and record the revision and results on the issue. Preserve human
review markers and leave review decisions on [#2216][i2216].

## Other coverage follow-ups

The earlier [coverage audit #2249][i2249] and the [Clippy chapter #2248][i2248]
are complete. Pointer provenance [#2270][i2270], CI benchmarking
[#2271][i2271], and panic handling [#2272][i2272] are published.

- **P2** [#2254][i2254] · Document version-dependent language and standard
  library changes against the Rust 1.98.1 baseline.
- **P2** [#2255][i2255] · Update Cargo dependency and publishing guidance.
- **P2** [#2256][i2256] · Publish a measured Cargo timings workflow.
- **P2** [#2258][i2258] · Show typed library errors through a CLI boundary.
- **P3** [#2261][i2261] · Compare explicit and distributed registration.
- **P3** [#2262][i2262] · Explain floating-point summation and SIMD trade-offs.
- **P3** [#2263][i2263] · Show Jiff DST ambiguity and nonexistent times.
- **P3** [#2264][i2264] · Add a small Creusot contract proof.
- **P3** [#2265][i2265] · Show a surviving mutation and the assertion that
  detects it.
- **P3** [#2268][i2268] · Test a local Iroh peer round trip and failure cases.

<!-- recommended-issue-order:start -->

## Ready and blocked issues

### Ready

Start with the fixes and promotions listed above.

### Blocked

#### CI unavailable ([#2207][i2207])

- See the deferred hosted checks in the issue.

#### Human decisions and reviews ([#2239][i2239])

- [#2216][i2216] · Owner review of chapters, recipes, and tools.
- [#2215][i2215] · Licensing, analytics, scope, and tooling ownership.

#### Waiting for upstream or dependency ([#2116][i2116])

- See the upstream and advisory waits in the issue.

#### Hardware, external host or account ([#2240][i2240])

- [#2211][i2211] · The locked Windows Maturin/PyO3 extension builds and
  imports with CPython 3.15.2 and Maturin 1.16.0. The full devcontainer build
  needs a valid GitHub read token for `john-cd/book_tooling`.
- [#2214][i2214] · Validate macOS and FreeBSD native examples.
- [#2213][i2213] · Validate GUI, window, and hardware examples on a desktop
  or device.
- [#2202][i2202] · Run native Linux and macOS linker and workspace checks.

<!-- recommended-issue-order:end -->

[issues]: https://github.com/john-cd/rust_howto/issues
[baseline]: https://github.com/john-cd/rust_howto/tree/31b76b925db6aaff1d57de64d01588d77fdd2655
[wasm]: later/src/categories/wasm/wasm_basics.md
[i2105]: https://github.com/john-cd/rust_howto/issues/2105
[i2116]: https://github.com/john-cd/rust_howto/issues/2116
[i2202]: https://github.com/john-cd/rust_howto/issues/2202
[i2207]: https://github.com/john-cd/rust_howto/issues/2207
[i2211]: https://github.com/john-cd/rust_howto/issues/2211
[i2213]: https://github.com/john-cd/rust_howto/issues/2213
[i2214]: https://github.com/john-cd/rust_howto/issues/2214
[i2215]: https://github.com/john-cd/rust_howto/issues/2215
[i2216]: https://github.com/john-cd/rust_howto/issues/2216
[i2239]: https://github.com/john-cd/rust_howto/issues/2239
[i2240]: https://github.com/john-cd/rust_howto/issues/2240
[i2248]: https://github.com/john-cd/rust_howto/issues/2248
[i2249]: https://github.com/john-cd/rust_howto/issues/2249
[i2250]: https://github.com/john-cd/rust_howto/issues/2250
[i2251]: https://github.com/john-cd/rust_howto/issues/2251
[i2252]: https://github.com/john-cd/rust_howto/issues/2252
[i2253]: https://github.com/john-cd/rust_howto/issues/2253
[i2254]: https://github.com/john-cd/rust_howto/issues/2254
[i2255]: https://github.com/john-cd/rust_howto/issues/2255
[i2256]: https://github.com/john-cd/rust_howto/issues/2256
[i2257]: https://github.com/john-cd/rust_howto/issues/2257
[i2258]: https://github.com/john-cd/rust_howto/issues/2258
[i2259]: https://github.com/john-cd/rust_howto/issues/2259
[i2260]: https://github.com/john-cd/rust_howto/issues/2260
[i2261]: https://github.com/john-cd/rust_howto/issues/2261
[i2262]: https://github.com/john-cd/rust_howto/issues/2262
[i2263]: https://github.com/john-cd/rust_howto/issues/2263
[i2264]: https://github.com/john-cd/rust_howto/issues/2264
[i2265]: https://github.com/john-cd/rust_howto/issues/2265
[i2266]: https://github.com/john-cd/rust_howto/issues/2266
[i2267]: https://github.com/john-cd/rust_howto/issues/2267
[i2268]: https://github.com/john-cd/rust_howto/issues/2268
[i2269]: https://github.com/john-cd/rust_howto/issues/2269
[i2270]: https://github.com/john-cd/rust_howto/issues/2270
[i2271]: https://github.com/john-cd/rust_howto/issues/2271
[i2272]: https://github.com/john-cd/rust_howto/issues/2272
[links]: https://github.com/john-cd/rust_howto/issues/2273
[index]: https://github.com/john-cd/rust_howto/issues/2274
[run]: https://github.com/john-cd/rust_howto/issues/2275
[owners]: https://github.com/john-cd/rust_howto/issues/2276
[life]: https://github.com/john-cd/rust_howto/issues/2277
[unicode]: https://github.com/john-cd/rust_howto/issues/2278
[ranges]: https://github.com/john-cd/rust_howto/issues/2279
[http]: https://github.com/john-cd/rust_howto/issues/2280
[macro]: https://github.com/john-cd/rust_howto/issues/2281
