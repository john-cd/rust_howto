# Example Code and Templates

{{#include example_code.incl.md}}{{hi:Example code}}

You can use the [Rust Playground][rust-playground~website]↗ to quickly test snippets of code.

## Lists of Popular Rust Repositories {#repositories}

- [GitHub Topic: "Rust" topic][topics-rust~github]↗: popular repositories on GitHub.
- [Trending GitHub repos][trending-rust~github]↗.
- [Awesome Rust][awesome-rust~github]↗: a large curated list of Rust resources, including code examples.
- [Open-source Projects Categorized as "Rust" (libhunt.com)][libhunt-topic-rust~website]↗.
- [`rustrepo.com`][rustrepo~website]↗: A curated list of awesome Rust resources.
- [Top 100 Best GitHub Repositories for Rust][bestofgithub-rust~website]↗.

## "RealWorld" Examples {#realworld}

The ["Real World" Example App][realworld~example-apps]↗ [(website)][docs.realworld.build~website]↗, dubbed "the mother of all demo apps", is a fullstack [`medium.com`][medium.com~website]↗ clone implemented in many languages and frameworks. A list of Rust examples is [found here][realworld-rust~website]↗.

For example, [`realworld-rust-axum-sqlx`][realworld~rust-axum-sqlx~github]↗ is a full-stack RealWorld implementation using [`axum`][c~axum~docs]↗{{hi:axum}}, [`sqlx`][c~sqlx~docs]↗{{hi:sqlx}}, and [`yew`][c~yew~docs]↗{{hi:yew}}.

## Templates {#rust-templates}

[![cargo-generate][c~cargo-generate~docs~badge]][c~cargo-generate~docs] [![cargo-generate~crates.io][c~cargo-generate~crates.io~badge]][c~cargo-generate~crates.io] [![cargo-generate~github][c~cargo-generate~github~badge]][c~cargo-generate~github] [![cargo-generate~lib.rs][c~cargo-generate~lib.rs~badge]][c~cargo-generate~lib.rs]{{hi:cargo-generate}}

[`cargo-generate`][c~cargo-generate~github]↗ helps you get up and running quickly with a new Rust project by leveraging a pre-existing git repository as a template.

You will find `cargo-generate`-ready templates in [this list][cargo-generate~github]↗.

Beyond `cargo-generate`, search for Rust projects on [GitHub][rust-templates~github]↗.

For example, you will find a full template for a REST API written in Rust, engineered for maximum testability: [`rust-rest`][rust_rest~github]↗.

## Source Code for Rust Books {#rust-book-source-code}

You may also consult the code examples from various Rust books:

- [Code][book~zero-to-production~github]↗ for the ["Zero To Production In Rust"][book~zero-to-production]↗ book.
- [Source Code][practical-rust-web-projects~github]↗ for ["Practical Rust Web Projects][book~Practical-Rust-Web-Projects-Applications]↗: Building Cloud and Web-Based Applications", 1st ed., by Shing Lyu.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">

## ChromeOS Virtual Machine Monitor {#skip}

ChromeOS Virtual Machine Monitor is written in Rust with over 300k LoC
People sometimes ask for examples of "good" Rust code. This repository contains many well-documented crates that appear from a glance to follow what I consider "idiomatic" Rust. There is a book using mdBook and thorough rustdoc documentation for all crates. Just thought I'd share if someone wants code to read!

[crosvm~github]: https://github.com/google/crosvm/tree/main

[book~crosvm]: https://crosvm.dev/book

[crosvm~website]: https://crosvm.dev/doc/crosvm

## Build your own x {#skip}

This repository is a compilation of well-written, step-by-step guides for re-creating our favorite technologies from scratch: [build-your-own-x~website][build-your-own-x~website].

[build-your-own-x~website]: https://github.com/codecrafters-io/build-your-own-x

</div>
