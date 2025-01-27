# NoSQL and friends

{{#include nosql.incl.md}}

## Connect to MongoDB {#mongodb}

[![mongodb][c-mongodb-badge]][c-mongodb]{{hi:mongodb}}
[![mongodb-crates.io][c-mongodb-crates.io-badge]][c-mongodb-crates.io]
[![mongodb-github][c-mongodb-github-badge]][c-mongodb-github]
[![mongodb-lib.rs][c-mongodb-lib.rs-badge]][c-mongodb-lib.rs]
[![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}}
[![cat-database][cat-database-badge]][cat-database]{{hi:Database interfaces}}
[![cat-web-programming][cat-web-programming-badge]][cat-web-programming]{{hi:Web programming}}

This is the officially supported MongoDB Rust driver, a client side library that can be used to interact with MongoDB deployments in Rust applications. It uses the bson crate for BSON support. The driver contains a fully async API that requires tokio. The driver also has a sync API that may be enabled via feature flags.

```rust,editable,noplayground
{{#include ../../../crates/ex/cats/database/tests/nosql/mongodb.rs:example}}
```

## Connect to Redis {#redis}

[![redis][c-redis-badge]][c-redis]{{hi:redis}}
[![redis-crates.io][c-redis-crates.io-badge]][c-redis-crates.io]
[![redis-github][c-redis-github-badge]][c-redis-github]
[![redis-lib.rs][c-redis-lib.rs-badge]][c-redis-lib.rs]

Redis-rs is a high level redis library for Rust. It provides convenient access to all Redis functionality through a very flexible but low-level API. It uses a customizable type conversion trait so that any operation can return results in just the type you are expecting. This makes for a very pleasant development experience.

```rust,editable,noplayground
{{#include ../../../crates/ex/cats/database/tests/nosql/redis.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">

[P1 add the following](https://github.com/john-cd/rust_howto/issues/1068)

## `cassandra_protocol` {#cassandra_protocol}

[![cassandra-protocol][c-cassandra_protocol-badge]][c-cassandra_protocol] [![cassandra-protocol-crates.io][c-cassandra_protocol-crates.io-badge]][c-cassandra_protocol-crates.io] [![cassandra-protocol-github][c-cassandra_protocol-github-badge]][c-cassandra_protocol-github] [![cassandra-protocol-lib.rs][c-cassandra_protocol-lib.rs-badge]][c-cassandra_protocol-lib.rs]{{hi:cassandra-protocol}}{{hi:Cassandra}}{{hi:Client}}{{hi:Cassandradb}} [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}} [![cat-database][cat-database-badge]][cat-database]{{hi:Database interfaces}}

Cassandra protocol implementation in Rust. This crate provides a low-level implementation of the Cassandra protocol. It is used by the `cdrs` crate to communicate with Cassandra.

```rust,editable,noplayground
{{#include ../../../crates/ex/cats/database/tests/cassandra/cassandra_protocol.rs:example}}
```

## `cdrs_tokio` {#cdrs_tokio}

[![cdrs-tokio][c-cdrs_tokio-badge]][c-cdrs_tokio] [![cdrs-tokio-crates.io][c-cdrs_tokio-crates.io-badge]][c-cdrs_tokio-crates.io] [![cdrs-tokio-github][c-cdrs_tokio-github-badge]][c-cdrs_tokio-github] [![cdrs-tokio-lib.rs][c-cdrs_tokio-lib.rs-badge]][c-cdrs_tokio-lib.rs]{{hi:cdrs-tokio}}{{hi:Cassandra}}{{hi:Driver}}{{hi:Client}}{{hi:Async}}{{hi:Cassandradb}} [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}} [![cat-database][cat-database-badge]][cat-database]{{hi:Database interfaces}}

Async Cassandra DB driver written in Rust. This crate provides an asynchronous driver for Cassandra. It is built on top of the `cdrs` crate and uses the `tokio` runtime.

```rust,editable,noplayground
{{#include ../../../crates/ex/cats/database/tests/cassandra/cdrs_tokio.rs:example}}
```

</div>
