# Worktrees

<!-- TODO Reviewed ??-??-2026 -->

Updated 2026-11-03 (America/Los_Angeles). This file describes the managed layout;
use `git worktree list --porcelain` and `git status --short` for live state.

- The primary checkout stays on `develop` and uses cache slot `main`.
- `main` is an independent empty root commit, with no development history.
- Auxiliary checkouts must be direct children of `.worktrees/`, named with one
  lowercase letter: a, b, or c. Reuse clean slots and preserve their caches.
- Before moving a detached checkout, preserve its unique commits on a named
  branch and push that branch. Dirty work must be committed before switching.
- Squash related unpublished commits, rebase or fast-forward to synchronize
  `develop`, and never introduce synchronization merge commits.
- Keep backup branches until their unique commits are integrated or deliberately
  retired. Delete ordinary topic branches only after verifying they are merged.

## Storage roots

Override the `CODE_*_ROOT` environment variables before invoking tooling.
Windows defaults are `CODE_REPOS_ROOT=D:/code`,
`CODE_TEMP_ROOT=D:/temp` (`C:/temp` fallback),
`CODE_PACKAGES_ROOT=D:/packages`, and `CODE_TOOLS_ROOT=C:/tools`.
Unix uses native home/cache/temp directories. Root overrides do not move existing
installations or rewrite independent package-manager settings.

## Rust

Use `just build-env` to inspect the effective environment and
`just cargo <arguments>` to run Cargo. Windows targets use
`CODE_TARGET_ROOT/rust_howto/<slot>` (default root `D:/target`);
intermediates use `CODE_BUILD_ROOT/rust_howto/<slot>` (`C:/code_build`).
The intermediate volume must be NTFS; the runner rejects ReFS.
Unix defaults use XDG cache or `~/.cache` under `cargo-target` and
`cargo-build`. The runner enables sccache with incremental compilation disabled
and computes base directories from registered allowed worktrees.

Run `just configure-cargo-host` once per host and after changing roots.
It configures the host fallback in `CARGO_HOME/config.toml`. Run the repository's
`build-setup` recipe per checkout; local Cargo output settings take precedence and
rust-analyzer follows the same target/build links. See [README](README.md).

## Python

Use each Python project's `justfile`; its imported `scripts/env.just` configures
the environment directly. Run `just --justfile <project>/justfile env` to inspect
it. `CODE_PYTHON_ENV_ROOT`, `CODE_PYTHON_BUILD_ROOT`, and
`CODE_PYTHON_OUTPUT_ROOT` select external per-project/per-worktree locations.
See the README for this repository's project paths and commands.

## Current state

Reviewed 2026-11-03. The primary checkout is on `develop`; integration is
linear and will be synchronized with `origin/develop` after the reviewed topic
commits are pushed. Slot `a` will be detached at the integrated `develop` tip
after delivery. Slot `b` remains on `codex/issue-2164-cargo-profiles`. Slot `c`
is detached at `931d6b84` with a cherry-pick in progress; preserve it and do not
reuse it.

| Checkout        | Branch or revision                | Status and cache                                            |
| --------------- | --------------------------------- | ----------------------------------------------------------- |
| Repository root | `develop`                         | Clean after integration; cache slot `main`.                 |
| `.worktrees/a`  | detached at integrated `develop`  | Clean after delivery; cache slot `a`.                       |
| `.worktrees/b`  | `codex/issue-2164-cargo-profiles` | Clean; cache slot `b`.                                      |
| `.worktrees/c`  | detached at `931d6b84`            | Cherry-pick in progress; preserve; C: intermediate cleared. |

The primary `main` cache and worktree `a`/`b` caches remain in place. Slot `c`'s
intermediate cache was removed on 2026-11-03 after confirming no Cargo or rustc
process was active and the checkout's worktree was clean at a detached tip
already contained in `develop`. Its empty `C:/code_build/rust_howto/c` directory
was recreated. The shared `D:/sccache` remains in place. A Git cherry-pick is
currently in progress in slot `c`; preserve it until that operation is resolved.

### Branch and pull request review

- `develop` was `962858a3` and matched `origin/develop` before this task. Commit
  `a6ce8788` was integrated by fast-forward and pushed; the existing merge-commit
  count remains 26.
- Preserved the shared local `develop` refresh commit `4bfa6544d` found during
  this review; it remains linear and will be pushed with the #2168 commits.
- Deleted these seven local topic refs after verifying each was an ancestor of
  `develop`: `codex/fix-proc-macro-error2-warning`,
  `codex/issue-2117-finish-epic`, `codex/issue-2118-native-platform-ci`,
  `codex/issue-2145-badge-migration`,
  `codex/issue-2169-language-enum-clippy`, `codex/issue-2168-review-worktrees`,
  and `codex/issue-2168-worktree-inventory` (after its inventory commit is
  integrated).
- Preserved `codex/issue-2120-kube-5`: it has two patch-unique commits and no
  worktree. One changes the `kube` dependency to 5.0.0; the other refreshes
  multiple lockfiles. The branch remains for scoped review. GitHub #2120 is a
  closed benchmark PR, so the branch label and current PR number do not match.
- Deleted the merged local branch `codex/issues-2163-2164-2165-config-policy`
  after detaching the clean slot-b checkout at the `develop` head. Its commits
  remain in `develop`.
- Preserved all eight local backup branches because each has unique commits:
  `backup/retired/2026-11-01/rust-howto-devcontainer-heavy-parity`,
  `backup/retired/2026-11-01/rust-howto-issue2117-windows-review`,
  `backup/retired/2026-11-01/rust-howto-issue2157-config`,
  `backup/retired/2026-11-01/rust-howto-repo-lint-config`,
  `backup/tooling-20261031/develop-before`,
  `backup/tooling-20261031/working-state`,
  `backup/tooling-20261031/worktree-a`, and
  `backup/tooling-20261031/worktree-c`.
- Preserved remote branches with unique work: `origin/codex/develop-sync-linux`
  (two patch-unique commits),
  `origin/codex/wsl-develop-pre-sync-adf3123b0` (67 patch-unique commits),
  `origin/jules-9822694969820978133-5365577d` (one post-merge unique commit),
  and the backup refs. Deleted
  `origin/backup/tooling-20261031/worktree-b` because its tip was already in
  `develop` and it had no patch-unique commits. `origin/main` remains the
  independent empty history. No PRs are open; the reviewed closed-without-merge
  PR refs #2124 and #2152 are absent locally and remotely.
- The Jules branch commit combines Dependabot YAML quoting with example-output
  comments that are inaccurate in at least two cases. Neither part was
  cherry-picked; the unique commit remains preserved for later review.

### Temporary and generated-file review

The only search hits were three identical
`mdbook-utils/.jules/sentinel.md` review markers in the submodule checkout and
its `a` and `b` worktrees. The marker is tracked in the separate
`D:/code/mdbook-utils` clone's history (`e3b80f0`, “chore(review): add
final-review markers”), so these copies were kept. The nested submodule Git
metadata could not be read from this checkout because of a permission denial,
so it was left untouched. No other LLM-related or temporary-file candidates
were found.

Use `git worktree list --porcelain` and `git status --short` to refresh this
snapshot.
