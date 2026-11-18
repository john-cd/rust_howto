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
- [#2275][run] · Audited 479 rendered Rust fences against the Playground
  crate list and local execution requirements. Moved 46 workspace-only
  blocks and excerpts out of the Playground and documented local commands.
  The final render has 329 runnable blocks, down from 375. The Slices and
  Ownership controls are unchanged, and the Slices book doctests passed.
  Offline hashing, cache, number-formatting, and plugin tests passed.
  The remaining Windows Cargo sweep passed on `ad96171a1` with Rust 1.100.0
  on `x86_64-pc-windows-msvc`. The locked, offline run covered 25 example
  targets with 212 tests passed, zero failures, and one existing multimap
  test ignored because it requires external resources.
- [#2276][owners] · Corrected `Clone`, `Copy`, reborrowing, and `Box`
  guarantees in `d31ce9992`. All 17 ownership example tests and strict
  Clippy passed.
- [#2277][life] · Corrected common lifetime selection, nested references,
  and static bounds in `d6ed36b28`. All 22 lifetime example tests and
  strict Clippy passed. The new negative example produced `E0597`.
- The duplicate future No Allocation preview was removed under
  [#2361](https://github.com/john-cd/rust_howto/issues/2361). Platform and ML
  migration readiness is recorded in [later/PLATFORM_READINESS.md](later/PLATFORM_READINESS.md).
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

## Publication follow-ups

The former draft book is retired under [#2349](https://github.com/john-cd/rust_howto/issues/2349). Its chapter migrations are complete; published chapters live in `bk/src`, and unpublished material stays in `later/src`. The historical migration and curation records remain in Git history. Publication alone does not close an issue's remaining runtime or acceptance checks.

- Verify any remaining acceptance criteria for WebSockets [#2250][i2250], Number Formatting [#2251][i2251], Images [#2252][i2252], `no_std` [#2253][i2253], and No Allocation [#2269][i2269].
- Continue the focused workflow work tracked by Tokio cancellation [#2257][i2257], the loopback HTTP client [#2280][http], Axum updates [#2259][i2259], bounded retries/backpressure [#2260][i2260], and Unicode [#2278][unicode]. Recheck each live issue and current published content before duplicating work.
- Review future content through macro diagnostics [#2281][macro], [Wasm basics][wasm], WIT guest/host work [#2105][i2105], and local Lambda invocation [#2267][i2267]. Validate the documented target and host/browser behavior before promotion.

For each promotion, move the chosen prose and references with its runnable example, update the published contents and indexes, and repair staging links. Run targeted locked tests, render the affected books, and record validation on the owning issue. Preserve human review markers and record review decisions on [#2216][i2216].

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
