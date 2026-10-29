<!-- TODO Reviewed ??-??-2026 -->

# Oracle DB

{{#include oracle.incl.md}}

Oracle [Database][p~database] is a multi-model [database][p~database] management system produced and marketed by Oracle Corporation.

The `diesel-oci`, `oracle`, and `sibyl` examples use Oracle's native OCI
library. Compiling, testing without running ignored integration tests, and
linting these feature-gated examples require the 64-bit Oracle Instant Client
Basic Light package and SDK. They do not require a running database. Running an
example or its ignored integration test also requires a reachable Oracle
service and the credentials described below.

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

The example reads `ORACLE_USER`, `ORACLE_PASSWORD`, and
`ORACLE_CONNECT_STRING` from the environment. Its optional integration test
requires an Oracle database and Oracle Instant Client. For example, start
Oracle Free with Docker, set the connection variables, then run the ignored
test:

```sh
docker run --detach --rm --name rust-howto-oracle-free -p 127.0.0.1:1521:1521 \
  -e ORACLE_PASSWORD=change-me gvenzl/oracle-free
export ORACLE_USER=system
export ORACLE_PASSWORD=change-me
export ORACLE_CONNECT_STRING=127.0.0.1:1521/FREEPDB1
cargo test -p database --example oracle --features oracle -- \
  oracle::tests::require_external_svc --ignored
```

Install Oracle Instant Client before running the test. On Linux, put its
library directory on `LD_LIBRARY_PATH`; on Windows, add the directory
containing `oci.dll` to `PATH`. The test is ignored during ordinary workspace
test runs so they do not connect to a database. Compiling the enabled Oracle
features still requires the native client and SDK described below.

## `sibyl` {#sibyl}

[![sibyl~website][c~sibyl~website~badge]][c~sibyl~website] [![sibyl][c~sibyl~docs~badge]][c~sibyl~docs] [![sibyl~crates.io][c~sibyl~crates.io~badge]][c~sibyl~crates.io] [![sibyl~repo][c~sibyl~repo~badge]][c~sibyl~repo] [![sibyl~lib.rs][c~sibyl~lib.rs~badge]][c~sibyl~lib.rs]{{hi:sibyl}}{{hi:Database}}{{hi:Sql}}{{hi:Ffi}}{{hi:Async}}{{hi:Oracle}} [![cat~database][cat~database~badge]][cat~database]{{hi:Database interfaces}}

[`sibyl`][c~sibyl~docs]↗{{hi:sibyl}} offers an OCI-based (synchronous or asynchronous) interface between Rust applications and Oracle databases.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/oracle/sibyl.rs:example}}
```

<div class="hidden">

The example reads the built-in `DUAL` table so it works without Oracle's sample schemas. Build it with `cargo run -p database --example oracle --features sibyl -- sibyl`. Run the ignored integration test with `cargo test -p database --example oracle --features sibyl -- sibyl::tests::connects_to_oracle_with_sibyl --ignored`; set `ORACLE_CONNECT_STRING`, `ORACLE_USERNAME`, and `ORACLE_PASSWORD` first.

</div>

## Prerequisites: Oracle Instant Client

Sibyl uses Oracle's native OCI library. The repository's Windows and Linux
install scripts and Dev Container install Oracle Instant Client Basic Light and
the SDK at version `23.26.4.0.0`. The scripts verify Oracle's published
SHA-256 checksums before extracting either package.

On Windows, run the setup script from the repository root in PowerShell:

```powershell
pwsh -File .devcontainer/install_deps.ps1
```

It installs the 64-bit client under the user's local programs directory, adds
the client directory to `PATH`, and sets `ORACLE_HOME` and `OCI_LIB_DIR` for the
current and future user sessions. `OCI_LIB_DIR` points to the MSVC SDK directory
containing `oci.lib`. Open a new shell after setup so other applications inherit
the updated user environment.

On Linux, run the setup script from the repository root:

```bash
bash .devcontainer/install_deps.sh
```

It installs the client under `~/.local/opt/oracle`, sets `ORACLE_HOME` and
`OCI_LIB_DIR`, and adds the client directory to `LD_LIBRARY_PATH` and
`LIBRARY_PATH` in `~/.profile`. Source that profile or open a new shell after
setup. On Ubuntu 24.04 and Debian 13, the script installs `libaio1t64`; because
Oracle requests the older `libaio.so.1` name, it adds a compatibility link to
the Instant Client directory when the system only provides `libaio.so.1t64`.

The Dev Container installs the client under
`/opt/oracle/instantclient_23_26` and configures the build and runtime library
paths automatically. For a manual installation, download the Basic or Basic
Light package and SDK for the matching OS and architecture from the [Oracle
Instant Client downloads][oracle~instant-client],
extract both into the same directory, and configure the library paths above.
Keep the Instant Client architecture consistent with the Rust target.

### Compile, lint, and run

From `bk/crates`, compile and lint each backend with the matching feature. The
SDK must be configured as described above; no database service is needed for
these checks:

```sh
cargo test -p database --example oracle --features oracle --no-run
cargo clippy -p database --example oracle --features oracle -- -D warnings
cargo test -p database --example oracle --features sibyl --no-run
cargo clippy -p database --example oracle --features sibyl -- -D warnings
```

Run the examples against an Oracle service with the matching command. The
`oracle` and `sibyl` examples read the environment variables shown in their
sections. The Diesel example reads `ORACLE_DB_URL`, `ORACLE_DB_USERNAME`, and
`ORACLE_DB_PASSWORD` and queries a `users` table:

```sh
cargo run -p database --example oracle --features oracle -- oracle
cargo run -p database --example oracle --features oracle -- diesel_oci
cargo run -p database --example oracle --features sibyl -- sibyl
```

The ignored integration tests require both the native client and a live Oracle
service. Ordinary tests and Clippy do not run or connect to the service.

For a local Oracle Free service, the community-maintained [`gvenzl/oracle-free` container][oracle~docker] listens on port 1521 and uses service name `FREEPDB1` by default. Create or provide a database user, then set the three environment variables documented above. The example's `DUAL` query does not require sample HR tables; the Diesel recipe separately needs its `users` table. Stop the disposable service with `docker stop rust-howto-oracle-free` after testing.

## Troubleshooting {#troubleshooting}

### Cannot locate a 64-bit Oracle Client library

If the native Oracle client cannot be loaded, the Oracle Instant Client is missing or is not configured in the system's library path.

To resolve this on Linux:

1. **Install dependencies** (package names differ by distribution):

   ```bash
   sudo apt-get install libaio1t64
   ```

2. **Download and extract** the Oracle Instant Client Basic and SDK packages into the same directory.
3. **Set `LD_LIBRARY_PATH`**:

   ```bash
   export LD_LIBRARY_PATH=/path/to/instantclient:$LD_LIBRARY_PATH
   ```

4. Alternatively, configure the system-wide library path by adding a file to `/etc/ld.so.conf.d/` and running `sudo ldconfig`.

On Windows, install the 64-bit Oracle Instant Client Basic package and add its
directory to `PATH` before running the Oracle example or its integration test.

## Related Topics {#related-topics .skip}

- [[database/sqlite | SQLite]]
- [[database/postgres | Postgres]]
- [[database/mssql | MSSQL]]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
