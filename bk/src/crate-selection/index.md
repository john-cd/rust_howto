<!-- TODO Reviewed ??-??-2026 -->

# Crate Selection

## Choose Dependencies for your Project

The Rust standard library is fairly minimal. Besides providing basic data types, the standard library is largely concerned with abstracting over differences between common platforms, e.g., Windows and Unix derivatives.

Instead, major functionality like regular expressions, random numbers, cryptography, serialization, async, and logging, are found in separate Rust crates that are available on [`crates.io`][crates.io~website]↗{{hi:crates.io}}. Rustaceans add these crates to their projects via the [`cargo`][c~cargo~docs]↗{{hi:cargo}} package manager.

A common complaint from new Rust developers is that they don't know which crates they ought to use.

### Use this Book to Select Crates for your Project

It is the main purpose of this book to provide expansive coverage of "key" or "foundational" crates - those crates that make up the most common programming tasks:

1. Browse the left sidebar for a relevant category.
1. Search by keyword for recipes for your specific problem.
1. Consult the index of examples.
1. Consult the [[word_index | word index]].
1. Consult [Crates by Alphabetical Order][p~crates-alphabetical] and [Crates by Category][p~crates-categories].
1. Use this book's [[links | links]] to Rust documentation, books, videos, cheatsheets, etc.
1. The [[example_code | Example Code and Templates]] chapter provides lists of popular Rust repositories.

Contributors can also see how this book builds its local category and alphabetical indices in [Generate Crate Indices Offline](crate_indices.md).

### Evaluate a Crate

Before adding a crate, check whether it fits your project's needs and maintenance expectations:

- **Maintenance and compatibility:** review the release history, recent activity, issue tracker, and support for your Rust version and target platforms.
- **Documentation and examples:** confirm that the API is documented and that examples cover the use case you need.
- **License and project fit:** check that the license and dependency policy work for your project, and that the crate's scope is appropriate.
- **Adoption:** download counts and reverse dependencies can provide context, but they do not establish quality or suitability.

Use the [alphabetical crate index][p~crates-alphabetical] and [category index][p~crates-categories] to compare options. [blessed.rs][blessed-rs~website]↗ and [lib.rs][lib.rs~website]↗ curate recommendations and crate information; confirm important details on the crate's own documentation and repository.

### Crate Recommendations

You may also use the following sites for crate recommendations:

- [`blessed.rs`][blessed-rs~website]↗{{hi:blessed.rs}} for commonly recommended libraries,
- [`lib.rs`][lib.rs~website]↗{{hi:lib.rs}} for crate discovery and statistics, including
  - [Most Popular Rust Libraries][lib.rs~most-popular~website]↗,
  - Crates [statistics][lib.rs~stats~website]↗,
- The Rust community's official crate registry: [`crates.io`][crates.io~website]↗{{hi:crates.io}},
  - [`crates.io`'s Most (Recent) Downloads][crates.io~most-recent-downloads]↗.
  - You should search for crates by [category][crates.io-categories~website]↗ and by [keywords][crates.io-keywords~website]↗.

You can also search or post on Rust chat servers and forums - see [[blogs | Blogs and Forums]].

### Consult Dated Lists and Reviews

- [Awesome Rust (libhunt.com)][rust-libhunt~website]↗ is a broad directory of Rust projects.
- [Top Rust Libraries 2026][libs.tech-rust~website]↗ and [State of the Crates 2025][blog~state-of-the-crates]↗ are dated snapshots; use them to discover candidates, then verify current maintenance and compatibility.
- [r/rust: "What are some less popular but well-made crates you'd like others to know about?"][reddit~what_are_some_less_popular_but_wellmade_crates]↗ contains community recommendations.
- ["Must know" Rust Crates][must_know_rust_crates~website]↗ and [RustRepo][rustrepo~website]↗ collect additional crate recommendations and project listings.

You may also consult older resources, such as:

- [Best of Rust Crates][reddit~best-of-rust-crates]↗.
- [Rust Starter Pack][rust-starter-pack~website]↗{{hi:Rust starter pack}}.
- [![stdx][c~stdx~repo~badge]][c~stdx~repo]↗{{hi:stdx}}.

## Related Topics

- [[state_of_the_art | State of the Rust Ecosystem]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
  accuracy review: links and crate selection
</div>
