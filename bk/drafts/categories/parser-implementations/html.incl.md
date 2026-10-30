<!-- TODO Reviewed ??-??-2026 -->

Choose the parser layer that matches the task: `html5ever` handles browser-grade HTML5 parsing, `scraper` offers a tree and CSS selectors for extracting data, and `cssparser` parses CSS syntax itself. `tl` is another fast, pure-Rust option for working with an HTML tree.

| Recipe                                                             | Crates                                                           | Categories                                                      |
| ------------------------------------------------------------------ | ---------------------------------------------------------------- | --------------------------------------------------------------- |
| [Parse HTML with `html5ever`][ex~parser-implementations~html5ever] | {{#crate html5ever}}                                             | {{#categories parser-implementations="Parser implementations"}} |
| [Parse HTML with `tl`][ex~parser-implementations~tl]               | {{#crate tl}}                                                    | {{#categories parser-implementations="Parser implementations"}} |
| [Parse CSS with `cssparser`][ex~parser-implementations~css-rs]     | {{#crate cssparser}}                                             | {{#categories parser-implementations="Parser implementations"}} |
| [`scraper`][ex~parser-implementations~scraper]                     | [![scraper][c~scraper~docs~badge]][c~scraper~docs]{{hi:scraper}} | {{#categories parser-implementations="Parser implementations"}} |
