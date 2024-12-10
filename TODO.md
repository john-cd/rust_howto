# TODO

- [ ] fix contributing section
- [ ] polish the intro, add links WIP
- [ ] polish crates section
- [ ] fix incl.md table for cargo plugins; add ref to crates
- [ ] fix incl.md tables for databases
- [ ] finish to polish the links section WIP
- [ ] fix error handling / customization section WIP
- [ ] fix examples marked TODO; make sure that there is an output when run in the playground
- [ ] review all examples and mark them as ```rust,noplayground as needed
- [ ] add badge refdefs for links to RBE book in language / standard_lib; use `just templ rbe` command - WIP
- [ ] review each .md file in turn - WIP
- [ ] replace current badge markdown mess by a preprocessor command
- [ ] modify index of crates to point to the relevant recipes
- [ ] review unused reference definitions (without corresponding links in the markdown) - WIP
- [ ] add examples where needed - WIP

## Tools

- [ ] finish mdbook preprocessor - {#badge <crate_name> <extra_cat1>...}, {#categories <cat1>,<cat2>... }
- [ ] finish autogen and tool_lib/src/tera --> generate index of examples? recipe tables? new subchapter? - WIP
- [ ] finish git hook setup with cargo husky - why are they not installed?
- [ ] consolidate templ / crate_indices / autogen / mdbook-utils tools and respective libs

## Links

## Markdown

- [ ] review the cookbook's old pending [issues](https://github.com/rust-lang-nursery/rust-cookbook/issues) and [PRs](https://github.com/rust-lang-nursery/rust-cookbook/pulls)

## Examples

- [ ] fix commented examples - listen_unused, backtrace, rate_limited, paginated
- [ ] rewrite rest_post.rs so that a username and password are not required?
- [ ] finish to review ignore / no_run examples - add noplayground if playground does not have the dependency
- [ ] Make addt'l examples out of mdbook-utils, crate_indices, templ, clean, autogen...
- [ ] integrate clap builder example
- [ ] fix failing examples in deps/drafts
- [ ] tower_http example polish; other examples have been checked against the rust playground
- [ ] fix leaky tests when using nextest on Windows
- [ ] add shuttle.rs example

## CI

- [ ] revisit Github Action workflow. Make it work with existing docker compose files + cache See <https://stackoverflow.com/questions/61491484/how-to-cache-docker-compose-build-inside-github-action> try direct call to docker buildx bake - see justfile and ./scripts/docker/push_ci.sh

## Others

- [ ] pull request to little book of rust books
- [ ] Consider adding project to [goodfirstissue.dev][goodfirstissue-website]
- [ ] review GA / GSC - issues with redirects??
- [ ] license
- [ ] add licenses to thanks page ?
- [ ] update thanks
- [ ] Search within the book improvements - see search.md; Algolia
- [ ] review bacon.toml
- [ ] review deny.toml
- Contributing
- Code of conduct

## Additions to consider

[Rust training](https://github.com/ferrous-systems/rust-training)

[Rust exercises](https://github.com/ferrous-systems/rust-exercises)

[Why rust](https://github.com/ferrous-systems/why-rust)

[WASM training](https://github.com/ferrous-systems/wasm-training-2022)

[https://github.com/ferrous-systems/mdslides](https://github.com/ferrous-systems/mdslides)

[c2rust transpiler][c2rust]

[c2rust]: https://github.com/immunant/c2rust

JavaScript tooling:

- [Volta][volta-website]⮳[(github)][volta-github]⮳ | |
- [Deno][deno-website]⮳ uses Rust for its JavaScript and TypeScript runtime.
- Bun

Binary encoders

- bincode
- prost
- protobuf
- rmp-serde
- ciborium

XML

- xml-rs
- quick-xml
- xmlparser
- xml5ever

JSON

- serde_json
- json5
- simd-json

TOML

- toml
- toml_edit
- basic_toml

Parser generators

- nom
- pest
- combine
- peg

Markdown parsers

- pulldown-cmark
- markdown
- comrak

Tempfiles

- tempfile
- tempdir

Reverse proxy

- [River](https://github.com/memorysafety/river/)
- [Pingora](https://github.com/cloudflare/pingora)

builder derives crates

bon - compile-time-checked builders, named function arguments via builders (foo().arg(...).call()), fallible/async builders, method builders (self.foo(...).arg(...).call()). The newest crate built based on lessons learned from typed-builder and derive_builder (33K downloads/month, but gaining popularity, 1095 stars, 3 months old).
typed-builder - compile-time-checked builders. The oldest crate for compile-time-checked builders that has (987K downloads/month, 916 stars, 7 years old)
derive_builder - runtime-checked builders, works with &self, &mut self builder patterns. The oldest crate for runtime-checked builders overall (1,58M downloads/month, 1285 stars, 8 years old)

sccache

[`fastrand`](https://crates.io/crates/fastrand) - 200M downloads (vs 370M for rand)
No dependency, non-cryptographically secure random numbers, lower complexity than rand

[https://github.com/nicoburns/blessed-rs/issues/127](https://github.com/nicoburns/blessed-rs/issues/127)

[https://github.com/nicoburns/blessed-rs/issues/120](https://github.com/nicoburns/blessed-rs/issues/120)

Alongside arrayvec and tinyvec, heapless has stack-allocated arrays, but also includes:

Arc – like std::sync::Arc but backed by a lock-free memory pool rather than #[global_allocator]
Box – like std::boxed::Box but backed by a lock-free memory pool rather than #[global_allocator]
BinaryHeap – priority queue
IndexMap – hash table
IndexSet – hash set
LinearMap
Object – objects managed by an object pool
String
Vec
mpmc::Q* – multiple producer multiple consumer lock-free queue
spsc::Queue – single producer single consumer lock-free queue

[https://github.com/nicoburns/blessed-rs/issues/100](https://github.com/nicoburns/blessed-rs/issues/100)

[https://github.com/nicoburns/blessed-rs/issues/97](https://github.com/nicoburns/blessed-rs/issues/97)

[Volvo](https://tweedegolf.nl/en/blog/137/rust-is-rolling-off-the-volvo-assembly-line)

[Reddit](https://www.reddit.com/r/rust/comments/1fyown4/rust_gpu_the_future_of_gpu_programming/?share_id=iN53A04F7PNykANQyE5tb&utm_content=1&utm_medium=ios_app&utm_name=iossmf&utm_source=share&utm_term=22&rdt=58853)

[Rust jobs-report (Sep. 2024)](https://filtra.io/rust/jobs-report/sep-24)

[`comemo`](https://github.com/typst/comemo)

[`roxygen`](https://github.com/geo-ant/roxygen)

[`csborrow`](https://em-tg.github.io/csborrow/)

[Why Rust is becoming a contender in AI development](https://www.analyticsinsight.net/artificial-intelligence/why-rust-is-becoming-a-contender-in-ai-development)

[Parsing arguments in Rust, without using dependencies](https://ntietz.com/blog/parsing-arguments-rust-no-deps/)

[`gh-workflow`](https://crates.io/crates/gh-workflow)

[Television](https://crates.io/crates/television)

[`roadmap.sh` for Rust](https://roadmap.sh/rust)

Hashset
BinaryHeap
LinkedList
Stack
Queue
BTreeMap
BTreeSet

ring
rust-crypto
sodiumoxide

json-rust

structopt
termion

rppal
nrf-hal
embdedded-hal

wasm-bindgen
wasm-pack
wasmer

rust-gdb
rust-lldb

criterion.rs

DSLs in Rust

[5 non-LLM software trends to be excited about](https://read.engineerscodex.com/p/5-non-llm-software-trends-to-be-excited)

[FFTW](https://www.fftw.org/)

[Rust Atomics](https://marabos.nl/atomics/)

[LakeSail](https://lakesail.com/)

The [Copenhagen Book](https://thecopenhagenbook.com/)

[`rerun`](https://github.com/rerun-io/rerun)

[typestate pattern](https://cliffle.com/blog/rust-typestate)
