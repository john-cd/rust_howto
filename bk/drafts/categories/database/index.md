# Databases

[![cat~database][cat~database~badge]][cat~database]

This section describes interfaces with database management systems{{hi:Database management systems}}.

| Database | Rust crates |
|---|---|
| SQL Databases | [`sqlx`][c~sqlx~docs]↗{{hi:sqlx}} (supports multiple databases) |
| PostgreSQL | [`tokio-postgres`][c~tokio-postgres~docs]↗{{hi:tokio-postgres}}, [`postgres`][c~postgres~docs]↗{{hi:postgres}} |
| MySQL | [`mysql_async`][c~mysql_async~docs]↗{{hi:mysql_async}}, [`sqlx`][c~sqlx~docs]↗{{hi:sqlx}} (with MySQL support) |
| SQLite | [`rusqlite`][c~rusqlite~docs]↗{{hi:rusqlite}} |
| MongoDB | [`mongodb`][c~mongodb~docs]↗{{hi:mongodb}} |
| Redis | [`redis`][c~redis~docs]↗{{hi:redis}} |
| Object-Relational Mappers (ORMs) | [`diesel`][c~diesel~docs]↗{{hi:diesel}}, [`sea-orm`][c~sea-orm~docs]↗{{hi:sea-orm}} |
| Database Migrations | [`sea-orm-cli`][c~sea-orm-cli~docs]↗{{hi:sea-orm-cli}} (for SeaORM), [`diesel_cli`](https://docs.rs/crate/diesel_cli/latest)↗{{hi:diesel_cli}} (for Diesel), [`migrate`][c~migrate~docs]↗{{hi:migrate}} |
| Connection Pooling | [`bb8`][c~bb8~docs]↗{{hi:bb8}}, [`deadpool`][c~deadpool~docs]↗{{hi:deadpool}} |

## Code Examples

### SQLite

{{#include sqlite.incl.md}}

### Postgres

{{#include postgres.incl.md}}

### Microsoft SQL Server (MSSQL)

{{#include mssql.incl.md}}

### Oracle DB

{{#include oracle.incl.md}}

### Connection Pools

{{#include connection_pool.incl.md}}

### Query Builders and ORMs

{{#include query_builders_orms.incl.md}}

### NoSQL and Friends

{{#include nosql.incl.md}}

### Key-Value Stores

{{#include key_value_stores.incl.md}}

### Search

{{#include search.incl.md}}

### Message Queues (AMQP)

{{#include amqp.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[organize](https://github.com/john-cd/rust_howto/issues/1065)
</div>
