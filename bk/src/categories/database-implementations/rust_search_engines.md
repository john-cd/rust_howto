# Search Engines Written in Rust

{{#include rust_search_engines.incl.md}}

## `meilisearch` {#meilisearch}

[![meilisearch][c-meilisearch-badge]][c-meilisearch]{{hi:meilisearch}}
[![meilisearch-crates.io][c-meilisearch-crates.io-badge]][c-meilisearch-crates.io]
[![meilisearch-github][c-meilisearch-github-badge]][c-meilisearch-github]
[![meilisearch-lib.rs][c-meilisearch-lib.rs-badge]][c-meilisearch-lib.rs]

[meilisearch][c-meilisearch-github]⮳ is a fast [search][p-search] API that fits into your apps, websites, and workflow.

```rust,editable
{{#include ../../../crates/cats/database_implementations/tests/search/meilisearch.rs:example}}
```

## `tantivy` {#tantivy}

[![tantivy][c-tantivy-badge]][c-tantivy]{{hi:tantivy}}
[![tantivy-crates.io][c-tantivy-crates.io-badge]][c-tantivy-crates.io]
[![tantivy-github][c-tantivy-github-badge]][c-tantivy-github]
[![tantivy-lib.rs][c-tantivy-lib.rs-badge]][c-tantivy-lib.rs]
[![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}}
[![cat-database-implementations][cat-database-implementations-badge]][cat-database-implementations]{{hi:Database implementations}}

[tantivy][c-tantivy-github]⮳ is a full-text [search][p-search] engine library inspired by Apache Lucene.

```rust,editable
{{#include ../../../crates/cats/database_implementations/tests/search/tantivy.rs:example}}
```

## Related Topics

| Topic | Rust crate(s) | Notes |
|---|---|---|
| Full-Text [Search][p-search] | `elastic` (for Elasticsearch) | |
| Fuzzy Searching | `fuzzy-matcher`, `strsim` | `strsim` provides string similarity metrics. See [[text-processing | Text Processing]]. |
| Regular Expressions | `regex` | See [[regex | Regex]]. |
| String Search (Multiple Patterns at Once) | `aho-corasick` | See [[text-processing | Text Processing]]. |
| Vector Search | `qdrant`, `faiss-rs` (bindings to FAISS) | |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[search: expand](https://github.com/john-cd/rust_howto/issues/291)
</div>
