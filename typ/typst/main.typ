#import "@preview/ilm:1.4.1": *

#set text(lang: "en")

#show: ilm.with(
  paper-size: "us-letter", // #link("https://typst.app/docs/reference/layout/page#parameters-paper")[paper size string]
  title: [The Rust How-to Book],
  author: "John CD",
  // date: datetime(year: 2025, month: 08, day: 20),
  abstract: [
        #include "other/abstract.typ"
  ],
  preface: [ #include "other/preface.typ" ],
  // table-of-contents: outline(title: "Custom Title"),
  // chapter-pagebreak: false,
  appendix: (
    enabled: true,
    // title: "Appendix", // optional
    // heading-numbering-format: "A.1.1.", // optional
    body: [#include "other/appendix.typ"],
  ),
  // external-link-circle: false,
  bibliography: bibliography("refs.bib"), // See Typst's #link("https://typst.app/docs/reference/model/bibliography/")[official documentation] for more information.
  figure-index: (
    enabled: true,
    // title: "Index of Figures" // optional
  ),
  table-index: (
    enabled: true,
    // title: "Index of Tables" // optional
  ),
  listing-index: (
    enabled: true,
    // title: "Index of Listings" // optional
  ),
)
// 'Ilm display's its content in the following order:
// + Cover page
// + Preface page (if defined)
// + Table of contents (unless disabled)
// + Body (your main content)
// + Appendix (if defined)
// + Bibliography (if defined)
// + Indices (if enabled) --- index of figures (images), tables, or listings (code blocks)
//
// By default, the date is shown in the format: `MMMM DD, YYYY`. You can change the date format by specifying a different format string:
// ```typst
// #show: ilm.with(
//   date-format: "[month repr:long] [day padding:zero], [year repr:full]",
// )
// ```
// See Typst's #link("https://typst.app/docs/reference/foundations/datetime/#format")[official documentation] for more info on how date format strings are defined.
//
// The `table-of-contents` option accepts the result of a call to the `outline()` function, so if you want to customize the behavior of table of contents then you can specify a custom `outline()` function:
// See Typst's #link("https://typst.app/docs/reference/model/outline/")[official documentation] for more information.
//
// 'Ilm also exports a `blockquote` function which can be used to create blockquotes. The function has one argument: `body` of the type content and can be used like so:
//
// ```typst
// #blockquote[
//  A wizard is never late, Frodo Baggins. Nor is he early. He arrives precisely when he means to.
//  --- Gandalf
// ]
// ```
//
// 'Ilm also exports functions for styling text in small caps and uppercase, namely: `smallcaps` and `upper` respectively.
// These functions will overwrite the standard #link("https://typst.app/docs/reference/text/smallcaps/")[`smallcaps`] and #link("https://typst.app/docs/reference/text/upper/")[`upper`] functions that Typst itself provides. This behavior is intentional as the functions that 'Ilm exports fit in better with the rest of the template's styling.
//
// #smallcaps[smallcaps] and #upper[upper]
//
// = Code
// #let snip(cap) = figure(caption: cap)[
//  ```rust
//  fn main() {
//      let user = ("Adrian", 38);
//      println!("User {} is {} years old", user.0, user.1);
//
//      // tuples within tuples
//      let employee = (("Adrian", 38), "die Mobiliar");
//      println!("User {} is {} years old and works for {}", employee.0.1, employee.0.1, employee.1);
//  }
//  ```
// ]
// #snip("Code snippet typeset in Fira Mono font")

#import "template.typ": *

#include "chapters/chapter1.typ"
