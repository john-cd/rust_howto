# CSV processing

{{#include csv.incl.md}}

## Read {{i:CSV}} records

[![csv][csv-badge]][csv]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

Reads standard CSV records into [`csv::StringRecord`][csv::StringRecord]⮳ — a weakly typed data representation which expects valid UTF-8 rows. Alternatively,
[`csv::ByteRecord`][csv::ByteRecord]⮳ makes no assumptions about UTF-8.

```rust,editable
{{#include ../../../deps/tests/read.rs}}
```

{{i:`serde`}} deserializes data into strongly type structures. See the [`csv::Reader::deserialize`][csv::Reader::deserialize]⮳ method.

```rust,editable
{{#include ../../../deps/tests/read1.rs}}
```

## Read CSV records with different {{i:delimiter}}

[![csv][csv-badge]][csv]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

Reads CSV records with a tab [`delimiter`][csv::ReaderBuilder::delimiter]⮳.

```rust,editable
{{#include ../../../deps/tests/delimiter.rs}}
```

## Filter CSV records matching a predicate

[![csv][csv-badge]][csv]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

Returns _only_ the rows from `data` with a field that matches `query`.

```rust,editable
{{#include ../../../deps/tests/filter.rs}}
```

_Disclaimer: this example has been adapted from [the csv crate tutorial](https://docs.rs/csv/*/csv/tutorial/index.html#filter-by-search)⮳_.

## Handle invalid CSV data with Serde

[![csv][csv-badge]][csv]  [![serde][serde-badge]][serde]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

CSV files often contain {{i:invalid data}}. For these cases, the `csv` crate provides a {{i:custom deserializer}}, [`csv::invalid_option`][csv::invalid_option]⮳ which automatically converts invalid data to `None` values.

```rust,editable
{{#include ../../../deps/tests/invalid.rs}}
```

## Serialize records to CSV

[![csv][csv-badge]][csv]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

This example shows how to serialize a Rust tuple. [`csv::writer`][csv::writer]⮳ supports automatic {{i:serialization}} from Rust types into CSV records. [`write_record`][csv::Writer::write_record]⮳ writes a simple record containing string data only. Data with more complex values such as numbers, floats, and options use [`serialize`][csv::Writer::serialize]⮳. Since CSV writer uses an internal buffer, always explicitly [`flush`][csv::Writer::flush]⮳ when done.

```rust,editable
{{#include ../../../deps/tests/serialize.rs}}
```

## Serialize records to CSV using Serde

[![csv][csv-badge]][csv]  [![serde][serde-badge]][serde]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

The following example shows how to {{i:serialize custom structs}} as CSV records using the [`serde`][serde]⮳ crate.

```rust,editable
{{#include ../../../deps/tests/serde-serialize.rs}}
```

## Transform CSV column

[![csv][csv-badge]][csv]  [![serde][serde-badge]][serde]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

Transform a CSV file containing a color name and a hex color into one with a color name and an rgb color. Utilizes the [`csv`][csv]⮳ crate to read and write the csv file, and [`serde`][serde]⮳ to deserialize and serialize the rows to and from bytes.

See [`csv::Reader::deserialize`][csv::Reader::deserialize]⮳, [`serde::Deserialize`][serde::Deserialize]⮳ and [`std::str::FromStr`][std::str::FromStr]⮳.

```rust,editable
{{#include ../../../deps/tests/transform.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
