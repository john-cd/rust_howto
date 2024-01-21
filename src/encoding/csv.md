# CSV processing

## Read CSV records

[![csv-badge]][csv] [![cat-encoding-badge]][cat-encoding]

Reads standard CSV records into [`csv::StringRecord`][csv::StringRecord] — a weakly typed
data representation which expects valid UTF-8 rows. Alternatively,
`[csv::ByteRecord]` makes no assumptions about UTF-8.

```rust,editable
{{#include ../../deps/examples/read.rs}}
```

Serde deserializes data into strongly type structures. See the
`[csv::Reader::deserialize]` method.

```rust,editable
{{#include ../../deps/examples/read1.rs}}
```

## Read CSV records with different delimiter

[![csv-badge]][csv] [![cat-encoding-badge]][cat-encoding]

Reads CSV records with a tab `[delimiter]`.

```rust,editable
{{#include ../../deps/examples/delimiter.rs}}
```

## Filter CSV records matching a predicate

[![csv-badge]][csv] [![cat-encoding-badge]][cat-encoding]

Returns _only_ the rows from `data` with a field that matches `query`.

```rust,editable
{{#include ../../deps/examples/filter.rs}}
```

_Disclaimer: this example has been adapted from [the csv crate tutorial](https://docs.rs/csv/*/csv/tutorial/index.html#filter-by-search)_.

## Handle invalid CSV data with Serde

[![csv-badge]][csv] [![serde-badge]][serde] [![cat-encoding-badge]][cat-encoding]

CSV files often contain invalid data. For these cases, the `csv` crate provides a custom deserializer, [`csv::invalid_option`][csv::invalid_option] which automatically converts invalid data to None values.

```rust,editable
{{#include ../../deps/examples/invalid.rs}}
```

## Serialize records to CSV

[![csv-badge]][csv] [![cat-encoding-badge]][cat-encoding]

This example shows how to serialize a Rust tuple. [`csv::writer`][csv::writer] supports automatic serialization from Rust types into CSV records. [`write_record`][write_record] writes a simple record containing string data only. Data with more complex values such as numbers, floats, and options use [`serialize`][serialize] Since CSV writer uses an internal buffer, always explicitly [`flush`][flush] when done.

```rust,editable
{{#include ../../deps/examples/serialize.rs}}
```

## Serialize records to CSV using Serde

[![csv-badge]][csv] [![serde-badge]][serde] [![cat-encoding-badge]][cat-encoding]

The following example shows how to serialize custom structs as CSV records using
the [serde] crate.

```rust,editable
{{#include ../../deps/examples/serde-serialize.rs}}
```

## Transform CSV column

[![csv-badge]][csv] [![serde-badge]][serde] [![cat-encoding-badge]][cat-encoding]

Transform a CSV file containing a color name and a hex color into one with a color name and an rgb color.  Utilizes the [csv] crate to read and write the csv file, and [serde] to deserialize and serialize the rows to and from bytes.

See [`csv::Reader::deserialize`][csv::Reader::deserialize] `[serde::Deserialize]` and `[std::str::FromStr]`

```rust,editable
{{#include ../../deps/examples/transform.rs}}
```

[csv::ByteRecord]: https://docs.rs/csv/*/csv/struct.ByteRecord.html
[csv::Reader::deserialize]: https://docs.rs/csv/*/csv/struct.Reader.html#method.deserialize
[csv::StringRecord]: https://docs.rs/csv/*/csv/struct.StringRecord.html
[delimiter]: https://docs.rs/csv/1.0.0-beta.3/csv/struct.ReaderBuilder.html#method.delimiter
[csv::Writer]: https://docs.rs/csv/*/csv/struct.Writer.html
[flush]: https://docs.rs/csv/*/csv/struct.Writer.html#method.flush
[serialize]: https://docs.rs/csv/*/csv/struct.Writer.html#method.serialize
[write_record]: https://docs.rs/csv/*/csv/struct.Writer.html#method.write_record
[csv::invalid_option]: https://docs.rs/csv/*/csv/fn.invalid_option.html
[serde::Deserialize]: https://docs.rs/serde/\*/serde/trait.Deserialize.html
[std::str::FromStr]: https://doc.rust-lang.org/std/str/trait.FromStr.html
{{#include ../refs/link-refs.md}}
