# TODO

- [ ] review all examples and mark them as ```rust,noplayground as needed

- [ ] fix contributing section
- [ ] polish the intro, add links WIP
- [ ] polish crates section
- [ ] finish to polish the links section WIP
- [ ] fix error handling / customization section WIP
- [ ] fix examples marked TODO; make sure that there is an output when run in the playground
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
- [ ] documentation for all tools pub fn

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

- [ ] revisit Github Action workflow. Make it work with existing docker compose files + cache See [https://stackoverflow.com/questions/61491484/how-to-cache-docker-compose-build-inside-github-action](https://stackoverflow.com/questions/61491484/how-to-cache-docker-compose-build-inside-github-action) try direct call to docker buildx bake - see justfile and ./scripts/docker/push_ci.sh

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

### Ferrous systems

[Rust training][rust training]

[rust training]: https://github.com/ferrous-systems/rust-training

[Rust exercises][rust exercises]

[rust exercises]: https://github.com/ferrous-systems/rust-exercises

[Why rust][why rust]

[why rust]: https://github.com/ferrous-systems/why-rust

[WASM training][wasm training]

[wasm training]: https://github.com/ferrous-systems/wasm-training-2022

[mdslides][mdslides]

[mdslides]: https://github.com/ferrous-systems/mdslides

## RabbitMQ

[![lapin][c-lapin-badge]][c-lapin] [![lapin-crates.io][c-lapin-crates.io-badge]][c-lapin-crates.io] [![lapin-github][c-lapin-github-badge]][c-lapin-github] [![lapin-lib.rs][c-lapin-lib.rs-badge]][c-lapin-lib.rs]{{hi:lapin}}{{hi:Amqp}}{{hi:Futures}}{{hi:Mio}}{{hi:Rabbitmq}} [![cat-database][cat-database-badge]][cat-database]{{hi:Database interfaces}}

AMQP client library

### Debugging

rust-gdb ??
rust-lldb ??

### Companies

[Volvo][volvo]

[volvo]: <https://tweedegolf.nl/en/blog/137/rust-is-rolling-off-the-volvo-assembly-line>

### GPU

[Reddit][reddit]

[reddit]: <https://www.reddit.com/r/rust/comments/1fyown4/rust_gpu_the_future_of_gpu_programming/?share_id=in53a04f7pnykanqye5tb&utm_content=1&utm_medium=ios_app&utm_name=iossmf&utm_source=share&utm_term=22&rdt=58853>

### Jobs

[Rust jobs-report (Sep. 2024)][rust-jobs-report]

[rust-jobs-report]: <https://filtra.io/rust/jobs-report/sep-24>

### Other langs

[A comparison of Rust’s borrow checker to the one in C#][csborrow]

[csborrow]: <https://em-tg.github.io/csborrow/>

### AI

[Why Rust is becoming a contender in AI development][why rust is becoming a contender in ai development]

[why rust is becoming a contender in ai development]: <https://www.analyticsinsight.net/artificial-intelligence/why-rust-is-becoming-a-contender-in-ai-development>

### CI

A type-safe GitHub Actions workflow generator [`gh-workflow`][gh-workflow]

[gh-workflow]: <https://crates.io/crates/gh-workflow>

### DSLs in Rust

### TO SORT

[5 non-LLM software trends to be excited about][software-trends]

[software-trends]: <https://read.engineerscodex.com/p/5-non-llm-software-trends-to-be-excited>

[FFTW][fftw]

[fftw]: https://www.fftw.org

Big Data Processing for the AI Era [LakeSail][lakesail]

[lakesail]: <https://lakesail.com/>

See also: spice.ai

Visualize streams of multimodal data. Free, fast, easy to use, and simple to integrate. Built in Rust.
[`rerun`][rerun]

[rerun]: <https://github.com/rerun-io/rerun>

### Review blessed suggestions

[127][127]

[127]: <https://github.com/nicoburns/blessed-rs/issues/127>

[120][120]

[120]: <https://github.com/nicoburns/blessed-rs/issues/120>

[issue 100][100]

[100]: <https://github.com/nicoburns/blessed-rs/issues/100>

[issue 97][97]

[97]: <https://github.com/nicoburns/blessed-rs/issues/97>
