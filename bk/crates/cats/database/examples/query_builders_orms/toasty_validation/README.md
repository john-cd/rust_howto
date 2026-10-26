# Toasty example validation

This small standalone workspace runs and tests the book's [`toasty.rs`](../toasty.rs)
example with in-memory SQLite. It is isolated because Toasty's SQLite driver uses
a `libsqlite3-sys` version that conflicts with the database examples workspace's
deliberately pinned Diesel and SQLx dependencies.

Run it with:

```powershell
cargo test --manifest-path bk/crates/cats/database/examples/query_builders_orms/toasty_validation/Cargo.toml
```
