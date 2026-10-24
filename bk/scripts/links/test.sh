#!/usr/bin/env bash
# TODO Reviewed ??-??-2026

set -euo pipefail

# Exercise both link generators against disposable Markdown under a spaced path.
script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
tmp_root="$(mktemp -d "${TMPDIR:-/tmp}/rust-howto links.XXXXXX")"
trap 'rm -rf -- "$tmp_root"' EXIT

book_root="${tmp_root}/book root"
mkdir -p "${book_root}/src/refs" "${book_root}/drafts"
cat > "${book_root}/src/refs/crate-refs.md" <<'EOF'
[c~serde~docs]: https://docs.rs/serde
EOF
cat > "${book_root}/src/recipe.md" <<'EOF'
# Crate links

Use `serde` once and `serde` again.
Existing [link][c~serde~docs] stays unchanged.
Existing inline link [`serde`](https://example.com) stays unchanged.

```rust
let example = `serde`;
```
EOF
printf '%s\n' 'Try `tokio` later.' > "${book_root}/drafts/draft.md"

bash "${script_dir}/insert_crate_links.sh" "${book_root}"
rg --quiet --fixed-strings '[`serde`][c~serde~docs]↗{{hi:serde}}' "${book_root}/src/recipe.md"
[[ $(rg --only-matching --fixed-strings '[`serde`][c~serde~docs]↗{{hi:serde}}' "${book_root}/src/recipe.md" | wc -l) -eq 2 ]]
rg --quiet --fixed-strings 'let example = `serde`;' "${book_root}/src/recipe.md"
rg --quiet --fixed-strings 'Existing inline link [`serde`](https://example.com) stays unchanged.' "${book_root}/src/recipe.md"
rg --quiet --fixed-strings 'tokio' "${book_root}/suggest.md"

before="$(sha256sum "${book_root}/src/recipe.md")"
bash "${script_dir}/insert_crate_links.sh" "${book_root}"
after="$(sha256sum "${book_root}/src/recipe.md")"
[[ "$before" == "$after" ]]

cat > "${book_root}/src/urls.md" <<'EOF'
Bare URLs include https://example.com/path, https://github.com/acme/widget).
Read https://guide.example.org/guide "the guide".
Existing link: [example](https://example.net/docs).
Code span: `https://ignored.example/code`.

```text
https://ignored.example/fenced
https://github.com/fenced/project
```
EOF
url_source_before="$(sha256sum "${book_root}/src/urls.md")"
links_output="${tmp_root}/generated links.md"
bash "${script_dir}/suggest_links_for_bare_urls.sh" "${book_root}" "$links_output"
rg --quiet --fixed-strings '[`example`][example~website] [example~website]: https://example.com/path' "$links_output"
rg --quiet --fixed-strings '[`acme/widget`][acme~widget~repo] [acme~widget~repo]: https://github.com/acme/widget' "$links_output"
rg --quiet --fixed-strings '[`guide`][guide~website] [guide~website]: https://guide.example.org/guide' "$links_output"
! rg --quiet 'ignored\.example|fenced~project|github~website' "$links_output"
[[ $(rg --fixed-strings --count '[`acme/widget`][acme~widget~repo]' "$links_output") -eq 1 ]]
[[ "$url_source_before" == "$(sha256sum "${book_root}/src/urls.md")" ]]

if bash "${script_dir}/suggest_links_for_bare_urls.sh" "${book_root}/missing" >/dev/null 2>&1; then
  echo "Expected missing input path to fail." >&2
  exit 1
fi
if bash "${script_dir}/suggest_links_for_bare_urls.sh" "${book_root}" "${tmp_root}/missing/output.md" >/dev/null 2>&1; then
  echo "Expected invalid output path to fail." >&2
  exit 1
fi

echo "Link-generation fixture checks passed."
