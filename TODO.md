# TODO

- [ ] enable / disable linkcheck on demand - need manipulating book.toml? second book.toml? https://www.michaelfbryan.com/mdbook-linkcheck/mdbook_linkcheck/index.html
- [ ] consider https://github.com/FauconFan/mdbook-cmdrun  A mdbook preprocessor for runnning arbitrary (shell) commands in a markdown file
- [ ] consider https://benfalk.github.io/mdbook-journal/ or https://github.com/avitex/mdbook-tera for templating
- [ ] review https://github.com/EngosSoftware/yapp A mdbook preprocessor that simply replaces text in chapters.
- [ ] review https://github.com/badboy/mdbook-toc  A preprocessor for mdbook to add inline Table of Contents support.

## Examples

- [ ] fix leaky tests when using nextest on Windows
- [ ] outputs of tests should go to a temporary folder
- [ ] Warning: The index-preprocessor plugin was built against version 0.4.35 of mdbook, but we're being called from version 0.4.37
- [ ] fix failing examples in drafts WIP
- [ ] integrate clap builder example
- [ ] fix build.rs (and skeptic.rs in drafts); create a copy of the Markdown, then remove #includes to only test directly-embedded examples

## Markdown

- [ ] review each .md file in turn
- [ ] create badge refdefs for links to RBE book
- [ ] pull request to little book of rust books
- [ ] Consider adding project to [goodfirstissue.dev][goodfirstissue]
[goodfirstissue]: https://goodfirstissue.dev/language/rust
- [ ] review GA / GSC - issues with redirects??
- [ ] incorporate cwd.rs example + any other new examples
- [ ] finish word index
- [ ] finish cd_ci.md
- [ ] finish improve_speed
- [ ] finish to review ignore / no_run examples
- [ ] Make addt'l examples out of mdbook-utils
- [ ] tower_http example polish; other examples have been checked against the rust playground
- [ ] add to each stub chapter in turn
- [ ] license
- [ ] update thanks
- [ ] add licenses to thanks page ?
- [ ] add page TOC ? [mdbook-theme][mdbook-theme]
[mdbook-theme]: https://github.com/zjp-CN/mdbook-theme
[alternative][mdbook-pagetoc]
[mdbook-pagetoc]: https://github.com/slowsage/mdbook-pagetoc
[mdbook-toc][mdbook-toc]
[mdbook-toc]: https://github.com/badboy/mdbook-toc
- [ ] better search? [infisearch][infisearch]
[infisearch]: https://github.com/ang-zeyu/infisearch
- [ ] [rust-module-system][rust-module-system]
[rust-module-system]: https://www.sheshbabu.com/posts/rust-module-system
- [ ] Review [awesome-rust][awesome-rust]
[awesome-rust]: https://github.com/rust-unofficial/awesome-rust - already in links.md
- [ ] Review [rust-unofficial/patterns][rust-unofficial-patterns] [rust-unofficial-patterns]: https://github.com/rust-unofficial/patterns - already in links.md
- [ ] [rust_tutorial_that_actually_teaches_rust][rust_tutorial_that_actually_teaches_rust] [rust_tutorial_that_actually_teaches_rust]: https://www.reddit.com/r/rust/comments/15b9rl5/rust_tutorial_that_actually_teaches_rust
- [ ] Link to [rust-gentle-intro][rust-gentle-intro] [rust-gentle-intro]: https://stevedonovan.github.io/rust-gentle-intro
- [ ] database access: sqlx, seaORM
- [ ] CI / CD: rust-cache
- [ ] cargo outdated / audit / license / deny
- [ ] [rust-state-machine-pattern][rust-state-machine-pattern]
[rust-state-machine-pattern]: https://hoverbear.org/blog/rust-state-machine-pattern
- [ ] review deny.toml
- [ ] more details in the standard library section
- [ ] add more to tokio.md
- [ ] async main
- [ ] spawning
- [ ] networking
- [ ] 'static and Send constraints
- [ ] I/O
- [ ] Join, select
- [ ] [tracing-next-steps][tracing-next-steps] [tracing-next-steps]: https://tokio.rs/tokio/topics/tracing-next-steps
- [ ] [async-what-is-blocking][async-what-is-blocking] [async-what-is-blocking]: https://ryhl.io/blog/async-what-is-blocking
- [ ] [actors-with-tokio][actors-with-tokio] [actors-with-tokio]: https://ryhl.io/blog/actors-with-tokio
- [ ] [communicating-between-sync-and-async-code][communicating-between-sync-and-async-code] [communicating-between-sync-and-async-code]: https://docs.rs/tokio/latest/tokio/sync/mpsc/index.html#communicating-between-sync-and-async-code
- [ ] Tower
- [ ] add to Comparison to other languages
- [ ] AWS
- [ ] walkdir
- [ ] [flagset][flagset] [flagset]: https://docs.rs/flagset/latest/flagset
- [ ] notify
- [ ] indicatif, ratatui
- [ ] time
- [ ] chrono
- [ ] traits
- [ ] [dyn clone][c-dyn-clone]
- [ ] testing: Approx example
- [ ] [fuzzing][c-fuzzing-github]
- [ ] axum
- [ ] loco
- [ ] reqwest
- [ ] tonic
- [ ] redis
- [ ] mongodb
- [ ] elasticsearch
- [ ] Macros
- [ ] egui
- [ ] Badges
- [ ] rust and Docker; multistage builds
- [ ] rust + docker compose
- [ ] rust + Mongo example
- [ ] add a rust axum example
- [ ] shuttle.rs example
- [ ] add to windows.md
- [ ] add to Data
- [ ] add to GPU.md
- [ ] add to interop / Python
- [ ] add to Books
- [ ] Languages
- [ ] finish actor.md
- [ ] Search within the book
- [ ] [tinysearch][tinysearch-website] [tinysearch-website]: https://endler.dev/2019/tinysearch
- [ ] [stork-search.net][stork-search.net] [stork-search.net]: https://stork-search.net
- [ ] [minisearch][minisearch] [minisearch]: https://github.com/lucaong/minisearch
- [ ] [minisearch-client-side-fulltext-search-engine][minisearch-client-side-fulltext-search-engine] [minisearch-client-side-fulltext-search-engine]: https://lucaongaro.eu/blog/2019/01/30/minisearch-client-side-fulltext-search-engine.html
- [ ] [typesense][typesense-github]  [typesense-github]: https://github.com/typesense/typesense
- [ ] [A Tiny, Static, Full-Text Search Engine using Rust and WebAssembly][tinysearch] [tinysearch]: https://news.ycombinator.com/item?id=23473303
- [ ] Algolia
