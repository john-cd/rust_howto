# Software Built in Rust

This section provides a few examples of software written in Rust, organized by topic, to give an idea of what has been and can be built with Rust. The [`awesome-rust`][awesome-rust~repo]↗ repository contains a far more complete list of Rust applications, dev tools, and libraries.

## Systems Programming

- Operating System Components:
  - Kernel modules (Parts of the Linux kernel are now being written in Rust.)
    - [Rust-for-Linux][rust-for-linux~repo]↗: Adding support for the Rust language to the Linux kernel.
  - Bootloaders.
  - Virtual machine monitors. See [[virtualization | Virtualization]].
  - Examples:
    - `Redox OS`: A microkernel operating system written in Rust, aiming for safety and security.
    - `Stratis`: A local storage management system for Linux.
    - `youki` [(GitHub)][youki~repo]↗: a container runtime.
    - `Kata Containers` [(GitHub)][kata-containers~repo]↗: lightweight Virtual Machines (VMs) that feel and perform like containers, but provide the workload isolation and security advantages of VMs.
    - `firecracker` [(GitHub)][firecracker~repo]↗: microVMs for serverless computing.
- Embedded Systems:
  - Firmware for microcontrollers.
  - Real-time operating systems (RTOS).
  - Examples:
    - `Tock OS`: An embedded operating system designed for low-power IoT devices.
    - [`esp-rs` (GitHub)][espressif~repo]↗: Rust on Espressif microcontrollers.
- Command-Line Tools:
  - Utilities for system administration.
  - Performance monitoring tools.
  - File manipulation tools - see [[filesystem_cli | Filesystem CLI]].
  - Examples:
    - [`ripgrep`][c~ripgrep~docs]↗ [(crates.io)][c~ripgrep~crates.io]↗: a line-oriented search tool that recursively searches the current directory for a regex pattern.
    - [`exa`][c~exa~docs]↗{{hi:exa}}: a modern replacement for `ls`.
    - [`eza`][eza~repo]↗: another alternative to `ls`.
    - [`lsd`][c~lsd~docs]↗ [(GitHub)][c~lsd~repo]↗{{hi:lsd}}: next-gen `ls` command.
    - [`bat`][c~bat~docs]↗ [(GitHub)][c~bat~repo]↗{{hi:bat}}: a `cat` clone with syntax highlighting and Git integration.
    - [`fd`][c~fd~docs]↗ [(GitHub)][c~fd-find~repo]↗: an alternative to 'find'.
    - [`fish`][fish-shell~repo]↗: a command-line shell.
    - [`nushell`][nushell~website]↗ [(GitHub)][nushell~repo]↗: a new type of shell.
    - [`zoxide`][c~zoxide~docs]↗{{hi:zoxide}}: a smarter `cd` command.
    - [`dust`][c~dust~docs]↗ [(GitHub)][dust~repo]↗: a more intuitive version of `du`.
    - [`bottom`][c~bottom~docs]↗ [(GitHub)][c~bottom~repo]↗: a cross-platform graphical process/system monitor.
    - [`sd`][c~sd~docs]↗{{hi:sd}}: an intuitive find & replace CLI.
    - [`starship`][c~starship~docs]↗ [(GitHub)][c~starship~repo]↗{{hi:starship}} - a customizable prompt for any shell. See [[shells | Shells]].
    - [`broot`][c~broot~docs]↗ [(GitHub)][c~broot~repo]↗: a tool to navigate directories.
    - [`television`][c~television~docs]↗ [(GitHub)][c~television~repo]↗{{hi:television}}: a cross-platform and extensible general purpose fuzzy finder TUI.
    - [`sd`][c~sd~docs]↗ [(crates.io)][c~sd~crates.io]↗: an intuitive find & replace CLI.
    - `uutils` [(GitHub)][coreutils~repo]↗: cross-platform Rust rewrite of the GNU coreutils.

## Web Development and Networking

- Web Servers:
  - High-performance web servers.
  - API servers.
  - Examples: Cloudflare uses Rust for performance critical services.
- Search Engines:
  - [`tantivy`][c~tantivy~docs]↗ [(GitHub)][c~tantivy~repo]↗: a full-text search engine library inspired by Apache Lucene.
  - [`meilisearch`][c~meilisearch~docs]↗ [(GitHub)][meilisearch~repo]↗: a fast search engine API bringing AI-powered hybrid search to sites and applications.
- WebAssembly ([[wasm | WASM]]):
  - Front-end web applications.
  - Portable code for web browsers.
    - Libraries like [`yew`][c~yew~docs]↗{{hi:yew}} and [`leptos`][c~leptos~docs]↗{{hi:leptos}} enable building complex front-end web applications with Rust.
    - [`wasmer`][c~wasmer~docs]↗ [(GitHub)][c~wasmer~repo]↗: lightweight containers based on WebAssembly.
- Networking:
  - Network protocols.
  - VPN software.
  - Proxy servers.
  - Examples:
    - [`boringtun`][boringtun~repo]↗: a `Wireguard` VPN implementations.
    - [`Hickory DNS`][hickory-dns~repo]↗: a DNS client, server, and resolver.
    - [`pingora`][c~pingora~docs]↗ [(GitHub)][c~pingora~repo]↗: a library for building fast, reliable and evolvable network services.
- Tools:
  - [`imager`][imager~repo]↗: automated image compression for efficiently distributing images on the web.

## Application Software

- Browsers:
  - Components of web browsers.
  - Example: [`Mozilla Firefox`][mozilla-firefox~website]↗{{hi:Firefox}} uses Rust in its `Servo` rendering engine.
- Databases:
  - Database engines.
  - Database drivers.
  - Examples:
    - [`TiKV`][tikv~repo]↗: an open-source, distributed, and transactional key-value database.
    - [`neon`][c~neon~docs]↗ [(GitHub)][neon~repo]↗: Serverless Postgres. Separated storage and compute to offer autoscaling, code-like database branching, and scale to zero.
    - [`ArangoDB`][arangodb.com~website]↗: a graph database.
- Analytics:
  - [`databend`][databend~repo]↗ - alternative to Snowflake.
  - [`rerun`][c~rerun~docs][(GitHub)][c~rerun~repo]↗{{hi:rerun}} visualizes streams of multimodal data.
  - [`polars`][c~polars~docs]↗ [(GitHub)][c~polars~repo]↗: dataframes powered by a multithreaded, vectorized query engine.
  - [`datafusion`][c~datafusion~docs]↗ [(GitHub)][c~datafusion~repo]↗: Apache DataFusion SQL Query Engine.
- Distributed Systems:
  - Blockchain technology.
  - Cloud computing tools.
  - Example: `Substrate`, a framework for building blockchains.
- Games:
  - [`Veloren`][veloren~repo]↗: a multiplayer voxel RPG written in Rust.
- Other:
  - [`hyperswitch`][hyperswitch~repo]↗: an open-source payments switch to make payments fast, reliable and affordable.
  - [`rustdesk`][c~rustdesk~docs]↗ [(GitHub)][c~rustdesk~repo]↗{{hi:rustdesk}}: an open-source remote desktop application designed for self-hosting, as an alternative to TeamViewer.

## Development Tools

- Terminal Emulators:
  - [`alacritty`][alacritty~repo]↗: a fast, cross-platform, OpenGL terminal emulator.
- Editors:
  - `lapce` [(GitHub)][lapce~repo]↗: a code editor.
  - `Zed` [(GitHub)][c~zed~repo]↗: a multiplayer code editor from the creators of Atom and Tree-sitter.
  - `helix` [(GitHub)][helix~repo]↗: a post-modern modal text editor.
- Compilers, Interpreters, Runtimes:
  - Language compilers.
  - Language interpreters.
  - Examples:
    - [`SWC`][c~swc~docs]↗{{hi:SWC}}: a very fast Typescript/Javascript compiler.
    - [`Deno`][c~deno~docs]↗ [(GitHub)][c~deno~repo]↗{{hi:Deno}}: a JavaScript, TypeScript, and WebAssembly runtime.
    - `starlark-rust` [(GitHub)][starlark-rust~repo]↗: a Rust implementation of the Starlark language. Starlark is a deterministic language inspired by Python3, used for configuration in the build systems `Bazel`, `Buck` and `Buck2`.
- Package Managers:
  - Software distribution tools.
  - Example: [`habitat`][habitat~repo]↗: open-source software that creates platform-independent build artifacts and provides built-in deployment and management capabilities.
- Linters and Formatters:
  - [`ruff`][c~ruff~docs]↗ [(GitHub)][c~ruff~repo]↗: an extremely fast Python linter and code formatter.
- Cache Cleaning Utilities:
  - [`kondo`][kondo~repo]↗ cleans dependencies and build artifacts from your projects.
- Desktop Frameworks:
  - [`tauri`][c~tauri~docs][(GitHub)][c~tauri~repo]↗: build desktop and mobile applications with a web frontend.
- Game Development:
  - Game engines.
  - Game tools.
  - Example: [`Bevy`][c~bevy~docs]↗{{hi:Bevy}} game engine.
- Observability:
  - [`openobserve`][openobserve~repo]↗: petabyte scale Elasticsearch/Splunk/Datadog alternative for logs, metrics, traces, RUM, Error tracking, Session replay.
  - [`opentelemetry-rust`][opentelemetry-rust~repo]↗: the Rust OpenTelemetry implementation.
  - [`influxdb`][influxdb~website]{{hi:influxdb}} [(GitHub)][influxdb~repo]↗: a scalable datastore for metrics, events, and real-time analytics.

{{#include development_tools.incl.md}}

## Python Tools Written in Rust

{{#include python_tools.incl.md}}

## Other Tools Written in Rust

{{#include other_tools.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[organize; decide if we need other pages or if we consolidate here; table?](https://github.com/john-cd/rust_howto/issues/993)

- [Awesome Rust Tools][awesome-rust-tools~repo]↗: Harness the power of Rust. Those fast productivity tools based on Rust.
- [Awesome alternatives in Rust][awesome-alternatives-in-rust~repo]↗: A curated list of replacements for existing software written in Rust.

- [RustDesk][rustdesk~website]↗.
- [OxidOS Automotive][oxidos~website]↗.
- [`ripgrep`][c~ripgrep~repo]↗: `ripgrep` recursively searches directories for a regex pattern while respecting your gitignore.
- [`zola`][zola~repo]↗: A fast static site generator in a single binary with everything built-in.
- [solana][c~solana~repo]↗: Web-Scale Blockchain for fast, secure, scalable, decentralized apps and marketplaces.
- [`linkerd2`][linkerd2~repo]↗: Ultralight, security-first service mesh for Kubernetes. Main repo for Linkerd 2.x.
- [`bottlerocket`][c~bottlerocket~repo]↗: An operating system designed for hosting containers.
- [`databend`][databend~repo]↗: 𝗗𝗮𝘁𝗮, 𝗔𝗻𝗮𝗹𝘆𝘁𝗶𝗰𝘀 & 𝗔𝗜. Modern alternative to Snowflake. Cost-effective and simple for massive-scale analytics.
- [`rerun`][c~rerun~repo]↗: Visualize streams of multimodal data. Free, fast, easy to use, and simple to integrate. Built in Rust.
- [`tock`][tock~repo]↗: A secure embedded operating system for microcontrollers.
- [`mail-server`][mail-server~repo]↗: Secure & Modern All-in-One Mail Server (IMAP, JMAP, POP3, SMTP).
- [`hickory-dns`][hickory-dns~repo]↗: A Rust based DNS client, server, and resolver.
- [`sniffnet`][sniffnet~repo]↗: Comfortably monitor your Internet traffic.
- [`ratatui`][ratatui-showcase-apps~website]↗: App Showcase.
- [`habitat`][habitat~repo]↗: Modern applications with built-in automation.
- [`cargo-binstall`][c~cargo-binstall~repo]↗: Binary installation for rust projects.
- [`cargo-update`][c~cargo-update~crates.io]↗.
- [`alacritty`][alacritty~repo]↗: A cross-platform, OpenGL terminal emulator.
- [`cargo-hakari`][c~cargo-hakari~crates.io]↗.
- [`spacedrive`][spacedrive~repo]↗: Spacedrive is an open source cross-platform file explorer, powered by a virtual distributed filesystem written in Rust.
- [`swc`][c~swc~repo]↗: Rust-based platform for the Web.
- [`influxdb`][influxdb~repo]↗: Scalable datastore for metrics, events, and real-time analytics.
- [`firecracker`][firecracker~repo]↗: Secure and fast microVMs for serverless computing.
- [`warp`][warp~repo]↗ is a modern, Rust-based terminal with AI built in.
- [RustPython][rustpython~repo]↗ is a Python Interpreter written in Rust.
- [`wezterm`][wezterm~repo]↗: A GPU-accelerated cross-platform terminal emulator and multiplexer written by @wez and implemented in Rust.
- [`redox-os`: Mirror of https://gitlab.redox-os.org/redox-os/redox][redox~repo]↗.
- [`neon`][neon~repo]↗: Serverless Postgres. We separated storage and compute to offer autoscaling, code-like database branching, and scale to zero.
- [`navi`][c~navi~repo]↗: An interactive cheatsheet tool for the command-line.
- [RustScan][rustscan~repo]↗: The Modern Port Scanner.
- [`workers-rs`][workers-rs~repo]↗: Write Cloudflare Workers in 100% Rust via WebAssembly.
- [Qdrant][c~qdrant~repo]↗ - High-performance, massive-scale Vector Database and Vector Search Engine for the next generation of AI. Also available in the Cloud.
- [Hyperlight][c~hyperlight~repo]↗ is a lightweight Virtual Machine Manager (VMM) designed to be embedded within applications. It enables safe execution of untrusted code within micro virtual machines with very low latency and minimal overhead.
- [Perseus][perseus~website]↗.
- [DORA][dora~repo]↗ (Dataflow-Oriented Robotic Architecture) is middleware designed to streamline and simplify the creation of AI-based robotic applications. It offers low latency, composable, and distributed dataflow capabilities. Applications are modeled as directed graphs, also referred to as pipelines.
- [`television`][c~television~crates.io]↗.
- [Blog: Fish 4.0][blog~fishshell-rustport]↗: The Fish Of Theseus.
- [`shoes`][c~shoes~repo]↗: A multi-protocol proxy server written in Rust (HTTP, SOCKS5, Vmess, Vless, Shadowsocks, Trojan, Snell, Hysteria2, TUIC v5).
- [`jujutsu` (docs)][doc~jujutsu]↗.
- [Steve's Jujutsu Tutorial][blog~steveklabnik-jujutsu-tutorial]↗.
- [Codename Goose][doc~block-goose]↗.
- [`grex`][c~grex~repo]↗: A command-line tool and Rust library with Python bindings for generating regular expressions from user-provided test cases.
- [`youki`][youki~repo]↗: A container runtime written in Rust.
- [`yazi`][yazi~repo]↗: Blazing fast terminal file manager written in Rust, based on async I/O.

---

[![evcxr_jupyter][c~evcxr_jupyter~docs~badge]][c~evcxr_jupyter~docs] [![evcxr_jupyter~crates.io][c~evcxr_jupyter~crates.io~badge]][c~evcxr_jupyter~crates.io] [![evcxr_jupyter~repo][c~evcxr_jupyter~repo~badge]][c~evcxr_jupyter~repo] [![evcxr_jupyter~lib.rs][c~evcxr_jupyter~lib.rs~badge]][c~evcxr_jupyter~lib.rs]{{hi:evcxr_jupyter}}

A Jupyter Kernel for Rust

---

- [`evcxr_jupyter` README][c~evcxr_jupyter~readme~repo]↗: A Rust REPL / Jupyter kernel.

- [`hyperswitch`][hyperswitch~repo]↗.

---

[![kget][c~kget~docs~badge]][c~kget~docs] [![kget~crates.io][c~kget~crates.io~badge]][c~kget~crates.io] [![kget~repo][c~kget~repo~badge]][c~kget~repo] [![kget~lib.rs][c~kget~lib.rs~badge]][c~kget~lib.rs]{{hi:kget}}{{hi:HTTP}}{{hi:Torrent}}{{hi:FTP}}{{hi:Download}}{{hi:SFTP}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~api-bindings][cat~api-bindings~badge]][cat~api-bindings]{{hi:API bindings}} [![cat~command-line-utilities][cat~command-line-utilities~badge]][cat~command-line-utilities]{{hi:Command line utilities}}

A powerful and versatile download manager and library.

</div>
