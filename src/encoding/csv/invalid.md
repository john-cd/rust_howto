## Handle invalid CSV data with Serde

[![csv-badge]][csv] [![serde-badge]][serde] [![cat-encoding-badge]][cat-encoding]

CSV files often contain invalid data. For these cases, the `csv` crate
provides a custom deserializer, [`csv::invalid_option`], which automatically
converts invalid data to None values.

```rust,editable
{#include ../../../deps/examples/invalid.rs}
```

[`csv::invalid_option`]: https://docs.rs/csv/*/csv/fn.invalid_option.html
