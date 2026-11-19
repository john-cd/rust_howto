#!/usr/bin/env bash
# TODO Reviewed ??-??-2026.

set -euo pipefail

fixture="$(mktemp -d)"
cleanup() {
    rm -f -- "$fixture/src/SUMMARY.md" "$fixture/src/chapters/standalone.md" \
        "$fixture/src/chapters/index.md" "$fixture/src/chapters/nested/oauth.md" \
        "$fixture/src/chapters/nested/oauth.incl.md" "$fixture/src/indices/examples_index.md"
    rmdir -- "$fixture/src/chapters/nested" "$fixture/src/chapters" \
        "$fixture/src/indices" "$fixture/src" "$fixture"
}
trap cleanup EXIT

mkdir -p "$fixture/src/chapters/nested" "$fixture/src/indices"
cat > "$fixture/src/SUMMARY.md" <<'EOF'
# Summary

- [Chapters](chapters/index.md)
  - [Run Standalone Rust](chapters/standalone.md)
  - [Use HTTP with OAuth](chapters/nested/oauth.md)
EOF
cat > "$fixture/src/chapters/standalone.md" <<'EOF'
# Run Standalone Rust

```rust
fn main() {}
```
EOF
printf '# Chapters\n' > "$fixture/src/chapters/index.md"
printf '# Use HTTP with OAuth\n' > "$fixture/src/chapters/nested/oauth.md"
printf '| [OAuth recipe][ex~sample~oauth] | crate |\n' > "$fixture/src/chapters/nested/oauth.incl.md"

index="$fixture/src/indices/examples_index.md"
printf '# Index of Examples\n' > "$index"

bash "$(dirname "$0")/generate_index_of_examples.sh" "$fixture"
rg -qF '### [Run Standalone Rust](../chapters/standalone.md)' "$index"
rg -qF '### [Use HTTP with OAuth](../chapters/nested/oauth.md)' "$index"
rg -qF '{{#include ../chapters/nested/oauth.incl.md}}' "$index"
