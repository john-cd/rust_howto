// Base page and heading rules for the book-style Typst edition.
#set text(
  font: "Libertinus Serif",
  size: 10.5pt,
  lang: "en",
  fill: rgb("#242a31"),
)
#set par(
  justify: true,
  linebreaks: "optimized",
  spacing: 0.55em,
)
#set block(spacing: 0.7em)
#set heading(numbering: "1.1.")
#set page(margin: (x: 0.85in, y: 0.75in))

// Pandoc renders the title block with a larger, bold text element.
#show "The Rust How-to Book": set text(
  size: 28pt,
  fill: rgb("#203a54"),
)

#show heading.where(level: 1): it => {
  pagebreak(weak: true)
  set text(size: 19pt, weight: "bold", fill: rgb("#203a54"))
  set block(above: 0.8em, below: 0.5em)
  it
}
#show heading.where(level: 2): set text(size: 14pt, weight: "bold", fill: rgb("#315777"))
#show heading.where(level: 3): set text(size: 11.5pt, weight: "bold", fill: rgb("#3c4c5d"))
