<!-- TODO Reviewed ??-??-2026 -->

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
- [Rust Quiz][rust-quizz~website]↗.
- [Rust Practice Questions][rust-practise-questions~repo]↗.
- Top 15 Rust projects to elevate your skills: [Rust Practice Projects][blog~rust-practice-projects]↗.

You can use the [Rust Playground][rust-playground~website]↗ to quickly test snippets of code.

## Notable Rust Code Repositories {#notable-repos}

Reading code is the fastest way to learn idiomatic Rust. Use the following links to locate useful Rust repositories.

### Browser and Desktop Tools {#browser-desktop-tools}

- [`geckodriver`][geckodriver~repo]↗ is a WebDriver proxy for automating Firefox through Marionette. Use it with a WebDriver client for browser tests.
- [Pake][pake~repo]↗ packages websites as desktop apps with Rust and Tauri.

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

The ChromeOS Virtual Machine Monitor [(GitHub)][crosvm~repo]↗ is written in Rust and organized into many well-documented crates. See its [book][book~crosvm]↗ and [crate documentation][crosvm~docs~website]↗.

### Build Your Own X {#build-your-own-x}

[Build Your Own X][build-your-own-x~repo]↗ is a compilation of well-written, step-by-step guides for re-creating your favorite technologies from scratch. It includes several Rust projects.

### Source Code for Rust Books {#source-code-for-rust-books}

You may also consult the code examples from various Rust books:

- [Code][book~zero-to-production~repo]↗ for the ["Zero to Production in Rust"][book~zero-to-production]↗ book.
- [Source Code][practical-rust-web-projects~repo]↗ for ["Practical Rust Web Projects][book~Practical-Rust-Web-Projects-Applications]↗: Building Cloud and Web-Based Applications", 1st ed., by Shing Lyu.

## Templates {#rust-templates}

{{#badges cargo-generate}}

[`cargo-generate`][c~cargo-generate~repo]↗ helps you get up and running quickly with a new Rust project by leveraging a pre-existing `git` repository as a template.

You will find `cargo-generate`-ready templates in [this list][cargo-generate~repo]↗. Beyond `cargo-generate`, search for Rust projects on [GitHub][rust-templates~repo]↗.

For example, you will find a full template for a REST API written in Rust, engineered for maximum testability in the [`rust-rest`][rust_rest~repo]↗ repo.

## Focused Library Implementations to Study

When studying a library, read its examples alongside its current API and platform requirements. These projects illustrate different design boundaries rather than interchangeable application templates:

- [Cloudflare Foundations][c~foundations~repo] groups service configuration, telemetry, and runtime building blocks. Its [introductory article][c~foundations~blog] explains the original motivation; [an independent walkthrough][blog~trying-out-cloudflare-foundations-library] provides another worked example.
- [`bye`][c~bye~repo] explores process handover for graceful upgrades. Study its descriptor and process-lifecycle assumptions before applying the technique to a different service.
- [`rust-shed`][rust-shed~repo] collects reusable crates from Meta projects; inspect an individual crate's examples and dependencies rather than treating the repository as a single framework.
- [`reactive_graph`][c~reactive_graph~lib.rs] and [`reactive_stores`][c~reactive_stores~lib.rs] illustrate reactive state and store APIs. [`optics`][c~optics~repo] is another approach to composing access to nested values with lenses and related abstractions.
- [`want`][c~want~repo] demonstrates signaling demand between asynchronous producers and consumers. Its focused scope makes it useful when studying backpressure and task coordination.
- [Samoyed][c~samoyed~repo] provides an example of a Git hook manager. Compare its workflow with your repository's existing hook policy before adopting another tool.

## Explore the Playground and Experimental APIs

The [Rust Playground dependency manifest][rust-playground-cargo.toml~repo] identifies crates configured for the hosted playground. Availability there does not imply that every feature or native system dependency is available; use a local Cargo project when a recipe needs a custom configuration.

For experiments with unstable APIs, consult the [Cargo unstable-feature reference][book~cargo~unstable] and the [`Coroutine` documentation][c~std::ops::Coroutine~docs]. These are separate from stable Rust recipes. For representation and ABI experiments, the Rustonomicon's [other representations chapter][book~nomicon~other-reprs] explains the relevant layout attributes and limitations.

## Related Topics {#related-topics .skip}

- [[blogs_and_forums | Blogs and Forums]].
- [[books | Books]].
- [[examples_index | Examples]].
- [[videos_and_podcasts | Videos and Podcasts]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
  accuracy review: links and crate selection
</div>
