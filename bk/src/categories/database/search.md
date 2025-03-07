# Search

{{#include search.incl.md}}

## Add a full-text search engine to a static website {#tinysearch}

[![tinysearch][c-tinysearch-badge]][c-tinysearch]{{hi:tinysearch}}
[![tinysearch-crates.io][c-tinysearch-crates.io-badge]][c-tinysearch-crates.io]
[![tinysearch-github][c-tinysearch-github-badge]][c-tinysearch-github]
[![tinysearch-lib.rs][c-tinysearch-lib.rs-badge]][c-tinysearch-lib.rs]

[`tinysearch`][c-tinysearch-website] is a lightweight, fast, full-text search engine. It is designed for static websites.

[`tinysearch`][c-tinysearch]⮳{{hi:tinysearch}} is compiled to WebAssembly to run in a browser. It can be used together with static site generators such as `Jekyll`, `Hugo`, [`Zola`][c-zola]⮳{{hi:Zola}}, `Cobalt`, or `Pelican`.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">

## Connect to Elasticsearch {#elasticsearch}

[![elasticsearch][c-elasticsearch-badge]][c-elasticsearch]{{hi:elasticsearch}}
[![elasticsearch-crates.io][c-elasticsearch-crates.io-badge]][c-elasticsearch-crates.io]
[![elasticsearch-github][c-elasticsearch-github-badge]][c-elasticsearch-github]
[![elasticsearch-lib.rs][c-elasticsearch-lib.rs-badge]][c-elasticsearch-lib.rs]
[![cat-api-bindings][cat-api-bindings-badge]][cat-api-bindings]{{hi:API bindings}}
[![cat-database][cat-database-badge]][cat-database]{{hi:Database interfaces}}

```rust,editable,noplayground
{{#include ../../../crates/cats/database/tests/search/elasticsearch.rs:example}}
```

- [`tinysearch`][c-tinysearch]⮳{{hi:tinysearch}} is not that popular.

[database/search.md: expand (P2)](https://github.com/john-cd/rust_howto/issues/288)
</div>
