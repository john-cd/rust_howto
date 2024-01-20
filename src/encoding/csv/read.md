## Read CSV records

[![csv-badge]][csv] [![cat-encoding-badge]][cat-encoding]

Reads standard CSV records into [`csv::StringRecord`] — a weakly typed
data representation which expects valid UTF-8 rows. Alternatively,
[`csv::ByteRecord`] makes no assumptions about UTF-8.

```rust,editable
{#include ../../../deps/examples/read.rs}
```

Serde deserializes data into strongly type structures. See the
[`csv::Reader::deserialize`] method.

```rust,editable
{#include ../../../deps/examples/read2.rs}
```

[`csv::ByteRecord`]: https://docs.rs/csv/*/csv/struct.ByteRecord.html
[`csv::Reader::deserialize`]: https://docs.rs/csv/*/csv/struct.Reader.html#method.deserialize
[`csv::StringRecord`]: https://docs.rs/csv/*/csv/struct.StringRecord.html
