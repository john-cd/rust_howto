<!-- TODO Reviewed ??-??-2026 -->

# Working with Postgres

{{#include postgres.incl.md}}

## Prerequisites {#postgres-prerequisites .skip}

These examples need a reachable PostgreSQL server. One way to start a disposable local server is Docker:

```console
docker run --rm --name rust-howto-postgres -e POSTGRES_PASSWORD=postgres -e POSTGRES_DB=rust_howto -p 5432:5432 -d postgres
```

Set `DATABASE_URL` in the shell where you run the examples:

```powershell
$env:DATABASE_URL = "postgresql://postgres:postgres@localhost:5432/rust_howto"
```

```sh
export DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rust_howto"
```

Stop the temporary service with `docker stop rust-howto-postgres`. The examples and ignored PostgreSQL integration tests use `DATABASE_URL`; the aggregate example expects an `artists` table. Its integration test creates a temporary table with a small fixture.

## Create Tables in a Postgres Database {#create-tables-postgres}

[![postgres][c~postgres~docs~badge]][c~postgres~docs]{{hi:postgres}} [![cat~database][cat~database~badge]][cat~database]{{hi:Databases}}

Use the synchronous [`postgres`][c~postgres~docs]↗{{hi:postgres}} crate to create tables in PostgreSQL.

[`postgres::Client::connect`][c~postgres::Client::connect~docs]↗{{hi:postgres::Client::connect}} opens a connection using `DATABASE_URL`. The example creates `author` and `book` tables with `CREATE TABLE IF NOT EXISTS`, so it can be run again against the same database.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/postgres/create_tables.rs:example}}
```

## Insert and Query Data {#insert-query-data-postgres}

[![postgres][c~postgres~docs~badge]][c~postgres~docs]{{hi:postgres}} [![cat~database][cat~database~badge]][cat~database]{{hi:Databases}}

The recipe creates a temporary `author` table for this connection, then uses [`postgres::Client::execute`][c~postgres::Client::execute~docs]↗{{hi:postgres::Client::execute}} with bound parameters to insert several rows in one statement. [`postgres::Client::query`][c~postgres::Client::query~docs]↗{{hi:postgres::Client::query}} reads them back in a stable order. The temporary table keeps repeated runs isolated.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/postgres/insert_query_data.rs:example}}
```

## Aggregate Data {#aggregate-data-postgres}

[![postgres][c~postgres~docs~badge]][c~postgres~docs]{{hi:postgres}} [![cat~database][cat~database~badge]][cat~database]{{hi:Database}} [![csv-sample~repo][csv-sample~repo~badge]][csv-sample~repo]

This recipe counts artist rows by nationality in an existing PostgreSQL `artists` table. Missing nationalities are grouped as `Unknown`, and equal counts are ordered by nationality. The linked [`Museum of Modern Art`][csv-sample~repo]↗ data can populate the table for a full dataset run.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/postgres/aggregate_data.rs:example}}
```

<div class="hidden">
  The ignored integration test creates a connection-local `artists` table and verifies the counts with a small fixture. Running the example itself reads the existing `artists` table from `DATABASE_URL`.
</div>

## Connect to and Query Postgres Asynchronously with `tokio-postgres` {#tokio-postgres}

{{#badges tokio-postgres}}

[`tokio-postgres`][c~tokio-postgres~docs]↗{{hi:tokio-postgres}} is an asynchronous PostgreSQL client for the [`tokio`][c~tokio~docs]↗{{hi:tokio}} runtime. The example starts the connection driver on a separate task, then uses parameterized statements inside a temporary table. `tokio-postgres` does not provide a pool; see [Connection Pools][ex~database~connection_pool] for pooling options. [`sqlx`][c~sqlx~docs]↗{{hi:SQLx}} is another async client with compile-time query checking when its offline or live metadata is configured.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/tests/postgres/tokio_postgres.rs:example}}
```

## `cornucopia` for Postgres {#cornucopia}

[![cornucopia~website][c~cornucopia~website~badge]][c~cornucopia~website] [![cornucopia][c~cornucopia~docs~badge]][c~cornucopia~docs] [![cornucopia~crates.io][c~cornucopia~crates.io~badge]][c~cornucopia~crates.io] [![cornucopia~repo][c~cornucopia~repo~badge]][c~cornucopia~repo] [![cornucopia~lib.rs][c~cornucopia~lib.rs~badge]][c~cornucopia~lib.rs]{{hi:cornucopia}}{{hi:Query}}{{hi:Generator}}{{hi:Tokio-postgres}}{{hi:Postgresql}}{{hi:Sql}} [![cat~database][cat~database~badge]][cat~database]{{hi:Database interfaces}}

Cornucopia is a command-line generator that prepares annotated SQL against a PostgreSQL database and writes a Rust crate of query bindings. The example uses the Cornucopia 1.0 CLI; the generated crate `cornucopia_queries` is a path dependency that re-exports the PostgreSQL driver, so no separate runtime crate is needed. Keep the generator, SQL, and checked-in generated crate in sync.

The sample's annotated queries are in `bk/crates/cats/database/examples/postgres/cornucopia/queries/greetings.sql`; its schema is in `schema.sql`; `cornucopia.toml` names the generated package. Apply the schema to the database from `DATABASE_URL`, then run `cornucopia live -c <cornucopia.toml> -q <queries-dir> -d <generated-crate-dir> --async true --sync false <DATABASE_URL>` from the repository root. Generation requires a live database so Cornucopia can validate the SQL and infer the result types.

The workflow is:

- Write annotated SQL and define its schema.
- Prepare the schema in PostgreSQL and generate the Rust bindings.
- Call the generated functions with a compatible PostgreSQL client.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/postgres/cornucopia.rs:example}}
```

Cornucopia 1.0 incorporated the Clorinde fork's rewritten code generator, so projects upgrading from 0.9 must regenerate their bindings. See the [Cornucopia migration guide][c~cornucopia~migration~1.0].

## Related Topics {#related-topics .skip}

- [Connection Pools][ex~database~connection_pool]
- [`sqlx`][ex~database~sqlx]
- [Query Builders and ORMs][ex~database~query-builders-orms]
- [Database Search][ex~database~search]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
