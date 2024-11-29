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

{{#example mongodb}}

## Connect to Redis {#redis}

[![redis][c-redis-badge]][c-redis]{{hi:redis}}
[![redis-crates.io][c-redis-crates.io-badge]][c-redis-crates.io]
[![redis-github][c-redis-github-badge]][c-redis-github]
[![redis-lib.rs][c-redis-lib.rs-badge]][c-redis-lib.rs]

Redis-rs is a high level redis library for Rust. It provides convenient access to all Redis functionality through a very flexible but low-level API. It uses a customizable type conversion trait so that any operation can return results in just the type you are expecting. This makes for a very pleasant development experience.

```rust,editable
{{#include ../../../deps/tests/cats/database/redis.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
