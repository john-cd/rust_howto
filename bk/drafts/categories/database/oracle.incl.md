<!-- TODO Reviewed ??-??-2026 -->

Choose [`oracle`][c~oracle~docs] for a blocking Rust API built on Oracle's ODPI-C layer, or [`sibyl`][c~sibyl~docs] for direct OCI integration with blocking and nonblocking APIs. Both need a compatible Oracle Instant Client; Sibyl's Cargo configuration must select exactly one mode, and its nonblocking mode also selects an async executor. This book uses Sibyl's blocking mode. Use `diesel-oci` when an application already relies on Diesel's query and schema abstractions.

| Recipe                                                            | Crates                                                      | Categories                                          |
| ----------------------------------------------------------------- | ----------------------------------------------------------- | --------------------------------------------------- |
| [`diesel-oci`][ex~database~diesel-oci]                            | [![diesel-oci][c~diesel-oci~docs~badge]][c~diesel-oci~docs] | [![cat~database][cat~database~badge]][cat~database] |
| [Connect to an Oracle Database with `oracle`][ex~database~oracle] | [![oracle][c~oracle~docs~badge]][c~oracle~docs]             | [![cat~database][cat~database~badge]][cat~database] |
| [`sibyl`][ex~database~sibyl]                                      | [![sibyl][c~sibyl~docs~badge]][c~sibyl~docs]                | [![cat~database][cat~database~badge]][cat~database] |
