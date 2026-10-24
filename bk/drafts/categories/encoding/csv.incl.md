<!-- TODO Reviewed ??-??-2026 -->

| Recipe                                                                                 | Crates                                                                              | Categories                                          |
| -------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- | --------------------------------------------------- |
| [Read CSV Records][ex~encoding~read-csv-records]                                       | [![csv][c~csv~docs~badge]][c~csv~docs]                                              | [![cat~encoding][cat~encoding~badge]][cat~encoding] |
| [Read CSV Records with Different Delimiters][ex~encoding~read-csv-different-delimiter] | [![csv][c~csv~docs~badge]][c~csv~docs]                                              | [![cat~encoding][cat~encoding~badge]][cat~encoding] |
| [Filter CSV Records][ex~encoding~filter-csv]                                           | [![csv][c~csv~docs~badge]][c~csv~docs]                                              | [![cat~encoding][cat~encoding~badge]][cat~encoding] |
| [Handle Invalid CSV data with `serde`][ex~encoding~handle-invalid-csv]                 | [![csv][c~csv~docs~badge]][c~csv~docs] [![serde][c~serde~docs~badge]][c~serde~docs] | [![cat~encoding][cat~encoding~badge]][cat~encoding] |
| [Serialize Records to CSV][ex~encoding~serialize-to-csv]                               | [![csv][c~csv~docs~badge]][c~csv~docs]                                              | [![cat~encoding][cat~encoding~badge]][cat~encoding] |
| [Serialize Records to CSV Using `serde`][ex~encoding~serialize-to-csv-using-serde]     | [![csv][c~csv~docs~badge]][c~csv~docs] [![serde][c~serde~docs~badge]][c~serde~docs] | [![cat~encoding][cat~encoding~badge]][cat~encoding] |
| [Transform a CSV Column][ex~encoding~transform-csv-column]                             | [![csv][c~csv~docs~badge]][c~csv~docs] [![serde][c~serde~docs~badge]][c~serde~docs] | [![cat~encoding][cat~encoding~badge]][cat~encoding] |

These recipes parse or produce inline data so they run without network access or external files. They use the [`csv`][c~csv~docs]↗{{hi:csv}} crate for CSV records and [`serde`][c~serde~docs]↗{{hi:serde}} when mapping records to typed Rust values.
