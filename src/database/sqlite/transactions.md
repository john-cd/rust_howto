## Using transactions

[![rusqlite-badge]][rusqlite] [![cat-database-badge]][cat-database]

[`Connection::open`] will open the `cats.db` database from the top recipe.

Begin a transaction with [`Connection::transaction`]. Transactions will
roll back unless committed explicitly with [`Transaction::commit`].

In the following example, colors add to a table having
a unique constraint on the color name. When an attempt to insert
a duplicate color is made, the transaction rolls back.

```rust,editable,no_run
{#include ../../../deps/examples/transactions.rs}
```

[`Connection::transaction`]: https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.transaction
[`Transaction::commit`]: https://docs.rs/rusqlite/*/rusqlite/struct.Transaction.html#method.commit
