# TODO

revisit workflow.yml; make new dockerfile work with it; revisit CI section of main README.md

polish the intro

add GA

finish to review examples marked `ignore`

review example testing using libraries outside of `std` - make mdbook-keeper work or use rustdoc / doctest instead

make it work: mdbook test --library-path /cargo-target-rust_howto/target/debug/deps/

Search within the book

<https://endler.dev/2019/tinysearch/>
<https://stork-search.net/>
<https://github.com/lucaong/minisearch>
<https://lucaongaro.eu/blog/2019/01/30/minisearch-client-side-fulltext-search-engine.html>
<https://github.com/typesense/typesense>
<https://news.ycombinator.com/item?id=23473303>

Algolia

<https://github.com/rust-unofficial/awesome-rust>

<https://github.com/rust-unofficial/patterns>

<https://github.com/brson/stdx>

<https://opheron.github.io/rust-starter-pack/>

explanation text for language pages

license

recreate index.hbs in theme

finish cd_ci.md

finish improve_speed

finish Cargo Plugins / cargo.md

finish faster linking

web examples into separate section

review Install Rust

review Configuration page

finish Option page

map, unwrap_or

CLI: clap
[Clap](https://github.com/clap-rs/clap)

move clap / CLI example to xmpl

database access:

- sqlx [sqlx](https://github.com/launchbadge/sqlx)
- seaORM

CI / CD: rust-cache

cargo outdated / audit / license / deny

review deny.toml

<https://lib.rs/crates/cargo-hack>

<https://github.com/matklad/cargo-xtask>

<https://github.com/ThePuzzlemaker/cargo-crates>

review rustfmt.toml

IDEs - add latest JetBrains tooling

Loco

[Loco article](https://www.shuttle.rs/blog/2023/12/28/using-loco-rust-rails)

more details in the standard library section

add more to tokio.md

- async main
- spawning
- networking
- 'static and Send constraints
- I/O
- Join, select
- <https://tokio.rs/tokio/topics/tracing-next-steps>
- [Async stream](https://github.com/tokio-rs/async-stream)

add more to async_channels

<https://docs.rs/async-channel/latest/async_channel/>

<https://ryhl.io/blog/async-what-is-blocking/>

<https://ryhl.io/blog/actors-with-tokio/>

<https://docs.rs/tokio/latest/tokio/sync/mpsc/index.html#communicating-between-sync-and-async-code>

Tower

[Building a SaaS with Rust and Next.js](<https://joshmo.bearblog.dev/lets-build-a-saas-with-rust/>)

finish actor.md

finish Comparison to other languages

AWS
<https://docs.aws.amazon.com/sdk-for-rust/latest/dg/lambda.html>
<https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples>
<https://github.com/awsdocs/aws-doc-sdk-examples>

walkdir, notify, indicatif, ratatui

- regex
[Regex](https://github.com/rust-lang/regex)
- time
- chrono

<https://lib.rs/crates/serde_json>

[monostate for serde](https://github.com/dtolnay/monostate)

[serde-ignored](https://github.com/dtolnay/serde-ignored)

traits

[dyn clone](https://github.com/dtolnay/dyn-clone)

errors: color-eyre, eyre

testing: Approx example
[fuzzing](https://github.com/rust-fuzz/afl.rs)

- axum
- loco
- reqwest
- tonic

- redis
- mongodb
- elasticsearch

Macros

- cargo expand
- syn, quote, paste

[paste](https://github.com/dtolnay/paste)

[proc-macro2](https://github.com/dtolnay/proc-macro2)

[syn](https://github.com/dtolnay/syn)

[proc macro workshop](https://github.com/dtolnay/proc-macro-workshop)

[Watt](https://github.com/dtolnay/watt)

[Dyn Clone]( https://github.com/dtolnay/dyn-clone )

egui

Badges

rust and Docker - multistage builds

rust + docker compose

rust + Mongo example

rust axum example

shuttle.rs example

Windows

[windows-rs](https://github.com/microsoft/windows-rs)

[native windows gui](https://github.com/gabdube/native-windows-gui)

CLI

[open-rs](https://github.com/Byron/open-rs)

Tools

[startship]( https://github.com/starship/starship )

[Bacon]( https://github.com/Canop/bacon )

[Roogle]( https://roogle.hkmatsumoto.com/search )

[Roogle]( https://github.com/roogle-rs/roogle )

[Bat]( https://github.com/sharkdp/bat )

Data

[polars](https://github.com/pola-rs/polars)

GPU

[rust-gpu](https://github.com/EmbarkStudios/rust-gpu)

Python

[pyOxidizer](https://github.com/indygreg/PyOxidizer)

Books

<https://github.com/nnethercote/perf-book>

[Salsa](https://github.com/salsa-rs/salsa)

Games

[Bevy](https://github.com/bevyengine/bevy )

Languages

[Rust quizz](https://dtolnay.github.io/rust-quiz)

## Topics of interest

- http crate, hyper
- pyo3
- tokio, async-std and related
- more database examples, including object databases, graph databases, BonsaiDB
- Raft Consensus library
- advanced data structures
- basic and advanced TCP/IP networking
- file system traversal
- zip files and other archives
- sound, graphics, game engines
- GTK, Qt, FLTK, Bevy + eGUI, other UI toolkits
- GPU processing, CUDA
- machine learning, Tensorflow
- high-performance computing: OpenMP, etc.
- network filesystems
- statistics, math, bignum libraries
- crypto, SSL, SSH, other public key encryption, X.509, RusTLS
- social media APIs
- personal file sharing: OwnCloud, etc.
- search engines
- AWS and other cloud services
- buffer pools, garbage collection, or other reference-counted examples
- IPv6 address processing
- authentication / authorization: Oauth2, LDAP/AD, DNS lookups
- cloud stuff: LB, status reporting (Vigil), routing, orchestration, containers
- version control: libgit2: clone, change branches, create commits, push, pull
- cargo & project integration via cargo-edit
- Games: bevy
