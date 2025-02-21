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

## Connection pools

{{#include connection_pool.incl.md}}

## Query builders and ORMs

{{#include query_builders_orms.incl.md}}

## NoSQL and friends

{{#include nosql.incl.md}}

## Key-Value stores

{{#include key_value_stores.incl.md}}

## Search

{{#include search.incl.md}}

## Message queues (AMQP)

{{#include amqp.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">

[P1 organize](https://github.com/john-cd/rust_howto/issues/1065)

PostgreSQL: `tokio-postgres`, `postgres`
MySQL: `mysql_async`, `sqlx` (with MySQL support)
SQLite: `rusqlite`
MongoDB: `mongodb`
Redis: `redis`
Generic SQL (using SQLx): `sqlx` (supports multiple databases)
Object-Relational Mappers (ORMs): `diesel`, `sea-orm`
Database Migrations: `sea-orm-cli` (for SeaORM), `diesel_cli` (for Diesel), `migrate`
Connection Pooling: `bb8`, `deadpool`

</div>
