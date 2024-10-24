# SQLite

{{#include sqlite.incl.md}}

## Create a SQLite database

[![rusqlite][c-rusqlite-badge]][c-rusqlite]{{hi:rusqlite}}  [![rusqlite-documentation][c-rusqlite-documentation-badge]][c-rusqlite-documentation]  [![cat-database][cat-database-badge]][cat-database]{{hi:Databases}}

Use the [`rusqlite`][c-rusqlite]{{hi:rusqlite}}⮳ crate to open SQLite databases. See
the [documentation][c-rusqlite-documentation]⮳ for compiling on Windows.

[`rusqlite::Connection::open`][c-rusqlite::Connection::open]{{hi:rusqlite::Connection::open}}⮳ will create the database if it doesn't already exist.

```rust,no_run
{{#include ../../../deps/tests/initialization.rs}}
```

## Insert and Select data

[![rusqlite][c-rusqlite-badge]][c-rusqlite]{{hi:rusqlite}}  [![cat-database][cat-database-badge]][cat-database]{{hi:Databases}}

[`rusqlite::Connection::open`][c-rusqlite::Connection::open]{{hi:rusqlite::Connection::open}}⮳ will open the database `cats` created in the earlier recipe. This recipe inserts data into `cat_colors` and `cats` tables using the [`rusqlite::Connection::execute`][c-rusqlite::Connection::execute]{{hi:rusqlite::Connection::execute}}⮳ method of [`rusqlite::Connection`][c-rusqlite::Connection]{{hi:rusqlite::Connection}}⮳. First, the data is inserted into the `cat_colors` table. After a record for a color is inserted, [`rusqlite::Connection::last_insert_rowid`][c-rusqlite::Connection::last_insert_rowid]{{hi:rusqlite::Connection::last_insert_rowid}}⮳ method of [`rusqlite::Connection`][c-rusqlite::Connection]{{hi:rusqlite::Connection}}⮳ is used to get `id` of the last color inserted. This `id` is used while inserting data into the `cats` table. Then, the select query is prepared using the [`rusqlite::Connection::prepare`][c-rusqlite::Connection::prepare]{{hi:rusqlite::Connection::prepare}}⮳ method which gives a [`rusqlite::Statement`][c-rusqlite::Statement]{{hi:rusqlite::Statement}}⮳ struct. Then, query is executed using [`rusqlite::Statement::query_map`][c-rusqlite::Statement::query_map]{{hi:rusqlite::Statement::query_map}}⮳ method of [`rusqlite::Statement`][c-rusqlite::Statement]{{hi:rusqlite::Statement}}⮳

```rust,no_run
{{#include ../../../deps/tests/insert_select.rs}}
```

## Using transactions

[![rusqlite][c-rusqlite-badge]][c-rusqlite]{{hi:rusqlite}}  [![cat-database][cat-database-badge]][cat-database]{{hi:Databases}}

[`rusqlite::Connection::open`][c-rusqlite::Connection::open]{{hi:rusqlite::Connection::open}}⮳ will open the `cats.db` database from the top recipe.

Begin a transaction with [`rusqlite::Connection::transaction`][c-rusqlite::Connection::transaction]{{hi:rusqlite::Connection::transaction}}⮳ Transactions will roll back unless committed explicitly with [`rusqlite::Transaction::commit`][c-rusqlite::Transaction::commit]{{hi:rusqlite::Transaction::commit}}⮳.

In the following example, colors add to a table having a unique constraint on the color name. When an attempt to insert a duplicate color is made, the transaction rolls back.

```rust,no_run
{{#include ../../../deps/tests/transactions.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
