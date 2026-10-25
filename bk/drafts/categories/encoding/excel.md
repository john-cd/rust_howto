<!-- TODO Reviewed ??-??-2026 -->

# Excel Spreadsheets

{{#include excel.incl.md}}

## Write an Excel Workbook {#write-xlsx}

[![rust_xlsxwriter][c~rust_xlsxwriter~docs~badge]][c~rust_xlsxwriter~docs]{{hi:rust_xlsxwriter}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}}{{hi:Excel}}{{hi:XLSX}}{{hi:Spreadsheet}}

[`rust_xlsxwriter::Workbook`][c~rust_xlsxwriter::Workbook~docs]↗{{hi:rust_xlsxwriter::Workbook}} creates a new `.xlsx` file. Add a worksheet, write cells by zero-based row and column, and call `save`. Reuse a [`rust_xlsxwriter::Format`][c~rust_xlsxwriter::Format~docs]↗{{hi:rust_xlsxwriter::Format}} for fonts and number formats.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/excel/xlsxwriter.rs:example}}
```

Points to remember:

- A date is a number of days since 1899-12-30. Write an [`rust_xlsxwriter::ExcelDateTime`][c~rust_xlsxwriter::ExcelDateTime~docs]↗{{hi:rust_xlsxwriter::ExcelDateTime}} and give the cell a date number format; otherwise Excel shows the raw serial number.
- A [`rust_xlsxwriter::Formula`][c~rust_xlsxwriter::Formula~docs]↗{{hi:rust_xlsxwriter::Formula}} is text. `rust_xlsxwriter` does not calculate it: Excel does when it opens the file. Set a cached result for tools that only read the file.

## Read an Excel Workbook {#read-xlsx}

[![calamine][c~calamine~docs~badge]][c~calamine~docs]{{hi:calamine}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}}{{hi:Excel}}{{hi:XLSX}}{{hi:Spreadsheet}}

[`calamine::open_workbook`][c~calamine::open_workbook~docs]↗{{hi:calamine::open_workbook}} opens `.xlsx`, `.xlsm`, `.xlsb`, `.xls` and `.ods` files. The [`calamine::Reader`][c~calamine::Reader~docs]↗{{hi:calamine::Reader}} trait gives access to the sheets. Each cell is a [`calamine::Data`][c~calamine::Data~docs]↗{{hi:calamine::Data}} value (`Int`, `Float`, `String`, `Bool`, `DateTime`, `Error` or `Empty`), so match on it instead of assuming a type.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/excel/calamine.rs:example}}
```

Points to remember:

- Excel stores every number as a floating-point value, so whole numbers are usually read as `Data::Float`. Convert explicitly when you need an integer.
- Dates are serial numbers, wrapped in `Data::DateTime`. Enable the `chrono` feature of `calamine` to convert them to `chrono` types.
- A formula cell yields its cached result. `worksheet_formula` returns the formula text. Neither recalculates anything.
- `worksheet_range` returns the used area only, so cell `(0, 0)` of the `Range` is not necessarily `A1`.

## Choosing a Crate {#choosing-a-crate}

| Need                         | Crate                                                                 |
| ---------------------------- | --------------------------------------------------------------------- |
| Read only                    | [`calamine`][c~calamine~docs]↗{{hi:calamine}}                         |
| Create a new file            | [`rust_xlsxwriter`][c~rust_xlsxwriter~docs]↗{{hi:rust_xlsxwriter}}    |
| Modify an existing file      | [`umya-spreadsheet`][c~umya-spreadsheet~docs]↗{{hi:umya-spreadsheet}} |
| Plain tabular text           | [`csv`][c~csv~docs]↗{{hi:csv}}, see [CSV Processing][p~csv-processing] |

## Related Topics {#related-topics .skip}

- [CSV Processing][p~csv-processing] for tabular text without the spreadsheet container.
- [Encoding index][p~encoding] for other formats.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
