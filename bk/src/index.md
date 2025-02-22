# Work In Progress

This book is still going through _heavy edits_. Pardon the dust.

## What you will find here

This book is **a compendium of Rust ecosystem examples and resources**. It is intended to be **everything you need for day-to-day Rust coding, in one place**. It demonstrates good practices to accomplish common programming tasks, using the crates of the Rust ecosystem. It summarizes the language and key features of the standard library. It includes numerous links to Rust resources.

## Who should read this book

This book is intended for

- new Rust programmers, to get an overview of the capabilities of the Rust ecosystem and pointers to other resources,
- experienced programmers, to find code examples and review best practices for common programming tasks.

Readers should have already some basic familiarity with [`Rust`][rust]{{hi:Rust}}⮳ concepts. The [`Rust book`][book-rust]{{hi:Rust book}}⮳ is an excellent resource for complete beginners to get started with. This said, key features of the language are succinctly summarized in this book's [language][p-lang] section.

## Why this book

Per the curated list of Rust crates [`blessed.rs`][blessed-rs-website]⮳, "the standard library in Rust is much smaller than in Python or Go, for example. Those languages come with "batteries included" support ... Rust, on the other hand, gets things like that from the [`crates.io`][crates.io-website]{{hi:crates.io}}⮳ ecosystem and the [`Cargo`][c-cargo]⮳{{hi:Cargo}} package manager. But with _more than 160 thousand crates_ (libraries) to choose from, a common complaint from new Rust developers is that they don't know where to start, which crates they ought to use, and which crates they ought to trust." There are no dominant frameworks or platforms akin to `Rails`, `Django`, `Spring` or `Node` in the Rust world at this time.

This book therefore intends to provide EXAMPLES to demonstrate the uses of KEY CRATES, that is libraries necessary for day-to-day Rust coding - examples which are absent from or scattered in the [reference documentation][docs-rs]⮳. It hopes to become a "cheat sheet on steroid" for the Rust ecosystem (_not just_ for the Rust language).

## This book includes most of the "Rust Cookbook"

The "Rust How-to" project started as a set of notes kept while the author was learning Rust and evolved in a standalone book. The author then came across the [Rust Cookbook][book-rust-cookbook]⮳ community project, which shares very similar goals. Unfortunately, no updates have been made to that book in more than 4 years. Many of its examples no longer work. Several crates it references are no longer maintained. The author thus decided to merge the contents of the `Rust Cookbook` into this book, testing and refreshing its examples, and expanding its coverage significantly.

## How to read this book

The left sidebar is organized by topic.

- The book first quickly summarizes the basics of the [language][p-lang] and often-used elements of the [standard library][p-standard-library].
- The crates section provides pointers on how to locate key crates and provides alphabetical and categorical indices of crates used in the book.
- The bulk of the book is divided in sections named after the [`crates.io`][crates.io-website]{{hi:crates.io}}⮳ [categories][crates.io-category_slugs]⮳ whenever possible.
- Each section contains a list of recipes. The recipes are simple statements of a task to accomplish, like "generate random numbers in a range"; and each recipe is tagged with badges indicating which _crates_ they use, like [![rand][c-rand-badge]][c-rand], and which categories on [`crates.io`][crates.io-website]{{hi:crates.io}} those crates belong to, like [![cat-algorithms][cat-algorithms-badge]][cat-algorithms]{{hi:Algorithms}}.
- The book focuses on cross-cutting concerns that affect most aspects of development e.g. [error handling][p-errors], [error customization][p-error-customization], [configuration][p-config], [debugging][p-debugging]...
- [Concurrency][p-concurrency], including [asynchronous programming][p-asynchronous], is covered in details. So are [development tools][p-development-tools].
- programming domains such as [CLI][p-cli] and [Web][p-web-programming] development.

The [links][p-links] section provides pointers to notable Rust websites, learning resources, cheat sheets, books, and code examples...

The [contributing][p-contributing] section details how to contribute to the book itself.

New Rust programmers should be comfortable reading from the first section to the last, and doing so should give one a strong overview of the crate ecosystem. Click on a topic in the sidebar to navigate to the page for that section of the book.

If you are simply looking for the solution to a simple task, the easiest ways to find a specific recipe are to

- use the search button,
- scan the left-side bar for categories you are interested in,
- scan the [Index of examples][p-index-examples], and from there, click on the name of the recipe to view it.
- look up into the [Word index][p-word-index] lists concepts, crates (in lowercase), and Rust items (using their full path e.g. [`parking_lot::ReentrantMutex`][c-parking_lot::ReentrantMutex]⮳{{hi:parking_lot::ReentrantMutex}}).
- consult the alphabetical and categorical crates indices.

## How to use the recipes

Recipes are designed to give you instant access to working code, along with a full explanation of what it is doing, and to guide you to further information. All recipes are self-contained programs, so that they may be copied directly into your own projects for experimentation. To do so follow the instructions below.

Consider this example for "generate random numbers within a range":

[![rand][c-rand-badge]][c-rand]{{hi:rand}} [![cat-algorithms][cat-algorithms-badge]][cat-algorithms]{{hi:Algorithms}}

```rust,editable
{{#include ../crates/about/tests/about.rs:example}}
```

To work with it locally we can run the following commands to create a new cargo project, and change to that directory:

```sh
cargo new my-example --bin
cd my-example
```

Now, we also need to add the necessary crates to [Cargo.toml][book-cargo-cargo-toml]⮳, as indicated by the crate badges, in this case just "rand". To do so, we'll use the `cargo add` command.

```sh
cargo add rand
```

Next you can replace `src/main.rs` with the full contents of the example and run it:

```sh
cargo run
```

The crate badges that accompany the examples link to the crates' full documentation on [`docs.rs`][docs-rs]{{hi:docs.rs}}⮳, and is often the next documentation you should read after deciding which crate suites your purpose.

## A note about error handling

Error handling in Rust is robust when done correctly, but can require a fair bit of boilerplate. Because of this, one often sees Rust examples filled with [`unwrap`][c-std::result::Result::unwrap]⮳{{hi:unwrap}} calls, instead of proper error handling.

Since this book's recipes are intended to be reused as-is and encourage best practices, they set up error handling correctly when there are [`Result`][c-std::result::Result]⮳{{hi:Result}} types involved. The structure generally looks like:

```rust,editable
{{#include ../crates/about/tests/about1.rs:example}}
```

```rust,editable
{{#include ../crates/about/tests/about2.rs:example}}
```

In most examples, we have chosen to use [`anyhow`][c-anyhow]⮳{{hi:anyhow}}'s [`Result`][c-anyhow::Result] as the return type of any fallible function, instead of writing `std::result::Result<_, Box<dyn std::error::Error>>` or using custom [`Result`][c-std::result::Result]⮳{{hi:Result}} / [`Error`][c-std::error::Error]⮳{{hi:Error}} types.

Within the code, we use the `?` operator to easily propagate any error that implements the [`std::error::Error`][c-std::error::Error]⮳{{hi:std::error::Error}} trait.

For more background on error handling in Rust, read [this page][book-rust-error-handling]⮳ of the 'Rust book'.

## Additional examples

The [`crates/xmpl`][rust-howto-xmpl-github] folder in the book's GitHub repo contains additional examples that can't be embedded into the book, due to their length.

## A note about crate representation

This book is intended to provide expansive coverage of "key" or "foundational" crates - those crates that make up the most common programming tasks, and that the rest of the ecosystem builds off of.

Key crates are identified by cross-referencing:

- [`blessed.rs`][blessed-rs-website]{{hi:blessed.rs}}⮳ and similar resources,
- most downloaded crates (overall and per category) in [`crates.io`][crates.io-website]{{hi:crates.io}}⮳,
- high-quality crates per [`lib.rs`][lib-rs]{{hi:lib.rs}}⮳ [statistics][lib-rs-stats]⮳.

The selection process is necessarily opinionated. Feel free to offer suggestions (or submit a PR), if the author missed an important, widely used crate.

## What other books should I consult?

[Rust by Example][book-rust-by-example-book]⮳ is similar in concept - a collection of runnable examples - but not in scope, as it focuses solely on the Rust language and standard library.

Consult the links section and its books page for other recommendations.

{{#include refs.incl.md}}
{{#include refs/link-refs.md}}

<div class="hidden">
[index: polish (P0)](https://github.com/john-cd/rust_howto/issues/536)
[review about2](https://github.com/john-cd/rust_howto/issues/984)
</div>
