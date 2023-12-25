# TODO

review example testing using libraries outside of `std` - make mdbook-keeper work or use doctest instead

write the intro

main README.md document the xmpl folder

add GA

{{#include file.rs:2}}
{{#include file.rs::10}}
{{#include file.rs:2:}}
{{#include file.rs:2:10}}
{{#playground example.rs editable}}
https://rust-lang.github.io/mdBook/format/mdbook.html

explanation text for language pages

finish cd_ci.md
finish improve_speed
finish Cargo Plugins / cargo.md
finish faster linking

separate web

review Install Rust

review Configuration page

finish Option page

map, unwrap_or

CLI: clap

move clap / CLI example to xmpl

database access:

- sqlx
- seaORM

incorporate examples

CI / CD: rust-cache

cargo outdated / audit / license / deny

review deny.toml

<https://lib.rs/crates/cargo-hack>

review rustfmt.toml

IDEs - add latest JetBrains tooling



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

<https://docs.rs/async-channel/latest/async_channel/>

<https://ryhl.io/blog/async-what-is-blocking/>

<https://ryhl.io/blog/actors-with-tokio/>

<https://docs.rs/tokio/latest/tokio/sync/mpsc/index.html#communicating-between-sync-and-async-code>

Tower

finish actor.md

finish Comparison to other languages

AWS
<https://docs.aws.amazon.com/sdk-for-rust/latest/dg/lambda.html>
<https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples>
<https://github.com/awsdocs/aws-doc-sdk-examples>

walkdir, notify, indicatif, ratatui

- regex
- time
- chrono

<https://lib.rs/crates/serde_json>

errors: color-eyre, eyre

testing: Approx example

- axum
- loco
- reqwest
- tonic

- redis
- mongodb
- elasticsearch

macros

- cargo expand
- syn, quote, paste

egui

Badges

rust and Docker - multistage builds

rust + docker compose

rust + Mongo example

rust axum example

shuttle.rs example

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
