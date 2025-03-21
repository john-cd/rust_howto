# Diffing & Patching

{{#include diffing.incl.md}}

## `diff` {#diff}

[![diff][c-diff-badge]][c-diff] [![diff-crates.io][c-diff-crates.io-badge]][c-diff-crates.io] [![diff-github][c-diff-github-badge]][c-diff-github] [![diff-lib.rs][c-diff-lib.rs-badge]][c-diff-lib.rs]{{hi:diff}}

`diff` is an LCS-based slice and string diffing implementation.

```rust,editable
{{#include ../../../crates/cats/text_processing/tests/diffing/diff.rs:example}}
```

## `similar` {#similar}

[![similar][c-similar-badge]][c-similar] [![similar-crates.io][c-similar-crates.io-badge]][c-similar-crates.io] [![similar-github][c-similar-github-badge]][c-similar-github] [![similar-lib.rs][c-similar-lib.rs-badge]][c-similar-lib.rs]{{hi:similar}}{{hi:Difference}}{{hi:Diff}}{{hi:Compare}}{{hi:Changes}}{{hi:Patience}}

`similar` is a dependency-free crate for Rust that implements different diffing algorithms and high level interfaces for it.

```rust,editable
{{#include ../../../crates/cats/text_processing/tests/diffing/similar.rs:example}}
```

## Related Topics {#skip}

- [[development-tools | Development Tools]].
- [[strings | Strings]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write; titles](https://github.com/john-cd/rust_howto/issues/1193)
</div>
