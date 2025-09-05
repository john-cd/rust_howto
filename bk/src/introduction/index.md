# Work In Progress

<div class="warning">
This book is going through HEAVY EDITS. Pardon the dust.

A large number of chapters and examples are currently hidden, as I am reorganizing the book.
</div>

## What You Will Find Here

This book is "everything you need for day-to-day Rust coding, in one place". It is a compendium of **code examples** and **resources** for the **Rust programming language** and its **ecosystem**.

It combines a summary of the language and of key features of the standard library; a "cookbook" with a large number of code examples to accomplish common programming tasks; and numerous links to Rust resources (books, blogs, example code, videos, meetings & events, etc).

In particular, it emphasizes coverage of **foundational Rust crates** (libraries) and tools, crates that are not part of the standard library but are very commonly used and that every experienced Rust developer should know - or at least know about.

## The Rust Programming Language, in a Few Words

You are likely reading this book, because you are aware that Rust is a modern programming language that offers high safety, reliability, and performance.

It emphasizes type safety{{hi:Type safety}}, memory safety{{hi:Memory safety}} (without a garbage collector), and fearless concurrency{{hi:Concurrency}} - that is, without code brittleness and refactoring fears. Rust is designed to prevent common errors such as memory leaks{{hi:Memory leaks}}, data races{{hi:Data races}}, and null pointer dereferences{{hi:Null pointer dereferences}}, by enforcing strict rules at compile time.

It is a language with both low-level control and high-level, yet zero-cost, abstractions, bridging the capabilities of, say, C or C++ with that of Python. No rewrite of prototypes in a "production" language is necessary. It supports powerful features such as pattern matching{{hi:Pattern matching}}, algebraic data types{{hi:Algebraic data types}}, higher-order functions{{hi:Higher-order functions}}, generics{{hi:Generics}}, traits{{hi:Traits}}, macros{{hi:Macros}}, and asynchronous programming{{hi:Async}}.

It compiles for many CPUs and operating systems (or no OS), making it suitable for a wide range of applications - from embedded development & systems programming to web development.

Above all, Rust's key strength is that it forces you - and assists you greatly in - writing high-quality code. Writing Rust can take more time upfront, but that translates into fewer flaky tests, fewer irreproducible bugs, fewer customer service tickets, fewer costly production environment failures (which, of course, happen in the middle of the night), fewer security breaches. Its rigor results in higher productivity overall.

It has also a very rich ecosystem of crates, for anything from algorithms to websockets - as we will discover in this book.

## Who Should Read This Book

This book is intended for:

- Experienced programmers, to locate the right crates for their projects, to find code examples for common programming tasks, and refresh their memory on the Rust language and standard library.
- New Rust programmers, to get an overview of the capabilities of the Rust ecosystem, learn best practices, and get pointers to learning resources.

This book is not intended for complete beginners. Readers should have already some basic familiarity with [Rust][rust-lang~website]↗ concepts. The [Rust Book][book~rust]↗{{hi:Rust book}} is an excellent resource to get started with. This said, most features of the language are summarized in this book's [language][p~lang] and [[standard_library | standard library]] sections.

## Why This Book

Per the curated list of Rust crates [`blessed.rs`][blessed-rs~website]↗, "the standard library in Rust is much smaller than in Python or Go, for example. Those languages come with "batteries included" support ... Rust, on the other hand, gets things like that from the [`crates.io`][crates.io~website]↗{{hi:crates.io}} ecosystem and the [`cargo`][c~cargo~docs]↗{{hi:cargo}} package manager. But with _more than 195 thousand crates_ (libraries) to choose from, a common complaint from new Rust developers is that they don't know where to start, which crates they ought to use, and which crates they ought to trust."

Furthermore, there are no dominant comprehensive frameworks or platforms akin to `Rails`, `Django`, `Spring` or `Node` in the Rust world at this time (although several are emerging in specific areas, such as asynchronous programming, web development, GUI...).

Instead, Rust developers learn which crates to use, and how to use them, in an osomotic manner - the knowledge is passed down from a Rust expert to its team members; or gleaned from blogs, forums, or mailing lists.

This book therefore aims at reducing this learning curve by providing numerous **examples** demonstrating the uses of the **foundational crates** that are necessary for day-to-day Rust coding - examples which are absent from or may be scattered in the [reference documentation][docs.rs~website]↗ of hundreds of crates. It hopes to become a "cheat sheet on steroid" for the Rust ECOSYSTEM (_not just_ for the Rust language).

This book is a complement to similar works:

- [Rust by Example][book~rust-by-example~book]↗ is similar in concept - a collection of runnable examples - but not in scope, as it focuses solely on the Rust language and standard library.
- The [Rust Cookbook][book~rust-cookbook]↗ community project shares very similar goals with this book. While it is the official "cookbook" for Rust, few updates have been made in the last 4 years. Many of its examples no longer work. Several crates it references are no longer maintained. The author therefore decided to merge the (public domain) contents of the `Rust Cookbook` into this book and refresh its examples.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
