# What you will find here

This book is **a compendium of Rust ecosystem examples, recipes, and links**. It is intended to be **everything you need for day-to-day Rust coding, in one place**.

It quickly summarizes the basics of the [language](lang/rust_language.md) and often-used elements of the [standard library](std/standard_library.md).

It then focuses on [cross-cutting concerns](concerns/cross_cutting_concerns.md) that affect most aspects of development e.g. [error handling](concerns/error_handling.md), [error customization](concerns/error_customization.md), [configuration](concerns/configuration.md), [logging](concerns/logging.md)...

[Concurrency](concurrency/concurrency.md), including [asynchronous programming](concurrency/async.md), are covered in details.

Next are [tools](tools/tools.md), such as Cargo, Clippy, Rustfmt, as well as links and examples specific to programming [domains](domains/domains.md) such as [CLI](domains/cli.md) and [Web](domains/web.md) development. The [links](links/links.md) section provides pointers to notable Rust websites, books, and code examples.

## Who should read this book?

This book is intended both for new Rust programmers (to get an overview of the capabilities of the Rust ecosystem and pointers to other resources) and for experienced programmers (to find code examples and review best practices for common programming tasks).

Readers should have already some basic familiarity with [Rust][rust]⮳ concepts. The [Rust book][rust-book]⮳ is an excellent resource for complete beginners to get started with.

## Why this book?

Per the curated list of Rust crates [blessed.rs][blessed-rs]⮳, "the standard library in Rust is much smaller than in Python or Go, for example. Those languages come with "batteries included" support ... Rust, on the other hand, gets things like that from the `crates.io` ecosystem and the `Cargo` package manager. But with ~~almost~~ _more than_ 100 thousand crates to choose from, a common complaint from new Rust developers is that they don't know where to start, which crates they ought to use, and which crates they ought to trust." There are (not yet) dominant frameworks or platforms like `Rails`, `Django`, `Spring` or `Node` in the Rust world.

This book intends to provide EXAMPLES to demonstrate the use of key Rust crates, examples which are absent from or scattered in the typically dry [reference docs][reference-docs]⮳, and hopes to become a "cheatsheet on steroid" for the Rust ecosystem (_not just_ the Rust language).

## What other books should I consult?

[Rust by Example][rust-by-example]⮳ is similar in concept - a collection of runnable examples - but not in scope, as it focuses solely on the Rust language and standard libraries.

The [Rust cookbook][rust-cookbook]⮳ demonstrate good practices to accomplish common programming tasks, using the crates of the Rust ecosystem. It focuses mainly on `std` and a few core crates.

## Call for contributions

This book is in its **early days** - feel free to submit an issue or a pull request to the [repo][repo].

Contributions, from small edits to whole chapters, are most welcome. Draft pages are kept in [this folder][drafts]. An informal (and very long) list of topics of interest is kept in [TODO][todo]. Embedded examples should be ideally _runnable_ on the [Rust playground][rust-playground]⮳ or at least directly copy-pastable into Rust code. Please read [CONTRIBUTING.md][contributing] for more details.

Its long-term goal is the coverage of the 'most commonly used' Rust crates, as defined by [blessed.rs][blessed-rs]⮳, the most downloaded libraries in [crates.io][crates-io]⮳, and 'high quality crates' per [lib.rs][lib-rs]⮳ [statistics][statistics]⮳. Review [essential crates](links/essential_crates.md) for topic ideas.

This site is not affiliated with the [Rust Foundation][rust-foundation]⮳.

[blessed-rs]: https://blessed.rs/crates
[contributing]: https://github.com/john-cd/rust_howto/blob/main/CONTRIBUTING.md
[crates-io]: https://crates.io/
[drafts]: https://github.com/john-cd/rust_howto/tree/main/drafts
[lib-rs]: https://lib.rs/
[reference-docs]: https://docs.rs/
[repo]: https://github.com/john-cd/rust_howto
[rust]: https://www.rust-lang.org/
[rust-book]: https://doc.rust-lang.org/book/
[rust-by-example]: https://doc.rust-lang.org/rust-by-example/index.html
[rust-cookbook]: https://rust-lang-nursery.github.io/rust-cookbook/
[rust-foundation]: https://foundation.rust-lang.org/
[rust-playground]: https://play.rust-lang.org/
[statistics]: https://lib.rs/stats
[todo]: https://github.com/john-cd/rust_howto/blob/main/TODO.md
{{#include ./links.md}}
