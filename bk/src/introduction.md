<!-- TODO Reviewed ??-??-2026. -->

# Introduction

<div class="warning">
Work in Progress

Some chapters and examples are hidden while the book is being reorganized.

</div>

## What You Will Find Here

This book pairs Rust recipes with explanations of the language, standard library, and ecosystem. Use it to find working examples for common tasks and links to the documentation for each tool or crate.

## The Rust Programming Language, in a Few Words

Rust gives you control over memory layout and allocation. Its ownership and type systems prevent use-after-free errors, double frees, and data races in safe Rust.{{hi:Type safety}}{{hi:Memory safety}}{{hi:Concurrency}} Memory leaks{{hi:Memory leaks}} are still possible, for example when reference-counted values form a cycle.

The recipes cover language features such as pattern matching{{hi:Pattern matching}}, generics{{hi:Generics}}, and traits{{hi:Traits}}, alongside crates for tasks ranging from [algorithms][p~category-algorithms]{{hi:algorithms}} to [embedded development][cat~embedded]↗{{hi:embedded}}.

## Who Should Read This Book

This book is intended for:

- Experienced programmers, to locate the right crates for their projects, to find code examples for common programming tasks, and refresh their memory on the Rust language and standard library.
- New Rust programmers, to get an overview of the capabilities of the Rust ecosystem, learn best practices, and get pointers to learning resources.

This book is not intended for complete beginners. Readers should have already some basic familiarity with [Rust][rust-lang~website]↗ concepts. The [Rust Book][book~rust]↗{{hi:Rust book}} is an excellent resource to get started with. This said, most features of the language are summarized in this book's [language][p~lang] and [[standard_library | standard library]] sections.

## Why This Book

Rust relies on the [`crates.io`][crates.io~website]↗{{hi:crates.io}} ecosystem and the [`cargo`][c~cargo~docs]↗{{hi:cargo}} package manager for many libraries that other languages include in their standard libraries. As of October 2026, more than 360,000 crates were published on crates.io. This large selection can make it difficult for new Rust developers to decide what to use and trust; curated resources such as [`blessed.rs`][blessed-rs~website]↗ can help.

Furthermore, there are no dominant comprehensive platforms akin to `Rails`, `Django`, `Spring` or `Node` in the Rust world at this time - although several frameworks are first-of-mind in specific areas, such as asynchronous programming or web development.

Instead, Rust developers learn which crates to use, and how to use them, in an osmotic manner - the knowledge is passed down from a Rust expert to its team members; or gleaned from blogs, forums, or mailing lists. This is Rust's "second learning curve" after mastering the basics of the language and standard library.

This book therefore aims at reducing this learning curve by providing numerous **examples** demonstrating the uses of the **foundational crates** that are necessary for day-to-day Rust coding - examples which are absent from or may be scattered in the [reference documentation][docs.rs~website]↗ of hundreds of crates. It hopes to become a "cheat sheet on steroid" for the Rust ecosystem (_not just_ for the Rust language).

## What Else to Read

This book is a complement to similar works:

- [Rust by Example][book~rust-by-example~book]↗ is similar in concept - a collection of runnable examples - but not in scope, as it focuses solely on the Rust language and standard library.
- The [Rust Cookbook][book~rust-cookbook]↗ community project shares very similar goals with this book. While it is the official "cookbook" for Rust, its updates have been sporadic, and some of its examples no longer work. Several crates it references are no longer maintained. The author therefore decided to merge the (public domain) contents of the `Rust Cookbook` into this book and refresh its examples.

Refer to the [[books | books]] chapter for a detailed list of Rust books.

[p~category-algorithms]: categories/algorithms/index.md
[p~lang]: language/index.md
[p~books]: links/books.md
[p~standard_library]: standard-library/index.md

{{#include refs/link-refs.md}}
