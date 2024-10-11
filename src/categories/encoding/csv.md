# CSV processing

{{#include csv.incl.md}}

## Read CSV records

[![csv][c-csv-badge]][c-csv]  [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:csv}}

Reads standard CSV records into {{hi:csv::StringRecord}}[`csv::StringRecord`][c-csv::StringRecord]⮳ — a weakly typed data representation which expects valid UTF-8 rows. Alternatively,
{{hi:csv::ByteRecord}}[`csv::ByteRecord`][c-csv::ByteRecord]⮳ makes no assumptions about UTF-8.

```rust,editable
{{#include ../../../deps/tests/read.rs}}
```

{{hi:serde}}[`serde`][c-serde]⮳ deserializes data into strongly type structures. See the {{hi:csv::Reader::deserialize}}[`csv::Reader::deserialize`][c-csv::Reader::deserialize]⮳ method.

```rust,editable
{{#include ../../../deps/tests/read1.rs}}
```

## Read CSV records with different delimiter

[![csv][c-csv-badge]][c-csv]  [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi: delimiter}}

Reads CSV records with a tab {{hi:delimiter}}[`delimiter`][c-csv::ReaderBuilder::delimiter]⮳.

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

CSV files often contain {{i:invalid data}}. For these cases, the {{hi:csv}}[`csv`][c-csv]⮳ crate provides a {{i:custom deserializer}}, {{hi:csv::invalid_option}}[`csv::invalid_option`][c-csv::invalid_option]⮳ which automatically converts invalid data to {{hi:None}}[`None`][c-std::option::Option::None]⮳ values.

```rust,editable
{{#include ../../../deps/tests/invalid.rs}}
```

## Serialize records to CSV

[![csv][c-csv-badge]][c-csv]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

This example shows how to serialize a Rust tuple. {{hi:csv::writer}}[`csv::writer`][c-csv::writer]⮳ supports automatic {{i:serialization}} from Rust types into CSV records. {{hi:write_record}}[`write_record`][c-csv::Writer::write_record]⮳ writes a simple record containing string data only. Data with more complex values such as numbers, floats, and options use {{hi:serialize}}[`serialize`][c-csv::Writer::serialize]⮳. Since CSV writer uses an internal buffer, always explicitly {{hi:flush}}[`flush`][c-csv::Writer::flush]⮳ when done.

```rust,editable
{{#include ../../../deps/tests/serialize.rs}}
```

## Serialize records to CSV using Serde

[![csv][c-csv-badge]][c-csv]  [![serde][c-serde-badge]][c-serde]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

The following example shows how to {{i:serialize custom structs}} as CSV records using the {{hi:serde}}[`serde`][c-serde]⮳ crate.

```rust,editable
{{#include ../../../deps/tests/serde-serialize.rs}}
```

## Transform CSV column

[![csv][c-csv-badge]][c-csv]  [![serde][c-serde-badge]][c-serde]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

Transform a CSV file containing a color name and a hex color into one with a color name and an rgb color. Utilizes the {{hi:csv}}[`csv`][c-csv]⮳ crate to read and write the csv file, and {{hi:serde}}[`serde`][c-serde]⮳ to deserialize and serialize the rows to and from bytes.

See {{hi:csv::Reader::deserialize}}[`csv::Reader::deserialize`][c-csv::Reader::deserialize]⮳, {{hi:serde::Deserialize}}[`serde::Deserialize`][c-serde::Deserialize]⮳ and {{hi:std::str::FromStr}}[`std::str::FromStr`][c-std::str::FromStr]⮳.

```rust,editable
{{#include ../../../deps/tests/transform.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
