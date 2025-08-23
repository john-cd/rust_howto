# How to Read This Book

Click on the "three stacked bars" (on the top left corner) to reveal the left sidebar and the table of contents.

The book is organized in a few major sections. Click on a heading to expand it and navigate to chapters within the book.

- The book first summarizes the basics of the [language][p~lang] and often-used elements of the [standard library][p~standard-library]. The [[code_organization | code organization]] section explains how Rust code should be structured.

- The bulk of the book focuses on the Rust ecosystem and is divided in sections named after the [`crates.io`][crates.io~website]↗{{hi:crates.io}} [categories][crates.io~category_slugs]↗, whenever possible.
- Each section contains a list of recipes. The recipes are simple statements of a task to accomplish, like "generate random numbers in a range"; and each recipe is tagged with badges indicating which _crates_ they use, like [![rand][c~rand~docs~badge]][c~rand~docs], and which categories on [`crates.io`][crates.io~website]↗{{hi:crates.io}} those crates belong to, like [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}.

- The [[crate_selection | crate selection]] chapter provides pointers on how to locate crates suitable for your project and provides [[crates_alphabetical | alphabetical]] and [[crates_by_category | categorical]] indices of crates used in the book.
- The [links][p~links] section provides pointers to notable Rust websites, [[learning | learning]] resources, cheat sheets, [[books | books]], and [[example_code | code examples]].

The [contributing][p~contributing] section details how to contribute to the book itself.

If you are simply looking for the solution to a given task, the easiest ways to find a specific recipe are to:

- Use the search button (in the top toolbar),
- Scan the left-side bar for categories you are interested in,
- Scan the [index of examples][p~index~examples], and from there, click on the name of the recipe to view it,
- Look up into the [word index][p~word-index] lists concepts, crates (in lower case), and Rust items (using their full path e.g. [`parking_lot::ReentrantMutex`][c~parking_lot::ReentrantMutex~docs]↗{{hi:parking_lot::ReentrantMutex}}),
- Consult the [[crates_alphabetical | alphabetical]] and [[crates_by_category | categorical]] crates indices.

## Important Sections

- The book covers cross-cutting concerns that affect most aspects of development e.g. [error handling][p~errors], [error customization][p~error-customization], [configuration][p~config], [debugging][p~debugging]...
- [Concurrency][p~concurrency], including [asynchronous programming][p~asynchronous], is covered in details.
- So are [development tools][p~development-tools] and programming domains such as [CLI][p~cli] and [Web][p~web-programming] development.

## How to Use the Code Examples

Code examples, a.k.a. Recipes, are designed to give you instant access to working code, along with a full explanation of what it is doing, and to guide you to further information. All recipes are self-contained programs, so that they may be copied directly into your own projects for experimentation. To do so, follow the instructions below.

Consider the following example for "generate random numbers within a range":

[![rand][c~rand~docs~badge]][c~rand~docs]{{hi:rand}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}

```rust,editable
{{#include ../../crates/about/examples/about.rs:example}}
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

The crate badges that accompany the examples link to the crates' full documentation on [`docs.rs`][docs.rs~website]↗{{hi:docs.rs}}, and is often the next documentation you should read after deciding which crate suites your purpose.

## A Note about Error Handling

Error handling in Rust is robust when done correctly, but can require a fair bit of boilerplate. Because of this, one often sees Rust examples filled with [`unwrap`][c~std::result::Result::unwrap~docs]↗{{hi:unwrap}} calls, instead of proper error handling.

Since this book's recipes are intended to be reused as-is and encourage best practices, they set up error handling correctly when there are [`Result`][c~std::result::Result~docs]↗{{hi:Result}} types involved. The structure generally looks like:

```rust,editable
{{#include ../../crates/about/examples/about1.rs:example}}
```

```rust,editable
{{#include ../../crates/about/examples/about2.rs:example}}
```

In most examples, we have chosen to use [`anyhow`][c~anyhow~docs]↗{{hi:anyhow}}'s [`Result`][c~anyhow::Result~docs]↗ as the return type of any fallible function, instead of writing `std::result::Result<_, Box<dyn std::error::Error>>` or using custom [`Result`][c~std::result::Result~docs]↗{{hi:Result}} / [`Error`][c~std::error::Error~docs]↗{{hi:std::error::Error}} types.

Within the code, we use the [`?`][book~rust~ch09-02-recoverable-errors-with-result-?]↗{{hi:?}} operator to easily propagate any error that implements the [`std::error::Error`][c~std::error::Error~docs]↗ trait.

For more background on error handling in Rust, read [this page][book~rust~error-handling]↗ of the 'Rust book'.

## Additional Examples

The [`xmpl`][rust-howto~xmpl~repo]↗ folder in the book's GitHub repo contains additional examples that can't be embedded into the book, due to their length.

## A Note about Crate Representation

This book provides expansive coverage of "key" or "foundational" crates - the crates that make up the most common programming tasks, and that the rest of the ecosystem builds off of.

Key crates were identified by cross-referencing [`blessed.rs`][blessed-rs~website]↗{{hi:blessed.rs}}, most downloaded crates (overall and per category) in [`crates.io`][crates.io~website]↗{{hi:crates.io}}, [most popular Rust libraries][lib.rs~most-popular~website]↗ and high-quality crates per [`lib.rs`][lib.rs~website]↗{{hi:lib.rs}} [statistics][lib.rs~stats~website]↗.

This said, the crate selection process is necessarily opinionated. Feel free to offer suggestions (or submit a PR to the book's GitHub repo), if the author missed an important, widely used crate.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
