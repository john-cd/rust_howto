# CSV processing

{{#include csv.incl.md}}

## Read CSV records

[![csv][c-csv-badge]][c-csv]  [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:csv}}

Reads standard CSV records into [`csv::StringRecord`][c-csv::StringRecord]{{hi:csv::StringRecord}}⮳ — a weakly typed data representation which expects valid UTF-8 rows. Alternatively,
[`csv::ByteRecord`][c-csv::ByteRecord]{{hi:csv::ByteRecord}}⮳ makes no assumptions about UTF-8.

```rust,editable
{{#include ../../../deps/tests/read.rs}}
```

[`serde`][c-serde]{{hi:serde}}⮳ deserializes data into strongly type structures. See the [`csv::Reader::deserialize`][c-csv::Reader::deserialize]{{hi:csv::Reader::deserialize}}⮳ method.

```rust,editable
{{#include ../../../deps/tests/read1.rs}}
```

## Read CSV records with different delimiter

[![csv][c-csv-badge]][c-csv]  [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi: delimiter}}

Reads CSV records with a tab [`csv::ReaderBuilder::delimiter`][c-csv::ReaderBuilder::delimiter]{{hi:csv::ReaderBuilder::delimiter}}⮳.

```rust,editable
{{#include ../../../deps/tests/delimiter.rs}}
```

## Filter CSV records matching a predicate

[![csv][c-csv-badge]][c-csv]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

Returns _only_ the rows from `data` with a field that matches `query`.

```rust,editable
{{#include ../../../deps/tests/filter.rs}}
```

_Disclaimer: this example has been adapted from [the csv crate tutorial](https://docs.rs/csv/*/csv/tutorial/index.html#filter-by-search)⮳_.

## Handle invalid CSV data with Serde

[![csv][c-csv-badge]][c-csv]  [![serde][c-serde-badge]][c-serde]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

CSV files often contain invalid data{{hi:invalid data}}. For these cases, the [`csv`][c-csv]{{hi:csv}}⮳ crate provides a custom deserializer{{hi:custom deserializer}}, [`csv::invalid_option`][c-csv::invalid_option]{{hi:csv::invalid_option}}⮳ which automatically converts invalid data to [`std::option::Option::None`][c-std::option::Option::None]{{hi:std::option::Option::None}}⮳ values.

```rust,editable
{{#include ../../../deps/tests/invalid.rs}}
```

## Serialize records to CSV

[![csv][c-csv-badge]][c-csv]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

This example shows how to serialize a Rust tuple. [`csv::writer`][c-csv::writer]{{hi:csv::writer}}⮳ supports automatic serialization{{hi:Serialization}} from Rust types into CSV records. [`csv::Writer::write_record`][c-csv::Writer::write_record]{{hi:csv::Writer::write_record}}⮳ writes a simple record containing string data only. Data with more complex values such as numbers, floats, and options use [`csv::Writer::serialize`][c-csv::Writer::serialize]{{hi:csv::Writer::serialize}}⮳. Since CSV writer uses an internal buffer, always explicitly [`csv::Writer::flush`][c-csv::Writer::flush]{{hi:csv::Writer::flush}}⮳ when done.

```rust,editable
{{#include ../../../deps/tests/serialize.rs}}
```

## Serialize records to CSV using Serde

[![csv][c-csv-badge]][c-csv]  [![serde][c-serde-badge]][c-serde]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

The following example shows how to serialize custom structs{{hi:serialize custom structs}} as CSV records using the [`serde`][c-serde]{{hi:serde}}⮳ crate.

```rust,editable
{{#include ../../../deps/tests/serde-serialize.rs}}
```

## Transform CSV column

[![csv][c-csv-badge]][c-csv]  [![serde][c-serde-badge]][c-serde]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

Transform a CSV file containing a color name and a hex color into one with a color name and an rgb color. Utilizes the [`csv`][c-csv]{{hi:csv}}⮳ crate to read and write the csv file, and [`serde`][c-serde]{{hi:serde}}⮳ to deserialize and serialize the rows to and from bytes.

See [`csv::Reader::deserialize`][c-csv::Reader::deserialize]{{hi:csv::Reader::deserialize}}⮳, [`serde::Deserialize`][c-serde::Deserialize]{{hi:serde::Deserialize}}⮳ and [`std::str::FromStr`][c-std::str::FromStr]{{hi:std::str::FromStr}}⮳.

```rust,editable
{{#include ../../../deps/tests/transform.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
<div class="hidden">
TODO:
</div>
