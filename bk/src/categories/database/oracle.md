# Oracle DB

Oracle [Database][p-database] is a multi-model [database][p-database] management system produced and marketed by Oracle Corporation.

## `diesel_oci` {#diesel_oci}

[![diesel-oci][c-diesel_oci-badge]][c-diesel_oci] [![diesel-oci-crates.io][c-diesel_oci-crates.io-badge]][c-diesel_oci-crates.io] [![diesel-oci-github][c-diesel_oci-github-badge]][c-diesel_oci-github] [![diesel-oci-lib.rs][c-diesel_oci-lib.rs-badge]][c-diesel_oci-lib.rs]{{hi:diesel-oci}}{{hi:Sql}}{{hi:Oci}}{{hi:Diesel}}{{hi:Oracle}} [![cat-database][cat-database-badge]][cat-database]{{hi:Database interfaces}}

[`diesel_oci`][c-diesel_oci]⮳{{hi:diesel}} is an OCI database adapter for [`diesel`][c-diesel]⮳{{hi:diesel}}. "Oracle Call Interface" is the comprehensive native C language interface to Oracle Database.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/tests/oracle/diesel_oci.rs:example}}
```

## Connect to an Oracle database with `oracle` {#oracle}

[![oracle][c-oracle-badge]][c-oracle] [![oracle-crates.io][c-oracle-crates.io-badge]][c-oracle-crates.io] [![oracle-github][c-oracle-github-badge]][c-oracle-github] [![oracle-lib.rs][c-oracle-lib.rs-badge]][c-oracle-lib.rs]{{hi:oracle}}{{hi:Database}}{{hi:oracle}}

[`oracle`][c-oracle]⮳{{hi:oracle}} provides Oracle DB bindings for Rust. This crate provides a safe and ergonomic interface to Oracle databases.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/tests/oracle/oracle.rs:example}}
```

## `sibyl` {#sibyl}

[![sibyl-website][c-sibyl-website-badge]][c-sibyl-website] [![sibyl][c-sibyl-badge]][c-sibyl] [![sibyl-crates.io][c-sibyl-crates.io-badge]][c-sibyl-crates.io] [![sibyl-github][c-sibyl-github-badge]][c-sibyl-github] [![sibyl-lib.rs][c-sibyl-lib.rs-badge]][c-sibyl-lib.rs]{{hi:sibyl}}{{hi:Database}}{{hi:Sql}}{{hi:Ffi}}{{hi:Async}}{{hi:Oracle}} [![cat-database][cat-database-badge]][cat-database]{{hi:Database interfaces}}

[`sibyl`][c-sibyl]⮳{{hi:sibyl}} offers an OCI-based (synchronous or asynchronous) interface between Rust applications and Oracle databases.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/tests/oracle/sibyl.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1069)
</div>
