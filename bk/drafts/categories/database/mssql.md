<!-- TODO Reviewed ??-??-2026 -->

# Microsoft SQL Server

{{#include mssql.incl.md}}

## Connect to SQL Server {#tiberius}

[![tiberius][c~tiberius~docs~badge]][c~tiberius~docs]

Load the host, login, and password at the application entry point. Require encrypted transport and verify the server certificate. The example permits a self-signed local test server only when `MSSQL_TRUST_CERT=1` is explicitly selected; leave that override unset for a normally trusted certificate.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/mssql/tiberius.rs:connect}}
```

For a disposable development server, review Microsoft's container licensing and system requirements, then start the official image if you accept those terms:

```sh
docker run --name rust-howto-mssql --rm -d -p 127.0.0.1:1433:1433 -e ACCEPT_EULA=Y -e MSSQL_SA_PASSWORD='Example_Only_42!' mcr.microsoft.com/mssql/server:2022-latest
```

Wait for the server to report that it is ready for client connections. In the same shell used for Cargo, configure the local example:

```sh
export MSSQL_HOST=localhost
export MSSQL_USER=sa
export MSSQL_PASSWORD='Example_Only_42!'
export MSSQL_TRUST_CERT=1
cargo run -p database --example mssql --features mssql
```

The shown password is only a disposable local-test value. Stop this test container with `docker stop rust-howto-mssql` when finished.

## Run a Parameterized Query {#mssql-query}

[![tiberius][c~tiberius~docs~badge]][c~tiberius~docs]

Bind values through the parameter slice; do not interpolate them into SQL text. This deterministic query binds `@P1`, adds one, and checks the typed result, including a missing-row or NULL-column error:

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/mssql/tiberius.rs:query}}
```

From `bk/crates`, `cargo test -p database --features mssql --example mssql` compiles the ignored service test. Run it with `-- --ignored` after configuring the server. A missing configuration produces an error rather than a successful skipped assertion.

## Related Topics {#related-topics .skip}

- [Database interfaces](index.md).
- [Query builders and ORMs](query_builders_orms.md).
- [Connection pools](connection_pool.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
