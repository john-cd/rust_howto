<!-- TODO Reviewed ??-??-2026 -->

# Database Implementations

[![cat~database-implementations][cat~database-implementations~badge]][cat~database-implementations]{{hi:Database implementations}}

This index covers database and data-system implementations written in Rust. Rust libraries that connect to other database servers are listed under [Database interfaces][p~database].

| Kind                               | Examples                                                                                                     | Chapter                                                                      |
| ---------------------------------- | ------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------- |
| Embedded key-value stores          | [`sled`][c~sled~repo]↗, [`redb`][c~redb~repo]↗, [`fjall`][c~fjall~repo]↗                                     | [Databases written in Rust][ex~database-implementations~databases]           |
| SQL and query engines              | [GlueSQL][c~gluesql~repo]↗, [Turso][c~turso~repo]↗ (formerly Limbo), [Apache DataFusion][c~datafusion~repo]↗ | [Databases written in Rust][ex~database-implementations~databases]           |
| Distributed and cloud data systems | [TiKV][c~tikv~repo]↗, [SurrealDB][c~surrealdb~repo]↗, [Databend][c~databend~repo]↗                           | [Databases written in Rust][ex~database-implementations~databases]           |
| Search engines                     | [`tantivy`][c~tantivy~repo]↗, [Meilisearch][c~meilisearch~repo]↗, [Qdrant][c~qdrant~repo]↗                   | [Search engines written in Rust][ex~database-implementations~search-engines] |
| Message brokers                    | [RobustMQ][c~robustmq~repo]↗, [RocketMQ-Rust][c~rocketmq-rust~repo]↗                                         | [Message queues and AMQP][p~message-queues]                                  |

GlueSQL provides a Rust SQL engine with multiple storage options. Turso is an in-process, SQLite-compatible database; Apache DataFusion is an extensible query engine rather than a database server. TiKV is a distributed transactional key-value database, SurrealDB is a document-graph database, and Databend is a cloud data warehouse. The message brokers are adjacent data infrastructure; the linked AMQP chapter describes a messaging client and protocol.

Check each project's current releases, support policy, and migration guidance before adopting it. `sled` describes itself as beta and warns that pre-1.0 storage-format changes may require manual migrations.

## Databases Written in Rust

{{#include databases.incl.md}}

## Search Engines Written in Rust

{{#include rust_search_engines.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
