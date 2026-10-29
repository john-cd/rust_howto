#!/usr/bin/env bash
# Purpose: Exercise todo_issues.sh in a temporary git repository with a fake gh; no GitHub writes.
# Usage: bash test_todo_issues.sh
#
# TODO Reviewed ??-??-2026.

set -euo pipefail

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
tmp_root="$(mktemp -d "${TMPDIR:-/tmp}/rust-howto todo.XXXXXX")"
trap 'rm -rf -- "$tmp_root"' EXIT

repo="${tmp_root}/my repo"
mkdir -p "${repo}/src" "${repo}/.worktrees/a" "${repo}/testdata"
git -C "$repo" init -q
git -C "$repo" config user.email test@example.invalid
git -C "$repo" config user.name test
printf '// TODO fix the parser\n// TODO Reviewed 01-01-2026.\nfn main() {}\n' >"${repo}/src/lib.rs"
printf 'TODO write intro\nTODO Reviewed ??-??-2026\n' >"${repo}/src/page.md"
printf '// TODO ignored fixture\n' >"${repo}/testdata/f.rs"
printf '// TODO other checkout\n' >"${repo}/.worktrees/a/w.rs"
printf '// TODO untracked\n' >"${repo}/untracked.rs"
git -C "$repo" add src testdata
git -C "$repo" add -f .worktrees/a/w.rs
git -C "$repo" commit -q -m init

# Fake gh: logs create calls and answers the issue list from $FAKE_TITLES.
fake="${tmp_root}/fake-gh.sh"
cat >"$fake" <<'EOS'
#!/usr/bin/env bash
if [[ $1 == issue && $2 == list ]]; then
  printf '%s\n' "${FAKE_TITLES:-}"
  exit 0
fi
if [[ $1 == issue && $2 == create ]]; then
  echo "$*" >>"${FAKE_LOG}"
  n=$(wc -l <"${FAKE_LOG}")
  echo "https://github.com/john-cd/rust_howto/issues/${n}"
  exit 0
fi
exit 1
EOS
chmod +x "$fake"
export GH="$fake" FAKE_LOG="${tmp_root}/gh.log"
: >"$FAKE_LOG"

# Fail with a message.
fail() {
  echo "FAIL: $*" >&2
  exit 1
}

# Dry run lists only tracked, in-scope, non-reviewed notes and writes nothing.
out="$(bash "${script_dir}/todo_issues.sh" "$repo")"
grep -q 'src/lib.rs: fix the parser' <<<"$out" || fail "missing code note"
grep -q 'src/page.md: write intro' <<<"$out" || fail "missing markdown note"
grep -q 'Reviewed' <<<"$out" && fail "TODO Reviewed listed"
grep -q 'fixture\|other checkout\|untracked' <<<"$out" && fail "out-of-scope file listed"
[[ ! -s $FAKE_LOG ]] || fail "dry run called gh issue create"
grep -q '^// TODO fix the parser$' "${repo}/src/lib.rs" || fail "dry run edited a file"

# Apply creates issues, rewrites the lines, and keeps human-review markers.
bash "${script_dir}/todo_issues.sh" --apply "$repo" >/dev/null
[[ $(wc -l <"$FAKE_LOG") -eq 2 ]] || fail "expected two issues, got $(wc -l <"$FAKE_LOG")"
grep -q '^// \[fix the parser\](https://github.com/john-cd/rust_howto/issues/' "${repo}/src/lib.rs" || fail "code line not linked"
grep -q '^// TODO Reviewed 01-01-2026.$' "${repo}/src/lib.rs" || fail "reviewed marker changed"
grep -q '^TODO Reviewed ??-??-2026$' "${repo}/src/page.md" || fail "markdown reviewed marker changed"
grep -q 'TODO ignored fixture' "${repo}/testdata/f.rs" || fail "fixture edited"
grep -q 'TODO other checkout' "${repo}/.worktrees/a/w.rs" || fail "other checkout edited"

# A note whose title already exists is skipped.
printf '// TODO again\n' >"${repo}/src/again.rs"
git -C "$repo" add src/again.rs
: >"$FAKE_LOG"
FAKE_TITLES='src/again.rs: again' bash "${script_dir}/todo_issues.sh" --apply "$repo" >/dev/null
[[ ! -s $FAKE_LOG ]] || fail "duplicate issue created"

echo "todo_issues tests passed"
