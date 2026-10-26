#!/usr/bin/env bash
# Purpose: Exercise the link generators against temporary Markdown files.
# Usage: From bk/, run the matching just recipe and pass its arguments.
#
# TODO Reviewed ??-??-2026

set -euo pipefail

# Exercise both link generators against disposable Markdown under a spaced path.
script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
tmp_root="$(mktemp -d "${TMPDIR:-/tmp}/rust-howto links.XXXXXX")"
trap 'rm -rf -- "$tmp_root"' EXIT

category_root="${tmp_root}/category book"
mkdir -p \
  "${category_root}/src/refs" \
  "${category_root}/src/categories/algorithms" \
  "${category_root}/src/topics" \
  "${category_root}/drafts/categories/science"
cat > "${category_root}/src/refs/category-refs.md" <<'EOF'
[cat~algorithms]: https://crates.io/categories/algorithms
[cat~science]: https://crates.io/categories/science
[cat~computer-vision]: https://crates.io/categories/computer-vision
EOF
cat > "${category_root}/src/categories/algorithms/index.md" <<'EOF'
# Algorithms

Algorithms link to their own page only from other chapters.
EOF
cat > "${category_root}/drafts/categories/science/index.md" <<'EOF'
# Science
EOF
cat > "${category_root}/src/topics/page.md" <<'EOF'
# Topic

Use algorithms, science, and computer vision in this example.

[[algorithms | Algorithms]], `algorithms`, and <!-- algorithms --> stay unchanged.
[Algorithms](https://example.invalid) stays unchanged.
[Walk algorithms][ex~walk] stays unchanged.

```text
algorithms science computer vision
```

{{#include refs.incl.md}}
EOF
printf '%s\n' '<!-- local references -->' > "${category_root}/src/topics/refs.incl.md"

category_source_before="$(sha256sum "${category_root}/src/topics/page.md")"
category_preview="${tmp_root}/category-preview.diff"
bash "${script_dir}/insert_category_links.sh" --dry-run "$category_root" > "$category_preview"
grep -Fq -- '[algorithms][p~algorithms]↗{{hi:algorithms}}' "$category_preview"
grep -Fq -- '[science][cat~science]↗{{hi:science}}' "$category_preview"
grep -Fq -- '[computer vision][cat~computer-vision]↗{{hi:computer-vision}}' "$category_preview"
[[ "$category_source_before" == "$(sha256sum "${category_root}/src/topics/page.md")" ]]

bash "${script_dir}/insert_category_links.sh" --apply "$category_root" > /dev/null
grep -Fq -- '[algorithms][p~algorithms]↗{{hi:algorithms}}' "${category_root}/src/topics/page.md"
grep -Fq -- '[science][cat~science]↗{{hi:science}}' "${category_root}/src/topics/page.md"
grep -Fq -- '[computer vision][cat~computer-vision]↗{{hi:computer-vision}}' "${category_root}/src/topics/page.md"
grep -Fq -- '[[algorithms | Algorithms]], `algorithms`, and <!-- algorithms --> stay unchanged.' "${category_root}/src/topics/page.md"
grep -Fq -- '[Algorithms](https://example.invalid) stays unchanged.' "${category_root}/src/topics/page.md"
grep -Fq -- '[Walk algorithms][ex~walk] stays unchanged.' "${category_root}/src/topics/page.md"
grep -Fq -- 'algorithms science computer vision' "${category_root}/src/topics/page.md"
grep -Fq -- '[p~algorithms]: ../categories/algorithms/index.md' "${category_root}/src/topics/refs.incl.md"

category_after_first_apply="$(sha256sum "${category_root}/src/topics/page.md" "${category_root}/src/topics/refs.incl.md")"
cp "${category_root}/src/topics/page.md" "${tmp_root}/first-category-page.md"
bash "${script_dir}/insert_category_links.sh" --apply "$category_root" > /dev/null
category_after_second_apply="$(sha256sum "${category_root}/src/topics/page.md" "${category_root}/src/topics/refs.incl.md")"
if [[ "$category_after_first_apply" != "$category_after_second_apply" ]]; then
  diff -u "${tmp_root}/first-category-page.md" "${category_root}/src/topics/page.md" || true
  exit 1
fi

if [[ ${1:-} == "--category-only" ]]; then
  echo "Category-link generation fixture checks passed."
  exit 0
fi

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
