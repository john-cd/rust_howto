# Working with Postgres

{{#include postgres.incl.md}}

## Create Tables in a Postgres Database {#create-tables-postgres}

[![postgres][c~postgres~docs~badge]][c~postgres~docs]{{hi:postgres}} [![cat~database][cat~database~badge]][cat~database]{{hi:Databases}}

Use the [`postgres`][c~postgres~docs]↗{{hi:postgres}} crate to create tables in a Postgres database.

[`postgres::Client::connect`][c~postgres::Client::connect~docs]↗{{hi:postgres::Client::connect}} helps in connecting to an existing database{{hi:Database}}. The recipe uses a URL string format with [`Client::connect`][c~postgres::Client::connect~docs]↗{{hi:Client::connect}}. It assumes an existing database named `library`, the username is `postgres` and the password is `postgres`.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/postgres/create_tables.rs:example}}
```

## Insert and Query Data {#insert-query-data-postgres}

[![postgres][c~postgres~docs~badge]][c~postgres~docs]{{hi:postgres}} [![cat~database][cat~database~badge]][cat~database]{{hi:Databases}}

The recipe inserts data into the `author` table using [`postgres::Client::execute`][c~postgres::Client::execute~docs]↗{{hi:postgres::Client::execute}} method of [`postgres::Client`][c~postgres::Client~docs]↗{{hi:postgres::Client}}. Then, displays the data from the `author`{{hi:author}} table using [`postgres::Client::query`][c~postgres::Client::query~docs]↗{{hi:postgres::Client::query}} method of [`postgres::Client`][c~postgres::Client~docs]↗{{hi:postgres::Client}}.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/postgres/insert_query_data.rs:example}}
```

## Aggregate Data {#aggregate-data-postgres}

[![postgres][c~postgres~docs~badge]][c~postgres~docs]{{hi:postgres}} [![cat~database][cat~database~badge]][cat~database]{{hi:Database}} [![csv-sample~github][csv-sample~github~badge]][csv-sample~github]

This recipe lists the nationalities of the first 7999 artists in the [database][p~database] of the [`Museum of Modern Art`][csv-sample~github]↗ in descending order.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/postgres/aggregate_data.rs:example}}
```

## Connect to and Query Postgres Asynchronously with `tokio-postgres` {#tokio-postgres}

[![tokio-postgres][c~tokio-postgres~docs~badge]][c~tokio-postgres~docs]{{hi:tokio-postgres}}
[![tokio-postgres~crates.io][c~tokio-postgres~crates.io~badge]][c~tokio-postgres~crates.io]
[![tokio-postgres~github][c~tokio-postgres~github~badge]][c~tokio-postgres~github]
[![tokio-postgres~lib.rs][c~tokio-postgres~lib.rs~badge]][c~tokio-postgres~lib.rs]

[`tokio-postgres`][c~tokio-postgres~docs]↗{{hi:tokio-postgres}} provides an asynchronous PostgreSQL client. It is built on top of the [`tokio`][c~tokio~docs]↗{{hi:tokio}} runtime and thus supports non-blocking interactions with PostgreSQL databases. This crate offers connection pooling, prepared statements, transactions, and support for various PostgreSQL data types. It performs better than [`SQLx`][c~sqlx~docs]↗{{hi:SQLx}}.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/postgres/tokio_postgres.rs:example}}
```

## `cornucopia` for Postgres {#cornucopia}

[![cornucopia~website][c~cornucopia~website~badge]][c~cornucopia~website] [![cornucopia][c~cornucopia~docs~badge]][c~cornucopia~docs] [![cornucopia~crates.io][c~cornucopia~crates.io~badge]][c~cornucopia~crates.io] [![cornucopia~github][c~cornucopia~github~badge]][c~cornucopia~github] [![cornucopia~lib.rs][c~cornucopia~lib.rs~badge]][c~cornucopia~lib.rs]{{hi:cornucopia}}{{hi:Query}}{{hi:Generator}}{{hi:Tokio-postgres}}{{hi:Postgresql}}{{hi:Sql}} [![cat~database][cat~database~badge]][cat~database]{{hi:Database interfaces}}

[`cornucopia`][c~cornucopia~docs]↗{{hi:cornucopia}} generates type-checked Rust from your PostgreSQL: [cornucopia-rs][c~cornucopia~github]↗.

[`cornucopia`][c~cornucopia~docs]↗{{hi:cornucopia}} is a tool powered by `rust-postgres` designed to generate type-checked Rust interfaces from your PostgreSQL queries. It works by preparing your queries against an actual [database][p~database] and then running an extensive validation suite on them. Once the queries are prepared and validated, Rust code is generated into a module, which can be imported and used in your project.

The basic premise is thus to:

- Write PostgreSQL queries.
- Use Cornucopia to generate Rust code.
- Use the generated code in your project.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/postgres/cornucopia.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[postgres: `cornucopia`](https://github.com/john-cd/rust_howto/issues/286)

If you are not familiar with Cornucopia, it is kinda like SQLc for Go - you write a query in Postgres SQL and then use cli to generate checked Rust code. So no macro compilation time overhead or complex types that are hard for the rust-analyzer to handle. It uses `rust-postgres` under the hood, so it supports query pipelining and the performance should be pretty good as well.

Fork:

[clorinde~github][clorinde~github].
[clorinde~github]: https://github.com/halcyonnouveau/clorinde

</div>
