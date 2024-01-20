## Transform CSV column

[![csv-badge]][csv] [![serde-badge]][serde] [![cat-encoding-badge]][cat-encoding]

Transform a CSV file containing a color name and a hex color into one with a
color name and an rgb color.  Utilizes the [csv] crate to read and write the
csv file, and [serde] to deserialize and serialize the rows to and from bytes.

See [`csv::Reader::deserialize`], [`serde::Deserialize`], and [`std::str::FromStr`]

```rust,editable
{#include ../../../deps/examples/transform.rs}
```

[`csv::Reader::deserialize`]: https://docs.rs/csv/\*/csv/struct.Reader.html#method.deserialize
[`csv::invalid_option`]: https://docs.rs/csv/*/csv/fn.invalid_option.html
[`serde::Deserialize`]: https://docs.rs/serde/\*/serde/trait.Deserialize.html
[`std::str::FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
