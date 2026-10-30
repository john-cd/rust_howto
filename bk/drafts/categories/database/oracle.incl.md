<!-- TODO Reviewed ??-??-2026 -->

Choose [`oracle`][c~oracle~docs] for a blocking Rust API built on Oracle's ODPI-C layer, or [`sibyl`][c~sibyl~docs] for direct OCI integration with blocking and nonblocking APIs. Both need a compatible Oracle Instant Client; Sibyl's Cargo configuration must select exactly one mode, and its nonblocking mode also selects an async executor. This book uses Sibyl's blocking mode. Use `diesel-oci` when an application already relies on Diesel's query and schema abstractions.

| Recipe                                                            | Crates                | Categories                                     |
| ----------------------------------------------------------------- | --------------------- | ---------------------------------------------- |
| [`diesel-oci`][ex~database~diesel-oci]                            | {{#crate diesel-oci}} | {{#categories database="Database interfaces"}} |
| [Connect to an Oracle Database with `oracle`][ex~database~oracle] | {{#crate oracle}}     | {{#categories database="Database interfaces"}} |
| [`sibyl`][ex~database~sibyl]                                      | {{#crate sibyl}}      | {{#categories database="Database interfaces"}} |
