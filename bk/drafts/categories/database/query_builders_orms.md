<!-- TODO Reviewed ??-??-2026 -->

# Query Builders and ORMs

{{#include query_builders_orms.incl.md}}

These libraries offer different levels of abstraction. [`diesel`][c~diesel~docs]↗ is a synchronous, schema-driven ORM and query builder with compile-time checks; [`diesel-async`][c~diesel-async~docs]↗ adds asynchronous connections. [SeaORM][c~sea-orm~docs]↗ is an asynchronous ORM built on SeaQuery and SQLx; its CLI can generate entities and manage migrations. [`sqlx`][c~sqlx~docs]↗ is an asynchronous SQL toolkit, not an ORM: it lets you write SQL directly and provides compile-time checked query macros when schema metadata is available. [`toasty`][c~toasty~docs]↗ is a newer asynchronous ORM for SQL and NoSQL databases that exposes backend-specific capabilities; its API and backend support continue to evolve.

## `sqlx` {#sqlx}

[![sqlx][c~sqlx~docs~badge]][c~sqlx~docs] [![sqlx~crates.io][c~sqlx~crates.io~badge]][c~sqlx~crates.io] [![sqlx~repo][c~sqlx~repo~badge]][c~sqlx~repo] [![sqlx~lib.rs][c~sqlx~lib.rs~badge]][c~sqlx~lib.rs]{{hi:sqlx}} [![cat~database][cat~database~badge]][cat~database]{{hi:Databases}}

[`sqlx`][c~sqlx~docs]↗ is a low-level, [asynchronous][p~asynchronous] SQL library for Rust. It supports various [databases][p~databases] like PostgreSQL{{hi:PostgreSQL}}, MySQL{{hi:MySQL}}, SQLite{{hi:SQLite}}, and MSSQL{{hi:MSSQL}}, and both [`tokio`][c~tokio~docs]↗{{hi:tokio}} and [`async-std`][c~async-std~docs]↗{{hi:async-std}} async runtimes. It is not an ORM: write SQL directly, and use compile-time checked macros when schema metadata is available.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/query_builders_orms/sqlx.rs:example}}
```

## SeaORM {#sea-orm}

[![sea-orm][c~sea-orm~docs~badge]][c~sea-orm~docs]{{hi:sea-orm}} [![sea-orm~website][c~sea-orm~website~badge]][c~sea-orm~website] [![sea-orm~cookbook][c~sea-orm~cookbook~badge]][c~sea-orm~cookbook] [![cat~database][cat~database~badge]][cat~database]{{hi:Databases}}

[Seaography GraphQL server][c~seaography~website]↗{{hi:seaography}}.

SeaORM is asynchronous and uses SeaQuery to build SQL, with SQLx handling database connections and query execution. [`sea-orm-cli`][c~sea-orm-cli~docs]↗ can generate entities and manage migrations.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/query_builders_orms/sea_orm.rs:example}}
```

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/query_builders_orms/seaography.rs:example}}
```

## `diesel` {#diesel}

[![diesel][c~diesel~docs~badge]][c~diesel~docs]{{hi:diesel}} [![diesel~lib.rs][c~diesel~lib.rs~badge]][c~diesel~lib.rs] [![cat~database][cat~database~badge]][cat~database]{{hi:Databases}}

The [`diesel`][c~diesel~docs]↗{{hi:diesel}} crate is a synchronous ORM and query builder. It uses a schema description and Rust types to check many queries at compile time.

[`diesel`][c~diesel~docs]↗{{hi:diesel}} supports PostgreSQL, MySQL, and SQLite. [`diesel`][c~diesel~docs]↗{{hi:diesel}} has excellent performance and takes an approach of strict compile time guarantees. The main crate is synchronous only, but [`diesel-async`][c~diesel-async~docs]↗{{hi:diesel-async}} provides an async connection implementation.

To create a new [`diesel`][c~diesel~docs]↗{{hi:diesel}} project targeting [`sqlite`][c~sqlite~docs]↗{{hi:sqlite}}, follow these steps:

- Add the necessary dependencies to your Cargo.toml file (update the versions as needed):

```toml
diesel = { version = "2.3", features = ["sqlite"] }
diesel_migrations = { version = "2.3", features = ["sqlite"] }
```

- Run the following commands in your terminal to create a [`.env`][.env-files]↗{{hi:.env}} file with the [database][p~database] [URL][p~url]: set up your schema.rs file and migrations.

```bash
echo DATABASE_URL=sqlite://app.db > .env
cargo install diesel_cli --no-default-features --features sqlite
diesel setup
diesel migration generate create_users
```

`DATABASE_URL` can point to a file-backed SQLite database for multi-command CLI workflows. An in-memory database exists only within one process; the example test below opens one directly so it needs no `.env` file. For other [databases][p~databases], the URL format is `protocol://user:password@host/database`. For MySQL, the URL would be as follows:

```bash
echo DATABASE_URL=mysql://<username>:<password>@localhost/<database> >> .env
```

- Add the following SQL to the generated `up.sql` file:

```sql
{{#include ../../../crates/cats/database/examples/query_builders_orms/migrations/2024-12-29-173417_create_users/up.sql}}
```

- Leave the `down.sql` file empty for simplicity.

```sql
{{#include ../../../crates/cats/database/examples/query_builders_orms/migrations/2024-12-29-173417_create_users/down.sql}}
```

- Create a `schema.rs` file by running:

```bash
diesel print-schema > src/schema.rs
```

- Write the Rust code to interact with the [database][p~database]:

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/query_builders_orms/diesel1.rs:example}}
```

The workspace example applies its checked-in migration to an in-memory SQLite database, inserts a user, and queries that row back. Run its focused test with `cargo test -p database --example query_builders_orms --features diesel -- diesel1::tests::applies_migration_and_round_trips_a_user` from `bk/crates`.

<div class="hidden">

Issue #709 replaces the SQL-printing placeholder with a runnable Diesel flow using `diesel_migrations::embed_migrations!`. The example owns its migration files and uses SQLite `:memory:`, so it needs neither `DATABASE_URL` nor an external database service.

</div>

## `toasty` {#toasty}

[![toasty][c~toasty~docs~badge]][c~toasty~docs]{{hi:toasty}}
[![toasty~crates.io][c~toasty~crates.io~badge]][c~toasty~crates.io]
[![toasty~repo][c~toasty~repo~badge]][c~toasty~repo]
[![toasty~lib.rs][c~toasty~lib.rs~badge]][c~toasty~lib.rs]

`toasty` is available on crates.io and supports SQLite, Turso, PostgreSQL, MySQL, and DynamoDB. Its model derive and query macros generate typed APIs; this example uses in-memory SQLite, so it runs without a service or network access.

```rust,editable
{{#include ../../../crates/cats/database/examples/query_builders_orms/toasty.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Database index][p~database] for database and migration tooling.
- [Connection pools][ex~database~connection-pool].
- [Diesel migrations](#diesel).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
