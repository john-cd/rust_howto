# Working with Postgres

## Create tables in a Postgres database

[![postgres-badge]][postgres] [![cat-database-badge]][cat-database]

Use the [`postgres`][postgres] crate to create tables in a Postgres database.

`[Client::connect]` helps in connecting to an existing database. The recipe uses a URL string format with `Client::connect`. It assumes an existing database named `library`, the username is `postgres` and the password is `postgres`.

```rust,editable,no_run
{{#include ../../deps/examples/create_tables.rs}}
```

## Insert and Query data

[![postgres-badge]][postgres] [![cat-database-badge]][cat-database]

The recipe inserts data into the `author` table using [`execute`][execute] method of `Client`. Then, displays the data from the `author` table  using [`query`][query] method of `Client`.

```rust,editable,no_run
{{#include ../../deps/examples/insert_query_data.rs}}
```

## Aggregate data

[![postgres-badge]][postgres] [![cat-database-badge]][cat-database]

This recipe lists the nationalities of the first 7999 artists in the database of the [`Museum of Modern Art`][csv-sample] in descending order.

```rust,editable,no_run
{{#include ../../deps/examples/aggregate_data.rs}}
```

[postgres]: https://docs.rs/postgres/
[Client::connect]: https://docs.rs/postgres/*/postgres/struct.Client.html#method.connect
[execute]: https://docs.rs/postgres/*/postgres/struct.Client.html#method.execute
[query]: https://docs.rs/postgres/*/postgres/struct.Client.html#method.query
[csv-sample]: https://github.com/MuseumofModernArt/collection/blob/master/Artists.csv
{{#include ../refs/link-refs.md}}