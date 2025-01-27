# MS SQL Server

{{#include mssql.incl.md}}

## `tiberius` {#tiberius}

[![tiberius][c-tiberius-badge]][c-tiberius] [![tiberius-crates.io][c-tiberius-crates.io-badge]][c-tiberius-crates.io] [![tiberius-github][c-tiberius-github-badge]][c-tiberius-github] [![tiberius-lib.rs][c-tiberius-lib.rs-badge]][c-tiberius-lib.rs]{{hi:tiberius}}{{hi:Sql}}{{hi:TDS}}{{hi:MSSQL}}

`tiberius` is a TDS (MSSQL) driver. It is a fully asynchronous, pure Rust implementation of the TDS protocol. It is built on top of `tokio` and `futures`.

```rust,editable,noplayground
{{#include ../../../crates/ex/cats/database/tests/mssql/tiberius.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">

TODO P1 organize / write

</div>
