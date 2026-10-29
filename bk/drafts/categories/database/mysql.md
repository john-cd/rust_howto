<!-- TODO Reviewed ??-??-2026 -->

# MySQL and MariaDB

{{#include mysql.incl.md}}

[`mysql_async`][c~mysql_async~docs]↗{{hi:mysql_async}} is an asynchronous, Tokio-based driver for MySQL and MariaDB, written in Rust. It includes a connection pool, so you create one `Pool` for the application and borrow connections from it. If you already use [SQLx][c~sqlx~docs]↗ for other databases, its MySQL support is an alternative with compile-time-checked queries; see [Query builders and ORMs](query_builders_orms.md).

## Connect and Run Parameterized Queries {#mysql_async}

[![mysql_async][c~mysql_async~docs~badge]][c~mysql_async~docs]{{hi:mysql_async}} [![mysql_async~crates.io][c~mysql_async~crates.io~badge]][c~mysql_async~crates.io] [![mysql_async~repo][c~mysql_async~repo~badge]][c~mysql_async~repo] [![mysql_async~lib.rs][c~mysql_async~lib.rs~badge]][c~mysql_async~lib.rs] [![cat~database][cat~database~badge]][cat~database]{{hi:Database}}

Read the connection URL (`mysql://user:password@host:3306/database`) from the environment at the application entry point rather than hard-coding credentials. `Pool::new` does not connect; the first `get_conn` call does.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/mysql/mysql_async.rs:connect}}
```

Bind values with `?` placeholders and a tuple, or with named `:name` placeholders and the `params!` macro; do not interpolate them into the SQL text. `exec_first` returns the first row converted to the requested type, or `None` if there is no row. Call `Pool::disconnect` before the program exits so open connections are closed cleanly.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/mysql/mysql_async.rs:query}}
```

For a disposable development server, start MariaDB or MySQL in a container, then create a user and database for the example:

```sh
docker run --name rust-howto-mysql --rm -d -p 127.0.0.1:3306:3306 -e MARIADB_ROOT_PASSWORD='Example_Only_42!' -e MARIADB_USER=u -e MARIADB_PASSWORD=p -e MARIADB_DATABASE=t mariadb:11
export MYSQL_URL='mysql://u:p@127.0.0.1:3306/t'
cargo run -p database --example mysql --features mysql
```

The credentials are disposable local-test values. Stop the container with `docker stop rust-howto-mysql` when finished.

From `bk/crates`, `cargo test -p database --features mysql --example mysql` compiles the ignored service test. Run it with `-- --ignored` after setting `MYSQL_URL`; a missing variable produces an error rather than a skipped assertion. The example was run against MariaDB 10.11.

## Related Topics {#related-topics .skip}

- [Database interfaces](index.md).
- [Query builders and ORMs](query_builders_orms.md).
- [Connection pools](connection_pool.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
