<!-- TODO Reviewed ??-??-2026. -->

<!-- TODO Reviewed 10-25-2026 -->

# Task tracking

Track implementation, acceptance criteria, and validation evidence in
[GitHub Issues][issues]. Recheck the live issue before starting work.

## Publication review, November 18, 2026

The book needs another editorial pass before publication. The [review on #2216][nov18-review] records the
chapter assessment and a crate-by-crate comparison for all 61 crates.io categories in the published sidebar.
The audit used local develop revision `5ca1a6f3d`. Its 471 TOC entries rendered successfully with mdBook 0.5.4.
All-chapters, word-index and search-index checks passed. Of the 336 pages with Rust code, 35 had no
incoming link from the example index. Some are overview pages that share recipes.

| Priority | Reader-facing work |
| --- | --- |
| P2 | Correct the [shared-reference call-trait rule](https://github.com/john-cd/rust_howto/issues/2394) in Closures. |
| P2 | Repair [stale draft links][nov18-links] and [missing profiling redirects][nov18-redirects]. |
| P2 | Reconcile [recipe and crate indices][nov18-indices] and [Playground eligibility][nov18-play]. |
| P2 | Fix the [benchmark correctness check][nov18-bench], [broadcast task completion][nov18-broadcast] and [standalone Moka setup][nov18-cache]. |
| P2 | Make the [example integrity report][nov18-integrity] distinguish command walkthroughs, retired roots and genuinely untested Rust. |
| P3 | Improve [sidebar and related-topic navigation][nov18-nav] and the [worked A* explanation][nov18-astar]. |
| P3 | Add recipes for [borrowed and owned JSON input][nov18-serde] and [streaming Zstandard][nov18-zstd]. |

The locked, offline Windows example targets for caching, async channels and graph algorithms passed 50 tests.
Removing both sorts from a temporary copy of the benchmark still passed its checksum test. Workspace tests alone
also miss setup instructions that omit a required feature. The full example workspace, other platforms and PDF
were outside this audit. The November 17 ready list is historical; #2364 and #2365 have since closed.

[nov18-review]: https://github.com/john-cd/rust_howto/issues/2216#issuecomment-6994339621
[nov18-links]: https://github.com/john-cd/rust_howto/issues/2389
[nov18-redirects]: https://github.com/john-cd/rust_howto/issues/2390
[nov18-indices]: https://github.com/john-cd/rust_howto/issues/2387
[nov18-play]: https://github.com/john-cd/rust_howto/issues/2393
[nov18-bench]: https://github.com/john-cd/rust_howto/issues/2382
[nov18-broadcast]: https://github.com/john-cd/rust_howto/issues/2383
[nov18-cache]: https://github.com/john-cd/rust_howto/issues/2384
[nov18-integrity]: https://github.com/john-cd/rust_howto/issues/2392
[nov18-nav]: https://github.com/john-cd/rust_howto/issues/2388
[nov18-astar]: https://github.com/john-cd/rust_howto/issues/2385
[nov18-serde]: https://github.com/john-cd/rust_howto/issues/2386
[nov18-zstd]: https://github.com/john-cd/rust_howto/issues/2391

## Historical review, November 17, 2026

The current work below supersedes the historical snapshots that follow. Recheck live issues
before implementation; a historical TODO link is not proof that an issue remains open.

### Ready

- **P2 — [#2364](https://github.com/john-cd/rust_howto/issues/2364):** Reject missing, empty,
  and incomplete rendered output in the all-chapters release check. Missing and empty
  directories currently return success.
- **P2 — [#2365](https://github.com/john-cd/rust_howto/issues/2365):** Keep Windows non-PDF
  builds usable when the optional Pandoc backend is installed. The current non-PDF route
  still invokes it and fails with Windows error 206 after HTML/linkcheck succeeds.
- **P3 — [#2264](https://github.com/john-cd/rust_howto/issues/2264):** Add a Creusot contract
  example. This remains open in the live issue snapshot.
- Related tooling: **P1** [book_tooling#89](https://github.com/john-cd/book_tooling/issues/89)
  tracks deletion of used Unicode reference definitions; **P2**
  [mdbook-utils#278](https://github.com/john-cd/mdbook-utils/issues/278) tracks invalid extracted
  Rust from hidden hash markers.

### Validation and owner follow-ups

- [#2207][i2207] now tracks remaining hosted checks and Book Tooling checkout failures.
  Actions is running jobs in this repository; it is not uniformly unavailable. The recent
  Windows run failed at `Check out Book Tooling`. PR #2303 is closed and unmerged.
- [#2116][i2116], [#2247](https://github.com/john-cd/rust_howto/issues/2247), and
  [#2245](https://github.com/john-cd/rust_howto/issues/2245) own dependency/advisory follow-ups;
  [#2282](https://github.com/john-cd/rust_howto/issues/2282) and
  [#2283](https://github.com/john-cd/rust_howto/issues/2283) own license metadata decisions.
- [#2216][i2216] and [#2215][i2215] retain human editorial and maintainer decisions.
  Automated review does not replace those decisions or date human review markers.
- [#2240][i2240], [#2211][i2211], [#2213][i2213], [#2214][i2214], and [#2202][i2202]
  retain host, credential, hardware, and platform acceptance work.
- The other chat owns the `later/` migration plan and its migration/readiness issues.
  This review does not change that plan or duplicate its checklist.

The fresh published HTML contains 397 HTML files and passes the current all-chapters and
word-index checks. The full mdBook command failed after HTML/linkcheck, in the installed
Pandoc backend (Windows error 206); it is not a successful full book build. The search index
is about 18.97 MB uncompressed; browser startup measurements remain unverified. The tooling
repositories pass 628 tests combined and strict all-target Clippy. Full book-example Cargo
tests, external links, PDF, deployment, and native Linux/macOS checks were not run here.

## Historical book review, November 16, 2026

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

## Historical publication follow-ups

The former draft book is retired under [#2349](https://github.com/john-cd/rust_howto/issues/2349). Its chapter migrations are complete; published chapters live in `bk/src`, and unpublished material stays in `later/src`. The historical migration and curation records remain in Git history. Publication alone does not close an issue's remaining runtime or acceptance checks.

- Verify any remaining acceptance criteria for WebSockets [#2250][i2250], Number Formatting [#2251][i2251], Images [#2252][i2252], `no_std` [#2253][i2253], and No Allocation [#2269][i2269].
- Continue the focused workflow work tracked by Tokio cancellation [#2257][i2257], the loopback HTTP client [#2280][http], Axum updates [#2259][i2259], bounded retries/backpressure [#2260][i2260], and Unicode [#2278][unicode]. Recheck each live issue and current published content before duplicating work.
- Review future content through macro diagnostics [#2281][macro], [Wasm basics][wasm], WIT guest/host work [#2105][i2105], and local Lambda invocation [#2267][i2267]. Validate the documented target and host/browser behavior before promotion.

For each promotion, move the chosen prose and references with its runnable example, update the published contents and indexes, and repair staging links. Run targeted locked tests, render the affected books, and record validation on the owning issue. Preserve human review markers and record review decisions on [#2216][i2216].

## Historical coverage follow-ups

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

## Historical ready and blocked snapshot

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
