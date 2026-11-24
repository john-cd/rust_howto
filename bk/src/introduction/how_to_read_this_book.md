<!-- TODO Reviewed ??-??-2026. -->

# How to Read This Book

Open the menu in the upper-left corner to show the table of contents. Select the arrow beside a section to see its chapters.

- The book first summarizes core concepts of the [language][p~lang] and often-used elements of the [standard library][p~standard-library] (if they are not covered elsewhere). The [[code_organization | code organization]] section explains how Rust code should be structured, depending on your project's needs.
- The ecosystem chapters cover Rust tools and libraries. Most sections follow the [categories][crates.io~category_slugs]↗ on [`crates.io`][crates.io~website]↗{{hi:crates.io}}, so you can browse the same category there or search [`lib.rs`][lib.rs~website]↗ for related crates.
- Each recipe answers a specific question, such as how to generate a random number in a range. Its badges identify the crates it uses, such as [![rand][c~rand~docs~badge]][c~rand~docs], and their categories, such as [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}.
- The [[crate_selection | crate selection]] chapter in the "Resources" section provides additional pointers on how to locate crates suitable for your project.
- The [links][p~links] section provides pointers to notable Rust websites, [learning](../links/learning_rust.md) resources, cheat sheets, [[books | books]], [[example_code | code examples]], etc.
- The indices include a word index, an index of examples in the book, and [[crates_alphabetical | alphabetical]] and [[crates_by_category | categorical]] lists of crates used in the book.
- The [contributing][p~contributing] section details how to contribute to the book itself.

## Find a Recipe {#look-for-solution-for-specific-task}

Search from the top toolbar or browse the categories in the sidebar. The [index of examples][p~index~examples] lists recipes by task. Use the [word index][p~word-index] to look up a concept, a crate name, or a fully qualified Rust item such as [`parking_lot::ReentrantMutex`][c~parking_lot::ReentrantMutex~docs]↗{{hi:parking_lot::ReentrantMutex}}. You can also browse crates [[crates_alphabetical | alphabetically]] or [[crates_by_category | by category]].

## Use the Code Examples {#how-to-use-code-examples}

Many recipes are self-contained programs that you can copy into your own projects. Use the Play button when the snippet fits the Playground's available crates and environment. Follow the recipe's Cargo command when it needs the workspace's dependency versions or features, local files, native tools, or sockets. An illustrative fragment shows the part to put in a larger program. The following instructions show how to set up a local project for a typical recipe.

Consider the following example for generating a random number:

[![rand~website][c~rand~website~badge]][c~rand~website] {{#badges rand}} {{hi:Random}} {{hi:Rng}} {{#categories no-std="No standard library",algorithms=Algorithms}}

```rust,editable
{{#include ../../crates/about/examples/about.rs:example}}
```

To work with it locally, you can run the following commands to create a new [`cargo`][c~cargo~docs]↗{{hi:cargo}} project, and change to that directory:

```sh
cargo new myexample
cd myexample
```

You also need to add the necessary crates to [Cargo.toml][book~cargo~cargo-toml]↗, as indicated by the crate badges, in this case just the "rand" crate. To do so, use the [`cargo add`][book~cargo~cargo-add]↗{{hi:cargo add}} command.

```sh
cargo add rand
```

Next you can replace `src/main.rs` with the full contents of the example and run it:

```sh
cargo run
```

The crate badges that accompany the examples link to several key webpages, including the crates' full documentation on [`docs.rs`][docs.rs~website]↗{{hi:docs.rs}}, which is often the next documentation you should read after deciding which crate suits your purpose. The category badges next to them link to the corresponding page on `crates.io`, which lists related crates.

## Read Additional Examples {#additional-examples}

The [`xmpl`][rust-howto~xmpl~repo]↗ folder in the book's GitHub repo contains additional examples that can't be embedded into the book, due to their length.

## A Note about Error Handling {#note-about-error-handling}

Calling [`unwrap`][c~std::result::Result::unwrap~docs]↗{{hi:unwrap}} on an `Err` panics. Many recipes instead return [`anyhow::Result`][c~anyhow::Result~docs]↗ and use the [`?`][book~rust~ch09-02-recoverable-errors-with-result-?]↗{{hi:?}} operator. On success, `?` yields the value inside `Ok`. On failure, it converts the error to the function’s error type and returns early.

```rust,editable
{{#include ../../crates/about/examples/about1.rs:example}}
```

The `main` function often returns a `Result` as well. Rust converts it into an exit code and displays the error, if any:

```rust,editable
{{#include ../../crates/about/examples/about2.rs:example}}
```

For more background on error handling in Rust, review the [[entry_points | entry point]] and [error handling](../standard-library/recoverable_error_handling.md) sections of this book or read [this section][book~rust~error-handling]↗ of the 'Rust book'.

## Find Cross-Cutting Topics {#important-sections .skip}

For working with failures, start with [Recoverable Error Handling](../standard-library/recoverable_error_handling.md). For language mechanics, see [Ownership and Borrowing](../language/ownership_and_borrowing.md). To choose among common collections, use the [Data Structures overview](../categories/data-structures/index.md).

## A Note about Crate Representation {#note-about-crate-representation}

This book intends to cover "key" or "foundational" crates - the crates that make up the most common programming tasks, and that the rest of the ecosystem builds off of.

Key crates were identified by cross-referencing [`blessed.rs`][blessed-rs~website]↗{{hi:blessed.rs}}, the most downloaded crates (overall and per category) in [`crates.io`][crates.io~website]↗{{hi:crates.io}}, [most popular Rust libraries][lib.rs~most-popular~website]↗ and high-quality crates per [`lib.rs`][lib.rs~website]↗{{hi:lib.rs}} [statistics][lib.rs~stats~website]↗, as well as poring over blogs, forums, and examples.

The crate selection process is necessarily opinionated. Feel free to offer suggestions (or submit a PR to the book's GitHub repo), if the author missed an important, widely used crate.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}
