# TODO

- utils:
  - retest all commands / subcommands

- merge dev branch into main! just prep:
  - fix failing examples
  - fix clippy warnings
  - create alternative SUMMARY.md

- polish extended SUMMARY.md
  - auto derive
  - config, concerns

- review GA / GSC - issues with redirects??

- fix commented examples

- Rename some of the link IDs in *refs.md - WIP

- pull request to little book of rust books

- incorporate cwd.rs example + any other new examples

- review generate.sh

- finish generation of urls from dependencies + append + sort + dedupe - WIP
  - update justfile

- review each new .md file in turn - see STATUS.md

finish cd_ci.md

finish improve_speed

- explanation text for language pages

- finish to review ignore / no_run examples

- finish links:
  - insert badges instead of (github) - see book/temp
  - create badge refdefs for links to crates.io, lib.rs, RBE book, etc

- finish utils
  - locate all autolink / inline references to external sites - WIP
  - identify duplicate links / labels
  - identify broken links
  - suggest label names based on URL type - WIP
  - generate categories.md
  - generate crates.md
  - autoreplace autolinks / inline links by ref links
  - generate or merge into *-refs.md files - WIP
  - identify .md files not in SUMMARY.md
  - identify .rs examples not used
  - review TODOs
  - consider moving utils to a separate repo? or publish as a separate crate?

- tower_http example polish; other examples have been checked against the rust playground

- license

- update thanks
  - add licenses to thanks page ?

- contributing
  - Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in _ by you, shall be licensed as __, without any additional terms or conditions.

Review <https://github.com/rust-unofficial/awesome-rust> - already in links.md

Review <https://github.com/rust-unofficial/patterns> - already in links.md

<https://www.reddit.com/r/rust/comments/15b9rl5/rust_tutorial_that_actually_teaches_rust/>

Link to <https://stevedonovan.github.io/rust-gentle-intro/>

review Configuration page

database access:

- sqlx
- seaORM

CI / CD: rust-cache

cargo outdated / audit / license / deny

<https://hoverbear.org/blog/rust-state-machine-pattern/>

review deny.toml

more details in the standard library section

add more to tokio.md

- async main
- spawning
- networking
- 'static and Send constraints
- I/O
- Join, select
- <https://tokio.rs/tokio/topics/tracing-next-steps>

<https://ryhl.io/blog/async-what-is-blocking/>

<https://ryhl.io/blog/actors-with-tokio/>

<https://docs.rs/tokio/latest/tokio/sync/mpsc/index.html#communicating-between-sync-and-async-code>

Tower

add to Comparison to other languages

AWS

walkdir

<https://docs.rs/flagset/latest/flagset/>

notify

indicatif, ratatui

- time
- chrono

traits

[dyn clone][dyn-clone]

[dyn-clone]: https://github.com/dtolnay/dyn-clone

testing: Approx example

[fuzzing][fuzzing-github]

[fuzzing-github]: https://github.com/rust-fuzz/afl.rs

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

add a rust axum example

shuttle.rs example

add to windows.md

add to Data

add to GPU.md

add to interop / Python

add to Books

Languages

finish actor.md

Search within the book

<https://endler.dev/2019/tinysearch/>
<https://stork-search.net/>
<https://github.com/lucaong/minisearch>
<https://lucaongaro.eu/blog/2019/01/30/minisearch-client-side-fulltext-search-engine.html>
<https://github.com/typesense/typesense>
<https://news.ycombinator.com/item?id=23473303>

Algolia
