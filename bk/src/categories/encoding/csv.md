<!-- TODO Reviewed ??-??-2026. -->

# CSV Processing

{{#include csv.incl.md}}

## Read CSV Records {#read-csv-records}

[![csv][c~csv~docs~badge]][c~csv~docs]{{hi:csv}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}}

Reads standard [CSV][p~csv] records into [`csv::StringRecord`][c~csv::StringRecord~docs]↗{{hi:csv::StringRecord}} - a weakly typed data representation which expects valid UTF-8 rows. Alternatively,
[`csv::ByteRecord`][c~csv::ByteRecord~docs]↗{{hi:csv::ByteRecord}} makes no assumptions about UTF-8.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/csv/read.rs:example}}
```

[`serde`][c~serde~docs]↗{{hi:serde}} deserializes data into strongly typed structures. See the [`csv::Reader::deserialize`][c~csv::Reader::deserialize~docs]↗{{hi:csv::Reader::deserialize}} method.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/csv/read1.rs:example}}
```

## Stream and Round-Trip Typed CSV Records {#read-and-write-csv-files}

The [`csv`][c~csv~docs]↗{{hi:csv}} crate reads and writes records, while [`serde`][c~serde~docs]↗{{hi:serde}} maps records to typed Rust values. This example processes each input row as it is read, then writes and reads back one typed record. It uses in-memory bytes so it runs without fixture files or network access. To stream a large file with the same loop, pass its path to `Reader::from_path` or an open `File` to `Reader::from_reader`.

```rust,ignore,noplayground
{{#include ../../../crates/cats/encoding/examples/csv/stream_and_round_trip.rs:example}}
```

From `bk/crates`, run `cargo run --locked -p encoding --example csv -- stream-and-round-trip`.

Use `Reader::records()` to handle fields by position. To select fields by name, resolve their positions from the header first. Use `Reader::deserialize()` when the CSV headers map cleanly to a struct. Both return iterators, so process rows as they arrive unless a later step needs to retain them.

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

This example shows how to serialize a Rust tuple. [`csv::Writer`][c~csv::Writer~docs]↗{{hi:csv::Writer}} supports automatic serialization{{hi:Serialization}} from Rust types into CSV records. [`csv::Writer::write_record`][c~csv::Writer::write_record~docs]↗{{hi:csv::Writer::write_record}} writes a simple record containing string data only. Data with more complex values such as numbers, floats, and options use [`csv::Writer::serialize`][c~csv::Writer::serialize~docs]↗{{hi:csv::Writer::serialize}}. Since CSV writer uses an internal buffer, always explicitly [`csv::Writer::flush`][c~csv::Writer::flush~docs]↗{{hi:csv::Writer::flush}} when done.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/csv/serialize.rs:example}}
```

## Serialize Records to CSV Using `serde` {#serialize-to-csv-using-serde}

[![csv][c~csv~docs~badge]][c~csv~docs]{{hi:csv}} [![serde][c~serde~docs~badge]][c~serde~docs]{{hi:serde}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}}

The following example shows how to serialize custom [structs](../../language/structs.md){{hi:Serialize custom structs}} as CSV records using the [`serde`][c~serde~docs]↗{{hi:serde}} crate.

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

## Manipulate CSV Files from the Command Line {#manipulate-csv-files-from-the-command-line}

{{#badges qsv}}

Use [`qsv`][c~qsv~repo]↗{{hi:qsv}} to inspect or reshape a file from the shell. For example, `qsv headers data.csv` lists the column names, `qsv stats data.csv` summarizes the columns, and `qsv slice --len 10 data.csv` writes the first ten data rows with the header. See the project's installation instructions for prebuilt binaries.

[`xan`][c~xan~repo]↗{{hi:xan}} also supports CSV inspection, joins, and filtering with expressions. Both tools descend from [`xsv`][c~xsv~repo]↗. Check the selected tool's help before reusing older scripts, since flags and commands can differ.

## Related Topics {#related-topics .skip}

- [Serde][c~serde~docs]↗{{hi:serde}} for mapping CSV fields to typed Rust data.
- [DataFrames](../../other/data-processing/dataframes.md) for CSV analytics with Polars.
- [Data Engineering](../../other/data-processing/data_engineering.md#arrow-csv) for reading CSV into typed Arrow columns.
- [Encoding index][p~encoding] for text, binary, and format-specific serialization guidance.

See [Third-Party Credits](../../appendices/third_party_credits.md) for source and license information.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
