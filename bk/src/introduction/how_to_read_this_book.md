# How to Read This Book

Click on the "three stacked bars" on the top left corner, in order to reveal the table of contents in the left sidebar.

The book is organized in a few major sections. Click on a heading (with an arrow) to expand it and navigate to chapters within the book.

- The book first summarizes core concepts of the [language][p~lang] and often-used elements of the [standard library][p~standard-library] (if they are not covered elsewhere). The [[code_organization | code organization]] section explains how Rust code should be structured, depending on your project's needs.
- The bulk of the book focuses on the Rust ecosystem, meaning Rust tools and public libraries that you reuse in your project. It is divided in subsections named, whenever possible, after the [categories][crates.io~category_slugs]↗ of [`crates.io`][crates.io~website]↗{{hi:crates.io}}, the central registry of public Rust crates. That means you can easily list and search for related crates using `crates.io` or [`lib.rs`][lib.rs~website]↗.
- Each chapter within each subsection contains a list of recipes. The recipes are simple statements of a task to accomplish, like "generate random numbers in a range"; and each recipe is tagged with badges indicating which _crates_ they use, like [![rand][c~rand~docs~badge]][c~rand~docs], and which categories on [`crates.io`][crates.io~website]↗{{hi:crates.io}} those crates belong to, like [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}.
- The [[crate_selection | crate selection]] chapter in the "Resources" section provides additional pointers on how to locate crates suitable for your project.
- The [links][p~links] section provides pointers to notable Rust websites, [[learning | learning]] resources, cheat sheets, [[books | books]], [[example_code | code examples]], etc.
- The appendices includes a word index, an index of examples in the book, and provides [[crates_alphabetical | alphabetical]] and [[crates_by_category | categorical]] lists of crates used in the book.
- The [contributing][p~contributing] section details how to contribute to the book itself.

## Look for a Solution for a Specific Task {#look-for-solution-for-specific-task}

If you are simply looking for the solution to a given task, the easiest ways to find a specific recipe are to:

- Use the search button (in the top toolbar),
- Scan the left-side bar for categories you are interested in,
- Scan the [index of examples][p~index~examples], and from there, click on the name of the recipe to view it,
- Look up into the [word index][p~word-index], which lists concepts, crates (in lower case), and Rust items (using their full path e.g. [`parking_lot::ReentrantMutex`][c~parking_lot::ReentrantMutex~docs]↗{{hi:parking_lot::ReentrantMutex}}),
- Consult the [[crates_alphabetical | alphabetical]] and [[crates_by_category | categorical]] crates indices, then search for the crate name.

## Use the Code Examples {#how-to-use-code-examples}

Code examples, a.k.a. recipes, are designed to give you instant access to working code, along with an explanation of what it is doing, and to guide you to further information. Most recipes can be copied to the clipboard; edited in place; and run on the Rust playground by clicking the "right arrow" button. The results, if any, are displayed below the example.

All recipes are self-contained programs, so that they may be copied directly into your own projects for experimentation. To do so, follow the instructions below.

Consider the following example for generating a random number:

[![rand~website][c~rand~website~badge]][c~rand~website] [![rand][c~rand~docs~badge]][c~rand~docs] [![rand~crates.io][c~rand~crates.io~badge]][c~rand~crates.io] [![rand~repo][c~rand~repo~badge]][c~rand~repo] [![rand~lib.rs][c~rand~lib.rs~badge]][c~rand~lib.rs]{{hi:rand}}{{hi:Random}}{{hi:Rng}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}

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

Error handling in Rust is robust but can require a bit of boilerplate. Because of this, one often sees Rust examples filled with [`unwrap`][c~std::result::Result::unwrap~docs]↗{{hi:unwrap}} calls, instead of proper error handling.

Since this book's recipes are intended to be reused as-is and encourage best practices, they set up error handling correctly when there are [`Result`][c~std::result::Result~docs]↗{{hi:Result}} types involved.

In most examples, we have chosen to use [`anyhow::Result`][c~anyhow::Result~docs]↗ (from the [`anyhow`][c~anyhow~docs]↗{{hi:anyhow}} crate) as the return type of any fallible function (instead of writing `std::result::Result<_, Box<dyn std::error::Error>>` or using custom [`Result`][c~std::result::Result~docs]↗{{hi:Result}} or [`Error`][c~std::error::Error~docs]↗{{hi:std::error::Error}} types). Within the code, we use the [`?`][book~rust~ch09-02-recoverable-errors-with-result-?]↗{{hi:?}} operator to easily propagate any error that implements the [`std::error::Error`][c~std::error::Error~docs]↗ trait. The structure generally looks like the following:

```rust,editable
{{#include ../../crates/about/examples/about1.rs:example}}
```

The `main` function often returns a `Result` as well. Rust converts it into an exit code and displays the error, if any:

```rust,editable
{{#include ../../crates/about/examples/about2.rs:example}}
```

For more background on error handling in Rust, review the [[entry_points | entry point]] and [[error_handling | error handling]] sections of this book or read [this section][book~rust~error-handling]↗ of the 'Rust book'.

## A Note about Crate Representation {#note-about-crate-representation}

This book intends to cover "key" or "foundational" crates - the crates that make up the most common programming tasks, and that the rest of the ecosystem builds off of.

Key crates were identified by cross-referencing [`blessed.rs`][blessed-rs~website]↗{{hi:blessed.rs}}, the most downloaded crates (overall and per category) in [`crates.io`][crates.io~website]↗{{hi:crates.io}}, [most popular Rust libraries][lib.rs~most-popular~website]↗ and high-quality crates per [`lib.rs`][lib.rs~website]↗{{hi:lib.rs}} [statistics][lib.rs~stats~website]↗, as well as pouring over countless blogs, forums, and examples.

The crate selection process is necessarily opinionated. Feel free to offer suggestions (or submit a PR to the book's GitHub repo), if the author missed an important, widely used crate.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">

## Important Sections {#important-sections .skip}

- The book covers cross-cutting concerns that affect most aspects of development e.g. [[error_handling | error handling]], [[error_customization | error customization]], [[config | configuration]], [[development-tools_debugging | debugging]]...
- [[concurrency | Concurrency]], including [[asynchronous | asynchronous programming]], is covered in details.
- So are [[development tools]] and programming domains such as [[command-line-interface | CLI]] and [[web_programming | Web]] development.

</div>
