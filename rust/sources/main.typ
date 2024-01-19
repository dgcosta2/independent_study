#import "style.typ": sty
#show: doc => sty(
  title: "Rust Sources - Notes",
  author: "Diego Souza Gomes da Cruz Costa",
  date: datetime.today().display("[month repr:long] [day], [year]"),
  doc
)

= The Book

- url: #link("https://doc.rust-lang.org/book/")

Seems to be a reliable and comprehensive source on Rust. The information on #link("https://doc.rust-lang.org/book/ch16-01-threads.html", "threads") and #link("https://doc.rust-lang.org/book/ch13-01-closures.html", "closures") were important for understanding how to implement a multi-threading sorting algorithm.\

The concept of ownership is well explained in #link("https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html", "its own chapter"). It is also the first chapter to go over lower-level implementations on `Rust` (Stack and Heap usage).