<!-- TODO Reviewed ??-??-2026 -->

# Oracle DB

Oracle [Database][p~database] is a multi-model [database][p~database] management system produced and marketed by Oracle Corporation.

## `diesel-oci` {#diesel-oci}

[![diesel-oci][c~diesel-oci~docs~badge]][c~diesel-oci~docs] [![diesel-oci~crates.io][c~diesel-oci~crates.io~badge]][c~diesel-oci~crates.io] [![diesel-oci~repo][c~diesel-oci~repo~badge]][c~diesel-oci~repo] [![diesel-oci~lib.rs][c~diesel-oci~lib.rs~badge]][c~diesel-oci~lib.rs]{{hi:diesel-oci}}{{hi:Sql}}{{hi:Oci}}{{hi:Diesel}}{{hi:Oracle}} [![cat~database][cat~database~badge]][cat~database]{{hi:Database interfaces}}

[`diesel-oci`][c~diesel-oci~docs]↗{{hi:diesel}} is an OCI database adapter for [`diesel`][c~diesel~docs]↗{{hi:diesel}}. "Oracle Call Interface" is the comprehensive native C language interface to Oracle Database.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/oracle/diesel_oci.rs:example}}
```

## Connect to an Oracle Database with `oracle` {#oracle}

[![oracle][c~oracle~docs~badge]][c~oracle~docs] [![oracle~crates.io][c~oracle~crates.io~badge]][c~oracle~crates.io] [![oracle~repo][c~oracle~repo~badge]][c~oracle~repo] [![oracle~lib.rs][c~oracle~lib.rs~badge]][c~oracle~lib.rs]{{hi:oracle}}{{hi:Database}}{{hi:oracle}}

[`oracle`][c~oracle~docs]↗{{hi:oracle}} provides Oracle DB bindings for Rust. This crate provides a safe and ergonomic interface to Oracle databases.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/oracle/oracle.rs:example}}
```

## `sibyl` {#sibyl}

[![sibyl~website][c~sibyl~website~badge]][c~sibyl~website] [![sibyl][c~sibyl~docs~badge]][c~sibyl~docs] [![sibyl~crates.io][c~sibyl~crates.io~badge]][c~sibyl~crates.io] [![sibyl~repo][c~sibyl~repo~badge]][c~sibyl~repo] [![sibyl~lib.rs][c~sibyl~lib.rs~badge]][c~sibyl~lib.rs]{{hi:sibyl}}{{hi:Database}}{{hi:Sql}}{{hi:Ffi}}{{hi:Async}}{{hi:Oracle}} [![cat~database][cat~database~badge]][cat~database]{{hi:Database interfaces}}

[`sibyl`][c~sibyl~docs]↗{{hi:sibyl}} offers an OCI-based (synchronous or asynchronous) interface between Rust applications and Oracle databases.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/oracle/sibyl.rs:example}}
```

<div class="hidden">

The example reads the built-in `DUAL` table so it works without Oracle's sample schemas. Build it with `cargo run -p database --example oracle --features sibyl -- sibyl`. Run the ignored integration test with `cargo test -p database --example oracle --features sibyl -- sibyl::tests::connects_to_oracle_with_sibyl --ignored`; set `ORACLE_CONNECT_STRING`, `ORACLE_USERNAME`, and `ORACLE_PASSWORD` first.

</div>

### Oracle Instant Client for Sibyl

Sibyl uses Oracle's native OCI library. Download the Basic (or Basic Light) and SDK packages for your OS from the [Oracle Instant Client downloads](https://www.oracle.com/database/technologies/instant-client/downloads.html).

On Linux, extract both packages into the same directory. Add that directory to the dynamic loader path, for example:

```sh
export LD_LIBRARY_PATH=/path/to/instantclient:$LD_LIBRARY_PATH
```

On Windows, extract the Basic and SDK packages into the same Instant Client directory and add it to `PATH` so `oci.dll` can be loaded. For an MSVC build, set `OCI_LIB_DIR` to the SDK directory containing `oci.lib` before building. Keep the Instant Client architecture consistent with the Rust target.

For a local Oracle Free service, the official [`gvenzl/oracle-free` container](https://github.com/gvenzl/oci-oracle-free) listens on port 1521 and uses service name `FREEPDB1` by default. Create or provide a database user, then set the three environment variables documented above. The example's `DUAL` query does not require sample HR tables.

## Troubleshooting {#troubleshooting}

### Cannot locate a 64-bit Oracle Client library

If the native Oracle client cannot be loaded, the Oracle Instant Client is missing or is not configured in the system's library path.

To resolve this on Linux:

1.  **Install dependencies** (package names differ by distribution):
    ```bash
    sudo apt-get install libaio1
    ```
2.  **Download and extract** the Oracle Instant Client Basic and SDK packages into the same directory.
3.  **Set `LD_LIBRARY_PATH`**:
    ```bash
    export LD_LIBRARY_PATH=/path/to/instantclient:$LD_LIBRARY_PATH
    ```
4.  Alternatively, configure the system-wide library path by adding a file to `/etc/ld.so.conf.d/` and running `sudo ldconfig`.

## Related Topics {#related-topics .skip}

- [[database/sqlite | SQLite]]
- [[database/postgres | Postgres]]
- [[database/mssql | MSSQL]]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
