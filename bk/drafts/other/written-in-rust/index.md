# Software Built in Rust

This section provides a few examples of software written in Rust, organized by topic, to give an idea of what has been and can be built with Rust. The [`awesome-rust`][awesome-rust-github] repository contains a far more complete list of Rust applications, dev tools, and libraries.

## Systems Programming

- Operating System Components:
  - Kernel modules (Parts of the Linux kernel are now being written in Rust.)
  - Bootloaders.
  - Virtual machine monitors. See [[virtualization | Virtualization]].
  - Examples:
    - `Redox OS`: A microkernel operating system written in Rust, aiming for safety and security.
    - `Stratis`: A local storage management system for Linux.
    - `youki` [(GitHub)](https://github.com/youki-dev/youki)⮳: a container runtime.
    - `Kata Containers` [(GitHub)](https://github.com/kata-containers/kata-containers)⮳: lightweight Virtual Machines (VMs) that feel and perform like containers, but provide the workload isolation and security advantages of VMs.
    - `firecracker` [(GitHub)](https://github.com/firecracker-microvm/firecracker): microVMs for serverless computing.
- Embedded Systems:
  - Firmware for microcontrollers.
  - Real-time operating systems (RTOS).
  - Examples:
    - `Tock OS`: An embedded operating system designed for low-power IoT devices.
    - [esp-rs (GitHub)](https://github.com/esp-rs)⮳: Rust on Espressif microcontrollers.
- Command-Line Tools:
  - Utilities for system administration.
  - Performance monitoring tools.
  - File manipulation tools - see [[filesystem_cli | Filesystem CLI]].
  - Examples:
    - [`ripgrep`][c-ripgrep] [(crates.io)](https://crates.io/crates/ripgrep)⮳: a line-oriented search tool that recursively searches the current directory for a regex pattern.
    - [`exa`][c-exa]⮳{{hi:exa}}: a modern replacement for `ls`.
    - [`eza`](https://github.com/eza-community/eza): another alternative to `ls`.
    - [`lsd`][c-lsd] [(GitHub)](https://github.com/lsd-rs/lsd)⮳{{hi:lsd}}: next-gen `ls` command.
    - [`bat`][c-bat] [(GitHub)][c-bat-github]⮳{{hi:bat}}: a `cat` clone with syntax highlighting and Git integration.
    - [`fd`][c-fd] [(GitHub)][c-fd_find-github]⮳: an alternative to 'find'.
    - [`fish`](https://github.com/fish-shell/fish-shell): a command-line shell.
    - [`nushell`][c-nushell]⮳ [(GitHub)](https://github.com/nushell/nushell): a new type of shell.
    - [`zoxide`][c-zoxide]⮳{{hi:zoxide}}: a smarter `cd` command.
    - [`dust`][c-dust] [(GitHub)](https://github.com/bootandy/dust)⮳: a more intuitive version of `du`.
    - [`bottom`][c-bottom] [(GitHub)][c-bottom-github]⮳: a cross-platform graphical process/system monitor.
    - [`sd`][c-sd]⮳{{hi:sd}}: an intuitive find & replace CLI.
    - [`starship`][c-starship] [(GitHub)][c-starship-github]⮳{{hi:starship}} - a customizable prompt for any shell. See [[shells | Shells]].
    - [`broot`][c-broot] [(GitHub)][c-broot-github]⮳: a tool to navigate directories.
    - [`television`][c-television] [(GitHub)](https://github.com/alexpasmantier/television){{hi:television}}⮳: a cross-platform and extensible general purpose fuzzy finder TUI.
    - [`sd`][c-sd] [(crates.io)](https://crates.io/crates/sd)⮳: an intuitive find & replace CLI.
    - `uutils` [(GitHub)](https://github.com/uutils/coreutils)⮳: cross-platform Rust rewrite of the GNU coreutils.

## Web Development and Networking

- Web Servers:
  - High-performance web servers.
  - API servers.
  - Examples: Cloudflare uses Rust for performance critical services.
- Search Engines:
  - [`tantivy`][c-tantivy] [(GitHub)](https://github.com/quickwit-oss/tantivy)⮳: a full-text search engine library inspired by Apache Lucene.
  - [`meilisearch`][c-meilisearch] [(GitHub)](https://github.com/meilisearch/MeiliSearch)⮳: a fast search engine API bringing AI-powered hybrid search to sites and applications.
- WebAssembly ([[wasm | WASM]]):
  - Front-end web applications.
  - Portable code for web browsers.
    - Libraries like [`yew`][c-yew]⮳{{hi:yew}} and [`leptos`][c-leptos]⮳{{hi:leptos}} enable building complex front-end web applications with Rust.
    - [`wasmer`][c-wasmer] [(GitHub)][c-wasmer-github]⮳: lightweight containers based on WebAssembly.
- Networking:
  - Network protocols.
  - VPN software.
  - Proxy servers.
  - Examples:
    - [`boringtun`](https://github.com/cloudflare/boringtun): a `Wireguard` VPN implementations.
    - [`Hickory DNS`](https://github.com/hickory-dns/hickory-dns): a DNS client, server, and resolver.
    - [`pingora`][c-pingora] [(GitHub)](https://github.com/cloudflare/pingora)⮳: a library for building fast, reliable and evolvable network services.
- Tools:
  - [`imager`](https://github.com/imager-io/imager): automated image compression for efficiently distributing images on the web.

## Application Software

- Browsers:
  - Components of web browsers.
  - Example: `Mozilla Firefox` uses Rust in its `Servo` rendering engine.
- Databases:
  - Database engines.
  - Database drivers.
  - Examples:
    - [`TiKV`](https://github.com/tikv/tikv): an open-source, distributed, and transactional key-value database.
    - [`neon`][c-neon] [(GitHub)](https://github.com/neondatabase/neon)⮳: Serverless Postgres. Separated storage and compute to offer autoscaling, code-like database branching, and scale to zero.
    - [`ArangoDB`](https://arangodb.com): a graph database.
- Analytics:
  - [`databend`](https://github.com/databendlabs/databend) - alternative to Snowflake.
  - [`rerun`][c-rerun][(GitHub)](https://github.com/rerun-io/rerun)⮳{{hi:rerun}} visualizes streams of multimodal data.
  - [`polars`][c-polars] [(GitHub)](https://github.com/pola-rs/polars)⮳: dataframes powered by a multithreaded, vectorized query engine.
  - [`datafusion`][c-datafusion] [(GitHub)](https://github.com/apache/datafusion)⮳: Apache DataFusion SQL Query Engine.
- Distributed Systems:
  - Blockchain technology.
  - Cloud computing tools.
  - Example: `Substrate`, a framework for building blockchains.
- Games:
  - [`Veloren`](https://gitlab.com/veloren/veloren): a multiplayer voxel RPG written in Rust.
- Other:
  - [`hyperswitch`](https://github.com/juspay/hyperswitch): an open-source payments switch to make payments fast, reliable and affordable.
  - [`rustdesk`][c-rustdesk] [(GitHub)](https://github.com/rustdesk/rustdesk)⮳{{hi:rustdesk}}: an open-source remote desktop application designed for self-hosting, as an alternative to TeamViewer.

## Development Tools

- Terminal Emulators:
  - [`alacritty`](https://github.com/alacritty/alacritty)⮳: a fast, cross-platform, OpenGL terminal emulator.
- Editors:
  - `lapce` [(GitHub)](https://github.com/lapce/lapce)⮳: a code editor.
  - `Zed` [(GitHub)](https://github.com/zed-industries/zed)⮳: a multiplayer code editor from the creators of Atom and Tree-sitter.
  - `helix` [(GitHub)](https://github.com/helix-editor/helix)⮳: a post-modern modal text editor.
- Compilers, Interpreters, Runtimes:
  - Language compilers.
  - Language interpreters.
  - Examples:
    - [`SWC`][c-swc]⮳{{hi:SWC}}: a very fast Typescript/Javascript compiler.
    - [`Deno`][c-deno] [(GitHub)][c-deno-github]⮳{{hi:Deno}}: a JavaScript, TypeScript, and WebAssembly runtime.
    - `starlark-rust` [(GitHub)](https://github.com/facebook/starlark-rust)⮳: a Rust implementation of the Starlark language. Starlark is a deterministic language inspired by Python3, used for configuration in the build systems `Bazel`, `Buck` and `Buck2`.
- Package Managers:
  - Software distribution tools.
  - Example: [`habitat`](https://github.com/habitat-sh/habitat): open-source software that creates platform-independent build artifacts and provides built-in deployment and management capabilities.
- Linters and Formatters:
  - [`ruff`][c-ruff] [(GitHub)](https://github.com/astral-sh/ruff)⮳: an extremely fast Python linter and code formatter.
- Cache Cleaning Utilities:
  - [`kondo`](https://github.com/tbillington/kondo)⮳ cleans dependencies and build artifacts from your projects.
- Desktop Frameworks
  - [`tauri`][c-tauri][(GitHub)][c-tauri-github]⮳: build desktop and mobile applications with a web frontend.
- Game Development:
  - Game engines.
  - Game tools.
  - Example: [`Bevy`][c-bevy]⮳{{hi:Bevy}} game engine.
- Observability:
  - [`openobserve`](https://github.com/openobserve/openobserve): petabyte scale Elasticsearch/Splunk/Datadog alternative for logs, metrics, traces, RUM, Error tracking, Session replay.
  - [`opentelemetry-rust`](https://github.com/open-telemetry/opentelemetry-rust): the Rust OpenTelemetry implementation.
  - `influxdb` [(GitHub)](https://github.com/influxdata/influxdb): a scalable datastore for metrics, events, and real-time analytics.

{{#include development_tools.incl.md}}

## Python Tools Written in Rust

{{#include python_tools.incl.md}}

## Other Tools Written in Rust

{{#include other_tools.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[organize; decide if we need other pages or if we consolidate here; table?](https://github.com/john-cd/rust_howto/issues/993)
</div>
