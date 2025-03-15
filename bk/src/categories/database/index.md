# Databases

[![cat-database][cat-database-badge]][cat-database]

Interface with database management systems{{hi:Database management systems}}.

## SQLite

{{#include sqlite.incl.md}}

## Postgres

{{#include postgres.incl.md}}

## Microsoft SQL Server (MSSQL)

{{#include mssql.incl.md}}

## Oracle DB

{{#include oracle.incl.md}}

## Connection Pools

{{#include connection_pool.incl.md}}

## Query Builders and ORMs

{{#include query_builders_orms.incl.md}}

## NoSQL and Friends

{{#include nosql.incl.md}}

## Key-Value Stores

{{#include key_value_stores.incl.md}}

## Search

{{#include search.incl.md}}

## Message Queues (AMQP)

{{#include amqp.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[organize](https://github.com/john-cd/rust_howto/issues/1065)

- PostgreSQL: [`tokio-postgres`][c-tokio_postgres]⮳{{hi:tokio-postgres}}, [`postgres`][c-postgres]⮳{{hi:postgres}}
- MySQL: [`mysql_async`][c-mysql_async]⮳{{hi:mysql_async}}, [`sqlx`][c-sqlx]⮳{{hi:sqlx}} (with MySQL support)
- SQLite: [`rusqlite`][c-rusqlite]⮳{{hi:rusqlite}}
- MongoDB: [`mongodb`][c-mongodb]⮳{{hi:mongodb}}
- Redis: [`redis`][c-redis]⮳{{hi:redis}}
- Generic SQL (using SQLx): [`sqlx`][c-sqlx]⮳{{hi:sqlx}} (supports multiple databases)
- Object-Relational Mappers (ORMs): [`diesel`][c-diesel]⮳{{hi:diesel}}, [`sea-orm`][c-sea_orm]⮳{{hi:sea-orm}}
- Database Migrations: [`sea-orm-cli`][c-sea_orm_cli]⮳{{hi:sea-orm-cli}} (for SeaORM), `diesel_cli` (for Diesel), [`migrate`][c-migrate]⮳{{hi:migrate}}
- Connection Pooling: [`bb8`][c-bb8]⮳{{hi:bb8}}, [`deadpool`][c-deadpool]⮳{{hi:deadpool}}

</div>
