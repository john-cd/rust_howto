#!/usr/bin/env bash
# Purpose: Exercise refresh_issues.sh with a fake gh; no GitHub reads or writes.
# Usage: bash test_refresh_issues.sh
#
# TODO Reviewed ??-??-2026.

set -euo pipefail

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
tmp_root="$(mktemp -d "${TMPDIR:-/tmp}/rust-howto refresh.XXXXXX")"
trap 'rm -rf -- "$tmp_root"' EXIT

repo="${tmp_root}/my repo"
mkdir -p "${repo}/src" "${repo}/.worktrees/a"
git -C "$repo" init -q
git -C "$repo" config user.email test@example.invalid
git -C "$repo" config user.name test
printf '// [fix the parser](https://github.com/john-cd/rust_howto/issues/7)\n// [closed note](https://github.com/john-cd/rust_howto/issues/8)\n' >"${repo}/src/lib.rs"
printf '// [other checkout](https://github.com/john-cd/rust_howto/issues/9)\n' >"${repo}/.worktrees/a/w.rs"
git -C "$repo" add src
git -C "$repo" add -f .worktrees/a/w.rs
git -C "$repo" commit -q -m init

# Fake gh with per-issue state files: view reads, edit logs.
state="${tmp_root}/state"
mkdir -p "$state"
printf 'OPEN' >"${state}/7.state"
printf 'src/lib.rs: old title' >"${state}/7.title"
printf 'Acceptance: the parser handles X.\nAssignee note kept.' >"${state}/7.body"
printf 'CLOSED' >"${state}/8.state"
fake="${tmp_root}/fake-gh.sh"
cat >"$fake" <<'EOS'
#!/usr/bin/env bash
if [[ $1 == issue && $2 == view ]]; then
  case $5 in
    state) cat "${FAKE_STATE}/$3.state" ;;
    title) cat "${FAKE_STATE}/$3.title" ;;
    body) cat "${FAKE_STATE}/$3.body" ;;
  esac
  exit 0
fi
if [[ $1 == issue && $2 == edit ]]; then
  printf '%s\n' "$@" >>"${FAKE_LOG}"
  exit 0
fi
exit 1
EOS
chmod +x "$fake"
export GH="$fake" FAKE_STATE="$state" FAKE_LOG="${tmp_root}/edit.log"
: >"$FAKE_LOG"

# Fail with a message.
fail() {
  echo "FAIL: $*" >&2
  exit 1
}

# Preview is read-only and mentions only the open, in-scope issue.
out="$(bash "${script_dir}/refresh_issues.sh" "$repo")"
grep -q 'would update #7' <<<"$out" || fail "preview missing #7"
grep -q '#9' <<<"$out" && fail "other checkout scanned"
[[ ! -s $FAKE_LOG ]] || fail "preview edited an issue"

# Apply edits the title and appends a delimited section, preserving the body text.
bash "${script_dir}/refresh_issues.sh" --apply "$repo" >/dev/null
grep -qx 'src/lib.rs: fix the parser' "$FAKE_LOG" || fail "title not updated"
grep -q 'Acceptance: the parser handles X.' "$FAKE_LOG" || fail "acceptance text lost"
grep -q 'Assignee note kept.' "$FAKE_LOG" || fail "body text lost"
grep -q 'source-link:start' "$FAKE_LOG" || fail "generated section missing"
grep -q -- '--add-assignee\|--add-label' "$FAKE_LOG" && fail "assignees or labels changed"
grep -q ' 8$\|^8$' "$FAKE_LOG" && fail "closed issue edited"

echo "refresh_issues tests passed"
