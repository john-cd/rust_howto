# Working with Postgres

{{#include postgres.incl.md}}

## Create tables in a Postgres database

[![postgres][c-postgres-badge]][c-postgres]{{hi:postgres}}  [![cat-database][cat-database-badge]][cat-database]{{hi:Databases}}

Use the [`postgres`][c-postgres]{{hi:postgres}}⮳ crate to create tables in a Postgres database.

[`postgres::Client::connect`][c-postgres::Client::connect]{{hi:postgres::Client::connect}}⮳ helps in connecting to an existing database{{hi:Database}}. The recipe uses a URL string format with `Client::connect`. It assumes an existing database named `library`, the username is `postgres` and the password is `postgres`.

```rust
{{#include ../../../deps/tests/cats/databases/create_tables.rs}}
```

## Insert and Query data

[![postgres][c-postgres-badge]][c-postgres]{{hi:postgres}}  [![cat-database][cat-database-badge]][cat-database]{{hi:Databases}}

The recipe inserts data into the `author` table using [`postgres::Client::execute`][c-postgres::Client::execute]{{hi:postgres::Client::execute}}⮳ method of [`postgres::Client`][c-postgres::Client]{{hi:postgres::Client}}⮳. Then, displays the data from the `author` table using [`postgres::Client::query`][c-postgres::Client::query]{{hi:postgres::Client::query}}⮳ method of [`postgres::Client`][c-postgres::Client]{{hi:postgres::Client}}⮳.

```rust
{{#include ../../../deps/tests/cats/databases/insert_query_data.rs}}
```

## Aggregate data

[![postgres][c-postgres-badge]][c-postgres]{{hi:postgres}}  [![cat-database][cat-database-badge]][cat-database]{{hi:Database}}  [![csv-sample][csv-sample-badge]][csv-sample]

This recipe lists the nationalities of the first 7999 artists in the database of the [`Museum of Modern Art`][csv-sample]⮳ in descending order.

```rust
{{#include ../../../deps/tests/cats/databases/aggregate_data.rs}}
```

## Cornucopia for postgres

Generate type-checked Rust from your PostgreSQL: [cornucopia-rs][c-cornucopia-github]⮳

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO: cornucopia

TODO: cover tokio-postgres
[![tokio-postgres][c-tokio_postgres-badge]][c-tokio_postgres]{{hi:tokio-postgres}}
[![tokio-postgres-crates.io][c-tokio_postgres-crates.io-badge]][c-tokio_postgres-crates.io]
[![tokio-postgres-github][c-tokio_postgres-github-badge]][c-tokio_postgres-github]
[![tokio-postgres-lib.rs][c-tokio_postgres-lib.rs-badge]][c-tokio_postgres-lib.rs]

Postgres-specific library. Performs better than SQLx

</div>
