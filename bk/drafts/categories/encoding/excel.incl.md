<!-- TODO Reviewed ??-??-2026 -->

| Recipe                                            | Crates                     | Categories                        |
| ------------------------------------------------- | -------------------------- | --------------------------------- |
| [Write an Excel Workbook][ex~encoding~write-xlsx] | {{#crate rust_xlsxwriter}} | {{#categories encoding=Encoding}} |
| [Read an Excel Workbook][ex~encoding~read-xlsx]   | {{#crate calamine}}        | {{#categories encoding=Encoding}} |

[`calamine`][c~calamine~docs]↗{{hi:calamine}} only reads spreadsheets and [`rust_xlsxwriter`][c~rust_xlsxwriter~docs]↗{{hi:rust_xlsxwriter}} only writes `.xlsx` files. The recipes below write a workbook to a temporary directory and read it back, so they need no binary fixture. To modify an existing workbook in place, look at [`umya-spreadsheet`][c~umya-spreadsheet~docs]↗{{hi:umya-spreadsheet}} or [`edit-xlsx`][c~edit-xlsx~docs]↗{{hi:edit-xlsx}}.
