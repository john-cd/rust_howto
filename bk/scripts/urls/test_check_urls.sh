#!/usr/bin/env bash
# Purpose: Exercise check_urls.sh with a fake lychee, without contacting the internet.
# Usage: bash test_check_urls.sh
#
# TODO Reviewed ??-??-2026.

set -euo pipefail

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
tmp_root="$(mktemp -d "${TMPDIR:-/tmp}/rust-howto urls.XXXXXX")"
trap 'rm -rf -- "$tmp_root"' EXIT

# A book root whose path contains a space.
root="${tmp_root}/book root"
mkdir -p "${root}/src/refs"
printf '[a]: https://example.invalid/a\n' > "${root}/src/refs/refs one.md"

# Fake lychee: logs its arguments, then exits with $FAKE_STATUS after printing $FAKE_OUTPUT.
fake="${tmp_root}/fake-lychee.sh"
cat > "$fake" <<'EOS'
#!/usr/bin/env bash
printf '%s\n' "$@" >> "${FAKE_LOG}"
printf '%s\n' "${FAKE_OUTPUT:-}"
exit "${FAKE_STATUS:-0}"
EOS
chmod +x "$fake"
export LYCHEE="$fake" FAKE_LOG="${tmp_root}/args.log"

# Assert a command exits with the expected status.
expect_status() {
  local want=$1
  shift
  local got=0
  "$@" > "${tmp_root}/out.txt" 2>&1 || got=$?
  if [[ $got -ne $want ]]; then
    echo "FAIL: expected exit $want, got $got: $*" >&2
    cat "${tmp_root}/out.txt" >&2
    exit 1
  fi
}

# Success path, with a root containing spaces and a refs file with a space.
FAKE_STATUS=0 expect_status 0 bash "${script_dir}/check_urls.sh" "$root"
grep -q "book root/\*\*/\*.md" "$FAKE_LOG" || { echo "FAIL: quoted glob not passed intact" >&2; exit 1; }

# Failing lychee exits nonzero and is classified.
FAKE_STATUS=2 FAKE_OUTPUT='[404] https://example.invalid/a' expect_status 1 bash "${script_dir}/check_urls.sh" "$root"
grep -q 'broken (404/410):        [1-9]' "${tmp_root}/out.txt" || { echo "FAIL: 404 not classified" >&2; exit 1; }
FAKE_STATUS=2 FAKE_OUTPUT='[429] too many requests' expect_status 1 bash "${script_dir}/check_urls.sh" "$root"
grep -q 'throttling (429):        [1-9]' "${tmp_root}/out.txt" || { echo "FAIL: 429 not classified" >&2; exit 1; }

# Advisory mode reports but exits 0.
FAKE_STATUS=2 expect_status 0 bash "${script_dir}/check_urls.sh" --advisory "$root"
grep -q 'failures ignored' "${tmp_root}/out.txt" || { echo "FAIL: advisory notice missing" >&2; exit 1; }

# Argument validation.
expect_status 2 bash "${script_dir}/check_urls.sh"
expect_status 2 bash "${script_dir}/check_urls.sh" "${tmp_root}/missing"

echo "check_urls tests passed"
