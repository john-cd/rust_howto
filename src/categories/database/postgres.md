# Working with Postgres

{{#include postgres.incl.md}}

## Create tables in a Postgres database {#create-tables-postgres}

[![postgres][c-postgres-badge]][c-postgres]{{hi:postgres}} [![cat-database][cat-database-badge]][cat-database]{{hi:Databases}}

Use the [`postgres`][c-postgres]{{hi:postgres}}⮳ crate to create tables in a Postgres database.

[`postgres::Client::connect`][c-postgres::Client::connect]{{hi:postgres::Client::connect}}⮳ helps in connecting to an existing database{{hi:Database}}. The recipe uses a URL string format with `Client::connect`. It assumes an existing database named `library`, the username is `postgres` and the password is `postgres`.

```rust,editable
{{#include ../../../deps/tests/cats/database/postgres/create_tables.rs:example}}
```

## Insert and Query data {#insert-query-data-postgres}

[![postgres][c-postgres-badge]][c-postgres]{{hi:postgres}} [![cat-database][cat-database-badge]][cat-database]{{hi:Databases}}

The recipe inserts data into the `author` table using [`postgres::Client::execute`][c-postgres::Client::execute]{{hi:postgres::Client::execute}}⮳ method of [`postgres::Client`][c-postgres::Client]{{hi:postgres::Client}}⮳. Then, displays the data from the `author` table using [`postgres::Client::query`][c-postgres::Client::query]{{hi:postgres::Client::query}}⮳ method of [`postgres::Client`][c-postgres::Client]{{hi:postgres::Client}}⮳.

```rust,editable
{{#include ../../../deps/tests/cats/database/postgres/insert_query_data.rs:example}}
```

## Aggregate data {#aggregate-data-postgres}

[![postgres][c-postgres-badge]][c-postgres]{{hi:postgres}} [![cat-database][cat-database-badge]][cat-database]{{hi:Database}} [![csv-sample][csv-sample-badge]][csv-sample]

This recipe lists the nationalities of the first 7999 artists in the database of the [`Museum of Modern Art`][csv-sample]⮳ in descending order.

```rust,editable
{{#include ../../../deps/tests/cats/database/postgres/aggregate_data.rs:example}}
```

## `tokio-postgres` {#tokio-postgres}

[![tokio-postgres][c-tokio_postgres-badge]][c-tokio_postgres]{{hi:tokio-postgres}}
[![tokio-postgres-crates.io][c-tokio_postgres-crates.io-badge]][c-tokio_postgres-crates.io]
[![tokio-postgres-github][c-tokio_postgres-github-badge]][c-tokio_postgres-github]
[![tokio-postgres-lib.rs][c-tokio_postgres-lib.rs-badge]][c-tokio_postgres-lib.rs]

Postgres-specific library. Performs better than SQLx.

## Cornucopia for postgres {#cornucopia}

Generate type-checked Rust from your PostgreSQL: [cornucopia-rs][c-cornucopia-github]⮳

Cornucopia is a tool powered by `rust-postgres` designed to generate type-checked Rust interfaces from your PostgreSQL queries. It works by preparing your queries against an actual database and then running an extensive validation suite on them. Once the queries are prepared and validated, Rust code is generated into a module, which can be imported and used in your project.

The basic premise is thus to:

- Write PostgreSQL queries.
- Use Cornucopia to generate Rust code.
- Use the generated code in your project.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P2 cornucopia
</div>
