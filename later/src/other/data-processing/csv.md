<!-- TODO Reviewed ??-??-2026. -->

# CSV Files

{{#include csv.incl.md}}

## Read and Write CSV Files {#read-and-write-csv-files}

The [`csv`][c~csv~docs]↗{{hi:csv}} crate reads and writes records, while [`serde`][c~serde~docs]↗{{hi:serde}} maps records to typed Rust values. This example processes each input row as it is read, then writes and reads back one typed record. It uses in-memory bytes so it runs without fixture files or network access. To stream a large file with the same loop, pass its path to `Reader::from_path` or an open `File` to `Reader::from_reader`.

```rust,noplayground
{{#include ../../../crates/other/examples/data_processing/csv/csv.rs:example}}
```

From `later/crates`, run it with `cargo run --locked -p other --example csv`.

Use `Reader::records()` to handle fields by position. To select fields by name, resolve their positions from the header first. Use `Reader::deserialize()` when the CSV headers map cleanly to a struct. Both return iterators, so process rows as they arrive unless a later step needs to retain them.

For analysis that needs filtering, grouping, or joins across columns, load data into a dataframe rather than growing a collection of hand-written row loops; see [Dataframes](https://github.com/john-cd/rust_howto/blob/ceaf4b050e21e3981d121bb1414fc26b01e16a4b/bk/src/categories/data-processing/dataframes.md).

## Manipulate CSV Files from the Command Line {#manipulate-csv-files-from-the-command-line}

{{#badges qsv}}

[qsv][c~qsv~repo]↗{{hi:qsv}} is a fast, actively maintained {{i:CSV}} command line toolkit written in Rust. It started as a fork of `xsv` and keeps its command names, so it is the closest drop-in replacement.

[`qsv`][c~qsv~docs]↗{{hi:qsv}} can be used to index, slice, analyze, split and join CSV files: for example, `qsv index data.csv`, `qsv stats data.csv`, `qsv slice --len 10 data.csv`, `qsv split outdir data.csv`, and `qsv join id a.csv id b.csv`. It is a large program with many optional features: install it with `cargo install qsv --locked`, or download a prebuilt binary.

[![xan][c~xan~docs~badge]][c~xan~docs]{{hi:xan}}
[![xan~repo][c~xan~repo~badge]][c~xan~repo]

If you want a smaller tool with a built-in expression language, [`xan`][c~xan~repo]↗{{hi:xan}} is another maintained CSV toolkit in Rust (`cargo install xan --locked`). Its command set is similar (`select`, `slice`, `stats`, `split`, `join`, `search`), but it has no `index` command and its options are not identical to `xsv`'s.

`xsv` is no longer maintained: its [README][c~xsv~repo]↗ recommends qsv or xan, and its last crates.io release dates from 2018. Prefer one of the tools above.

## Related Topics {#related-topics .skip}

- [[data_processing | Data and ETL]].
- [Dataframes](https://github.com/john-cd/rust_howto/blob/ceaf4b050e21e3981d121bb1414fc26b01e16a4b/bk/src/categories/data-processing/dataframes.md).
- [[development_tools | Development Tools]].
- [[parsing | Parsing]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
