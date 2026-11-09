<!-- TODO Reviewed 10-25-2026 -->

# Task tracking

Open work is tracked in [GitHub Issues](https://github.com/john-cd/rust_howto/issues).
Keep one topic per issue, except epics, with a problem, task list, and acceptance criteria.
Record status and validation evidence in the issue rather than duplicating checklists here.

<!-- recommended-issue-order:start -->

## Ready and blocked issues

Snapshot: November 9, 2026, from live GitHub issues. Issues #2238 and #2203 were completed on November 9.
Ready issues can start now, in priority order. Blocked issues are grouped by blocker; each group
has one tracking issue labelled `tracking`. Epics are listed where they wait on a blocker.
Recheck the live issue before starting.

### Ready

- [#2235: Remaining live checks: Lambda invoke, MinIO, OIDC PKCE, OTel collector](https://github.com/john-cd/rust_howto/issues/2235)
- [#2211: Validate the Python 3.15.2 devcontainer and Maturin/PyO3 example](https://github.com/john-cd/rust_howto/issues/2211)
- [#2225: Remaining tool-crate test gaps and mdbook-utils vendoring decision (split from #2206)](https://github.com/john-cd/rust_howto/issues/2225)

### Blocked

#### CI unavailable — tracker [#2207](https://github.com/john-cd/rust_howto/issues/2207)

- Only the tracker itself (its body lists the deferred checks).

#### Human decisions and reviews — tracker [#2239](https://github.com/john-cd/rust_howto/issues/2239)

- [#2216: Owner review of chapters, recipes and tools](https://github.com/john-cd/rust_howto/issues/2216)
- [#2215: Maintainer decisions: licensing, analytics, scope, tooling ownership](https://github.com/john-cd/rust_howto/issues/2215)

#### Waiting for upstream or dependency — tracker [#2116](https://github.com/john-cd/rust_howto/issues/2116)

- Only the tracker itself (its body lists the deferred checks).

#### Hardware, external host or account — tracker [#2240](https://github.com/john-cd/rust_howto/issues/2240)

- [#2236: Build heavy native stacks and re-run all-feature Clippy in `later` crates](https://github.com/john-cd/rust_howto/issues/2236)
- [#2214: Validate macOS and FreeBSD native examples](https://github.com/john-cd/rust_howto/issues/2214)
- [#2213: Validate GUI, window and hardware examples on a desktop or device](https://github.com/john-cd/rust_howto/issues/2213)
- [#2202: Run native Linux and macOS linker and workspace validation for Cargo profiles](https://github.com/john-cd/rust_howto/issues/2202)

<!-- recommended-issue-order:end -->

## Blocker tracking

- [Upstream release and advisory waits](https://github.com/john-cd/rust_howto/issues/2116).
- Host, service and human-dependent work: #2207-#2216 above.
