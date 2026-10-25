<!-- TODO Reviewed ??-??-2026 -->

# String Search and Fuzzy Matching

{{#include string_search.incl.md}}

## Choose a search approach

- For one literal substring, start with `str::find` or `str::contains`.
- For one byte, a few possible bytes, or a byte substring, use [`memchr`][c~memchr~docs]↗{{hi:memchr}}.
- For many fixed literal patterns in one text, use [`aho-corasick`][c~aho-corasick~docs]↗{{hi:aho-corasick}} and choose how overlapping patterns should match.
- To rank candidates for a partial, in-order query, use [`fuzzy-matcher`][c~fuzzy-matcher~docs]↗{{hi:fuzzy-matcher}}.
- To compare two complete strings with a distance or similarity formula, use [`strsim`][c~strsim~docs]↗{{hi:strsim}}.

Search offsets usually count bytes. They are not character or grapheme positions. If a UI highlights user-visible text, convert byte ranges only at valid UTF-8 boundaries and account for grapheme clusters. The `fuzzy-matcher` crate returns character indices; those are still not grapheme-cluster indices. Byte-search APIs also work on arbitrary bytes and do not interpret Unicode.

Exact search, fuzzy ranking, and similarity scores answer different questions. A fuzzy score depends on the matcher and its scoring rules; a distance or similarity metric depends on the selected formula. Choose thresholds and ranking behavior for the application's data, and normalize text explicitly if the application needs Unicode normalization or case folding.

## `aho-corasick` {#aho-corasick}

[![aho-corasick][c~aho-corasick~docs~badge]][c~aho-corasick~docs] [![aho-corasick~crates.io][c~aho-corasick~crates.io~badge]][c~aho-corasick~crates.io] [![aho-corasick~repo][c~aho-corasick~repo~badge]][c~aho-corasick~repo] [![aho-corasick~lib.rs][c~aho-corasick~lib.rs~badge]][c~aho-corasick~lib.rs]{{hi:aho-corasick}}{{hi:Multi}}{{hi:Text}}{{hi:String}}{{hi:Pattern}}{{hi:Search}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

[`aho-corasick`][c~aho-corasick~docs]↗{{hi:aho-corasick}} searches for multiple fixed string patterns in one haystack. A match reports the pattern and a half-open byte range. `find_iter` returns non-overlapping matches; use `find_overlapping_iter` to enumerate overlapping matches. Configure the match kind to choose how the search resolves multiple overlapping candidates.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/string_search/aho_corasick.rs:example}}
```

## Redact Many Words at Once with `aho-corasick` {#aho-corasick-redact}

[![aho-corasick][c~aho-corasick~docs~badge]][c~aho-corasick~docs] [![aho-corasick~crates.io][c~aho-corasick~crates.io~badge]][c~aho-corasick~crates.io]{{hi:aho-corasick}}{{hi:Redaction}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

Building the automaton is the expensive step, so build it once and reuse it for every text. `ascii_case_insensitive(true)` ignores ASCII case only. Use `MatchKind::LeftmostLongest` when one pattern is a prefix of another (`pass` and `password`); the default `Standard` kind reports the match that ends first, which would redact only part of the longer word. `replace_all` takes one replacement per pattern, in the order the patterns were added. For a large dictionary of words that must match on word boundaries, check the match edges yourself, because the automaton also matches inside longer words.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/string_search/aho_corasick_redact.rs:example}}
```

## `memchr` {#memchr}

[![memchr][c~memchr~docs~badge]][c~memchr~docs] [![memchr~crates.io][c~memchr~crates.io~badge]][c~memchr~crates.io] [![memchr~repo][c~memchr~repo~badge]][c~memchr~repo] [![memchr~lib.rs][c~memchr~lib.rs~badge]][c~memchr~lib.rs]{{hi:memchr}}{{hi:Search}}{{hi:Find}}{{hi:memchr}}{{hi:Substring}}{{hi:Memmem}}

[`memchr`][c~memchr~docs]↗{{hi:memchr}} provides routines for searching one, two, or three bytes and for finding byte substrings. The search operates on bytes without interpreting their encoding, so results are byte offsets and can fall inside a multi-byte UTF-8 character when searching arbitrary byte values. Use the standard string methods when their semantics fit; benchmark representative data before choosing a specialized routine for speed.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/string_search/memchr.rs:example}}
```

## `fuzzy-matcher` {#fuzzy-matcher}

[![fuzzy-matcher][c~fuzzy-matcher~docs~badge]][c~fuzzy-matcher~docs] [![fuzzy-matcher~crates.io][c~fuzzy-matcher~crates.io~badge]][c~fuzzy-matcher~crates.io] [![fuzzy-matcher~repo][c~fuzzy-matcher~repo~badge]][c~fuzzy-matcher~repo] [![fuzzy-matcher~lib.rs][c~fuzzy-matcher~lib.rs~badge]][c~fuzzy-matcher~lib.rs]{{hi:fuzzy-matcher}}{{hi:Match}}{{hi:Search}}{{hi:Fuzzy}}{{hi:Text}}

[`fuzzy-matcher`][c~fuzzy-matcher~docs]↗{{hi:fuzzy-matcher}} scores candidates that contain a query's characters in order, even when those characters are separated. Use it for ranking choices such as filenames or commands. `fuzzy_indices` can also return matched character positions for highlighting; map them as character indices rather than using them as byte offsets.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/string_search/fuzzy_matcher.rs:example}}
```

## `strsim` {#strsim}

[![strsim][c~strsim~docs~badge]][c~strsim~docs] [![strsim~crates.io][c~strsim~crates.io~badge]][c~strsim~crates.io] [![strsim~repo][c~strsim~repo~badge]][c~strsim~repo] [![strsim~lib.rs][c~strsim~lib.rs~badge]][c~strsim~lib.rs]{{hi:strsim}}{{hi:String}}{{hi:Similarity}}{{hi:Jaro}}{{hi:Hamming}}{{hi:Levenshtein}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

[`strsim`][c~strsim~docs]↗{{hi:strsim}} implements string distance and similarity metrics, including Hamming, Levenshtein, optimal string alignment, Damerau-Levenshtein, Jaro, Jaro-Winkler, and Sørensen-Dice. These formulas have different input requirements and score meanings. For example, Hamming distance requires equal-length inputs; read each metric's contract before interpreting or comparing its result.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/string_search/strsim.rs:example}}
```

## Related topics {#related-topics .skip}

- [[rust_search_engines | Rust Search Engines]]
- [[search | Search]]
- [[strings | Strings]]
- [[unicode | Unicode]]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1191)
</div>
