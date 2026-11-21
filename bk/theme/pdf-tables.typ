// Use consistent padding, alignment, and restrained rules across book tables.
#set table(
  align: top,
  inset: (x: 5pt, y: 3pt),
  gutter: 1pt,
  stroke: (x, y) => if y == 0 {
    (bottom: 0.8pt + rgb("#718096"))
  } else {
    (bottom: 0.35pt + rgb("#d8dee7"))
  },
)
#show table: set text(size: 9pt)
#show table.cell: set text(size: 9pt)
#show table.cell: set par(justify: false)
#show table.cell.where(y: 0): set table.cell(fill: rgb("#edf1f6"))
#show table.cell.where(y: 0): set text(weight: "bold", fill: rgb("#293e53"))

// Pandoc may wrap a large table in a figure. Keep the wrapper breakable so its
// content can flow through the page area.
#show figure: set block(breakable: true)
