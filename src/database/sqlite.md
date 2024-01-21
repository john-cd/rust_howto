# SQLite

## Create a SQLite database

[![rusqlite-badge]][rusqlite] [![cat-database-badge]][cat-database]

Use the `rusqlite` crate to open SQLite databases. See
[crate][documentation] for compiling on Windows.

`[Connection::open]` will create the database if it doesn't already exist.

```rust,editable,no_run
{{#include ../../deps/examples/initialization.rs}}
```

## Insert and Select data

[![rusqlite-badge]][rusqlite] [![cat-database-badge]][cat-database]

`[Connection::open]` will open the database `cats` created in the earlier recipe.
This recipe inserts data into `cat_colors` and `cats` tables using the [`execute`][execute] method of `Connection`. First, the data is inserted into the `cat_colors` table. After a record for a color is inserted, [`last_insert_rowid`][last_insert_rowid] method of `Connection` is used to get `id` of the last color inserted. This `id` is used while inserting data into the `cats` table. Then, the select query is prepared using the [`prepare`][prepare] method which gives a [`statement`][statement] struct. Then, query is executed using [`query_map`][query_map] method of `[statement]`

```rust,no_run
{{#include ../../deps/examples/insert_select.rs}}
```

## Using transactions

[![rusqlite-badge]][rusqlite] [![cat-database-badge]][cat-database]

`[Connection::open]` will open the `cats.db` database from the top recipe.

Begin a transaction with [`Connection::transaction`][Connection::transaction] Transactions will
roll back unless committed explicitly with `[Transaction::commit]`

In the following example, colors add to a table having
a unique constraint on the color name. When an attempt to insert
a duplicate color is made, the transaction rolls back.

```rust,editable,no_run
{{#include ../../deps/examples/transactions.rs}}
```

[Connection::open]: https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.open
[documentation]: https://github.com/jgallagher/rusqlite#user-content-notes-on-building-rusqlite-and-libsqlite3-sys
[prepare]: https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.prepare
[statement]: https://docs.rs/rusqlite/*/rusqlite/struct.Statement.html
[query_map]: https://docs.rs/rusqlite/*/rusqlite/struct.Statement.html#method.query_map
[execute]: https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.execute
[last_insert_rowid]: https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.last_insert_rowid
[Connection::transaction]: https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.transaction
[Transaction::commit]: https://docs.rs/rusqlite/*/rusqlite/struct.Transaction.html#method.commit
{{#include ../refs/link-refs.md}}
