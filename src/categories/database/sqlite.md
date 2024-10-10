# SQLite

{{#include sqlite.incl.md}}

## Create a SQLite database

[![rusqlite][c-rusqlite-badge]][c-rusqlite]  [![rusqlite-documentation][c-rusqlite-documentation-badge]][c-rusqlite-documentation]  [![cat-database][cat-database-badge]][cat-database]

Use the [`{{i:rusqlite}}`][c-rusqlite]⮳ crate to open SQLite databases. See
the [documentation][c-rusqlite-documentation]⮳ for compiling on Windows.

[`{{i:Connection::open}}`][c-rusqlite::Connection::open]⮳ will create the database if it doesn't already exist.

```rust,editable,no_run
{{#include ../../../deps/tests/initialization.rs}}
```

## Insert and Select data

[![rusqlite][c-rusqlite-badge]][c-rusqlite]  [![cat-database][cat-database-badge]][cat-database]

[`{{i:Connection::open}}`][c-rusqlite::Connection::open]⮳ will open the database `cats` created in the earlier recipe. This recipe inserts data into `cat_colors` and `cats` tables using the [`{{i:execute}}`][c-rusqlite::Connection::execute]⮳ method of [`{{i:Connection}}`][rusqlite::Connection]⮳. First, the data is inserted into the `cat_colors` table. After a record for a color is inserted, [`{{i:last_insert_rowid}}`][c-rusqlite::Connection::last_insert_rowid]⮳ method of [`{{i:Connection}}`][c-rusqlite::Connection]⮳ is used to get `id` of the last color inserted. This `id` is used while inserting data into the `cats` table. Then, the select query is prepared using the [`{{i:prepare}}`][c-rusqlite::Connection::prepare]⮳ method which gives a [`{{i:statement}}`][c-rusqlite::Statement]⮳ struct. Then, query is executed using [`{{i:query_map}}`][c-rusqlite::Statement::query_map]⮳ method of [`{{i:statement}}`][rusqlite::Statement]⮳

```rust,no_run
{{#include ../../../deps/tests/insert_select.rs}}
```

## Using transactions

[![rusqlite][c-rusqlite-badge]][c-rusqlite]  [![cat-database][cat-database-badge]][cat-database]

[`{{i:Connection::open}}`][c-rusqlite::Connection::open]⮳ will open the `cats.db` database from the top recipe.

Begin a transaction with [`{{i:Connection::transaction}}`][c-rusqlite::Connection::transaction]⮳ Transactions will roll back unless committed explicitly with [`{{i:Transaction::commit}}`][c-rusqlite::Transaction::commit]⮳.

In the following example, colors add to a table having a unique constraint on the color name. When an attempt to insert a duplicate color is made, the transaction rolls back.

```rust,editable,no_run
{{#include ../../../deps/tests/transactions.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
