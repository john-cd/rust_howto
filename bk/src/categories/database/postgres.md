# Working with Postgres

{{#include postgres.incl.md}}

## Create tables in a Postgres database {#create-tables-postgres}

[![postgres][c-postgres-badge]][c-postgres]{{hi:postgres}} [![cat-database][cat-database-badge]][cat-database]{{hi:Databases}}

Use the [`postgres`][c-postgres]{{hi:postgres}}⮳ crate to create tables in a Postgres database.

[`postgres::Client::connect`][c-postgres::Client::connect]{{hi:postgres::Client::connect}}⮳ helps in connecting to an existing database{{hi:Database}}. The recipe uses a URL string format with [`Client::connect`][c-postgres::Client::connect]⮳{{hi:Client::connect}}. It assumes an existing database named `library`, the username is `postgres` and the password is `postgres`.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/tests/postgres/create_tables.rs:example}}
```

## Insert and query data {#insert-query-data-postgres}

[![postgres][c-postgres-badge]][c-postgres]{{hi:postgres}} [![cat-database][cat-database-badge]][cat-database]{{hi:Databases}}

The recipe inserts data into the `author` table using [`postgres::Client::execute`][c-postgres::Client::execute]{{hi:postgres::Client::execute}}⮳ method of [`postgres::Client`][c-postgres::Client]{{hi:postgres::Client}}⮳. Then, displays the data from the `author`{{hi:author}} table using [`postgres::Client::query`][c-postgres::Client::query]{{hi:postgres::Client::query}}⮳ method of [`postgres::Client`][c-postgres::Client]{{hi:postgres::Client}}⮳.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/tests/postgres/insert_query_data.rs:example}}
```

## Aggregate data {#aggregate-data-postgres}

[![postgres][c-postgres-badge]][c-postgres]{{hi:postgres}} [![cat-database][cat-database-badge]][cat-database]{{hi:Database}} [![csv-sample][csv-sample-badge]][csv-sample]

This recipe lists the nationalities of the first 7999 artists in the [database][p-database] of the [`Museum of Modern Art`][csv-sample]⮳ in descending order.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/tests/postgres/aggregate_data.rs:example}}
```

## Connect to and query Postgres asynchronously with `tokio-postgres` {#tokio-postgres}

[![tokio-postgres][c-tokio_postgres-badge]][c-tokio_postgres]{{hi:tokio-postgres}}
[![tokio-postgres-crates.io][c-tokio_postgres-crates.io-badge]][c-tokio_postgres-crates.io]
[![tokio-postgres-github][c-tokio_postgres-github-badge]][c-tokio_postgres-github]
[![tokio-postgres-lib.rs][c-tokio_postgres-lib.rs-badge]][c-tokio_postgres-lib.rs]

[`tokio-postgres`][c-tokio_postgres]⮳{{hi:tokio-postgres}} provides an asynchronous PostgreSQL client. It is built on top of the [`tokio`][c-tokio]⮳{{hi:tokio}} runtime and thus supports non-blocking interactions with PostgreSQL databases. This crate offers connection pooling, prepared statements, transactions, and support for various PostgreSQL data types. It performs better than [`SQLx`][c-sqlx]⮳{{hi:SQLx}}.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/tests/postgres/tokio_postgres.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[postgres: `cornucopia` (P2)](https://github.com/john-cd/rust_howto/issues/286)

## `cornucopia` for postgres {#cornucopia}

[![cornucopia-website][c-cornucopia-website-badge]][c-cornucopia-website] [![cornucopia][c-cornucopia-badge]][c-cornucopia] [![cornucopia-crates.io][c-cornucopia-crates.io-badge]][c-cornucopia-crates.io] [![cornucopia-github][c-cornucopia-github-badge]][c-cornucopia-github] [![cornucopia-lib.rs][c-cornucopia-lib.rs-badge]][c-cornucopia-lib.rs]{{hi:cornucopia}}{{hi:Query}}{{hi:Generator}}{{hi:Tokio-postgres}}{{hi:Postgresql}}{{hi:Sql}} [![cat-database][cat-database-badge]][cat-database]{{hi:Database interfaces}}

[`cornucopia`][c-cornucopia]⮳{{hi:cornucopia}} generates type-checked Rust from your PostgreSQL: [cornucopia-rs][c-cornucopia-github]⮳.

[`cornucopia`][c-cornucopia]⮳{{hi:cornucopia}} is a tool powered by `rust-postgres` designed to generate type-checked Rust interfaces from your PostgreSQL queries. It works by preparing your queries against an actual [database][p-database] and then running an extensive validation suite on them. Once the queries are prepared and validated, Rust code is generated into a module, which can be imported and used in your project.

The basic premise is thus to:

- Write PostgreSQL queries.
- Use Cornucopia to generate Rust code.
- Use the generated code in your project.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/tests/postgres/cornucopia.rs:example}}
```

</div>
