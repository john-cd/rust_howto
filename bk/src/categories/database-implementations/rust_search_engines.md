<!-- TODO Reviewed ??-??-2026. -->

# Search Engines Written in Rust

{{#include rust_search_engines.incl.md}}

## Meilisearch {#meilisearch}

{{#badges meilisearch-sdk}}

[Meilisearch][c~meilisearch~repo]↗ is an HTTP search service with typo-tolerant search-as-you-type. The [`meilisearch-sdk`][c~meilisearch-sdk~docs]↗ crate used here is its Rust client. Start a local Meilisearch service before running this recipe; set `MEILISEARCH_URL` and, when required, `MEILI_MASTER_KEY` to configure the connection. The service-backed test is ignored by default.

Test this Cargo-backed example from `bk/crates` with `cargo test --locked -p database_implementations --example search_engines`.

```rust,editable,noplayground
{{#include ../../../crates/cats/database_implementations/examples/search_engines/meilisearch.rs:example}}
```

## `tantivy` {#tantivy}

{{#badges tantivy data-structures="Data structures" database-implementations="Database implementations"}}

[tantivy][c~tantivy~repo]↗ is a full-text [search][p~search] engine library inspired by Apache Lucene.

```rust,editable,noplayground
{{#include ../../../crates/cats/database_implementations/examples/search_engines/tantivy.rs:example}}
```

## Choosing a Search Engine {#choosing}

The examples show two common approaches: embed `tantivy` in the application, or call the Meilisearch HTTP service. Other search systems target distributed logs, vector similarity, or identifier lookup. Client libraries for Elasticsearch and other search servers belong to the [Search][p~search] page.

| Engine | Architecture | Best suited to | Practical note |
| --- | --- | --- | --- |
| [`tantivy`][c~tantivy~repo]↗ | Embedded library | Full-text search inside an application | You manage the index, schema, and storage. |
| [Meilisearch][c~meilisearch~repo]↗ | HTTP service | Typo-tolerant, search-as-you-type | Requires a running service; indexing completes asynchronously. |
| [`quickwit`][quickwit~repo]↗ | Distributed server | Logs and traces on object storage | Built on `tantivy`; suited to append-oriented workloads. |
| [`qdrant`][c~qdrant~repo]↗ | Vector database | Semantic and similarity search over embeddings | Combine with a keyword engine for hybrid search. |
| [`lancedb`][lancedb~repo]↗ | Embedded vector database | Local vector search and retrieval-augmented generation prototypes | Evaluate it with the embedding model and data you plan to use. |
| [`sonic`][sonic~repo]↗ | Lightweight search server | Identifier-oriented search | Search returns identifiers; retrieve full records separately. |

See [Meilisearch vs Qdrant][blog~meilisearch-vs-qdrant-tradeoffs-strengths-and-weaknesses]↗ for the keyword versus vector trade-off, and [Algolia vs Elasticsearch vs Meilisearch vs Typesense][blog~typesense-vs-algolia-vs-elasticsearch-vs-meilisearch]↗ for a feature comparison with non-Rust engines. Note that the second comparison is published by a vendor.

### How a Search Engine Works {#design}

Full-text engines commonly use a pipeline like this:

1. **Schema**: declare which fields are text, numbers or facets, and which are stored.
2. **Analysis**: split text into tokens, lowercase and optionally stem them.
3. **Inverted index**: map each token to the documents that contain it.
4. **Segments**: write immutable segments that are merged in the background, so reads never block writes.
5. **Ranking**: score matches, typically with BM25, and return the top results.

The `tantivy` example above shows each of these steps: the `Schema`, the `IndexWriter` that builds segments, the `QueryParser`, and the `TopDocs` collector that ranks hits.

## Related Topics {#related-topics .skip}

| Topic | Rust crate(s) | Notes |
| --- | --- | --- |
| Full-Text [Search][p~search] | [`elasticsearch`][c~elasticsearch~docs]↗{{hi:elasticsearch}} | |
| Fuzzy Searching | [`fuzzy-matcher`][c~fuzzy-matcher~docs]↗{{hi:fuzzy-matcher}}, [`strsim`][c~strsim~docs]↗{{hi:strsim}} | [`strsim`][c~strsim~docs]↗{{hi:strsim}} provides string similarity metrics. See [Text Processing](../text-processing/index.md). |
| Regular Expressions | [`regex`][c~regex~docs]↗{{hi:regex}} | See [Regular Expressions](../text-processing/regex.md). |
| String Search (Multiple Patterns at Once) | [`aho-corasick`][c~aho-corasick~docs]↗{{hi:aho-corasick}} | See [Text Processing](../text-processing/index.md). |
| Vector Search | [`qdrant`][c~qdrant~docs]↗{{hi:qdrant}}, [`faiss-rs`][c~faiss~docs]↗{{hi:faiss-rs}} (bindings to FAISS) | |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
