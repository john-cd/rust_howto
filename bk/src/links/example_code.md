# Example Code and Templates

{{#include example_code.incl.md}}{{hi:Example code}}

You can use the [Rust Playground][rust-playground~website]↗ to quickly test snippets of code.

## Lists of Popular Rust Repositories {#repositories}

- [GitHub Topic: "Rust" topic][topics-rust~github]↗: Popular repositories on GitHub.
- [Trending GitHub repos][trending-rust~github]↗.
- [Awesome Rust][awesome-rust~github]↗: A large curated list of Rust resources, including code examples.
- [Open-source Projects Categorized as "Rust" (libhunt.com)][libhunt-topic-rust~website]↗.
- [`rustrepo.com`][rustrepo~website]↗: A curated list of awesome Rust resources.
- [Top 100 Best GitHub Repositories for Rust][bestofgithub-rust~website]↗.

## Templates {#rust-templates}

[![cargo-generate][c~cargo-generate~docs~badge]][c~cargo-generate~docs] [![cargo-generate~crates.io][c~cargo-generate~crates.io~badge]][c~cargo-generate~crates.io] [![cargo-generate~github][c~cargo-generate~github~badge]][c~cargo-generate~github] [![cargo-generate~lib.rs][c~cargo-generate~lib.rs~badge]][c~cargo-generate~lib.rs]{{hi:cargo-generate}}

[`cargo-generate`][c~cargo-generate~github]↗ helps you get up and running quickly with a new Rust project by leveraging a pre-existing git repository as a template.

You will find `cargo-generate`-ready templates in [this list][cargo-generate~github]↗. Beyond `cargo-generate`, search for Rust projects on [GitHub][rust-templates~github]↗.

For example, you will find a full template for a REST API written in Rust, engineered for maximum testability: [`rust-rest`][rust_rest~github]↗.

## Notable Repositories {#notable-repos}

### "RealWorld" Examples

The ["Real World" Example App][realworld~example-apps]↗ [(website)][docs.realworld.build~website]↗, dubbed "the mother of all demo apps", is a fullstack [`medium.com`][medium.com~website]↗ clone implemented in many languages and frameworks. A list of Rust examples is [found here][realworld-rust~website]↗.

For example, [`realworld-rust-axum-sqlx`][realworld~rust-axum-sqlx~github]↗ is a full-stack RealWorld implementation using [`axum`][c~axum~docs]↗{{hi:axum}}, [`sqlx`][c~sqlx~docs]↗{{hi:sqlx}}, and [`yew`][c~yew~docs]↗{{hi:yew}}.

### Source Code of the ChromeOS Virtual Machine Monitor

The ChromeOS Virtual Machine Monitor [(GitHub)][crosvm~github] is written in Rust, with over 300k LoC. Its repository contains many well-documented crates. There is a [book][book~crosvm] and thorough [documentation][crosvm~docs~website] for all crates.

### Build your own x

[Build your own x][build-your-own-x~github] is a compilation of well-written, step-by-step guides for re-creating our favorite technologies from scratch. Includes several Rust projects.

### Source Code for Rust Books

You may also consult the code examples from various Rust books:

- [Code][book~zero-to-production~github]↗ for the ["Zero To Production In Rust"][book~zero-to-production]↗ book.
- [Source Code][practical-rust-web-projects~github]↗ for ["Practical Rust Web Projects][book~Practical-Rust-Web-Projects-Applications]↗: Building Cloud and Web-Based Applications", 1st ed., by Shing Lyu.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
