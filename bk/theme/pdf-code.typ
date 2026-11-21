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
