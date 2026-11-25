<!-- TODO Reviewed 2026-11-20. -->

# Task tracking

[Open GitHub issues][issues] track acceptance criteria, validation results, and blockers.

<!-- recommended-issue-order:start -->

## Open work

| Area | Next action |
| --- | --- |
| [Dependency updates and advisories][deps] | Audit the [staging and platform ZIP exceptions][zip] against their current lockfiles. The [book ZIP alerts][book-zip] still depend on ggez. |
| [Maintainer decisions][decisions] | Decide the [analytics policy][analytics] and [first-edition PDF requirements][edition]. The epic also tracks external listings, NLP and DSL coverage, bookmarks, and draft selection. |
| [Human review][review] | Select a chapter or repository file group from the epic and record the editorial decision there. |
| [Platform and hardware validation][hosts] | Run [Wasmer Singlepass on macOS][wasmer], verify [screen-reader and backend behavior][gui], or exercise the [firmware and physical-device examples][devices]. |
| [Code examples][examples] | Compare the client-only and embedded [SurrealDB examples][surreal] and audit their locked license graphs. |
| [PDF accessibility][pdf] | Verify [reading order, multi-page tables, and keyboard link access][reading] in a current full-book PDF. |
| [Book link checking][linkcheck] | Identify links that depend on parent-directory traversal and verify the strict local check with traversal disabled. |

<!-- recommended-issue-order:end -->

[issues]: https://github.com/john-cd/rust_howto/issues?q=is%3Aissue+is%3Aopen
[deps]: https://github.com/john-cd/rust_howto/issues/2116
[zip]: https://github.com/john-cd/rust_howto/issues/2434
[book-zip]: https://github.com/john-cd/rust_howto/issues/2402
[decisions]: https://github.com/john-cd/rust_howto/issues/2215
[analytics]: https://github.com/john-cd/rust_howto/issues/2409
[edition]: https://github.com/john-cd/rust_howto/issues/2417
[review]: https://github.com/john-cd/rust_howto/issues/2216
[hosts]: https://github.com/john-cd/rust_howto/issues/2240
[wasmer]: https://github.com/john-cd/rust_howto/issues/2431
[gui]: https://github.com/john-cd/rust_howto/issues/2428
[devices]: https://github.com/john-cd/rust_howto/issues/2355
[examples]: https://github.com/john-cd/rust_howto/issues/2435
[surreal]: https://github.com/john-cd/rust_howto/issues/2282
[pdf]: https://github.com/john-cd/rust_howto/issues/2444
[reading]: https://github.com/john-cd/rust_howto/issues/2454
[linkcheck]: https://github.com/john-cd/rust_howto/issues/2559
