# TO REVIEW

## Std {#skip}

- [black_box in `core::hint`][c~core::hint::black_box~docs]↗.
- [`std::pin`][c~std::pin~projections-and-structural-pinning~docs]↗.
- [`core::iter`][c~core::iter~docs]↗.

[tour-of-rusts-standard-library-traits][tour-of-rusts-standard-library-traits].
[tour-of-rusts-standard-library-traits]: https://github.com/pretzelhammer/rust-blog/blob/master/posts/tour-of-rusts-standard-library-traits.md

[stackoverflow~understanding-the-send-trait][stackoverflow~understanding-the-send-trait].
[stackoverflow~understanding-the-send-trait]: https://stackoverflow.com/questions/59428096/understanding-the-send-trait

## Actors

https://riker.rs/actors
https://github.com/uazu/stakker

## AI {#skip}

- [foreverVM][forevervm~website]↗: The sessionless code interpreter. ForeverVM is a code execution API that allows you to securely run arbitrary Python code in a remote sandbox and get back results.

## Build your own x {#skip}

This repository is a compilation of well-written, step-by-step guides for re-creating our favorite technologies from scratch: [build-your-own-x~website][build-your-own-x~website].

[build-your-own-x~website]: https://github.com/codecrafters-io/build-your-own-x

## Books

The online version of the Rust book titled "Rust for C Programmers" got a dedicated website at www.rust-for-c-programmers.com. Despite the title, the book doesn’t require prior experience with the C language. The name is mainly meant to indicate that the book is not aimed at complete beginners who have never written code or lack any basic understanding of systems programming. That said, even newcomers should find it accessible, though they may occasionally need to refer to supplementary material.

The book focuses on teaching Rust’s core concepts with clarity and precision, avoiding unnecessary verbosity. At around 500 pages, it covers most of Rust's fundamentals:

https://www.rust-for-c-programmers.com

https://dev.rust-quest.com/en/

## CD / CI {#skip}

- [Run a Database in GitHub Actions, Persisting Data to S3, and Access it Publicly][blog~use-github-actions-as-database]↗.

## Cloud {#skip}

- [Deploy app servers close to your users][fly~website].

https://github.com/shuttle-hq/shuttle-examples


## Code Example

ChromeOS Virtual Machine Monitor is written in Rust with over 300k LoC
People sometimes ask for examples of "good" Rust code. This repository contains many well-documented crates that appear from a glance to follow what I consider "idiomatic" Rust. There is a book using mdBook and thorough rustdoc documentation for all crates. Just thought I'd share if someone wants code to read!

https://github.com/google/crosvm/tree/main

https://crosvm.dev/book/

https://crosvm.dev/doc/crosvm/

## Code Generators {#skip}

https://loco.rs/

https://gerust.rs/

https://github.com/Keats/kickstart

[protypo~github]: https://github.com/dinosath/protypo

[rise~github]: https://github.com/Dexter2038/rise (early)

## Compilers / Interpreters

cover [book~writing-interpreters-in-rust~github]

## Crates.io {#skip}

- [`crates.io`: development update][blog~crates-io-development-update]↗.

## Data {#skip}

https://crates.io/crates/jsonschema
https://crates.io/crates/jsonpath-rust
https://datawithrust.com
https://docs.rs/plotters/latest/plotters
https://docs.pola.rs
https://github.com/rust-ndarray/ndarray

## Databases

I have seen quite a number of positive mentions of Cornucopia on this sub, but for some reason no mentions of its maintained fork - Clorinde.

If you are not familiar with Cornucopia, it is kinda like SQLc for Go - you write a query in Postgres SQL and then use cli to generate checked Rust code. So no macro compilation time overhead or complex types that are hard for the rust-analyzer to handle. It uses rust-postgres driver under the hood, so it supports query pipelining and the perfomance should be pretty good as well.

[clorinde~github]: https://github.com/halcyonnouveau/clorinde

[cornucopia~github]: https://github.com/cornucopia-rs/cornucopia

## Dataflow Graphs {#skip}

- [`fractalide`][fractalide~github]: Reusable Reproducible Composable Software.

## Data Structures {#skip}

- [reactive_stores — data structures in Rust][c~reactive_stores~lib.rs].
- [reactive_graph][c~reactive_graph~lib.rs].

[optics~crates.io][optics~crates.io].
[optics~crates.io]: https://crates.io/crates/optics

## Embedded {#skip}

cover [5-rust-runtimes-every-embedded-developer-needs-to-know]: https://www.designnews.com/embedded-systems5-rust-runtimes-every-embedded-developer-needs-to-know

[drone-os~website][drone-os~website] [drone-os~website]: https://www.drone-os.com

[blog~rust-procedural-macros-replace-panic][blog~rust-procedural-macros-replace-panic] [blog~rust-procedural-macros-replace-panic]: https://www.infoq.com/articles/rust-procedural-macros-replace-panic

## Enterprise Backend

### Foundations {#skip}

[foundations~github]: https://github.com/cloudflare/foundations
https://blog.cloudflare.com/introducing-foundations-our-open-source-rust-service-foundation-library/
https://cprimozic.net/blog/trying-out-cloudflare-foundations-library/?utm_source=tldrnewsletter

### Graceful Upgrades {#skip}

[bye~github]: https://github.com/dgagn/bye A system for graceful upgrades: it's a fork+exec mechanism with a Linux pipe for passing data between the original process and the "upgraded" process. It's designed to work well with `systemd` for zero-downtime upgrades. In production I use it alongside systemd's socket activation, but it should be tweakable to work with alternatives. It mostly follows systemd conventions so you can drop it into a typical service setup without too much fuss.
If you're doing long-lived services in Rust and want painless, no-downtime upgrades, I'd love for you to give it a try (or tear it apart, your choice

## Example Code {#skip}

- [`rust-shed`][rust-shed~github]: Repository containing Rust crates common between other Facebook open source projects (like Mononoke or Eden).

## Futures {#skip}

[c~want~docs]. [c~want~docs]: https://docs.rs/want

## GPU {#fix}

[rust-on-every-gpu]: https://rust-gpu.github.io/blog/2025/07/25/rust-on-every-gpu/

## GUI {#skip}

[egui.info~examples~website][egui.info~examples~website] [egui.info~examples~website]: https://egui.info/examples/

[Porting a cross-platform GUI application to Rust][blog~porting-a-cross-platform-gui-application-to-rust]
[blog~porting-a-cross-platform-gui-application-to-rust]: https://hacks.mozilla.org/2024/04/porting-a-cross-platform-gui-application-to-rust/

## Hash

[foldhash][foldhash].
[foldhash]: https://github.com/orlp/foldhash

## Infra / Dev {#skip}

- [Sonatype supports secure development in Rust][blog~sonatype-supports-secure-development-in-rust]↗.

[blog~why_you_need_sccache][blog~why_you_need_sccache].
[blog~why_you_need_sccache]: https://elijahpotter.dev/articles/why_you_need_sccache

[samoyed~github]: https://github.com/nutthead/samoyed

[blog~every-infrastructure-decision-i-endorse-or-regret-after-4-years-running-infrastructure-at-a-startup]: https://cep.dev/posts/every-infrastructure-decision-i-endorse-or-regret-after-4-years-running-infrastructure-at-a-startup/

## Jujutsu {#skip}

[jujutsu-tutorial]: https://steveklabnik.github.io/jujutsu-tutorial/hello-world/creating-new-changes.html

https://github.com/jkoppel/jj-workshop

[gg][gg].
[gg]: https://github.com/gulbanana/gg

## Jobs {#skip}

- [filtra][filtra.io-rust-sep-24~website].

[matic-apr-25]: https://filtra.io/rust/interviews/matic-apr-25

## Language Features {#skip}

- [Converting between different collection types][blog~converting-between-different-collection-types]↗: from `Vec` to `HashSet` or `HashMap`.

review [common-rust-lifetime-misconceptions][common-rust-lifetime-misconceptions]
[common-rust-lifetime-misconceptions]: https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md

review [6-rust-programming-mistakes-to-watch-out-for]: https://www.infoworld.com/article/2337218/6-rust-programming-mistakes-to-watch-out-for.html

### Const fn

https://felixwrt.dev/posts/const-fn

### Mut Refs

https://www.reddit.com/r/rust/comments/1mr3kt4/the_mutable_reference_i_return_from_a_function_in/

### methods expecting a closure can also accept an enum variant (tuple-like)

https://www.reddit.com/r/rust/comments/1l8okiy/im_blown_that_this_is_a_thing/

Enum variants and tuple-like structs can be used as function items in general https://doc.rust-lang.org/reference/types/function-item.html

## Layouts {#skip}

- [Other reprs - The Rustonomicon][book~nomicon~other-reprs].

## List of Crates Available in the Rust Playground {#skip}

- [rust-lang/rust-playground][rust-playground-cargo.toml~github].

## ML / Search {#skip}

- [`hora`][hora~github]: Efficient approximate nearest neighbor search algorithm collections library written in Rust.
- [mistral.rs~github] [mistral.rs~github]: https://github.com/EricLBuehler/mistral.rs

## Numbers

[fraction][fraction~crates.io].

[fraction~crates.io]: https://crates.io/crates/fraction

## ORMs {#skip}

[SeaORM]: https://www.sea-ql.org/SeaORM

## Patterns {#skip}

[blog~rust-patterns-ref][blog~rust-patterns-ref].
[blog~rust-patterns-ref]: http://xion.io/post/code/rust-patterns-ref.html

## Parsing {#skip}

https://github.com/rosetta-rs/parse-rosetta-rs

## Python

https://www.infoworld.com/article/2335770/how-to-use-rust-with-python-and-python-with-rust.html
https://docs.astral.sh/uv/
https://ofek.dev/pyapp/latest/

## Reflection {#skip}

[reflect~github][reflect~github] [reflect~github]: https://github.com/dtolnay/reflect

## Reviews / Evolution of Rust {#skip}

- [A Review of Rust in 2024][blog~rust-review-2024]↗: What Next?.

- [Why Rust is Becoming a Contender in AI Development][blog~why-rust-is-becoming-a-contender-in-ai-development].

- [Rust in 2025][blog~lang-interop-extensibility]↗: Language interop and the extensible compiler.

## Robotics {#skip}

https://github.com/roboplc/ehmi
https://www.bohemia-automation.com/software/roboplc/

## Scripting

https://blog.nlnetlabs.nl/introducing-roto-a-compiled-scripting-language-for-rust/
https://github.com/NLnetLabs/roto/tree/main

https://docs.rs/rhai/latest/rhai/

## Semver Checks {#skip}

https://github.com/obi1kenobi/cargo-semver-checks

https://glitchlesscode.ca/posts/2025-05-30a/

## TOML {#skip}

[taplo~github][taplo~github]: A TOML toolkit written in Rust.
[taplo~github]: https://github.com/tamasfe/taplo

## Unstable Features {#skip}

- [Coroutine in `std::ops`][c~std::ops::Coroutine~docs]↗.
- [Unstable Features][book~cargo~unstable]↗ - The Cargo Book.

## VS Code {#skip}

- [Fixing Rust memory allocation slowdown in VS Code on Windows][blog~vs-code-debug-heap]↗.

## WASM

https://rustwasm.github.io/docs/book/introduction.html
https://docs.wasmtime.dev/introduction.html
https://github.com/jetli/rust-yew-realworld-example-app
https://github.com/flosse/rust-web-framework-comparison#frontend-frameworks-wasm

## Web {#skip}

- [`geckodriver`][geckodriver~github]: WebDriver for Firefox.
- [Pake][pake~github]: Turn any webpage into a desktop app with Rust.

cover [Hands-On-Microservices-with-Rust]: https://github.com/PacktPublishing/Hands-On-Microservices-with-Rust/blob/master/Chapter08/chat/src/lib.rs

[rust-web-framework-comparison~github]: https://github.com/flosse/rust-web-framework-comparison
[book~async-book]: https://rust-lang.github.io/async-book
[tokio~website]: https://tokio.rs
[tokio~examples~github]: https://github.com/tokio-rs/tokio/tree/master/examples
https://github.com/snamiki1212/realworld-v1-rust-actix-web-diesel
https://gill.net.in/posts/auth-microservice-rust-actix-web1.0-diesel-complete-tutorial/
https://github.com/Bechma/realworld-actix-fullstack
https://github.com/Apress/practical-rust-web-projects/tree/main
https://github.com/JoeyMckenzie/realworld-rust-axum-sqlx
https://github.com/tokio-rs/axum/tree/main/examples

## Written in Rust {#skip}

[yazi~github][yazi~github]:Blazing fast terminal file manager written in Rust, based on async I/O.
[yazi~github]: https://github.com/sxyazi/yazi

[evcxr_jupyter~github][evcxr_jupyter~github]:  A Rust REPL / Jupyter kernel.
[evcxr_jupyter~github]: https://github.com/evcxr/evcxr/blob/main/evcxr_jupyter/README.md

review [hyperswitch]: https://github.com/juspay/hyperswitch/tree/main

[kget~github][kget~github].
[kget~github]: https://github.com/davimf721/KGet

## Zed {#skip}

- [zed hanging on startup because of 'unsupportedversion' · Issue #14126 · zed-industries/zed][zed-issue-14126-comment~github].
- [zed hanging on startup because of 'unsupportedversion' · Issue #14126 · zed-industries/zed][zed-issue-14126~github].

## Satire {#skip}

- [hello-world.rs][hello-world.rs~github]: Memory safe, blazing fast, configurable, minimal hello world written in rust() in a few lines of code with few(1092) dependencies.

## TO SORT {#skip}

[talent-plan][talent-plan]: open source training courses about distributed database and distributed systems.
[talent-plan]: https://github.com/pingcap/talent-plan/tree/master

## Architecture {#skip}

[tangleguard~website]: https://docs.tangleguard.com
