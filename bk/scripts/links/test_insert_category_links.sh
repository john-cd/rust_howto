#!/usr/bin/env bash
# TODO Reviewed ??-??-2026.

set -euo pipefail

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)
test_root=$(mktemp -d "${TMPDIR:-/tmp}/rust howto category links.XXXXXX")
trap 'rm -rf -- "$test_root"' EXIT

book_root="$test_root/book root"
mkdir -p \
    "$book_root/src/refs" \
    "$book_root/src/categories/accessibility" \
    "$book_root/src/categories/development-tools_testing" \
    "$book_root/src/categories/rendering_engine" \
    "$book_root/src/chapters" \
    "$book_root/drafts/categories/command-line-interface" \
    "$book_root/drafts/chapters"

cat > "$book_root/src/refs/category-refs.md" <<'EOF'
[cat~accessibility]: https://crates.io/categories/accessibility
[cat~development-tools::testing]: https://crates.io/categories/development-tools::testing
[cat~rendering::engine]: https://crates.io/categories/rendering::engine
[cat~command-line-interface]: https://crates.io/categories/command-line-interface
EOF
cat > "$book_root/src/categories/accessibility/index.md" <<'EOF'
# Accessibility
EOF
cat > "$book_root/src/categories/development-tools_testing/index.md" <<'EOF'
# Testing
EOF
cat > "$book_root/src/categories/rendering_engine/index.md" <<'EOF'
# Rendering Engine
EOF
cat > "$book_root/drafts/categories/command-line-interface/index.md" <<'EOF'
# Command-line interface
EOF
cat > "$book_root/src/chapters/topic.md" <<'EOF'
Accessibility, development-tools::testing, rendering engine, and command-line-interface.
# Accessibility headings stay unchanged.
## Accessibility {#accessibility-heading}
The engine is configured separately.
[[Accessibility]]
[Accessibility](https://example.test/Accessibility)
[Accessibility][existing]
Code: `Accessibility`
{{hi:Accessibility}}
```text
Accessibility in a fenced code block.
```
{{#include refs.incl.md}}
EOF
cat > "$book_root/src/chapters/refs.incl.md" <<'EOF'
<!-- Existing local references stay in place. -->
[existing]: https://example.test/accessibility
EOF
cat > "$book_root/drafts/chapters/topic.md" <<'EOF'
Command-line-interface is still a draft category.
{{#include refs.incl.md}}
EOF
cat > "$book_root/drafts/chapters/refs.incl.md" <<'EOF'
<!-- Draft references. -->
EOF

if ! "$script_dir/insert_category_links.sh" "$book_root" > "$test_root/dry-run.diff"; then
    printf 'Dry-run command failed.\n' >&2
    exit 1
fi
if ! grep -Fq '[Accessibility][p~category-accessibility]{{hi:accessibility}}' "$test_root/dry-run.diff"; then
    printf 'Dry-run did not link to the published category page.\n' >&2
    exit 1
fi
if ! grep -Fq '[command-line-interface][cat~command-line-interface]↗{{hi:command-line-interface}}' "$test_root/dry-run.diff"; then
    printf 'Published content did not fall back when only a draft category page exists.\n' >&2
    exit 1
fi
if [[ $(wc -l < "$book_root/src/chapters/refs.incl.md") -ne 2 ]]; then
    printf 'Dry-run changed a reference file.\n' >&2
    exit 1
fi

"$script_dir/insert_category_links.sh" "$book_root" --apply > "$test_root/apply.log"
if ! grep -Fq '[development-tools::testing][p~category-development-tools::testing]{{hi:development-tools::testing}}' "$book_root/src/chapters/topic.md"; then
    printf 'Namespaced category did not use its published page.\n' >&2
    exit 1
fi
if ! grep -Fq '[rendering engine][p~category-rendering::engine]{{hi:rendering::engine}}' "$book_root/src/chapters/topic.md"; then
    printf 'Full namespaced category phrase did not use its published page.\n' >&2
    exit 1
fi
for unchanged_line in \
    '# Accessibility headings stay unchanged.' \
    '## Accessibility {#accessibility-heading}' \
    'The engine is configured separately.' \
    '[[Accessibility]]' \
    '[Accessibility](https://example.test/Accessibility)' \
    '[Accessibility][existing]' \
    'Code: `Accessibility`' \
    '{{hi:Accessibility}}' \
    'Accessibility in a fenced code block.'; do
    if ! grep -Fqx -- "$unchanged_line" "$book_root/src/chapters/topic.md"; then
        printf 'Markup or code was changed: %s\n' "$unchanged_line" >&2
        exit 1
    fi
done
if ! grep -Fq 'Command-line-interface is still a draft category.' "$book_root/drafts/chapters/topic.md"; then
    printf 'The default apply mode changed draft content.\n' >&2
    exit 1
fi
if ! "$script_dir/insert_category_links.sh" "$book_root" --include-drafts > "$test_root/draft-dry-run.diff"; then
    printf 'Draft dry-run command failed.\n' >&2
    exit 1
fi
if ! grep -Fq '[Command-line-interface][p~category-command-line-interface]{{hi:command-line-interface}}' "$test_root/draft-dry-run.diff"; then
    printf 'Draft dry-run did not link to its draft-only category page.\n' >&2
    exit 1
fi
"$script_dir/insert_category_links.sh" "$book_root" --include-drafts --apply > "$test_root/draft-apply.log"
if ! grep -Fq '[Command-line-interface][p~category-command-line-interface]{{hi:command-line-interface}}' "$book_root/drafts/chapters/topic.md"; then
    printf 'Draft content did not use its draft-only category page.\n' >&2
    exit 1
fi
if ! grep -Fq '[p~category-accessibility]: ../categories/accessibility/index.md' "$book_root/src/chapters/refs.incl.md"; then
    printf 'Published category page reference was not added.\n' >&2
    exit 1
fi
if ! grep -Fq '[p~category-rendering::engine]: ../categories/rendering_engine/index.md' "$book_root/src/chapters/refs.incl.md"; then
    printf 'Namespaced category page reference was not added.\n' >&2
    exit 1
fi
if ! grep -Fq '[p~category-command-line-interface]: ../categories/command-line-interface/index.md' "$book_root/drafts/chapters/refs.incl.md"; then
    printf 'Draft category page reference was not added.\n' >&2
    exit 1
fi

printf 'Category link generation checks passed.\n'
