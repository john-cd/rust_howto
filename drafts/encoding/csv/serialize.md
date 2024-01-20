## Serialize records to CSV

[![csv-badge]][csv] [![cat-encoding-badge]][cat-encoding]

This example shows how to serialize a Rust tuple. [`csv::writer`] supports automatic
serialization from Rust types into CSV records. [`write_record`] writes
a simple record containing string data only. Data with more complex values
such as numbers, floats, and options use [`serialize`]. Since CSV
writer uses internal buffer, always explicitly [`flush`] when done.

```rust,editable
{#include ../../../deps/examples/serialize.rs}
```

[`csv::Writer`]: https://docs.rs/csv/*/csv/struct.Writer.html
[`flush`]: https://docs.rs/csv/*/csv/struct.Writer.html#method.flush
[`serialize`]: https://docs.rs/csv/*/csv/struct.Writer.html#method.serialize
[`write_record`]: https://docs.rs/csv/*/csv/struct.Writer.html#method.write_record
