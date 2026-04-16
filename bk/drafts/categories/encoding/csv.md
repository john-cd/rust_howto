# CSV Processing

{{#include csv.incl.md}}

## Read CSV Records {#read-csv-records}

[![csv][c~csv~docs~badge]][c~csv~docs]{{hi:csv}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}}

Reads standard [CSV][p~csv] records into [`csv::StringRecord`][c~csv::StringRecord~docs]↗{{hi:csv::StringRecord}} - a weakly typed data representation which expects valid UTF-8 rows. Alternatively,
[`csv::ByteRecord`][c~csv::ByteRecord~docs]↗{{hi:csv::ByteRecord}} makes no assumptions about UTF-8.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/csv/read.rs:example}}
```

[`serde`][c~serde~docs]↗{{hi:serde}} deserializes data into strongly type structures. See the [`csv::Reader::deserialize`][c~csv::Reader::deserialize~docs]↗{{hi:csv::Reader::deserialize}} method.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/csv/read1.rs:example}}
```

## Read CSV Records with Different Delimiters {#read-csv-different-delimiter}

[![csv][c~csv~docs~badge]][c~csv~docs]{{hi:csv}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}}{{hi: Delimiter}}

Reads [CSV][p~csv] records with a tab [`csv::ReaderBuilder::delimiter`][c~csv::ReaderBuilder::delimiter~docs]↗{{hi:csv::ReaderBuilder::delimiter}}.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/csv/delimiter.rs:example}}
```

## Filter CSV Records {#filter-csv}

[![csv][c~csv~docs~badge]][c~csv~docs]{{hi:csv}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}}

Returns _only_ the rows from `data` with a field that matches `query`.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/csv/filter.rs:example}}
```

This example has been adapted from [the `csv` crate tutorial][c~csv~tutorial]↗.

## Handle Invalid CSV data with `serde` {#handle-invalid-csv}

[![csv][c~csv~docs~badge]][c~csv~docs]{{hi:csv}} [![serde][c~serde~docs~badge]][c~serde~docs]{{hi:serde}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}}

CSV files often contain invalid data{{hi:Invalid data}}. For these cases, the [`csv`][c~csv~docs]↗{{hi:csv}} crate provides a custom deserializer{{hi:Custom deserializer}}, [`csv::invalid_option`][c~csv::invalid_option~docs]↗{{hi:csv::invalid_option}} which automatically converts invalid data to [`std::option::Option::None`][c~std::option::Option::None~docs]↗{{hi:std::option::Option::None}} values.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/csv/invalid.rs:example}}
```

## Serialize Records to CSV {#serialize-to-csv}

[![csv][c~csv~docs~badge]][c~csv~docs]{{hi:csv}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}}

This example shows how to serialize a Rust tuple. [`csv::writer`][c~csv::Writer~docs]↗{{hi:csv::Writer}} supports automatic serialization{{hi:Serialization}} from Rust types into CSV records. [`csv::Writer::write_record`][c~csv::Writer::write_record~docs]↗{{hi:csv::Writer::write_record}} writes a simple record containing string data only. Data with more complex values such as numbers, floats, and options use [`csv::Writer::serialize`][c~csv::Writer::serialize~docs]↗{{hi:csv::Writer::serialize}}. Since CSV writer uses an internal buffer, always explicitly [`csv::Writer::flush`][c~csv::Writer::flush~docs]↗{{hi:csv::Writer::flush}} when done.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/csv/serialize.rs:example}}
```

## Serialize Records to CSV Using `serde` {#serialize-to-csv-using-serde}

[![csv][c~csv~docs~badge]][c~csv~docs]{{hi:csv}} [![serde][c~serde~docs~badge]][c~serde~docs]{{hi:serde}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}}

The following example shows how to serialize custom [structs][p~structs]{{hi:Serialize custom structs}} as CSV records using the [`serde`][c~serde~docs]↗{{hi:serde}} crate.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/csv/serde_serialize.rs:example}}
```

## Transform a CSV Column {#transform-csv-column}

[![csv][c~csv~docs~badge]][c~csv~docs]{{hi:csv}} [![serde][c~serde~docs~badge]][c~serde~docs]{{hi:serde}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}}

Transform a [CSV][p~csv] file containing a color name and a hex color into one with a color name and an rgb color. Utilizes the [`csv`][c~csv~docs]↗{{hi:csv}} crate to read and write the CSV file, and [`serde`][c~serde~docs]↗{{hi:serde}} to deserialize and serialize the rows to and from bytes.

See [`csv::Reader::deserialize`][c~csv::Reader::deserialize~docs]↗{{hi:csv::Reader::deserialize}}, [`serde::Deserialize`][c~serde::Deserialize~docs]↗{{hi:serde::Deserialize}} and [`std::str::FromStr`][c~std::str::FromStr~docs]↗{{hi:std::str::FromStr}}.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/csv/transform.rs:example}}
```

## Related Topics {#related-topics .skip}

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/928)
</div>
