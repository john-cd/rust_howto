# String Search and Fuzzy Matching

{{#include string_search.incl.md}}

## `aho-corasick` {#aho-corasick}

[![aho-corasick][c~aho-corasick~docs~badge]][c~aho-corasick~docs] [![aho-corasick~crates.io][c~aho-corasick~crates.io~badge]][c~aho-corasick~crates.io] [![aho-corasick~repo][c~aho-corasick~repo~badge]][c~aho-corasick~repo] [![aho-corasick~lib.rs][c~aho-corasick~lib.rs~badge]][c~aho-corasick~lib.rs]{{hi:aho-corasick}}{{hi:Multi}}{{hi:Text}}{{hi:String}}{{hi:Pattern}}{{hi:Search}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

[`aho-corasick`][c~aho-corasick~docs]↗{{hi:aho-corasick}} implements fast multiple substring searching.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/string_search/aho_corasick.rs:example}}
```

## `memchr` {#memchr}

[![memchr][c~memchr~docs~badge]][c~memchr~docs] [![memchr~crates.io][c~memchr~crates.io~badge]][c~memchr~crates.io] [![memchr~repo][c~memchr~repo~badge]][c~memchr~repo] [![memchr~lib.rs][c~memchr~lib.rs~badge]][c~memchr~lib.rs]{{hi:memchr}}{{hi:Search}}{{hi:Find}}{{hi:memchr}}{{hi:Substring}}{{hi:Memmem}}

[`memchr`][c~memchr~docs]↗{{hi:memchr}} provides extremely fast routines for 1, 2 or 3 byte search and single substring search (that use SIMD on x86_64, aarch64 and wasm32).

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/string_search/memchr.rs:example}}
```

## `fuzzy-matcher` {#fuzzy-matcher}

[![fuzzy-matcher][c~fuzzy-matcher~docs~badge]][c~fuzzy-matcher~docs] [![fuzzy-matcher~crates.io][c~fuzzy-matcher~crates.io~badge]][c~fuzzy-matcher~crates.io] [![fuzzy-matcher~repo][c~fuzzy-matcher~repo~badge]][c~fuzzy-matcher~repo] [![fuzzy-matcher~lib.rs][c~fuzzy-matcher~lib.rs~badge]][c~fuzzy-matcher~lib.rs]{{hi:fuzzy-matcher}}{{hi:Match}}{{hi:Search}}{{hi:Fuzzy}}{{hi:Text}}

[`fuzzy-matcher`][c~fuzzy-matcher~docs]↗{{hi:fuzzy-matcher}} is a fuzzy matching library.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/string_search/fuzzy_matcher.rs:example}}
```

## `strsim` {#strsim}

[![strsim][c~strsim~docs~badge]][c~strsim~docs] [![strsim~crates.io][c~strsim~crates.io~badge]][c~strsim~crates.io] [![strsim~repo][c~strsim~repo~badge]][c~strsim~repo] [![strsim~lib.rs][c~strsim~lib.rs~badge]][c~strsim~lib.rs]{{hi:strsim}}{{hi:String}}{{hi:Similarity}}{{hi:Jaro}}{{hi:Hamming}}{{hi:Levenshtein}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

[`strsim`][c~strsim~docs]↗{{hi:strsim}} implement string similarity metrics. That includes Hamming, Levenshtein, OSA, Damerau-Levenshtein, Jaro, Jaro-Winkler, and Sørensen-Dice.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/string_search/strsim.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[rust_search_engines | Rust Search Engines]].
- [[search | Search]].
- [[strings | Strings]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1191)
</div>
