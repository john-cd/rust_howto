// Keep inline code legible and distinguish code examples from prose.
#show raw: set text(font: "DejaVu Sans Mono")
#show raw.where(block: false): it => box(
  fill: rgb("#f0f3f7"),
  inset: (x: 2.5pt, y: 1pt),
  radius: 1.5pt,
  it,
)

// Highlighted examples use a quiet background and a subtle frame that remains
// visible in grayscale and across page breaks.
#show block.where(fill: rgb("#f4f6f9")): set block(
  width: 100%,
  breakable: true,
  fill: rgb("#f4f6f9"),
  stroke: 0.5pt + rgb("#c9d3df"),
  inset: (x: 6pt, y: 5pt),
  radius: 2pt,
)
#show block.where(fill: rgb("#f4f6f9")): set text(font: "DejaVu Sans Mono", size: 7.8pt)
#show block.where(fill: rgb("#f4f6f9")): set par(
  linebreaks: "simple",
  hanging-indent: 1.2em,
)

// Pandoc emits raw newlines between syntax-highlighted source lines. Typst
// otherwise treats them as spaces and can lay a whole example out as one long
// line. Preserve source breaks; soft wraps use the hanging indent above.
#let Skylighting(fill: none, number: false, start: 1, sourcelines) = {
  let blocks = []
  let line_number = start - 1
  let background = rgb("#f4f6f9")
  for source_line in sourcelines {
    if number {
      line_number = line_number + 1
      blocks += box(
        width: if start + sourcelines.len() > 999 { 30pt } else { 24pt },
        text(fill: rgb("#667085"), [#line_number]),
      )
    }
    blocks += source_line + linebreak()
  }
  block(fill: background, blocks)
}
