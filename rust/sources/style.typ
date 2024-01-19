#let sty(
  title: none,
  author: none,
  date: none,
  doc
) = {
  set page(
    paper: "us-letter",
    numbering: "1"
  )
  set text(
    size: 12pt,
    font: "New Computer Modern"
  )
  set heading(
    numbering: "1."
  )
  set par(
    leading: 0.55em, 
    first-line-indent: 1.8em, 
    justify: true
  )

  show heading: it => block(below: 1em, [#counter(heading).display() #it.body])

  align(center,[
  #text(22pt, title)\
  
  #author\
  #date
  #parbreak()])
  set align(left)
  doc
}