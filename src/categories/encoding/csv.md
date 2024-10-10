# CSV processing

{{#include csv.incl.md}}

## Read {{i:CSV}} records

[![csv][csv-badge]][csv]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

Reads standard CSV records into [`{{i:csv::StringRecord}}`][c-csv::StringRecord]⮳ — a weakly typed data representation which expects valid UTF-8 rows. Alternatively,
[`{{i:csv::ByteRecord}}`][c-csv::ByteRecord]⮳ makes no assumptions about UTF-8.

```rust,editable
{{#include ../../../deps/tests/read.rs}}
```

[`{{i:serde}}`][serde]⮳ deserializes data into strongly type structures. See the [`{{i:csv::Reader::deserialize}}`][c-csv::Reader::deserialize]⮳ method.

```rust,editable
{{#include ../../../deps/tests/read1.rs}}
```

## Read CSV records with different {{i:delimiter}}

[![csv][csv-badge]][csv]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

Reads CSV records with a tab [`{{i:delimiter}}`][c-csv::ReaderBuilder::delimiter]⮳.

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

CSV files often contain {{i:invalid data}}. For these cases, the [`{{i:csv}}`][csv]⮳ crate provides a {{i:custom deserializer}}, [`{{i:csv::invalid_option}}`][c-csv::invalid_option]⮳ which automatically converts invalid data to [`{{i:None}}`][c-std::option::Option::None]⮳ values.

```rust,editable
{{#include ../../../deps/tests/invalid.rs}}
```

## Serialize records to CSV

[![csv][csv-badge]][csv]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

This example shows how to serialize a Rust tuple. [`{{i:csv::writer}}`][c-csv::writer]⮳ supports automatic {{i:serialization}} from Rust types into CSV records. [`{{i:write_record}}`][c-csv::Writer::write_record]⮳ writes a simple record containing string data only. Data with more complex values such as numbers, floats, and options use [`{{i:serialize}}`][c-csv::Writer::serialize]⮳. Since CSV writer uses an internal buffer, always explicitly [`{{i:flush}}`][c-csv::Writer::flush]⮳ when done.

```rust,editable
{{#include ../../../deps/tests/serialize.rs}}
```

## Serialize records to CSV using Serde

[![csv][csv-badge]][csv]  [![serde][serde-badge]][serde]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

The following example shows how to {{i:serialize custom structs}} as CSV records using the [`{{i:serde}}`][serde]⮳ crate.

```rust,editable
{{#include ../../../deps/tests/serde-serialize.rs}}
```

## Transform CSV column

[![csv][csv-badge]][csv]  [![serde][serde-badge]][serde]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

Transform a CSV file containing a color name and a hex color into one with a color name and an rgb color. Utilizes the [`{{i:csv}}`][csv]⮳ crate to read and write the csv file, and [`{{i:serde}}`][serde]⮳ to deserialize and serialize the rows to and from bytes.

See [`{{i:csv::Reader::deserialize}}`][c-csv::Reader::deserialize]⮳, [`{{i:serde::Deserialize}}`][c-serde::Deserialize]⮳ and [`{{i:std::str::FromStr}}`][c-std::str::FromStr]⮳.

```rust,editable
{{#include ../../../deps/tests/transform.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
