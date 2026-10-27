<!-- TODO Reviewed ??-??-2026 -->

# Search Engines Written in Rust

{{#include rust_search_engines.incl.md}}

## `meilisearch` {#meilisearch}

[![meilisearch][c~meilisearch~docs~badge]][c~meilisearch~docs]{{hi:meilisearch}}
[![meilisearch~crates.io][c~meilisearch~crates.io~badge]][c~meilisearch~crates.io]
[![meilisearch~repo][c~meilisearch~repo~badge]][c~meilisearch~repo]
[![meilisearch~lib.rs][c~meilisearch~lib.rs~badge]][c~meilisearch~lib.rs]

[meilisearch][c~meilisearch~repo]↗ is a fast [search][p~search] API that fits into your apps, websites, and workflow.

```rust,editable
{{#include ../../../crates/cats/database_implementations/examples/search_engines/meilisearch.rs:example}}
```

## `tantivy` {#tantivy}

[![tantivy][c~tantivy~docs~badge]][c~tantivy~docs]{{hi:tantivy}}
[![tantivy~crates.io][c~tantivy~crates.io~badge]][c~tantivy~crates.io]
[![tantivy~repo][c~tantivy~repo~badge]][c~tantivy~repo]
[![tantivy~lib.rs][c~tantivy~lib.rs~badge]][c~tantivy~lib.rs]
[![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}
[![cat~database-implementations][cat~database-implementations~badge]][cat~database-implementations]{{hi:Database implementations}}

[tantivy][c~tantivy~repo]↗ is a full-text [search][p~search] engine library inspired by Apache Lucene.

```rust,editable
{{#include ../../../crates/cats/database_implementations/examples/search_engines/tantivy.rs:example}}
```

## Choosing a Search Engine {#choosing}

All of the engines below are written in Rust. Use `tantivy` when you want to embed search in your own process. Use a server when several applications share the index. Client libraries for Elasticsearch and other search servers belong to the [Search][p~search] page.

| Engine                               | Kind                     | Best for                                                     | Trade-offs                                                 |
| ------------------------------------ | ------------------------ | ------------------------------------------------------------ | ---------------------------------------------------------- |
| [`tantivy`][c~tantivy~repo]↗         | Embedded library         | Full-text search inside an application, Lucene-style scoring | You manage storage, schema and merging                     |
| [`meilisearch`][c~meilisearch~repo]↗ | Server (REST)            | Typo-tolerant, search-as-you-type for websites and apps      | Fewer analytics features than Elasticsearch                |
| [`quickwit`][quickwit~repo]↗         | Distributed server       | Logs and traces on object storage; built on `tantivy`        | Optimized for append-only data                             |
| [`qdrant`][c~qdrant~repo]↗           | Vector database          | Semantic and similarity search over embeddings               | Not a keyword engine; combine with one for hybrid search   |
| [`lancedb`][lancedb~repo]↗           | Embedded vector database | Local vector search and RAG prototypes                       | Younger ecosystem                                          |
| [`sonic`][sonic~repo]↗               | Lightweight server       | Small ID-based index that runs in a few MB of RAM            | Returns identifiers, not documents; little recent activity |

See [Meilisearch vs Qdrant][blog~meilisearch-vs-qdrant-tradeoffs-strengths-and-weaknesses]↗ for the keyword versus vector trade-off, and [Algolia vs Elasticsearch vs Meilisearch vs Typesense][blog~typesense-vs-algolia-vs-elasticsearch-vs-meilisearch]↗ for a feature comparison with non-Rust engines. Note that the second comparison is published by a vendor.

### How a Search Engine Works {#design}

Most engines share the same design:

1. **Schema**: declare which fields are text, numbers or facets, and which are stored.
2. **Analysis**: split text into tokens, lowercase and optionally stem them.
3. **Inverted index**: map each token to the documents that contain it.
4. **Segments**: write immutable segments that are merged in the background, so reads never block writes.
5. **Ranking**: score matches, typically with BM25, and return the top results.

The `tantivy` example above shows each of these steps: the `Schema`, the `IndexWriter` that builds segments, the `QueryParser`, and the `TopDocs` collector that ranks hits.

## Related Topics {#related-topics .skip}

| Topic                                     | Rust crate(s)                                                                                           | Notes                                                                                             |
| ----------------------------------------- | ------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| Full-Text [Search][p~search]              | [`elastic`][c~elastic~docs]↗{{hi:elastic}} (for Elasticsearch)                                          |                                                                                                   |
| Fuzzy Searching                           | [`fuzzy-matcher`][c~fuzzy-matcher~docs]↗{{hi:fuzzy-matcher}}, [`strsim`][c~strsim~docs]↗{{hi:strsim}}   | [`strsim`][c~strsim~docs]↗{{hi:strsim}} provides string similarity metrics. See [[text-processing | Text Processing]]. |
| Regular Expressions                       | [`regex`][c~regex~docs]↗{{hi:regex}}                                                                    | See [[regex                                                                                       | Regex]]. |
| String Search (Multiple Patterns at Once) | [`aho-corasick`][c~aho-corasick~docs]↗{{hi:aho-corasick}}                                               | See [[text-processing                                                                             | Text Processing]]. |
| Vector Search                             | [`qdrant`][c~qdrant~docs]↗{{hi:qdrant}}, [`faiss-rs`][c~faiss~docs]↗{{hi:faiss-rs}} (bindings to FAISS) |                                                                                                   |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
