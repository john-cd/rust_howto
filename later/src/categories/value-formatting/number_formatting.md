<!-- TODO Reviewed ??-??-2026 -->

# Format Values for Display to a User, Potentially Adapting the Display to Various Languages and Regions

[![cat~value-formatting][cat~value-formatting~badge]][cat~value-formatting]{{hi:Value formatting}}

{{#include number_formatting.incl.md}}

## Convert Floating Point to String Quickly with `ryu` {#ryu}

[![ryu][c~ryu~docs~badge]][c~ryu~docs] [![ryu~crates.io][c~ryu~crates.io~badge]][c~ryu~crates.io] [![ryu~repo][c~ryu~repo~badge]][c~ryu~repo] [![ryu~lib.rs][c~ryu~lib.rs~badge]][c~ryu~lib.rs]{{hi:ryu}}{{hi:Float}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~no-std::no-alloc][cat~no-std::no-alloc~badge]][cat~no-std::no-alloc]{{hi:No dynamic allocation}} [![cat~value-formatting][cat~value-formatting~badge]][cat~value-formatting]{{hi:Value formatting}}

Fast floating point to string conversion.

```rust,editable
{{#include ../../../crates/cats/value_formatting/examples/number_formatting/ryu.rs:example}}
```

## Convert Identifiers Between Case Styles {#case-conversion}

[`heck`][c~heck~docs]↗{{hi:heck}} converts identifiers between common case conventions.

```rust,editable
{{#include ../../../crates/cats/value_formatting/examples/case_conversion/case_conversion.rs:example}}
```
## Convert Integers to String Quickly with `itoa` {#itoa}

[![itoa][c~itoa~docs~badge]][c~itoa~docs] [![itoa~crates.io][c~itoa~crates.io~badge]][c~itoa~crates.io] [![itoa~repo][c~itoa~repo~badge]][c~itoa~repo] [![itoa~lib.rs][c~itoa~lib.rs~badge]][c~itoa~lib.rs]{{hi:itoa}}{{hi:Integer}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~no-std::no-alloc][cat~no-std::no-alloc~badge]][cat~no-std::no-alloc]{{hi:No dynamic allocation}} [![cat~value-formatting][cat~value-formatting~badge]][cat~value-formatting]{{hi:Value formatting}}

Fast integer primitive to string conversion.

## Format a Byte Count for Display with `humansize` {#humansize}

[![humansize][c~humansize~docs~badge]][c~humansize~docs]{{hi:humansize}} [![humansize~crates.io][c~humansize~crates.io~badge]][c~humansize~crates.io] [![humansize~repo][c~humansize~repo~badge]][c~humansize~repo] [![humansize~lib.rs][c~humansize~lib.rs~badge]][c~humansize~lib.rs]  [![cat~value-formatting][cat~value-formatting~badge]][cat~value-formatting]

[`humansize::format_size`][c~humansize~docs]↗{{hi:humansize}} formats a number of bytes as text. `DECIMAL` uses powers of 1000 (`kB`, `MB`), and `BINARY` uses powers of 1024 (`KiB`, `MiB`). `format_size_i` accepts signed values, such as a change in memory use.

```rust,editable
{{#include ../../../crates/cats/value_formatting/examples/number_formatting/humansize_example.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[internationalization | Internationalization]].
- [[localization | Localization]].
- [[text-processing | Text Processing]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[value-formatting: choose crates, write](https://github.com/john-cd/rust_howto/issues/490)
cover [`num-format`][c~num-format~docs]↗{{hi:num-format}}, [`icu`][c~icu~docs]↗{{hi:icu}}, [`fluent`][c~fluent~docs]↗{{hi:fluent}}
</div>
