<!-- TODO Reviewed ??-??-2026 -->

# NoSQL and Friends

{{#include nosql.incl.md}}

## Connect to MongoDB {#mongodb}

[![mongodb][c~mongodb~docs~badge]][c~mongodb~docs]{{hi:mongodb}}
[![mongodb~crates.io][c~mongodb~crates.io~badge]][c~mongodb~crates.io]
[![mongodb~repo][c~mongodb~repo~badge]][c~mongodb~repo]
[![mongodb~lib.rs][c~mongodb~lib.rs~badge]][c~mongodb~lib.rs]
[![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}
[![cat~database][cat~database~badge]][cat~database]{{hi:Database interfaces}}
[![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}}

The official MongoDB Rust driver is an asynchronous client for MongoDB deployments. It uses the `bson` crate for BSON support and Tokio for its async API; a synchronous API is available through a feature flag. See the [MongoDB Rust driver docs][c~mongodb~docs]↗ for server compatibility and driver features.

For an isolated local instance, install Docker and run MongoDB Community's
[official image][mongodb~docker~docs]↗:

```bash
docker run --detach --rm --name rust-howto-mongodb \
  -p 127.0.0.1:27017:27017 \
  mongodb/mongodb-community-server:8.0
```

Watch startup with `docker logs --follow rust-howto-mongodb` and press Ctrl+C
after the server is ready. Set `MONGO_URI` to
`mongodb://127.0.0.1:27017`. This local container has no authentication and is
bound to loopback; use a secured deployment and its authenticated connection
URI outside local development. The service-backed example test is ignored by
default. Run it explicitly with the service available:

```bash
export MONGO_URI="mongodb://127.0.0.1:27017"
cargo test -p database --example nosql --features mongodb -- \
  --ignored require_external_svc
```

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/nosql/mongodb.rs:example}}
```

## Connect to Redis {#redis}

[![redis][c~redis~docs~badge]][c~redis~docs]{{hi:redis}}
[![redis~crates.io][c~redis~crates.io~badge]][c~redis~crates.io]
[![redis~repo][c~redis~repo~badge]][c~redis~repo]
[![redis~lib.rs][c~redis~lib.rs~badge]][c~redis~lib.rs]

The [`redis`][c~redis~docs]↗ crate is the Rust client for Redis and other
RESP-compatible servers. It provides typed command traits as well as a
lower-level command API. This example uses the synchronous `Commands` trait.
See the crate documentation for current Redis versions and optional TLS
features.

Start a local Redis Open Source server with the [official Docker image][redis~docker~docs]↗:

```bash
docker run --detach --rm --name rust-howto-redis \
  -p 127.0.0.1:6379:6379 redis:8.2
```

The example defaults to `127.0.0.1:6379`. Set `REDIS_HOSTNAME` and, when the
server requires it, `REDIS_PASSWORD`; set `IS_TLS=true` only when the endpoint
uses TLS. Its ignored service test uses the default endpoint unless overridden:

```bash
cargo test -p database --example nosql --features redis -- \
  --ignored require_external_svc
```

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/nosql/redis.rs:example}}
```

## Connect to Cassandra Using `cdrs-tokio` {#cdrs-tokio}

[![cdrs-tokio][c~cdrs-tokio~docs~badge]][c~cdrs-tokio~docs] [![cdrs-tokio~crates.io][c~cdrs-tokio~crates.io~badge]][c~cdrs-tokio~crates.io] [![cdrs-tokio~repo][c~cdrs-tokio~repo~badge]][c~cdrs-tokio~repo] [![cdrs-tokio~lib.rs][c~cdrs-tokio~lib.rs~badge]][c~cdrs-tokio~lib.rs]{{hi:cdrs-tokio}}{{hi:Cassandra}}{{hi:Driver}}{{hi:Client}}{{hi:Async}}{{hi:CassandraDB}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~database][cat~database~badge]][cat~database]{{hi:Database interfaces}}

[`cdrs-tokio`][c~cdrs-tokio~docs]↗{{hi:cdrs-tokio}} is an async driver for Apache Cassandra and uses the [`tokio`][c~tokio~docs]↗{{hi:tokio}} runtime. ScyllaDB maintains its own Rust driver, which also supports Apache Cassandra; see the [Scylla Rust driver docs][scylla~docs]↗.

Start a local Cassandra service with the [official Docker image][cassandra~docker~docs]↗:

```bash
docker run --detach --rm --name rust-howto-cassandra \
  -p 127.0.0.1:9042:9042 cassandra:5.0
docker logs --follow rust-howto-cassandra
```

Press Ctrl+C after the logs report that Cassandra is ready, then run the
ignored CRUD test:

```bash
cargo test -p database --example cassandra --features cassandra -- \
  --ignored cdrs_tokio::tests::require_external_svc
```

Stop the container with `docker stop rust-howto-cassandra` when finished. The
example creates a keyspace and table, then deletes the row it inserts.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/cassandra/cdrs_tokio.rs:example}}
```

## Encode a Cassandra Protocol Frame {#cassandra-protocol}

[![cassandra-protocol][c~cassandra-protocol~docs~badge]][c~cassandra-protocol~docs] [![cassandra-protocol~crates.io][c~cassandra-protocol~crates.io~badge]][c~cassandra-protocol~crates.io] [![cassandra-protocol~repo][c~cassandra-protocol~repo~badge]][c~cassandra-protocol~repo] [![cassandra-protocol~lib.rs][c~cassandra-protocol~lib.rs~badge]][c~cassandra-protocol~lib.rs]{{hi:cassandra-protocol}}{{hi:Cassandra}}{{hi:Client}}{{hi:CassandraDB}} [![cat~database][cat~database~badge]][cat~database]{{hi:Database interfaces}}

[`cassandra-protocol`][c~cassandra-protocol~docs]↗{{hi:Cassandra}} provides a low-level Cassandra protocol implementation in Rust. This example encodes and decodes a STARTUP frame entirely in memory, so it does not need a running database. Prefer the higher-level [`cdrs-tokio`][c~cdrs-tokio~docs]↗{{hi:cdrs-tokio}} driver when connecting to Cassandra.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/cassandra/cassandra_protocol.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Database overview][p~database]
- [Connection pools][ex~database~connection-pool]
- [Caching][p~caching]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
