# Working with Postgres

{{#include postgres.incl.md}}

## Create tables in a Postgres database

[![postgres][c-postgres-badge]][c-postgres]  [![cat-database][cat-database-badge]][cat-database]

Use the [`postgres`][c-postgres]{{hi:postgres}}⮳ crate to create tables in a Postgres database.

[`postgres::Client::connect`][c-postgres::Client::connect]{{hi:postgres::Client::connect}}⮳ helps in connecting to an existing database{{hi:Database}}. The recipe uses a URL string format with `Client::connect`. It assumes an existing database named `library`, the username is `postgres` and the password is `postgres`.

```rust,editable,no_run
{{#include ../../../deps/tests/create_tables.rs}}
```

## Insert and Query data

[![postgres][c-postgres-badge]][c-postgres]  [![cat-database][cat-database-badge]][cat-database]

The recipe inserts data into the `author` table using [`postgres::Client::execute`][c-postgres::Client::execute]{{hi:postgres::Client::execute}}⮳ method of [`postgres::Client`][c-postgres::Client]{{hi:postgres::Client}}⮳. Then, displays the data from the `author` table using [`postgres::Client::query`][c-postgres::Client::query]{{hi:postgres::Client::query}}⮳ method of [`postgres::Client`][c-postgres::Client]{{hi:postgres::Client}}⮳.

```rust,editable,no_run
{{#include ../../../deps/tests/insert_query_data.rs}}
```

## Aggregate data

[![postgres][c-postgres-badge]][c-postgres]  [![cat-database][cat-database-badge]][cat-database]  [![csv-sample][csv-sample-badge]][csv-sample]

This recipe lists the nationalities of the first 7999 artists in the database of the [`Museum of Modern Art`][csv-sample]⮳ in descending order.

```rust,editable,no_run
{{#include ../../../deps/tests/aggregate_data.rs}}
```

## Cornucopia for postgres

Generate type-checked Rust from your PostgreSQL: [cornucopia-rs][c-cornucopia-github]⮳

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO: cornucopia
</div>
