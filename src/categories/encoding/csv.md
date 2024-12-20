# CSV processing

{{#include csv.incl.md}}

## Read CSV records {#read-csv-records}

[![csv][c-csv-badge]][c-csv]{{hi:csv}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}}

Reads standard CSV records into [`csv::StringRecord`][c-csv::StringRecord]{{hi:csv::StringRecord}}⮳ — a weakly typed data representation which expects valid UTF-8 rows. Alternatively,
[`csv::ByteRecord`][c-csv::ByteRecord]{{hi:csv::ByteRecord}}⮳ makes no assumptions about UTF-8.

```rust,editable
{{#include ../../../crates/ex/categories/efghijkl/tests/encoding/read.rs:example}}
```

[`serde`][c-serde]{{hi:serde}}⮳ deserializes data into strongly type structures. See the [`csv::Reader::deserialize`][c-csv::Reader::deserialize]{{hi:csv::Reader::deserialize}}⮳ method.

```rust,editable
{{#include ../../../crates/ex/categories/efghijkl/tests/encoding/read1.rs:example}}
```

## Read CSV records with different delimiter {#read-csv-different-delimiter}

[![csv][c-csv-badge]][c-csv]{{hi:csv}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}}{{hi: Delimiter}}

Reads CSV records with a tab [`csv::ReaderBuilder::delimiter`][c-csv::ReaderBuilder::delimiter]{{hi:csv::ReaderBuilder::delimiter}}⮳.

```rust,editable
{{#include ../../../crates/ex/categories/efghijkl/tests/encoding/delimiter.rs:example}}
```

## Filter CSV records matching a predicate {#filter-csv}

[![csv][c-csv-badge]][c-csv]{{hi:csv}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}}

Returns _only_ the rows from `data` with a field that matches `query`.

```rust,editable
{{#include ../../../crates/ex/categories/efghijkl/tests/encoding/filter.rs:example}}
```

This example has been adapted from [the csv crate tutorial][c-csv-tutorial]⮳

## Handle invalid CSV data with `serde` {#handle-invalid-csv}

[![csv][c-csv-badge]][c-csv]{{hi:csv}} [![serde][c-serde-badge]][c-serde]{{hi:serde}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}}

CSV files often contain invalid data{{hi:Invalid data}}. For these cases, the [`csv`][c-csv]{{hi:csv}}⮳ crate provides a custom deserializer{{hi:Custom deserializer}}, [`csv::invalid_option`][c-csv::invalid_option]{{hi:csv::invalid_option}}⮳ which automatically converts invalid data to [`std::option::Option::None`][c-std::option::Option::None]{{hi:std::option::Option::None}}⮳ values.

```rust,editable
{{#include ../../../crates/ex/categories/efghijkl/tests/encoding/invalid.rs:example}}
```

## Serialize records to CSV {#serialize-to-csv}

[![csv][c-csv-badge]][c-csv]{{hi:csv}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}}

This example shows how to serialize a Rust tuple. [`csv::writer`][c-csv::Writer]{{hi:csv::Writer}}⮳ supports automatic serialization{{hi:Serialization}} from Rust types into CSV records. [`csv::Writer::write_record`][c-csv::Writer::write_record]{{hi:csv::Writer::write_record}}⮳ writes a simple record containing string data only. Data with more complex values such as numbers, floats, and options use [`csv::Writer::serialize`][c-csv::Writer::serialize]{{hi:csv::Writer::serialize}}⮳. Since CSV writer uses an internal buffer, always explicitly [`csv::Writer::flush`][c-csv::Writer::flush]{{hi:csv::Writer::flush}}⮳ when done.

```rust,editable
{{#include ../../../crates/ex/categories/efghijkl/tests/encoding/serialize.rs:example}}
```

## Serialize records to CSV using `serde` {#serialize-to-csv-using-serde}

[![csv][c-csv-badge]][c-csv]{{hi:csv}} [![serde][c-serde-badge]][c-serde]{{hi:serde}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}}

The following example shows how to serialize custom structs{{hi:Serialize custom structs}} as CSV records using the [`serde`][c-serde]{{hi:serde}}⮳ crate.

```rust,editable
{{#include ../../../crates/ex/categories/efghijkl/tests/encoding/serde_serialize.rs:example}}
```

## Transform a CSV column {#transform-csv-column}

[![csv][c-csv-badge]][c-csv]{{hi:csv}} [![serde][c-serde-badge]][c-serde]{{hi:serde}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}}

Transform a CSV file containing a color name and a hex color into one with a color name and an rgb color. Utilizes the [`csv`][c-csv]{{hi:csv}}⮳ crate to read and write the csv file, and [`serde`][c-serde]{{hi:serde}}⮳ to deserialize and serialize the rows to and from bytes.

See [`csv::Reader::deserialize`][c-csv::Reader::deserialize]{{hi:csv::Reader::deserialize}}⮳, [`serde::Deserialize`][c-serde::Deserialize]{{hi:serde::Deserialize}}⮳ and [`std::str::FromStr`][c-std::str::FromStr]{{hi:std::str::FromStr}}⮳.

```rust,editable
{{#include ../../../crates/ex/categories/efghijkl/tests/encoding/transform.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
