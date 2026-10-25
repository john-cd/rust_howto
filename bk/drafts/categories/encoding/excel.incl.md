<!-- TODO Reviewed ??-??-2026 -->

| Recipe                                          | Crates                                                                                  | Categories                                          |
| ----------------------------------------------- | --------------------------------------------------------------------------------------- | --------------------------------------------------- |
| [Write an Excel Workbook][ex~encoding~write-xlsx] | [![rust_xlsxwriter][c~rust_xlsxwriter~docs~badge]][c~rust_xlsxwriter~docs]              | [![cat~encoding][cat~encoding~badge]][cat~encoding] |
| [Read an Excel Workbook][ex~encoding~read-xlsx]   | [![calamine][c~calamine~docs~badge]][c~calamine~docs]                                   | [![cat~encoding][cat~encoding~badge]][cat~encoding] |

[`calamine`][c~calamine~docs]↗{{hi:calamine}} only reads spreadsheets and [`rust_xlsxwriter`][c~rust_xlsxwriter~docs]↗{{hi:rust_xlsxwriter}} only writes `.xlsx` files. The recipes below write a workbook to a temporary directory and read it back, so they need no binary fixture. To modify an existing workbook in place, look at [`umya-spreadsheet`][c~umya-spreadsheet~docs]↗{{hi:umya-spreadsheet}}.
