<!-- TODO Reviewed ??-??-2026 -->

# Text Analysis

{{#include text_analysis.incl.md}}

Recipes for finding things inside text beyond what `regex` offers: ranking by fuzzy match, extracting key phrases and finding word boundaries. Multi-pattern search with `aho-corasick` is in [[string_search | String Search and Fuzzy Matching]]. A spell checker can use Unicode word boundaries to find candidate tokens, then compare each token with a language-specific dictionary; handle names, code, and unknown words as product choices instead of treating every miss as a typo.

## Rank Candidates with Fuzzy Search (`nucleo`) {#nucleo}

[![nucleo][c~nucleo~docs~badge]][c~nucleo~docs] [![nucleo~crates.io][c~nucleo~crates.io~badge]][c~nucleo~crates.io]{{hi:nucleo}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Fuzzy search}}

[`nucleo`][c~nucleo~docs]↗{{hi:nucleo}} is the fuzzy matcher used by the Helix editor. A `Pattern` parsed from the user's query scores each candidate, and `match_list` returns the matches sorted by descending score. Create the `Matcher` once and reuse it, since it owns scratch memory. For a smaller dependency, see [`fuzzy-matcher`][c~fuzzy-matcher~docs]↗{{hi:fuzzy-matcher}} in [[string_search | String Search and Fuzzy Matching]].

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/text_analysis/fuzzy_search.rs:example}}
```

## Extract Keywords with RAKE (`rake`) {#rake}

[![rake][c~rake~docs~badge]][c~rake~docs] [![rake~crates.io][c~rake~crates.io~badge]][c~rake~crates.io]{{hi:rake}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Keyword extraction}}

[`rake`][c~rake~docs]↗{{hi:rake}} implements Rapid Automatic Keyword Extraction: it splits text into candidate phrases at stop words and punctuation, then scores phrases by how often their words occur and co-occur. It needs a stop-word list for the text's language. Phrases with equal scores come back in no particular order.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/text_analysis/keyword_extraction.rs:example}}
```

## Find Word Boundaries (`unicode-segmentation`) {#word-boundaries}

[![unicode-segmentation][c~unicode-segmentation~docs~badge]][c~unicode-segmentation~docs] [![unicode-segmentation~crates.io][c~unicode-segmentation~crates.io~badge]][c~unicode-segmentation~crates.io]{{hi:unicode-segmentation}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Unicode}}

Splitting on whitespace is wrong for most real text. [`unicode-segmentation`][c~unicode-segmentation~docs]↗{{hi:unicode-segmentation}} implements the Unicode UAX #29 word boundary rules; `unicode_word_indices` also returns byte offsets, which is what a spell checker or a highlighter needs to map a word back into the original string.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/text_analysis/word_boundaries.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[string_search | String Search and Fuzzy Matching]].
- [[unicode | Unicode]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
