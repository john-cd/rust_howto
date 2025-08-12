# Work In Progress

<div class="warning">
This book is going through HEAVY EDITS. Pardon the dust.

A large number of chapters, pages, and examples are currently hidden, as we are reorganizing the book.
</div>

Rust is a modern programming language that offers high performance, reliability, and productivity. It is designed to prevent common errors such as memory leaks{{hi:Memory leaks}}, data races{{hi:Data races}}, and null pointer dereferences{{hi:Null pointer dereferences}}, by enforcing strict rules at compile time. Rust supports powerful features such as generics{{hi:Generics}}, traits{{hi:Traits}}, macros{{hi:Macros}}, and concurrency{{hi:Concurrency}}, making it suitable for a wide range of applications.

## What You Will Find Here

This book is **a compendium of Rust ecosystem examples and resources**. It is intended to be **everything you need for day-to-day Rust coding, in one place**. It demonstrates good practices to accomplish common programming tasks, using the crates of the Rust ecosystem. It summarizes the language and key features of the standard library. It includes numerous links to Rust resources.

## Who Should Read This Book

This book is intended for:

- new Rust programmers, to get an overview of the capabilities of the Rust ecosystem and pointers to other resources.
- experienced programmers, to find code examples and review best practices for common programming tasks.

Readers should have already some basic familiarity with [Rust][rust-lang~website]↗ concepts. The [Rust Book][book~rust]{{hi:Rust book}}↗ is an excellent resource for complete beginners to get started with. This said, key features of the language are summarized in this book's [language][p~lang] section.

## Why This Book

Per the curated list of Rust crates [`blessed.rs`][blessed-rs~website]↗, "the standard library in Rust is much smaller than in Python or Go, for example. Those languages come with "batteries included" support ... Rust, on the other hand, gets things like that from the [`crates.io`][crates.io~website]{{hi:crates.io}}↗ ecosystem and the [`cargo`][c~cargo~docs]↗{{hi:cargo}} package manager. But with _more than 180 thousand crates_ (libraries) to choose from, a common complaint from new Rust developers is that they don't know where to start, which crates they ought to use, and which crates they ought to trust." There are no dominant frameworks or platforms akin to `Rails`, `Django`, `Spring` or `Node` in the Rust world at this time.

This book therefore intends to provide EXAMPLES to demonstrate the uses of KEY CRATES, that is libraries necessary for day-to-day Rust coding - examples which are absent from or scattered in the [reference documentation][docs.rs~website]↗ of hundreds of crates. It hopes to become a "cheat sheet on steroid" for the Rust ECOSYSTEM (_not just_ for the Rust language).

## This Book Includes Most of the "Rust Cookbook"

The "Rust How-to" project started as a set of notes kept while the author was learning Rust and evolved in a standalone book. The author then came across the [Rust Cookbook][book~rust-cookbook]↗ community project, which shares very similar goals. Unfortunately, few updates have been made to that book in the last 4 years. Many of its examples no longer work. Several crates it references are no longer maintained. The author thus decided to merge the contents of the `Rust Cookbook` into this book, testing and refreshing its examples.

## How to Read This Book

The left sidebar is organized by topic. Click on a topic in the sidebar to navigate to the page for that section of the book.

- The book first summarizes the basics of the [language][p~lang] and often-used elements of the [standard library][p~standard-library]. The [[code_organization | code organization]] section explains how Rust code should be structured.
- The [[crate_selection | crate selection]] chapter provides pointers on how to locate crates suitable for your project and provides [[crates_alphabetical | alphabetical]] and [[crates_by_category | categorical]] indices of crates used in the book.
- The bulk of the book is divided in sections named after the [`crates.io`][crates.io~website]{{hi:crates.io}}↗ [categories][crates.io~category_slugs]↗, whenever possible.
- Each section contains a list of recipes. The recipes are simple statements of a task to accomplish, like "generate random numbers in a range"; and each recipe is tagged with badges indicating which _crates_ they use, like [![rand][c~rand~docs~badge]][c~rand~docs], and which categories on [`crates.io`][crates.io~website]{{hi:crates.io}} those crates belong to, like [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}.

The [contributing][p~contributing] section details how to contribute to the book itself.

If you are simply looking for the solution to a given task, the easiest ways to find a specific recipe are to:

- use the search button (in the top toolbar),
- scan the left-side bar for categories you are interested in,
- scan the [index of examples][p~index~examples], and from there, click on the name of the recipe to view it,
- look up into the [word index][p~word-index] lists concepts, crates (in lower case), and Rust items (using their full path e.g. [`parking_lot::ReentrantMutex`][c~parking_lot::ReentrantMutex~docs]↗{{hi:parking_lot::ReentrantMutex}}),
- consult the [[crates_alphabetical | alphabetical]] and [[crates_by_category | categorical]] crates indices.

The [links][p~links] section provides pointers to notable Rust websites, [[learning | learning]] resources, cheat sheets, [[books | books]], and [[example_code | code examples]]...

## Important Sections

- The book covers cross-cutting concerns that affect most aspects of development e.g. [error handling][p~errors], [error customization][p~error-customization], [configuration][p~config], [debugging][p~debugging]...
- [Concurrency][p~concurrency], including [asynchronous programming][p~asynchronous], is covered in details.
- So are [development tools][p~development-tools] and programming domains such as [CLI][p~cli] and [Web][p~web-programming] development.

## How to Use the Recipes

Recipes are designed to give you instant access to working code, along with a full explanation of what it is doing, and to guide you to further information. All recipes are self-contained programs, so that they may be copied directly into your own projects for experimentation. To do so, follow the instructions below.

Consider the following example for "generate random numbers within a range":

[![rand][c~rand~docs~badge]][c~rand~docs]{{hi:rand}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}

```rust,editable
{{#include ../crates/about/examples/about.rs:example}}
```

To work with it locally we can run the following commands to create a new [`cargo`][c~cargo~docs]↗{{hi:cargo}} project, and change to that directory:

```sh
cargo new my-example --bin
cd my-example
```

Now, we also need to add the necessary crates to [Cargo.toml][book~cargo~cargo-toml]↗, as indicated by the crate badges, in this case just "rand". To do so, we'll use the [`cargo add`][book~cargo~cargo-add]↗{{hi:cargo add}} command.

```sh
cargo add rand
```

Next you can replace `src/main.rs` with the full contents of the example and run it:

```sh
cargo run
```

The crate badges that accompany the examples link to the crates' full documentation on [`docs.rs`][docs.rs~website]{{hi:docs.rs}}↗, and is often the next documentation you should read after deciding which crate suites your purpose.

## A Note about Error Handling

Error handling in Rust is robust when done correctly, but can require a fair bit of boilerplate. Because of this, one often sees Rust examples filled with [`unwrap`][c~std::result::Result::unwrap~docs]↗{{hi:unwrap}} calls, instead of proper error handling.

Since this book's recipes are intended to be reused as-is and encourage best practices, they set up error handling correctly when there are [`Result`][c~std::result::Result~docs]↗{{hi:Result}} types involved. The structure generally looks like:

```rust,editable
{{#include ../crates/about/examples/about1.rs:example}}
```

```rust,editable
{{#include ../crates/about/examples/about2.rs:example}}
```

In most examples, we have chosen to use [`anyhow`][c~anyhow~docs]↗{{hi:anyhow}}'s [`Result`][c~anyhow::Result~docs]↗ as the return type of any fallible function, instead of writing `std::result::Result<_, Box<dyn std::error::Error>>` or using custom [`Result`][c~std::result::Result~docs]↗{{hi:Result}} / [`Error`][c~std::error::Error~docs]↗{{hi:std::error::Error}} types.

Within the code, we use the [`?`][book~rust~ch09-02-recoverable-errors-with-result-?]↗{{hi:?}} operator to easily propagate any error that implements the [`std::error::Error`][c~std::error::Error~docs]↗ trait.

For more background on error handling in Rust, read [this page][book~rust~error-handling]↗ of the 'Rust book'.

## Additional Examples

The [`xmpl`][rust-howto~xmpl~github]↗ folder in the book's GitHub repo contains additional examples that can't be embedded into the book, due to their length.

## A Note about Crate Representation

This book is intended to provide expansive coverage of "key" or "foundational" crates - those crates that make up the most common programming tasks, and that the rest of the ecosystem builds off of.

Key crates are identified by cross-referencing:

- [`blessed.rs`][blessed-rs~website]{{hi:blessed.rs}}↗ and similar resources,
- Most downloaded crates (overall and per category) in [`crates.io`][crates.io~website]{{hi:crates.io}}↗,
- [Most popular Rust libraries][lib.rs~most-popular~website]↗,
- High-quality crates per [`lib.rs`][lib.rs~website]{{hi:lib.rs}}↗ [statistics][lib.rs~stats~website]↗.

The selection process is necessarily opinionated. Feel free to offer suggestions (or submit a PR), if the author missed an important, widely used crate.

## What Other Books Should I Consult?

[Rust by Example][book~rust-by-example~book]↗ is similar in concept - a collection of runnable examples - but not in scope, as it focuses solely on the Rust language and standard library.

Consult the [[links | Links]] section and its [[books | Books]] page for other recommendations.

{{#include refs.incl.md}}
{{#include refs/link-refs.md}}

<div class="hidden">
[index: polish NOW](https://github.com/john-cd/rust_howto/issues/536)
link to categories.md
</div>
