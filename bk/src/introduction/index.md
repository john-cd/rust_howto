# Work In Progress

<div class="warning">
This book is going through HEAVY EDITS. Pardon the dust.

A large number of chapters and examples are currently hidden, as we are reorganizing the book.
</div>

## What You Will Find Here

This book is "everything you need for day-to-day Rust coding, in one place". It is a compendium of **code examples** and **resources** for the **Rust programming language** and its **ecosystem**.

In particular, it provides expansive coverage of "key" or "foundational" Rust crates (libraries) and tools that are used , and that the rest of the ecosystem builds off of.

This book combines a summary of the language and key features of the standard library; a "cookbook" with a large number of code examples to accomplish common programming tasks; and numerous links to Rust resources (books, blogs, example code, videos, meetings & events, etc).

- It helps you select the proper crates (Rust libraries) for your project,
- It includes TODO

## The Rust Programming Language, in a Few Words

Rust is a modern programming language that offers high performance, reliability, and productivity. It is designed to prevent common errors such as memory leaks{{hi:Memory leaks}}, data races{{hi:Data races}}, and null pointer dereferences{{hi:Null pointer dereferences}}, by enforcing strict rules at compile time. Rust supports powerful features such as generics{{hi:Generics}}, traits{{hi:Traits}}, macros{{hi:Macros}}, and concurrency{{hi:Concurrency}}, making it suitable for a wide range of applications.

## Who Should Read This Book

This book is intended for:

- Experienced programmers, to find code examples for common programming tasks, and refresh their memory on the Rust language and standard library.
- New Rust programmers, to get an overview of the capabilities of the Rust ecosystem, learn best practices, and get pointers to learning resources.

This book is not intended for complete beginners. Readers should have already some basic familiarity with [Rust][rust-lang~website]↗ concepts. The [Rust Book][book~rust]↗{{hi:Rust book}} is an excellent resource to get started with. This said, most features of the language are summarized in this book's [language][p~lang] and [[standard_library | standard library]] sections.

## Why This Book

Per the curated list of Rust crates [`blessed.rs`][blessed-rs~website]↗, "the standard library in Rust is much smaller than in Python or Go, for example. Those languages come with "batteries included" support ... Rust, on the other hand, gets things like that from the [`crates.io`][crates.io~website]↗{{hi:crates.io}} ecosystem and the [`cargo`][c~cargo~docs]↗{{hi:cargo}} package manager. But with _more than 180 thousand crates_ (libraries) to choose from, a common complaint from new Rust developers is that they don't know where to start, which crates they ought to use, and which crates they ought to trust."

There are no dominant frameworks or platforms akin to `Rails`, `Django`, `Spring` or `Node` in the Rust world at this time. Instead, Rust developers, as they gain experience, learn about TODO

"key" or "foundational" crates, crates that are not part of the standard library but are very commonly used.

This book therefore provides EXAMPLES demonstrating the uses of the KEY CRATES necessary for day-to-day Rust coding - examples which are absent from or may be scattered in the [reference documentation][docs.rs~website]↗ of hundreds of crates. It hopes to become a "cheat sheet on steroid" for the Rust ECOSYSTEM (_not just_ for the Rust language).

This book is a complement to similar works:

- [Rust by Example][book~rust-by-example~book]↗ is similar in concept - a collection of runnable examples - but not in scope, as it focuses solely on the Rust language and standard library.
- The [Rust Cookbook][book~rust-cookbook]↗ community project shares very similar goals with this book. While it is the official "cookbook" for Rust, few updates have been made in the last 4 years. Many of its examples no longer work. Several crates it references are no longer maintained. The author therefore decided to merge the contents of the `Rust Cookbook` into this book and refresh its examples.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[polish](https://github.com/john-cd/rust_howto/issues/536)
</div>
