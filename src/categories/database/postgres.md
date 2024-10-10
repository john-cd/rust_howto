# Working with Postgres

{{#include postgres.incl.md}}

## Create tables in a Postgres database

[![postgres][postgres-badge]][c-postgres]  [![cat-database][cat-database-badge]][cat-database]

Use the [`{{i:postgres}}`][c-postgres]⮳ crate to create tables in a Postgres database.

[`{{i:Client::connect}}`][c-postgres::Client::connect]⮳ helps in connecting to an existing {{i:database}}. The recipe uses a URL string format with `Client::connect`. It assumes an existing database named `library`, the username is `postgres` and the password is `postgres`.

```rust,editable,no_run
{{#include ../../../deps/tests/create_tables.rs}}
```

## Insert and Query data

[![postgres][postgres-badge]][c-postgres]  [![cat-database][cat-database-badge]][cat-database]

The recipe inserts data into the `author` table using [`{{i:execute}}`][postgres::Client::execute]⮳ method of [`{{i:Client}}`][c-postgres::Client]⮳. Then, displays the data from the `author` table using [`{{i:query}}`][postgres::Client::query]⮳ method of [`{{i:Client}}`][c-postgres::Client]⮳.

```rust,editable,no_run
{{#include ../../../deps/tests/insert_query_data.rs}}
```

## Aggregate data

[![postgres][postgres-badge]][c-postgres]  [![cat-database][cat-database-badge]][cat-database]  [![csv-sample][csv-sample-badge]][csv-sample]

This recipe lists the nationalities of the first 7999 artists in the database of the [`Museum of Modern Art`][csv-sample]⮳ in descending order.

```rust,editable,no_run
{{#include ../../../deps/tests/aggregate_data.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
