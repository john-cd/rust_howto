# Query builders and ORMs

{{#include query_builders_orms.incl.md}}

## Sqlx {#sqlx}

[![sqlx][c-sqlx-badge]][c-sqlx]{{hi:sqlx}}  [![sqlx-github][c-sqlx-github-badge]][c-sqlx-github]  [![sqlx-lib.rs][c-sqlx-lib.rs-badge]][c-sqlx-lib.rs]  [![cat-database][cat-database-badge]][cat-database]{{hi:Databases}}

[`sqlx`][c-sqlx]{{hi:sqlx}}⮳ is the Rust SQL Toolkit. An async, pure Rust SQL crate featuring compile-time checked queries without a DSL. Supports PostgreSQL{{hi:PostgreSQL}}, MySQL{{hi:MySQL}}, SQLite{{hi:SQLite}}, and MSSQL{{hi:MSSQL}}.

Works with Postgres, MySQL, SQLite, and MS SQL.
Supports compile time checking of queries. Async: supports both tokio and async-std.

## SeaORM {#sea-orm}

[![sea-orm][c-sea_orm-badge]][c-sea_orm]{{hi:sea-orm}}  [![sea_orm-website][c-sea_orm-website-badge]][c-sea_orm-website]  [![sea_orm-cookbook][c-sea_orm-cookbook-badge]][c-sea_orm-cookbook]  [![cat-database][cat-database-badge]][cat-database]{{hi:Databases}}

[Seaography GraphQL server][c-seaography-website]{{hi:seaography}}⮳

Built on top of sqlx (see above). There is also a related sea-query crate that provides a query builder without full ORM functionality.

## Diesel {#diesel}

[![diesel][c-diesel-badge]][c-diesel]{{hi:diesel}}  [![diesel-lib.rs][c-diesel-lib.rs-badge]][c-diesel-lib.rs]  [![cat-database][cat-database-badge]][cat-database]{{hi:Databases}}

Has excellent performance and takes an approach of strict compile time guarantees. The main crate is Sync only, but diesel-async provides an async connection implementation.

## Toasty {#toasty}

[![toasty][c-toasty-badge]][c-toasty]{{hi:toasty}}
[![toasty-crates.io][c-toasty-crates.io-badge]][c-toasty-crates.io]
[![toasty-github][c-toasty-github-badge]][c-toasty-github]
[![toasty-lib.rs][c-toasty-lib.rs-badge]][c-toasty-lib.rs]

Toasty is an ORM for the Rust programming language that prioritizes ease-of-use. It supports both SQL datases as well as some NoSQL databases, including DynamoDB and Cassandra. Note that Toasty does not hide the database capabilities. Instead, Toasty exposes features based on the target database.

It is currently in active development and not yet published to crates.io. You can try using it directly from Github.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
Cover https://tokio.rs/blog/2024-10-23-announcing-toasty
</div>
