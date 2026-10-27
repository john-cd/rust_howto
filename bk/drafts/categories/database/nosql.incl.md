<!-- TODO Reviewed ??-??-2026 -->

These examples cover document, key-value, and wide-column data models, plus a
low-level Cassandra wire-protocol example. MongoDB and Cassandra examples need
a running database; Redis needs a running Redis server. The protocol example
encodes and decodes a frame in memory and needs no service.

| Recipe                                                              | Crate(s)                                           | Service requirement                       |
| ------------------------------------------------------------------- | -------------------------------------------------- | ----------------------------------------- |
| [Connect to MongoDB][ex~database~mongodb]                           | [`mongodb`][c~mongodb~docs]↗                       | MongoDB; configure `MONGO_URI`            |
| [Connect to Redis][ex~database~redis]                               | [`redis`][c~redis~docs]↗                           | Redis; defaults to `127.0.0.1:6379`       |
| [Connect to Cassandra Using `cdrs-tokio`][ex~database~cdrs-tokio]   | [`cdrs-tokio`][c~cdrs-tokio~docs]↗                 | Cassandra at `127.0.0.1:9042`             |
| [Encode a Cassandra Protocol Frame][ex~database~cassandra-protocol] | [`cassandra-protocol`][c~cassandra-protocol~docs]↗ | None; the example runs entirely in memory |
