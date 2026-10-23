<!-- TODO Reviewed ??-??-2026 -->
# Text Diffing

{{#include diffing.incl.md}}

A diff describes how two versions of text differ. It usually groups content as unchanged, removed, or inserted. Choose the unit that makes changes useful to the reader: lines for source files and logs, words or characters for short prose, and graphemes when highlighting user-perceived characters. A diff is a comparison result; applying changes safely also requires checking the base version and the context where edits belong.

## `diff` {#diff}

[![diff][c~diff~docs~badge]][c~diff~docs] [![diff~crates.io][c~diff~crates.io~badge]][c~diff~crates.io] [![diff~repo][c~diff~repo~badge]][c~diff~repo] [![diff~lib.rs][c~diff~lib.rs~badge]][c~diff~lib.rs]{{hi:diff}}

[`diff`][c~diff~docs]↗{{hi:diff}} computes differences between slices and strings. Use `diff::lines` for a simple line-oriented result, or `diff::chars` when character-level changes are useful. Each result is unchanged content (`Both`), content only in the old input (`Left`), or content only in the new input (`Right`). The example prints these as context, removal, and insertion lines.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/diffing/diff.rs:example}}
```

## `similar` {#similar}

[![similar][c~similar~docs~badge]][c~similar~docs] [![similar~crates.io][c~similar~crates.io~badge]][c~similar~crates.io] [![similar~repo][c~similar~repo~badge]][c~similar~repo] [![similar~lib.rs][c~similar~lib.rs~badge]][c~similar~lib.rs]{{hi:similar}}{{hi:Difference}}{{hi:Diff}}{{hi:Compare}}{{hi:Changes}}{{hi:Patience}}

[`similar`][c~similar~docs]↗{{hi:similar}} provides text and sequence diff APIs with several algorithms and output helpers. `TextDiff` can compare lines, words, or characters, produce inline changes, and render a unified diff with context. It supports multiple algorithms, including Myers and patience; choose one based on the output quality and work limits your use case needs. Grapheme-level diffing is available when the crate's `unicode` feature is enabled.

The example compares lines and words, then renders a unified diff. The output is for review; it does not validate that a later patch still applies to the expected input.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/diffing/similar.rs:example}}
```

## Related topics {#related-topics .skip}

- [[development-tools | Development Tools]]
- [[strings | Strings]]
- [[unicode | Unicode]]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write; titles](https://github.com/john-cd/rust_howto/issues/1193)
</div>
