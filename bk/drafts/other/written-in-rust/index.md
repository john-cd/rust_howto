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
    - `Firecracker`: A virtual machine monitor (VMM) from Amazon Web Services, designed for serverless computing.
- Embedded Systems:
  - Firmware for microcontrollers.
  - Real-time operating systems (RTOS).
  - Examples:
    - `Tock OS`: An embedded operating system designed for low-power IoT devices.
- Command-Line Tools:
  - Utilities for system administration.
  - Performance monitoring tools.
  - File manipulation tools.
  - Examples:
    - `ripgrep`: A line-oriented search tool.
    - `exa`: A modern replacement for ls.
    - `bat`: A `cat` clone with syntax highlighting and Git integration.
    - `fd`: A simple, fast, and user-friendly alternative to find.
    - `zoxide`: A smarter cd command.
    - `dust`: A more intuitive version of du.
    - `bottom`: A graphical process/system monitor.
    - `sd`: An intuitive find & replace CLI.

## Web Development

- Web Servers:
  - High-performance web servers.
  - API servers.
  - Examples: Cloudflare uses Rust for performance critical services.
- WebAssembly ([[wasm | WASM]]):
  - Front-end web applications.
  - Portable code for web browsers.
    - Libraries like `yew` and `leptos` enable building complex front-end web applications with Rust.
- Networking:
  - Network protocols.
  - VPN software.
  - Proxy servers.
  - Examples:
    - `boringtun` - a `Wireguard` VPN implementations.

## Application Software

- Browsers:
  - Components of web browsers.
  - Example: `Mozilla Firefox` uses Rust in its `Servo` rendering engine.
- Databases:
  - Database engines.
  - Database drivers.
  - Examples:
    - `TiKV`: A distributed transactional key-value database.
    - `Neon`: serverless postgres.
- Game Development:
  - Game engines.
  - Game tools.
  - Example: Game engines and game tools are being developed using the `Bevy` game engine.
- Distributed Systems:
  - Blockchain technology.
  - Cloud computing tools.
  - Example: `Substrate`, a framework for building blockchains.

## Development Tools

- Compilers, Interpreters, Runtimes:
  - Language compilers.
  - Language interpreters.
  - Examples:
    - `SWC`, a very fast Typescript/Javascript compiler.
    - `Deno`, a JavaScript, TypeScript, and WebAssembly runtime.
- Package Managers:
  - Software distribution tools.
- Linters and Formatters:
  - Code quality tools.

{{#include development_tools.incl.md}}

## Python Tools Written in Rust

{{#include python_tools.incl.md}}

## Other Tools Written in Rust

{{#include other_tools.incl.md}}

[awesome-rust-github]: https://github.com/rust-unofficial/awesome-rust
{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[organize; decide if we need other pages or if we consolidate here](https://github.com/john-cd/rust_howto/issues/993)

Incorporate the below:

[`starship`][c-starship]⮳
https://github.com/starship/starship

[`bat`][c-bat]⮳
https://github.com/sharkdp/bat

[`tauri`][c-tauri]⮳
Build smaller, faster, and more secure desktop and mobile applications with a web frontend.
https://github.com/tauri-apps/tauri

[`deno`][c-deno]⮳
https://github.com/denoland/deno

`Alacritty` - A fast, cross-platform, OpenGL terminal emulator
https://github.com/alacritty/alacritty

https://github.com/cloudflare/boringtun

`habitat` - Modern applications with built-in automation. It is open source software that creates platform-independent build artifacts and provides built-in deployment and management capabilities.
https://github.com/habitat-sh/habitat

`Hickory DNS` - a Rust based DNS client, server, and resolver.
https://github.com/hickory-dns/hickory-dns

[`wasmer`][c-wasmer]⮳ - Fast, secure, lightweight containers based on WebAssembly
https://github.com/wasmerio/wasmer

`databend` - Modern alternative to Snowflake. Cost-effective and simple for massive-scale analytics.
https://github.com/databendlabs/databend

[`neon`][c-neon]⮳: Serverless Postgres. Separated storage and compute to offer autoscaling, code-like database branching, and scale to zero.
https://github.com/neondatabase/neon

`TiKV` is an open-source, distributed, and transactional key-value database.
https://github.com/tikv/tikv

[`broot`][c-broot]⮳ - A new way to see and navigate directory
https://github.com/Canop/broot

`Veloren` is a multiplayer voxel RPG written in Rust.
https://gitlab.com/veloren/veloren

`imager` - Automated image compression for efficiently distributing images on the web
https://github.com/imager-io/imager

`openobserve` - petabyte scale Elasticsearch/Splunk/Datadog alternative for logs, metrics, traces, RUM, Error tracking, Session replay.
https://github.com/openobserve/openobserve

`opentelemetry-rust` - the Rust OpenTelemetry implementation
https://github.com/open-telemetry/opentelemetry-rust

`hyperswitch` - An open source payments switch written in Rust to make payments fast, reliable and affordable
https://github.com/juspay/hyperswitch

[`dust`][c-dust]⮳
https://github.com/bootandy/dust

`eza` - A modern alternative to ls
https://github.com/eza-community/eza

[`bottom`][c-bottom]⮳  Yet another cross-platform graphical process/system monitor.
https://github.com/ClementTsang/bottom

[`fd`][c-fd]⮳ A simple, fast and user-friendly alternative to 'find'
https://github.com/sharkdp/fd

`fish` - user-friendly command line shell.
https://github.com/fish-shell/fish-shell

`kondo` Cleans dependencies and build artifacts from your projects.
https://github.com/tbillington/kondo

[`tantivy`][c-tantivy]⮳  is a full-text search engine library inspired by Apache Lucene and written in Rust.
https://github.com/quickwit-oss/tantivy

[`meilisearch`][c-meilisearch]⮳ A lightning-fast search engine API bringing AI-powered hybrid search to your sites and applications.
https://github.com/meilisearch/MeiliSearch

`starlark-rust` provides a Rust implementation of the Starlark language. Starlark is a deterministic language inspired by Python3, used for configuration in the build systems `Bazel`, `Buck` and `Buck2`.
https://github.com/facebook/starlark-rust

https://arangodb.com/

`rerun` visualizes streams of multimodal data.
https://github.com/rerun-io/rerun

[![rerun-website][c-rerun-website-badge]][c-rerun-website] [![rerun][c-rerun-badge]][c-rerun] [![rerun-crates.io][c-rerun-crates.io-badge]][c-rerun-crates.io] [![rerun-github][c-rerun-github-badge]][c-rerun-github] [![rerun-lib.rs][c-rerun-lib.rs-badge]][c-rerun-lib.rs]{{hi:rerun}}{{hi:Mesh}}{{hi:Plotting}}{{hi:Point-cloud}}{{hi:Robotics}}{{hi:Visualization}} [![cat-visualization][cat-visualization-badge]][cat-visualization]{{hi:Visualization}} [![cat-computer-vision][cat-computer-vision-badge]][cat-computer-vision]{{hi:Computer vision}}

[`polars`][c-polars]⮳ Dataframes powered by a multithreaded, vectorized query engine, written in Rust
https://github.com/pola-rs/polars

[`datafusion`][c-datafusion]⮳ Apache DataFusion SQL Query Engine
https://github.com/apache/datafusion

`Rust on Espressif microcontrollers`
https://github.com/esp-rs

[`pingora`][c-pingora]⮳  A library for building fast, reliable and evolvable network services.
https://github.com/cloudflare/pingora

`youki` - A container runtime written in Rust
https://github.com/youki-dev/youki

`Kata Containers` is an open source project and community working to build a standard implementation of lightweight Virtual Machines (VMs) that feel and perform like containers, but provide the workload isolation and security advantages of VMs.
https://github.com/kata-containers/kata-containers

`firecracker` Secure and fast microVMs for serverless computing.
https://github.com/firecracker-microvm/firecracker

`television`
https://github.com/alexpasmantier/television

`rustdesk` - An open-source remote desktop application designed for self-hosting, as an alternative to TeamViewer.
https://github.com/rustdesk/rustdesk

[`sd`][c-sd]⮳ An intuitive find & replace CLI
https://crates.io/crates/sd

[`ripgrep`][c-ripgrep]⮳  is a line-oriented search tool that recursively searches the current directory for a regex pattern.
https://crates.io/crates/ripgrep

`lapce` - Lightning-fast and Powerful Code Editor written in Rust
https://github.com/lapce/lapce

`Zed` is a high-performance, multiplayer code editor from the creators of Atom and Tree-sitter.
https://github.com/zed-industries/zed

helix A post-modern modal text editor.
https://github.com/helix-editor/helix

Cross-platform Rust rewrite of the GNU coreutils
https://github.com/uutils/coreutils

[`nushell`][c-nushell]⮳  A new type of shell
https://github.com/nushell/nushell

`lsd` The next gen ls command
https://github.com/lsd-rs/lsd

[`ruff`][c-ruff]⮳  An extremely fast Python linter and code formatter, written in Rust.
https://github.com/astral-sh/ruff

`influxdb` Scalable datastore for metrics, events, and real-time analytics
https://github.com/influxdata/influxdb
</div>
