<!-- TODO Reviewed ??-??-2026 -->

# Database

Book code examples for Database interactions with SQLx, Diesel, MongoDB, and Redis. It demonstrates integration with key ecosystem crates including `anyhow`, `byteorder`, `cassandra-protocol`, `cdrs-tokio`, `clap`, `config`.

## Examples Covered

- **Amqp** (`examples/amqp/`): Demonstrates concepts like `lapin`.
- **Cassandra** (`examples/cassandra/`): Demonstrates concepts like `cassandra_protocol`, `cdrs_tokio`.
- **Connection Pool** (`examples/connection_pool/`): Demonstrates concepts like `deadpool`, `deadpool2`.
- **Kv** (`examples/kv/`): Demonstrates concepts like `heed`, `rocksdb`.
- **Mssql** (`examples/mssql/`): Demonstrates concepts like `tiberius`.
- **Nosql** (`examples/nosql/`): Demonstrates concepts like `mongodb`, `redis`.
- **Oracle** (`examples/oracle/`): Demonstrates concepts like `diesel_oci`, `oracle`, `sibyl`.
- **Postgres** (`examples/postgres/`): Demonstrates concepts like `aggregate_data`, `cornucopia`, `create_tables`, `insert_query_data`.
- **Query Builders ORMS** (`examples/query_builders_orms/`): Demonstrates concepts like `diesel1`, `sea_orm`, `seaography`, `sqlx`, `toasty`.
- **Search** (`examples/search/`): Demonstrates concepts like `elasticsearch`.
- **Sqlite** (`examples/sqlite/`): Demonstrates concepts like `initialization`, `insert_select`, `transactions`.

## Usage

To check or run examples in this crate:

```console
cargo check --package database --examples
cargo test --package database
```

Code examples correspond to chapters in `bk/src/categories/database` or related book sections.
