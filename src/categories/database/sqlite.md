# SQLite

{{#include sqlite.incl.md}}

## Create a SQLite database

[![rusqlite][rusqlite-badge]][rusqlite]  [![rusqlite-documentation][rusqlite-documentation-badge]][rusqlite-documentation]  [![cat-database][cat-database-badge]][cat-database]

Use the `rusqlite` crate to open SQLite databases. See
[crate][rusqlite-documentation] for compiling on Windows.

[`Connection::open`][rusqlite::Connection::open] will create the database if it doesn't already exist.

```rust,editable,no_run
{{#include ../../../deps/tests/initialization.rs}}
```

## Insert and Select data

[![rusqlite][rusqlite-badge]][rusqlite]  [![cat-database][cat-database-badge]][cat-database]

[`Connection::open`][rusqlite::Connection::open] will open the database `cats` created in the earlier recipe. This recipe inserts data into `cat_colors` and `cats` tables using the [`execute`][rusqlite::Connection::execute] method of `Connection`. First, the data is inserted into the `cat_colors` table. After a record for a color is inserted, [`last_insert_rowid`][rusqlite::Connection::last_insert_rowid] method of `Connection` is used to get `id` of the last color inserted. This `id` is used while inserting data into the `cats` table. Then, the select query is prepared using the [`prepare`][rusqlite::Connection::prepare] method which gives a [`statement`][rusqlite::Statement] struct. Then, query is executed using [`query_map`][rusqlite::Statement::query_map] method of [`statement`][rusqlite::Statement]

```rust,no_run
{{#include ../../../deps/tests/insert_select.rs}}
```

## Using transactions

[![rusqlite][rusqlite-badge]][rusqlite]  [![cat-database][cat-database-badge]][cat-database]

[`Connection::open`][rusqlite::Connection::open] will open the `cats.db` database from the top recipe.

Begin a transaction with [`Connection::transaction`][rusqlite::Connection::transaction] Transactions will roll back unless committed explicitly with [`Transaction::commit`][rusqlite::Transaction::commit]

In the following example, colors add to a table having a unique constraint on the color name. When an attempt to insert a duplicate color is made, the transaction rolls back.

```rust,editable,no_run
{{#include ../../../deps/tests/transactions.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
