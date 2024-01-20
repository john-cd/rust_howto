## Insert and Query data

[![postgres-badge]][postgres] [![cat-database-badge]][cat-database]

The recipe inserts data into the `author` table using [`execute`] method of `Client`. Then, displays the data from the `author` table  using [`query`] method of `Client`.

```rust,editable,no_run
{#include ../../../deps/examples/insert_query_data.rs}
```

[`execute`]: https://docs.rs/postgres/0.17.2/postgres/struct.Client.html#method.execute
[`query`]: https://docs.rs/postgres/0.17.2/postgres/struct.Client.html#method.query
