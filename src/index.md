# Work In Progress

This book is still going through _heavy edits_. Pardon the dust.

## What you will find here

This book is **a compendium of Rust ecosystem examples, recipes, and links**. It is intended to be **everything you need for day-to-day Rust coding, in one place**.

It quickly summarizes the basics of the [language][lang] and often-used elements of the [standard library][standard-library].

It then focuses on cross-cutting concerns that affect most aspects of development e.g. [error handling][errors], [error customization][error-customization], [configuration][config], [logging][logging]...

[Concurrency][concurrency], including [asynchronous programming][asynchronous], are covered in details.

Next are [tools][development-tools], such as Cargo, Clippy, Rustfmt, as well as links and examples specific to programming domains such as [CLI][cli] and [Web][web-programming] development. The [links][links] section provides pointers to notable Rust websites, books, and code examples.

## Who should read this book?

This book is intended both for new Rust programmers (to get an overview of the capabilities of the Rust ecosystem and pointers to other resources) and for experienced programmers (to find code examples and review best practices for common programming tasks).

Readers should have already some basic familiarity with [`Rust`][rust]{{hi:Rust}}⮳ concepts. The [`Rust book`][book-rust]{{hi:Rust book}}⮳ is an excellent resource for complete beginners to get started with.

## Why this book?

Per the curated list of Rust crates [blessed.rs][blessed-rs-website]⮳, "the standard library in Rust is much smaller than in Python or Go, for example. Those languages come with "batteries included" support ... Rust, on the other hand, gets things like that from the `crates.io` ecosystem and the `Cargo` package manager. But with ~~almost~~ _more than_ 100 thousand crates to choose from, a common complaint from new Rust developers is that they don't know where to start, which crates they ought to use, and which crates they ought to trust." There are (not yet) dominant frameworks or platforms like `Rails`, `Django`, `Spring` or `Node` in the Rust world.

This book intends to provide EXAMPLES to demonstrate the use of key Rust crates, examples which are absent from or scattered in the [reference docs][docs-rs]⮳, and hopes to become a "cheat sheet on steroid" for the Rust ecosystem (_not just_ for the Rust language).

## What other books should I consult?

The [Rust cookbook][book-rust-cookbook]⮳ demonstrate good practices to accomplish common programming tasks, using the crates of the Rust ecosystem. It focuses mainly on `std` and a few core crates. The _Rust Cookbook_ is a collection of simple examples that demonstrate good practices to accomplish common programming tasks, using the crates of the Rust ecosystem.

[Rust by Example][book-rust-by-example-book]⮳ is similar in concept - a collection of runnable examples - but not in scope, as it focuses solely on the Rust language and standard libraries.

[asynchronous]: categories/asynchronous/index.md
[cli]: categories/command-line-interface/index.md
[concurrency]: categories/concurrency/index.md
[config]: categories/rust-patterns/errors/error_customization.md
[development-tools]: categories/development-tools/index.md
[errors]: categories/rust-patterns/errors/index.md
[error-customization]: categories/rust-patterns/errors/error_customization.md
[lang]: lang/index.md
[links]: links/index.md
[logging]: categories/development-tools_debugging/index.md
[standard-library]: standard_library/index.md
[web-programming]: categories/web-programming/index.md
{{#include refs/link-refs.md}}
<div class="hidden">
TODO: rewrite the intro
</div>
