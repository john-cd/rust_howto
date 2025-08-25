# TO REVIEW

## AI {#skip}

- [foreverVM][forevervm~website]↗: The sessionless code interpreter. ForeverVM is a code execution API to securely run arbitrary Python code in a remote sandbox and get back results.

## CD / CI {#skip}

- [Run a Database in GitHub Actions, Persisting Data to S3, and Access it Publicly][blog~use-github-actions-as-database]↗.

## Crates.io {#skip}

- [`crates.io`: development update][blog~crates-io-development-update]↗.

## Dataflow Graphs {#skip}

- [`fractalide`][fractalide~repo]↗: Reusable Reproducible Composable Software.

## Data Structures {#skip}

- [reactive_stores][c~reactive_stores~lib.rs]↗ - data structures in Rust.
- [reactive_graph][c~reactive_graph~lib.rs]↗.

---

[![optics][c~optics~docs~badge]][c~optics~docs] [![optics~crates.io][c~optics~crates.io~badge]][c~optics~crates.io] [![optics~repo][c~optics~repo~badge]][c~optics~repo] [![optics~lib.rs][c~optics~lib.rs~badge]][c~optics~lib.rs]{{hi:optics}}{{hi:No_std}}{{hi:Iso}}{{hi:Lens}}{{hi:Prism}}{{hi:optics}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

A `no_std`-compatible optics library providing composable lenses, prisms, isomorphisms, and fallible isomorphisms.

## Distributed Systems {#skip}

[Open-source training courses about distributed database and distributed systems][talent-plan~repo]↗.

## Enterprise Backend {#skip}

### Foundations {#skip}

[![foundations][c~foundations~docs~badge]][c~foundations~docs] [![foundations~crates.io][c~foundations~crates.io~badge]][c~foundations~crates.io] [![foundations~repo][c~foundations~repo~badge]][c~foundations~repo] [![foundations~lib.rs][c~foundations~lib.rs~badge]][c~foundations~lib.rs]{{hi:foundations}}{{hi:Settings}}{{hi:Service}}{{hi:Metrics}}{{hi:Telemetry}}{{hi:Seccomp}} [![cat~external-ffi-bindings][cat~external-ffi-bindings~badge]][cat~external-ffi-bindings]{{hi:External FFI bindings}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}} [![cat~config][cat~config~badge]][cat~config]{{hi:Configuration}} [![cat~memory-management][cat~memory-management~badge]][cat~memory-management]{{hi:Memory management}}

A Rust service foundations library.

- [Introducing Foundations - our open source Rust service foundation library][c~foundations~blog]↗.
- [Trying Out Cloudflare's `foundations` Library for Rust][blog~trying-out-cloudflare-foundations-library]↗.

### Graceful Upgrades {#skip}

[![bye][c~bye~docs~badge]][c~bye~docs] [![bye~crates.io][c~bye~crates.io~badge]][c~bye~crates.io] [![bye~repo][c~bye~repo~badge]][c~bye~repo] [![bye~lib.rs][c~bye~lib.rs~badge]][c~bye~lib.rs]{{hi:bye}}

A library for graceful shutdown with no downtime.

A system for graceful upgrades: it's a fork+exec mechanism with a Linux pipe for passing data between the original process and the "upgraded" process. It's designed to work well with `systemd` for zero-downtime upgrades. In production I use it alongside systemd's socket activation, but it should be tweakable to work with alternatives. It mostly follows systemd conventions, so it can be dropped into a typical service setup without too much fuss.


## Example Code {#skip}

- [`rust-shed`][rust-shed~repo]↗: Repository containing Rust crates common between other Facebook open source projects (like Mononoke or Eden).

## Futures {#skip}

[![want][c~want~docs~badge]][c~want~docs] [![want~crates.io][c~want~crates.io~badge]][c~want~crates.io] [![want~repo][c~want~repo~badge]][c~want~repo] [![want~lib.rs][c~want~lib.rs~badge]][c~want~lib.rs]{{hi:want}}{{hi:Channel}}{{hi:Async}}{{hi:Futures}}

Detect when another Future wants a result.

## Infra / Dev {#skip}

- [Sonatype supports secure development in Rust][blog~sonatype-supports-secure-development-in-rust]↗.
- [Why You Need sccache][blog~why_you_need_sccache]↗.

---

[![samoyed][c~samoyed~docs~badge]][c~samoyed~docs] [![samoyed~crates.io][c~samoyed~crates.io~badge]][c~samoyed~crates.io] [![samoyed~repo][c~samoyed~repo~badge]][c~samoyed~repo] [![samoyed~lib.rs][c~samoyed~lib.rs~badge]][c~samoyed~lib.rs]{{hi:samoyed}}{{hi:Git}}{{hi:Git-hooks}}{{hi:Husky}}{{hi:Git-hooks-manager}}

A single-binary Git hooks manager.

---

- [Every infrastructure decision I endorse or regret after 4 years running infrastructure at a startup][blog~every-infrastructure-decision-i-endorse-or-regret-after-4-years-running-infrastructure-at-a-startup]↗.

## Language Features {#skip}

review
[6 Rust programming mistakes to watch out for][blog~6-rust-programming-mistakes-to-watch-out-for]↗.

## Layouts {#skip}

- [Other reprs][book~nomicon~other-reprs] - The Rustonomicon.

## List of Crates Available in the Rust Playground {#skip}

- [rust-lang/rust-playground][rust-playground-cargo.toml~repo]↗.

## Reviews / Evolution of Rust {#skip}

- [A Review of Rust in 2024][blog~rust-review-2024]↗: What Next?.

- [Why Rust is Becoming a Contender in AI Development][blog~why-rust-is-becoming-a-contender-in-ai-development]↗.

- [Rust in 2025][blog~lang-interop-extensibility]↗: Language interop and the extensible compiler.

## Semver Checks {#skip}

[![cargo-semver-checks][c~cargo-semver-checks~docs~badge]][c~cargo-semver-checks~docs] [![cargo-semver-checks~crates.io][c~cargo-semver-checks~crates.io~badge]][c~cargo-semver-checks~crates.io] [![cargo-semver-checks~repo][c~cargo-semver-checks~repo~badge]][c~cargo-semver-checks~repo] [![cargo-semver-checks~lib.rs][c~cargo-semver-checks~lib.rs~badge]][c~cargo-semver-checks~lib.rs]{{hi:cargo-semver-checks}}{{hi:Cargo}}{{hi:Crate}}{{hi:Linter}}{{hi:Check}}{{hi:Semver}} [![cat~command-line-utilities][cat~command-line-utilities~badge]][cat~command-line-utilities]{{hi:Command line utilities}} [![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]{{hi:Cargo plugins}}

Scan your Rust crate for semver violations.

---

- [Making SemVer Breakage Obvious – GSoC 2025][blog~making-semver-breakage-obvious]↗.

## Unstable Features {#skip}

- [Coroutine in `std::ops`][c~std::ops::Coroutine~docs]↗.
- [Unstable Features][book~cargo~unstable]↗ - The Cargo Book.

## VS Code {#skip}

- [Fixing Rust memory allocation slowdown in VS Code on Windows][blog~vs-code-debug-heap]↗.

## Web {#skip}

- [`geckodriver`][geckodriver~repo]↗: WebDriver for Firefox.
- [Pake][pake~repo]↗: Turn any webpage into a desktop app with Rust.

cover [Hands-On Microservices with Rust 2018 (GitHub)][book~Hands-On-Microservices-with-Rust~repo]↗.
review in particular https://github.com/PacktPublishing/Hands-On-Microservices-with-Rust/blob/master/Chapter08/chat/src/lib.rs

## Zed {#skip}

- [zed hanging on startup because of 'unsupportedversion' Issue #14126][zed-issue-14126-comment~repo]↗.
- [zed hanging on startup because of 'unsupportedversion' Issue #14126][zed-issue-14126~repo]↗.

## Satire {#skip}

- [hello-world.rs][hello-world.rs~repo]↗: Memory safe, blazing fast, configurable, minimal hello world written in Rust, in a few lines of code with few(1092) dependencies.

## To Sort {#skip}
