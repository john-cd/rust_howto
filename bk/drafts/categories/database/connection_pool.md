<!-- TODO Reviewed ??-??-2026 -->

# Connection Pool

{{#include connection_pool.incl.md}}

## Create a Connection Pool {#connection-pool}

[![deadpool][c~deadpool~docs~badge]][c~deadpool~docs] [![deadpool~crates.io][c~deadpool~crates.io~badge]][c~deadpool~crates.io] [![deadpool~repo][c~deadpool~repo~badge]][c~deadpool~repo] [![deadpool~lib.rs][c~deadpool~lib.rs~badge]][c~deadpool~lib.rs]{{hi:deadpool}}{{hi:Pool}}{{hi:Async}}{{hi:Database}}

[`deadpool`][c~deadpool~docs]↗{{hi:deadpool}} is a simple async pool for connections and objects of any type.

A pool avoids opening a new database connection for every request. It keeps a
bounded set of connections and lends one to a caller for the duration of a
checkout. Set `max_size` to cap concurrent connections. The wait timeout limits
how long a caller waits when all connections are checked out; create and recycle
timeouts bound connection setup and health checks. Choose values with the
database's connection limit and the application's request deadline in mind.

`pool.get().await?` waits for an available object or creates one through the
manager. The returned guard returns the object to the pool when it is dropped.
The first example uses a one-connection pool and checks that dropping and
checking out the guard again leaves the pool at one allocated connection. If
the pool is full, a checkout waits up to its configured wait timeout. If the
database is down, creating a connection fails and the error is returned to the
caller; a failed recycle check discards the unhealthy connection.

The PostgreSQL example reads `DATABASE_URL` or the individual `PG__HOST`,
`PG__USER`, `PG__PASSWORD`, and `PG__DBNAME` settings. It also reads
`PG__POOL__MAX_SIZE` and the optional
`PG__POOL__TIMEOUTS__WAIT__SECS`, `PG__POOL__TIMEOUTS__CREATE__SECS`, and
`PG__POOL__TIMEOUTS__RECYCLE__SECS` settings. It also accepts the corresponding
`_NANOS` values. Keep credentials in environment variables or a local `.env`
file that is excluded from version control. For other designs, compare
[`bb8`][c~bb8~docs]↗, synchronous [`r2d2`][c~r2d2~docs]↗, and the pool included
in [`sqlx`][c~sqlx~docs]↗.

The ignored PostgreSQL integration test submits only `SELECT` queries and
doesn't create tables or rows, so dropping its pool is sufficient cleanup. Both
the example and test propagate checkout and query failures with `?`.

```rust,editable
{{#include ../../../crates/cats/database/examples/connection_pool/deadpool.rs:example}}
```

Here is an example demonstrating the use of [`deadpool`][c~deadpool~docs]↗{{hi:deadpool}} to connect to a [Postgres][p~postgres] database:

```rust,editable
{{#include ../../../crates/cats/database/examples/connection_pool/deadpool2.rs:example}}
```

## Related Topics {#related-topics .skip}

- [PostgreSQL][ex~database~tokio-postgres]
- [SQLx][ex~database~sqlx]
- [Query Builders and ORMs][ex~database~query-builders-orms]
- [Database Access][p~database]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
