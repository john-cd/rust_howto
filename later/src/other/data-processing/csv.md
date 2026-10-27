<!-- TODO Reviewed ??-??-2026 -->

# CSV Files

{{#include csv.incl.md}}

## Read and Write CSV Files {#read-and-write-csv-files}

Reading, writing, filtering, transforming and (de)serializing CSV records with [`csv`][c~csv~docs]↗{{hi:csv}} and [`serde`][c~serde~docs]↗{{hi:serde}} are covered in the [draft CSV recipes][p~csv]↗ in the book repository, so this page does not repeat them.

For data processing, keep two points in mind:

- **Stream large files.** `csv::Reader::records()` and `deserialize()` return iterators that process records incrementally. Memory still depends on the largest record and on anything your application retains; avoid collecting every row into a `Vec` unless needed.
- **Load into a frame for analysis.** To filter, group or join CSV data, read it into a dataframe instead of hand-written loops: see [[dataframes | Dataframes]].

## Manipulate CSV Files from the Command Line {#manipulate-csv-files-from-the-command-line}

[![xsv][c~xsv~docs~badge]][c~xsv~docs]{{hi:xsv}}
[![xsv~crates.io][c~xsv~crates.io~badge]][c~xsv~crates.io]
[![xsv~repo][c~xsv~repo~badge]][c~xsv~repo]
[![xsv~lib.rs][c~xsv~lib.rs~badge]][c~xsv~lib.rs]

[xsv][c~xsv~repo]↗{{hi:xsv}} is a fast {{i:CSV}} command line toolkit written in Rust.

[`xsv`][c~xsv~docs]↗{{hi:xsv}} can be used to index, slice, analyze, split and join CSV files.

## Related Topics {#related-topics .skip}

- [[data_processing | Data and ETL]].
- [[dataframes | Dataframes]].
- [[development_tools | Development Tools]].
- [[parsing | Parsing]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
