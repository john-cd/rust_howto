## Create tables in a Postgres database

[![postgres-badge]][postgres] [![cat-database-badge]][cat-database]

Use the [`postgres`] crate to create tables in a Postgres database.

[`Client::connect`] helps in connecting to an existing database. The recipe uses a URL string format with `Client::connect`. It assumes an existing database named `library`, the username is `postgres` and the password is `postgres`.

```rust,editable,no_run
{#include ../../../deps/examples/create_tables.rs}
```

[`postgres`]: https://docs.rs/postgres/0.17.2/postgres/
[`Client::connect`]: https://docs.rs/postgres/0.17.2/postgres/struct.Client.html#method.connect
