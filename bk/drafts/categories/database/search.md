<!-- TODO Reviewed ??-??-2026 -->

# Search

{{#include search.incl.md}}

Search indexes are optimized for finding and ranking documents, rather than
for the transactional reads and writes covered by the [database index][p~database].
Choose a search system based on the shape of the query and where you want its
index to run:

| Search system                      | Rust client                                             | Typical use                                                                                                                             |
| ---------------------------------- | ------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| [Meilisearch][c~meilisearch~repo]↗ | [`meilisearch-sdk`][c~meilisearch-sdk~docs]↗            | Hosted full-text search with typo tolerance and search-as-you-type.                                                                     |
| Elasticsearch                      | [`elasticsearch`][c~elasticsearch~docs]↗                | General-purpose search through an asynchronous REST API. The example below uses this client.                                            |
| OpenSearch                         | [`opensearch`][c~opensearch~docs]↗                      | The OpenSearch service and REST API. Use its client rather than assuming Elasticsearch clients and server versions are interchangeable. |
| [Qdrant][c~qdrant~repo]↗           | [`qdrant-client`][c~qdrant-client~docs]↗                | Vector similarity search over embeddings; the Rust client connects to Qdrant's gRPC endpoint.                                           |
| Tantivy                            | [Search engines written in Rust][p~rust-search-engines] | Embed a full-text index in the Rust process instead of running a separate service.                                                      |

These client crates target different services and are not interchangeable
query builders. The recipe table above lists only examples included on this
page; see [search engines written in Rust][p~rust-search-engines] for the
embedded Tantivy example and other engine implementations.

## Connect to Elasticsearch {#elasticsearch}

[![elasticsearch][c~elasticsearch~docs~badge]][c~elasticsearch~docs]{{hi:elasticsearch}}
[![elasticsearch~crates.io][c~elasticsearch~crates.io~badge]][c~elasticsearch~crates.io]
[![elasticsearch~repo][c~elasticsearch~repo~badge]][c~elasticsearch~repo]
[![elasticsearch~lib.rs][c~elasticsearch~lib.rs~badge]][c~elasticsearch~lib.rs]
[![cat~api-bindings][cat~api-bindings~badge]][cat~api-bindings]{{hi:API bindings}}
[![cat~database][cat~database~badge]][cat~database]{{hi:Database interfaces}}

The example requires a running Elasticsearch service. For a local-only
development instance, install Docker and start Elastic's single-node image
using its [local Docker guide][elastic~local-docker~docs]↗:

```bash
docker run --rm --name rust-howto-elasticsearch \
  -p 127.0.0.1:9200:9200 -m 1GB \
  -e "discovery.type=single-node" \
  -e "xpack.security.enabled=false" \
  docker.elastic.co/elasticsearch/elasticsearch:9.5.5
```

Allocate at least 4 GB to Docker Desktop; the example container is limited to
1 GB.

This disables authentication and TLS, so bind the port to loopback as shown;
never use this configuration on a public or production host. The example
defaults to `http://localhost:9200`; set `ELASTIC_URL` to use another endpoint.
Remote endpoints must use HTTPS. For a secured endpoint, set both
`ELASTIC_USERNAME` and `ELASTIC_PASSWORD`; the client retains certificate and
hostname validation. The service-backed test is ignored during normal test
runs. Start Elasticsearch before running that test explicitly.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/search/elasticsearch.rs:example}}
```

## Add a Full-text Search Engine to a Static Website {#tinysearch}

[![tinysearch][c~tinysearch~docs~badge]][c~tinysearch~docs]{{hi:tinysearch}}
[![tinysearch~crates.io][c~tinysearch~crates.io~badge]][c~tinysearch~crates.io]
[![tinysearch~repo][c~tinysearch~repo~badge]][c~tinysearch~repo]
[![tinysearch~lib.rs][c~tinysearch~lib.rs~badge]][c~tinysearch~lib.rs]

[`tinysearch`][c~tinysearch~website]↗ is a lightweight, fast, full-text search engine. It is designed for static websites.

[`tinysearch`][c~tinysearch~docs]↗{{hi:tinysearch}} is compiled to WebAssembly to run in a browser. It can be used together with static site generators such as `Jekyll`, `Hugo`, [`Zola`][zola~website]↗{{hi:Zola}}, [`Cobalt`][cobalt~website]↗{{hi:Cobalt}}, or `Pelican`.

## Related Topics {#related-topics .skip}

- [Search engines written in Rust][p~rust-search-engines] — embedded libraries
  and server implementations such as Tantivy and Meilisearch.
- [Database index][p~database] — the database chapters and search-related
  integrations.
- [PostgreSQL][p~postgres] — use its built-in
  [full-text search][postgres~full-text~docs]↗ when search belongs with
  relational data.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
