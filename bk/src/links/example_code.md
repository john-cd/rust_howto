# Example Code and Templates

{{#include example_code.incl.md}}{{hi:Example code}}

## Small Examples and Exercises {#examples-exercises}

- [Rust by Example][book~rust-by-example]↗: Learn Rust features with small, targeted examples. Starting with "hello world" and moving up to more complex features like traits and generics.
- [Rustlings][c~rustlings~repo]↗: Small exercises to get you used to reading and writing Rust code, including reading and responding to compiler error messages.
- [100 Exercises to Learn Rust][100-exercises-to-learn-rust~repo]↗.
- [Advent of Code][adventofcode~website]↗: Language-agnostic programming challenges that can be done in Rust.
- [Exercism][exercism-rust~website]↗.
- [Project Euler][projecteuler~website]↗: Language-agnostic programming challenges focused mostly on math problems.
- [Rust by Practice][practice.course.rs~website]↗.
- The [Rust Cookbook][book~rust-cookbook]↗: A collection of simple examples that demonstrate good practices to accomplish common programming tasks, using the crates of the Rust ecosystem. Some examples are outdated.
- [Rust Mastery Exercises][app.codecrafters.io~rust]↗.
- [Rust Quizz][rust-quizz~website]↗.
- [Rust Practice Questions][rust-practise-questions~repo]↗.
- Top 15 Rust projects to elevate your skills: [Rust Practice Projects][blog~rust-practice-projects]↗.

You can use the [Rust Playground][rust-playground~website]↗ to quickly test snippets of code.

## Notable Rust Code Repositories {#notable-repos}

Reading code is the fastest way to learn idiomatic Rust. Use the following links to locate useful Rust repositories.

### Lists of Popular Rust Repositories {#repositories}

- GitHub Topic: ["Rust"][topics-rust~repo]↗: Popular repositories on GitHub.
- [Trending GitHub repos][trending-rust~repo]↗.
- [Awesome Rust][awesome-rust~repo]↗: A large curated list of Rust resources, including code examples.
- [Open-source Projects Categorized as "Rust"][libhunt-topic-rust~website]↗ (`libhunt.com`).
- [`rustrepo.com`][rustrepo~website]↗: A curated list of awesome Rust resources.
- [Top 100 Best GitHub Repositories for Rust][bestofgithub-rust~website]↗.

### "RealWorld" Examples {#realworld-examples}

The ["Real World" Example App][realworld~example-apps]↗ [(website)][docs.realworld.build~website]↗, dubbed "the mother of all demo apps", is a fullstack [`medium.com`][medium.com~website]↗ clone implemented in many languages and frameworks. A list of Rust implementations is [found here][realworld-rust~website]↗.

For example, [`realworld-rust-axum-sqlx`][realworld~rust-axum-sqlx~repo]↗ is a full-stack RealWorld implementation using [`axum`][c~axum~docs]↗{{hi:axum}}, [`sqlx`][c~sqlx~docs]↗{{hi:sqlx}}, and [`yew`][c~yew~docs]↗{{hi:yew}}.

### ChromeOS Virtual Machine Monitor {#crosvm}

The ChromeOS Virtual Machine Monitor [(github)][crosvm~repo]↗ is written in Rust, with over 300k LoC. Its repository contains many well-documented crates. There is a [book][book~crosvm]↗ and thorough [documentation][crosvm~docs~website]↗ for all crates.

### Build Your Own X {#build-your-own-x}

[Build Your Own X][build-your-own-x~repo]↗ is a compilation of well-written, step-by-step guides for re-creating your favorite technologies from scratch. It includes several Rust projects.

### Source Code for Rust Books {#source-code-for-rust-books}

You may also consult the code examples from various Rust books:

- [Code][book~zero-to-production~repo]↗ for the ["Zero to Production in Rust"][book~zero-to-production]↗ book.
- [Source Code][practical-rust-web-projects~repo]↗ for ["Practical Rust Web Projects][book~Practical-Rust-Web-Projects-Applications]↗: Building Cloud and Web-Based Applications", 1st ed., by Shing Lyu.

## Templates {#rust-templates}

[![cargo-generate][c~cargo-generate~docs~badge]][c~cargo-generate~docs] [![cargo-generate~crates.io][c~cargo-generate~crates.io~badge]][c~cargo-generate~crates.io] [![cargo-generate~repo][c~cargo-generate~repo~badge]][c~cargo-generate~repo] [![cargo-generate~lib.rs][c~cargo-generate~lib.rs~badge]][c~cargo-generate~lib.rs]{{hi:cargo-generate}}

[`cargo-generate`][c~cargo-generate~repo]↗ helps you get up and running quickly with a new Rust project by leveraging a pre-existing `git` repository as a template.

You will find `cargo-generate`-ready templates in [this list][cargo-generate~repo]↗. Beyond `cargo-generate`, search for Rust projects on [GitHub][rust-templates~repo]↗.

For example, you will find a full template for a REST API written in Rust, engineered for maximum testability in the [`rust-rest`][rust_rest~repo]↗ repo.

## Related Topics {#related-topics .skip}

- [[blogs_and_forums | Blogs and Forums]].
- [[books | Books]].
- [[examples_index | Examples]].
- [[videos_and_podcasts | Videos and Podcasts]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
