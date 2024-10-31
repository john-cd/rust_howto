# About "Cookin' with Rust"

## Table of contents

- [Who this book is for][p-who-this-book-is-for]⮳
- [How to read this book][p-how-to-read-this-book]⮳
- [How to use the recipes][p-how-to-use-the-recipes]⮳
- [A note about error handling][p-a-note-about-error-handling]⮳
- [A note about crate representation][p-a-note-about-crate-representation]⮳

## Who this book is for

This cookbook is intended for new Rust programmers, so that they may quickly get an overview of the capabilities of the Rust crate ecosystem. It is also intended for experienced Rust programmers, who should find in the recipes an easy reminder of how to accomplish common tasks.

## How to read this book

The cookbook [`index`][p-index] contains the full list of recipes, organized into a number of sections: "basics", "encoding", "concurrency", etc. The sections themselves are more or less ordered in progression, with later sections being more advanced, and occasionally building on concepts from earlier sections.

Within the index, each section contains a list of recipes. The recipes are simple statements of a task to accomplish, like "generate random numbers in a range"; and each recipe is tagged with badges indicating which _crates_ they use, like [![rand][c-rand-badge]][c-rand], and which categories on [`crates.io`][crates.io-website]{{hi:crates.io}} those crates belong to, like
[![cat-algorithms][cat-algorithms-badge]][cat-algorithms]{{hi:Algorithms}}.

New Rust programmers should be comfortable reading from the first section to the last, and doing so should give one a strong overview of the crate ecosystem. Click on the section header in the index, or in the sidebar to navigate to the page for that section of the book.

If you are simply looking for the solution to a simple task, the cookbook is today more difficult to navigate. The easiest way to find a specific recipe is to scan the index looking for the crates and categories one is interested in. From there, click on the name of the recipe to view it. This will improve in the future.

## How to use the recipes

Recipes are designed to give you instant access to working code, along with a full explanation of what it is doing, and to guide you to further information.

All recipes in the cookbook are full, self contained programs, so that they may be copied directly into your own projects for experimentation. To do so follow the instructions below.

Consider this example for "generate random numbers within a range":

[![rand][c-rand-badge]][c-rand]{{hi:rand}}  [![cat-algorithms][cat-algorithms-badge]][cat-algorithms]{{hi:Algorithms}}

```rust
{{#include ../deps/tests/about/about.rs}}
```

To work with it locally we can run the following commands to create a new cargo project, and change to that directory:

```sh
cargo new my-example --bin cd my-example
```

Now, we also need to add the necessary crates to [Cargo.toml][book-cargo-cargo-toml]⮳, as indicated by the crate badges, in this case just "rand". To do so, we'll use the `cargo add` command.

Now you can replace `src/main.rs` with the full contents of the example and run it:

```sh
cargo run
```

The crate badges that accompany the examples link to the crates' full documentation on [`docs.rs`][docs-rs]{{hi:docs.rs}}⮳, and is often the next documentation you should read after deciding which crate suites your purpose.

## A note about error handling

Error handling in Rust is robust when done correctly, but in today's Rust it requires a fair bit of boilerplate. Because of this one often sees Rust examples filled with `unwrap` calls instead of proper error handling.

Since these recipes are intended to be reused as-is and encourage best practices, they set up error handling correctly when there are `Result` types involved.

The basic pattern we use is to have a `fn main() -> Result`.

The structure generally looks like:

```rust
{{#include ../deps/tests/about/about1.rs}}
```

This is using the `error_chain!` macro to define a custom `Error` and `Result` type, along with automatic conversions from two standard library error types. The automatic conversions make the `?` operator work.

For the sake of readability error handling boilerplate is hidden by default like below. In order to read full contents click on the "expand" (<i class="fa fa-expand"></i>) button located in the top right corner of the snippet.

```rust
{{#include ../deps/tests/about/about2.rs}}
```

For more background on error handling in Rust, read [this page of the Rust book][book-rust-error-handling]⮳ and [this blog post][blog-error]⮳.

## A note about crate representation

This cookbook is intended eventually to provide expansive coverage of the Rust crate ecosystem, but today is limited in scope while we get it bootstrapped and work on the presentation. Hopefully, starting from a small scope and slowly expanding will help the cookbook become a high-quality resource sooner, and allow it to maintain consistent quality levels as it grows.

At present the cookbook is focused on the standard library, and on "core" or "foundational" crates - those crates that make up the most common programming tasks, and that the rest of the ecosystem builds off of.

[p-a-note-about-crate-representation]: #a-note-about-crate-representation
[p-a-note-about-error-handling]: #a-note-about-error-handling
[p-how-to-use-the-recipes]: #how-to-use-the-recipes
[p-how-to-read-this-book]: #how-to-read-this-book
[p-who-this-book-is-for]: #who-this-book-is-for
[p-index]: index.md

{{#include refs/link-refs.md}}

<div class="hidden">
TODO: rewrite
</div>
