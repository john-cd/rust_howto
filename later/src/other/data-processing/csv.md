<!-- TODO Reviewed ??-??-2026 -->

# CSV Files

{{#include csv.incl.md}}

## Read and Write CSV Files {#read-and-write-csv-files}

Reading, writing, filtering, transforming and (de)serializing CSV records with [`csv`][c~csv~docs]↗{{hi:csv}} and [`serde`][c~serde~docs]↗{{hi:serde}} are covered in the [draft CSV recipes][p~csv]↗ in the book repository, so this page does not repeat them.

For data processing, keep two points in mind:

- **Stream large files.** `csv::Reader::records()` and `deserialize()` return iterators that process records incrementally. Memory still depends on the largest record and on anything your application retains; avoid collecting every row into a `Vec` unless needed.
- **Load into a frame for analysis.** To filter, group or join CSV data, read it into a dataframe instead of hand-written loops: see [[dataframes | Dataframes]].

## Manipulate CSV Files from the Command Line {#manipulate-csv-files-from-the-command-line}

[![qsv][c~qsv~docs~badge]][c~qsv~docs]{{hi:qsv}}
[![qsv~crates.io][c~qsv~crates.io~badge]][c~qsv~crates.io]
[![qsv~repo][c~qsv~repo~badge]][c~qsv~repo]
[![qsv~lib.rs][c~qsv~lib.rs~badge]][c~qsv~lib.rs]

[qsv][c~qsv~repo]↗{{hi:qsv}} is a fast, actively maintained {{i:CSV}} command line toolkit written in Rust. It started as a fork of `xsv` and keeps its command names, so it is the closest drop-in replacement.

[`qsv`][c~qsv~docs]↗{{hi:qsv}} can be used to index, slice, analyze, split and join CSV files: for example, `qsv index data.csv`, `qsv stats data.csv`, `qsv slice --len 10 data.csv`, `qsv split outdir data.csv`, and `qsv join id a.csv id b.csv`. It is a large program with many optional features: install it with `cargo install qsv --locked`, or download a prebuilt binary.

[![xan][c~xan~docs~badge]][c~xan~docs]{{hi:xan}}
[![xan~repo][c~xan~repo~badge]][c~xan~repo]

If you want a smaller tool with a built-in expression language, [`xan`][c~xan~repo]↗{{hi:xan}} is another maintained CSV toolkit in Rust (`cargo install xan --locked`). Its command set is similar (`select`, `slice`, `stats`, `split`, `join`, `search`), but it has no `index` command and its options are not identical to `xsv`'s.

`xsv` is no longer maintained: its [README][c~xsv~repo]↗ recommends qsv or xan, and its last crates.io release dates from 2018. Prefer one of the tools above.

## Related Topics {#related-topics .skip}

- [[data_processing | Data and ETL]].
- [[dataframes | Dataframes]].
- [[development_tools | Development Tools]].
- [[parsing | Parsing]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
