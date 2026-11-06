<!-- TODO Reviewed ??-??-2026 -->

# database-cornucopia-queries

Checked-in output of [`cornucopia`](https://github.com/cornucopia-rs/cornucopia) for the
`cornucopia_greetings` example in the book's Postgres chapter. **Do not edit the
files under `src/` by hand**: regenerate them with `cornucopia` instead.

## Layout

- `src/queries/greetings.rs`: typed statements `add_greeting()` and `list_greetings()`.
- `src/client/`: the `GenericClient` abstraction over `tokio-postgres` and `deadpool-postgres`.
- `src/array_iterator.rs`, `src/domain.rs`, `src/type_traits.rs`, `src/utils.rs`: support code for arrays and domains.

## Features

- `deadpool` (default): enables `deadpool-postgres`.
- `wasm-async`: enables `tokio-postgres/js`.

## Tests

`tests/offline.rs` needs **no database**. It checks the array/domain
(de)serialization helpers and that the query constructors build. Run:

```sh
cargo test -p database-cornucopia-queries
```

Tests that run the queries against a live server belong with the Postgres examples in
`../examples/postgres/` and `../tests/postgres/`.
