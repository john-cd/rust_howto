<!-- TODO Reviewed ??-??-2026 -->

# Databases

[![cat~database][cat~database~badge]][cat~database]

This section covers Rust libraries for connecting to database management systems and related data services. Choose a chapter by database type or by the task you need to perform.

| Group     | Topic                                                      | Recommended Rust crates                                                                                                                                                                                                                                      |
| --------- | ---------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| SQL       | [SQLite][ex~database~create-database-sqlite]               | [`rusqlite`][c~rusqlite~docs]↗, [`sqlx`][c~sqlx~docs]↗                                                                                                                                                                                                       |
| SQL       | [PostgreSQL][p~postgres]                                   | [`tokio-postgres`][c~tokio-postgres~docs]↗, [`postgres`][c~postgres~docs]↗, [`sqlx`][c~sqlx~docs]↗                                                                                                                                                           |
| SQL       | [MySQL and MariaDB][ex~database~mysql_async]               | [`mysql_async`][c~mysql_async~docs]↗                                                                                                                                                                                                                         |
| SQL       | [Microsoft SQL Server][ex~database~tiberius]               | [`tiberius`][c~tiberius~docs]↗                                                                                                                                                                                                                               |
| SQL       | [Oracle][ex~database~oracle]                               | [`oracle`][c~oracle~docs]↗, [`sibyl`][c~sibyl~docs]↗, [`diesel-oci`][c~diesel-oci~docs]↗                                                                                                                                                                     |
| NoSQL     | [MongoDB, Redis, and Cassandra][ex~database~mongodb]       | [`mongodb`][c~mongodb~docs]↗, [`redis`][c~redis~docs]↗, [`cdrs-tokio`][c~cdrs-tokio~docs]↗, [`cassandra-protocol`][c~cassandra-protocol~docs]↗                                                                                                               |
| NoSQL     | [Key-value stores][ex~database~heed]                       | [`heed`][c~heed~docs]↗, [`rocksdb`][c~rocksdb~docs]↗                                                                                                                                                                                                         |
| NoSQL     | [Search][ex~database~search]                               | [`elastic`][c~elastic~docs]↗, [`tinysearch`][c~tinysearch~docs]↗                                                                                                                                                                                             |
| Messaging | [AMQP message queues][ex~database~lapin]                   | [`lapin`][c~lapin~docs]↗                                                                                                                                                                                                                                     |
| Tooling   | [Connection pools][ex~database~connection-pool]            | [`bb8`][c~bb8~docs]↗, [`deadpool`][c~deadpool~docs]↗, [`r2d2`][c~r2d2~docs]↗                                                                                                                                                                                 |
| Tooling   | [Query builders and ORMs][ex~database~query-builders-orms] | [`sqlx`][c~sqlx~docs]↗ (PostgreSQL, MySQL, SQLite), [`sea-orm`][c~sea-orm~docs]↗, [`diesel`][c~diesel~docs]↗, [`toasty`][c~toasty~docs]↗; migrations: [`sea-orm-cli`][c~sea-orm-cli~docs]↗, [`diesel_cli`][c~diesel_cli~docs]↗, [`migrate`][c~migrate~docs]↗ |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
