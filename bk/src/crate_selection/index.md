# Crate Selection

## Choose Dependencies for your Project {#choose-dependencies}

The Rust standard library is fairly minimal. Besides providing basic data types, the standard library is largely concerned with abstracting over differences between common platforms, e.g., Windows and Unix derivatives.

Instead, major functionality like regular expressions, random numbers, cryptography, serialization, async, and logging, are found in separate Rust crates that are available on [`crates.io`][crates.io~website]↗{{hi:crates.io}}. Rustaceans add these crates to their projects via the [`cargo`][c~cargo~docs]↗{{hi:cargo}} package manager.

A common complaint from new Rust developers is that they don't know which crates they ought to use.

### Use this Book to Select Crates for your Project {#crates}

It is the main purpose of this book to provide expansive coverage of "key" or "foundational" crates - those crates that make up the most common programming tasks.

1. Browse the left sidebar for a relevant category.
1. Search by keyword for recipes for your specific problem.
1. Consult the index of examples.
1. Consult the [[word index | word_index]].
1. Consult [Crates by Alphabetical Order][p~crates-alphabetical] and [Crates by Category][p~crates-categories].
1. Use this book's [[links | links]] to Rust documentation, books, videos, cheatsheets, etc.
1. The [[example_code | Example Code and Templates]] chapter provides lists of popular Rust repositories.

### Crate Recommendations {#crate-recommendations}

You may also use the following sites for crate recommendations:

- [`blessed.rs`][blessed-rs~website]↗{{hi:blessed.rs}},
- [`lib.rs`][lib.rs~website]↗{{hi:lib.rs}}, and in particular,
  - [Most Popular Rust Libraries][lib.rs~most-popular~website]↗,
  - Crates [statistics][lib.rs~stats~website]↗,
- The Rust community's official crate registry: [`crates.io`][crates.io~website]↗{{hi:crates.io}},
  - [`crates.io`'s Most (Recent) Downloads][crates.io~most-recent-downloads]↗.
  - You should search for crates by [category][crates.io-categories~website]↗ and by [keywords][crates.io-keywords~website]↗.

You can also search or post on Rust chat servers and forums - see [[blogs | Blogs and Forums]].

### Additional Lists & Reviews {#lists-reviews}

- [Awesome Rust (libhunt.com)][rust-libhunt~website]↗.
- [Top Rust Libraries 2025][libs.tech-rust~website]↗.
- [State of the Crates 2025][blog~state-of-the-crates]↗.
- [r/rust: "What are some less popular but well-made crates you'd like others to know about?"][reddit~what_are_some_less_popular_but_wellmade_crates]↗.
- ["Must know" Rust Crates][must_know_rust_crates~website]↗.
- [RustRepo][rustrepo~website]↗.

You may also consult older resources, such as:

- [Best of Rust Crates][reddit~best-of-rust-crates]↗.
- [Rust Starter Pack][rust-starter-pack~website]↗{{hi:Rust starter pack}}.
- [![stdx][c~stdx~github~badge]][c~stdx~github]↗{{hi:stdx}}.

## Related Topics {#related-topics}

- [[state_of_the_art | State of the Rust Ecosystem]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
