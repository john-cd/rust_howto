## Create a SQLite database

[![rusqlite-badge]][rusqlite] [![cat-database-badge]][cat-database]

Use the `rusqlite` crate to open SQLite databases. See
[crate][documentation] for compiling on Windows.

[`Connection::open`] will create the database if it doesn't already exist.

```rust,editable,no_run
{#include ../../../deps/examples/initialization.rs}
```

[`Connection::open`]: https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.open

[documentation]: https://github.com/jgallagher/rusqlite#user-content-notes-on-building-rusqlite-and-libsqlite3-sys
