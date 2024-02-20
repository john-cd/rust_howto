# Call for contributions

This book is in its **early days** - it is intended to be easy for new Rust programmers to contribute to, and an easy way to get involved with the Rust community. It needs and welcomes help.

Feel free to submit an issue or a pull request to the [repo][rust-howto-github].

Contributions, from small edits to whole chapters, are most welcome. Draft pages are kept in [this folder][rust-howto-drafts]. An informal (and very long) list of topics of interest is kept in the section below and in [TODO][rust-howto-todo-github]. Embedded examples should be ideally _runnable_ on the [Rust playground][rust-playground]⮳ or at least directly copy-pasteable into Rust code. Please read [CONTRIBUTING.md][rust-howto-contributing] for more details.

Its long-term goal is the coverage of the 'most commonly used' Rust crates, as defined by [`blessed.rs`][blessed-rs]⮳, the most downloaded libraries in [`crates.io`][crates-io]⮳, and 'high quality crates' per [`lib.rs`][lib-rs]⮳ [statistics][lib-rs-stats]⮳. Review [key crates](key_crates.md) for topic ideas.

## Topics of interest

- Async: tokio, async-std and related, async_channels
- CD / CI, rust-cache
- Macros
- `http` crate, `hyper`
- Python interop: `pyo3`
- AWS and other Cloud services
- Advanced data structures
- Basic and advanced TCP/IP networking
- `notify`
- `indicatif`
- `ratatui`
- File system traversal, `walkdir`
- `time`, `chrono`
- Errors: `color-eyre`, `eyre`
- Testing: `approx`, `nextest`
- `axum`
- `loco`
- shuttle.rs
- `reqwest`
- `tonic`
- More database examples, including object databases, graph databases, BonsaiDB, SurrealDB
- `redis`, `mongodb`, `elasticsearch`
- Cargo & project integration via `cargo-edit`
- Zip files and other archives
- Authentication / authorization: OAuth2, LDAP/AD, DNS lookups
- GTK, Qt, FLTK, Bevy + eGUI, other UI toolkits
- GPU processing, CUDA
- Machine learning, Tensorflow
- Raft Consensus library
- Network file systems
- Statistics, math, bignum libraries
- Crypto, SSL, SSH, other public key encryption, X.509, RusTLS
- Sound, graphics
- Games: bevy
- Search engines
- Buffer pools, garbage collection, or other reference-counted examples
- IPv6 address processing
- Cloud stuff: LB, status reporting (Vigil), routing, orchestration, containers
- Version control: libgit2: clone, change branches, create commits, push, pull
- High-performance computing: OpenMP, etc.
- Social media APIs
- Personal file sharing: OwnCloud, etc.

## See also

[![rust-howto-contributing][rust-howto-contributing-badge]][rust-howto-contributing]

[![rust-howto-drafts][rust-howto-drafts-badge]][rust-howto-drafts]

[![rust-howto-github][rust-howto-github-badge]][rust-howto-github]

[![rust-howto-todo-github][rust-howto-todo-github-badge]][rust-howto-todo-github]

{{#include refs/link-refs.md}}
