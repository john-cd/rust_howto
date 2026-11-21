// The draft watermark is decorative and excluded from the tagged content tree.
#set page(
  background: align(center + horizon, pdf.artifact(rotate(-35deg, text(
    64pt,
    weight: "bold",
    fill: rgb("#e9e9e9"),
    [DRAFT],
  )))),
  footer: context [
    #text(size: 8pt, fill: rgb("#626c77"), [Draft edition])
    #h(1fr)
    #text(size: 8pt, fill: rgb("#626c77"), counter(page).display())
  ],
)
