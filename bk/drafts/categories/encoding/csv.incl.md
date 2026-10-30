<!-- TODO Reviewed ??-??-2026 -->

| Recipe                                                                                 | Crates                          | Categories                        |
| -------------------------------------------------------------------------------------- | ------------------------------- | --------------------------------- |
| [Read CSV Records][ex~encoding~read-csv-records]                                       | {{#crate csv}}                  | {{#categories encoding=Encoding}} |
| [Read CSV Records with Different Delimiters][ex~encoding~read-csv-different-delimiter] | {{#crate csv}}                  | {{#categories encoding=Encoding}} |
| [Filter CSV Records][ex~encoding~filter-csv]                                           | {{#crate csv}}                  | {{#categories encoding=Encoding}} |
| [Handle Invalid CSV data with `serde`][ex~encoding~handle-invalid-csv]                 | {{#crate csv}} {{#crate serde}} | {{#categories encoding=Encoding}} |
| [Serialize Records to CSV][ex~encoding~serialize-to-csv]                               | {{#crate csv}}                  | {{#categories encoding=Encoding}} |
| [Serialize Records to CSV Using `serde`][ex~encoding~serialize-to-csv-using-serde]     | {{#crate csv}} {{#crate serde}} | {{#categories encoding=Encoding}} |
| [Transform a CSV Column][ex~encoding~transform-csv-column]                             | {{#crate csv}} {{#crate serde}} | {{#categories encoding=Encoding}} |

These recipes parse or produce inline data so they run without network access or external files. They use the [`csv`][c~csv~docs]↗{{hi:csv}} crate for CSV records and [`serde`][c~serde~docs]↗{{hi:serde}} when mapping records to typed Rust values.
