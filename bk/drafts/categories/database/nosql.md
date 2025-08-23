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

This is the officially supported MongoDB Rust driver, a client side library that can be used to interact with MongoDB deployments in Rust applications. It uses the bson crate for BSON support. The driver contains a fully [async][p~async] API that requires [tokio][p~tokio]. The driver also has a sync API that may be enabled via feature flags.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/nosql/mongodb.rs:example}}
```

## Connect to Redis {#redis}

[![redis][c~redis~docs~badge]][c~redis~docs]{{hi:redis}}
[![redis~crates.io][c~redis~crates.io~badge]][c~redis~crates.io]
[![redis~repo][c~redis~repo~badge]][c~redis~repo]
[![redis~lib.rs][c~redis~lib.rs~badge]][c~redis~lib.rs]

Redis-rs is a high level [Redis][redis~website]↗{{hi:redis}} library for Rust. It provides convenient access to all Redis functionality through a very flexible but low-level API. It uses a customizable type conversion trait so that any operation can return results in just the type you are expecting. This makes for a very pleasant development experience.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/nosql/redis.rs:example}}
```

## Connect to Cassandra Using `cdrs-tokio` {#cdrs-tokio}

[![cdrs-tokio][c~cdrs-tokio~docs~badge]][c~cdrs-tokio~docs] [![cdrs-tokio~crates.io][c~cdrs-tokio~crates.io~badge]][c~cdrs-tokio~crates.io] [![cdrs-tokio~repo][c~cdrs-tokio~repo~badge]][c~cdrs-tokio~repo] [![cdrs-tokio~lib.rs][c~cdrs-tokio~lib.rs~badge]][c~cdrs-tokio~lib.rs]{{hi:cdrs-tokio}}{{hi:Cassandra}}{{hi:Driver}}{{hi:Client}}{{hi:Async}}{{hi:CassandraDB}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~database][cat~database~badge]][cat~database]{{hi:Database interfaces}}

[`cdrs-tokio`][c~cdrs-tokio~docs]↗{{hi:cdrs-tokio}} is an async Cassandra DB (or Scylla DB) driver. It uses the [`tokio`][c~tokio~docs]↗{{hi:tokio}} async runtime.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/cassandra/cdrs_tokio.rs:example}}
```

## Connect to Cassandra Using `cassandra-protocol` {#cassandra-protocol}

[![cassandra-protocol][c~cassandra-protocol~docs~badge]][c~cassandra-protocol~docs] [![cassandra-protocol~crates.io][c~cassandra-protocol~crates.io~badge]][c~cassandra-protocol~crates.io] [![cassandra-protocol~repo][c~cassandra-protocol~repo~badge]][c~cassandra-protocol~repo] [![cassandra-protocol~lib.rs][c~cassandra-protocol~lib.rs~badge]][c~cassandra-protocol~lib.rs]{{hi:cassandra-protocol}}{{hi:Cassandra}}{{hi:Client}}{{hi:CassandraDB}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~database][cat~database~badge]][cat~database]{{hi:Database interfaces}}

[`cassandra-protocol`][c~cassandra-protocol~docs]↗{{hi:Cassandra}} provides a Cassandra protocol implementation in Rust. Prefer the [`cdrs-tokio`][c~cdrs-tokio~docs]↗{{hi:cdrs-tokio}} crate unless you need a low-level implementation.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/cassandra/cassandra_protocol.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/1068)
</div>
