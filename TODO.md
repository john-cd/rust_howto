# TODO

tower_http example polish; other examples have been checked against the rust playground

polish the intro

review example testing using libraries outside of `std` - WIP; review generate.sh

- make mdbook-keeper work or use rustdoc / doctest instead
- make it work: mdbook test --library-path /cargo-target-rust_howto/target/debug/deps/

update GA settings

license

logo / favicon

Search within the book

<https://endler.dev/2019/tinysearch/>
<https://stork-search.net/>
<https://github.com/lucaong/minisearch>
<https://lucaongaro.eu/blog/2019/01/30/minisearch-client-side-fulltext-search-engine.html>
<https://github.com/typesense/typesense>
<https://news.ycombinator.com/item?id=23473303>

Algolia

Review <https://github.com/rust-unofficial/awesome-rust> - already in links.md

Review <https://github.com/rust-unofficial/patterns> - already in links.md

explanation text for language pages

web examples into separate section

review Install Rust

[Error Chain]( https://docs.rs/error-chain/latest/error_chain/ )

finish cd_ci.md

finish improve_speed

finish Cargo Plugins / cargo.md

finish faster linking

review Configuration page

finish Option page

map, unwrap_or

CLI: clap example

move clap / CLI example to xmpl

database access:

- sqlx
- seaORM

CI / CD: rust-cache

cargo outdated / audit / license / deny

review deny.toml

review rustfmt.toml

IDEs - add latest JetBrains tooling

Loco.md in drafts

more details in the standard library section

add more to tokio.md

- async main
- spawning
- networking
- 'static and Send constraints
- I/O
- Join, select
- <https://tokio.rs/tokio/topics/tracing-next-steps>

add more to async_channels

<https://ryhl.io/blog/async-what-is-blocking/>

<https://ryhl.io/blog/actors-with-tokio/>

<https://docs.rs/tokio/latest/tokio/sync/mpsc/index.html#communicating-between-sync-and-async-code>

Tower

finish actor.md

finish Comparison to other languages

AWS

walkdir, notify, indicatif, ratatui

- regex.md
- time
- chrono

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

egui

Badges

rust and Docker - multistage builds

rust + docker compose

rust + Mongo example

rust axum example

shuttle.rs example

windows.md

other_tools.md

Data

GPU.md

Python

Books

games.md

Languages

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
