# Software Built in Rust

This section provides a few examples of software written in Rust, organized by topic, to give an idea of what has been and can be built with Rust. The [`awesome-rust`][awesome-rust~github]↗ repository contains a far more complete list of Rust applications, dev tools, and libraries.

## Systems Programming

- Operating System Components:
  - Kernel modules (Parts of the Linux kernel are now being written in Rust.)
    - [Rust-for-Linux: Adding support for the Rust language to the Linux kernel.][rust-for-linux~github]↗.
  - Bootloaders.
  - Virtual machine monitors. See [[virtualization | Virtualization]].
  - Examples:
    - `Redox OS`: A microkernel operating system written in Rust, aiming for safety and security.
    - `Stratis`: A local storage management system for Linux.
    - `youki` [(GitHub)][youki~github]↗: a container runtime.
    - `Kata Containers` [(GitHub)][kata-containers~github]↗: lightweight Virtual Machines (VMs) that feel and perform like containers, but provide the workload isolation and security advantages of VMs.
    - `firecracker` [(GitHub)][firecracker~github]↗: microVMs for serverless computing.
- Embedded Systems:
  - Firmware for microcontrollers.
  - Real-time operating systems (RTOS).
  - Examples:
    - `Tock OS`: An embedded operating system designed for low-power IoT devices.
    - [esp-rs (GitHub)][espressif~github]↗: Rust on Espressif microcontrollers.
- Command-Line Tools:
  - Utilities for system administration.
  - Performance monitoring tools.
  - File manipulation tools - see [[filesystem_cli | Filesystem CLI]].
  - Examples:
    - [`ripgrep`][c~ripgrep~docs]↗ [(crates.io)][c~ripgrep~crates.io]↗: a line-oriented search tool that recursively searches the current directory for a regex pattern.
    - [`exa`][c~exa~docs]↗{{hi:exa}}: a modern replacement for `ls`.
    - [`eza`][eza~github]↗: another alternative to `ls`.
    - [`lsd`][c~lsd~docs]↗ [(GitHub)][c~lsd~github]↗{{hi:lsd}}: next-gen `ls` command.
    - [`bat`][c~bat~docs]↗ [(GitHub)][c~bat~github]↗{{hi:bat}}: a `cat` clone with syntax highlighting and Git integration.
    - [`fd`][c~fd~docs]↗ [(GitHub)][c~fd-find~github]↗: an alternative to 'find'.
    - [`fish`][fish-shell~github]↗: a command-line shell.
    - [`nushell`][nushell~website]↗ [(GitHub)][nushell~github]↗: a new type of shell.
    - [`zoxide`][c~zoxide~docs]↗{{hi:zoxide}}: a smarter `cd` command.
    - [`dust`][c~dust~docs]↗ [(GitHub)][dust~github]↗: a more intuitive version of `du`.
    - [`bottom`][c~bottom~docs]↗ [(GitHub)][c~bottom~github]↗: a cross-platform graphical process/system monitor.
    - [`sd`][c~sd~docs]↗{{hi:sd}}: an intuitive find & replace CLI.
    - [`starship`][c~starship~docs]↗ [(GitHub)][c~starship~github]↗{{hi:starship}} - a customizable prompt for any shell. See [[shells | Shells]].
    - [`broot`][c~broot~docs]↗ [(GitHub)][c~broot~github]↗: a tool to navigate directories.
    - [`television`][c~television~docs]↗ [(GitHub)][c~television~github]{{hi:television}}↗: a cross-platform and extensible general purpose fuzzy finder TUI.
    - [`sd`][c~sd~docs]↗ [(crates.io)][c~sd~crates.io]↗: an intuitive find & replace CLI.
    - `uutils` [(GitHub)][coreutils~github]↗: cross-platform Rust rewrite of the GNU coreutils.

## Web Development and Networking

- Web Servers:
  - High-performance web servers.
  - API servers.
  - Examples: Cloudflare uses Rust for performance critical services.
- Search Engines:
  - [`tantivy`][c~tantivy~docs]↗ [(GitHub)][c~tantivy~github]↗: a full-text search engine library inspired by Apache Lucene.
  - [`meilisearch`][c~meilisearch~docs]↗ [(GitHub)][meiliSearch~github]↗: a fast search engine API bringing AI-powered hybrid search to sites and applications.
- WebAssembly ([[wasm | WASM]]):
  - Front-end web applications.
  - Portable code for web browsers.
    - Libraries like [`yew`][c~yew~docs]↗{{hi:yew}} and [`leptos`][c~leptos~docs]↗{{hi:leptos}} enable building complex front-end web applications with Rust.
    - [`wasmer`][c~wasmer~docs]↗ [(GitHub)][c~wasmer~github]↗: lightweight containers based on WebAssembly.
- Networking:
  - Network protocols.
  - VPN software.
  - Proxy servers.
  - Examples:
    - [`boringtun`][boringtun~github]↗: a `Wireguard` VPN implementations.
    - [`Hickory DNS`][hickory-dns~github]↗: a DNS client, server, and resolver.
    - [`pingora`][c~pingora~docs]↗ [(GitHub)][c~pingora~github]↗: a library for building fast, reliable and evolvable network services.
- Tools:
  - [`imager`][imager~github]↗: automated image compression for efficiently distributing images on the web.

## Application Software

- Browsers:
  - Components of web browsers.
  - Example: [`Mozilla Firefox`]( ){{hi: }} uses Rust in its `Servo` rendering engine.
- Databases:
  - Database engines.
  - Database drivers.
  - Examples:
    - [`TiKV`][tikv~github]↗: an open-source, distributed, and transactional key-value database.
    - [`neon`][c~neon~docs]↗ [(GitHub)][neon~github]↗: Serverless Postgres. Separated storage and compute to offer autoscaling, code-like database branching, and scale to zero.
    - [`ArangoDB`][arangodb.com~website]↗: a graph database.
- Analytics:
  - [`databend`][databend~github]↗ - alternative to Snowflake.
  - [`rerun`][c~rerun~docs][(GitHub)][c~rerun~github]↗{{hi:rerun}} visualizes streams of multimodal data.
  - [`polars`][c~polars~docs]↗ [(GitHub)][c~polars~github]↗: dataframes powered by a multithreaded, vectorized query engine.
  - [`datafusion`][c~datafusion~docs]↗ [(GitHub)][c~datafusion~github]↗: Apache DataFusion SQL Query Engine.
- Distributed Systems:
  - Blockchain technology.
  - Cloud computing tools.
  - Example: `Substrate`, a framework for building blockchains.
- Games:
  - [`Veloren`][veloren~github]↗: a multiplayer voxel RPG written in Rust.
- Other:
  - [`hyperswitch`][hyperswitch~github]↗: an open-source payments switch to make payments fast, reliable and affordable.
  - [`rustdesk`][c~rustdesk~docs]↗ [(GitHub)][c~rustdesk~github]↗{{hi:rustdesk}}: an open-source remote desktop application designed for self-hosting, as an alternative to TeamViewer.

## Development Tools

- Terminal Emulators:
  - [`alacritty`][alacritty~github]↗: a fast, cross-platform, OpenGL terminal emulator.
- Editors:
  - `lapce` [(GitHub)][lapce~github]↗: a code editor.
  - `Zed` [(GitHub)][c~zed~github]↗: a multiplayer code editor from the creators of Atom and Tree-sitter.
  - `helix` [(GitHub)][helix~github]↗: a post-modern modal text editor.
- Compilers, Interpreters, Runtimes:
  - Language compilers.
  - Language interpreters.
  - Examples:
    - [`SWC`][c~swc~docs]↗{{hi:SWC}}: a very fast Typescript/Javascript compiler.
    - [`Deno`][c~deno~docs]↗ [(GitHub)][c~deno~github]↗{{hi:Deno}}: a JavaScript, TypeScript, and WebAssembly runtime.
    - `starlark-rust` [(GitHub)][starlark-rust~github]↗: a Rust implementation of the Starlark language. Starlark is a deterministic language inspired by Python3, used for configuration in the build systems `Bazel`, `Buck` and `Buck2`.
- Package Managers:
  - Software distribution tools.
  - Example: [`habitat`][habitat~github]↗: open-source software that creates platform-independent build artifacts and provides built-in deployment and management capabilities.
- Linters and Formatters:
  - [`ruff`][c~ruff~docs]↗ [(GitHub)][c~ruff~github]↗: an extremely fast Python linter and code formatter.
- Cache Cleaning Utilities:
  - [`kondo`][kondo~github]↗ cleans dependencies and build artifacts from your projects.
- Desktop Frameworks
  - [`tauri`][c~tauri~docs][(GitHub)][c~tauri~github]↗: build desktop and mobile applications with a web frontend.
- Game Development:
  - Game engines.
  - Game tools.
  - Example: [`Bevy`][c~bevy~docs]↗{{hi:Bevy}} game engine.
- Observability:
  - [`openobserve`][openobserve~github]↗: petabyte scale Elasticsearch/Splunk/Datadog alternative for logs, metrics, traces, RUM, Error tracking, Session replay.
  - [`opentelemetry-rust`][opentelemetry-rust~github]↗: the Rust OpenTelemetry implementation.
  - [`influxdb`]( ){{hi: }} [(GitHub)][influxdb~github]↗: a scalable datastore for metrics, events, and real-time analytics.

{{#include development_tools.incl.md}}

## Python Tools Written in Rust

{{#include python_tools.incl.md}}

## Other Tools Written in Rust

{{#include other_tools.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[organize; decide if we need other pages or if we consolidate here; table?](https://github.com/john-cd/rust_howto/issues/993)

- [awesome-rust-tools: Harness the power of Rust. Those fast productivity tools based on Rust.][awesome-rust-tools~github]↗
- [awesome-alternatives-in-rust: A curated list of replacements for existing software written in Rust][awesome-alternatives-in-rust~github]↗

- [RustDesk][rustdesk~website]↗
- [OxidOS Automotive][oxidos~website]↗
- [ripgrep: `ripgrep` recursively searches directories for a regex pattern while respecting your gitignore][c~ripgrep~github]↗
- [zola: A fast static site generator in a single binary with everything built-in. https://www.getzola.org][zola~github]↗
- [solana: Web-Scale Blockchain for fast, secure, scalable, decentralized apps and marketplaces.][c~solana~github]↗
- [linkerd2: Ultralight, security-first service mesh for Kubernetes. Main repo for Linkerd 2.x.][linkerd2~github]↗
- [bottlerocket: An operating system designed for hosting containers][c~bottlerocket~github]↗
- [`databend`: 𝗗𝗮𝘁𝗮, 𝗔𝗻𝗮𝗹𝘆𝘁𝗶𝗰𝘀 & 𝗔𝗜. Modern alternative to Snowflake. Cost-effective and simple for massive-scale analytics. https://databend.com][databend~github]↗
- [`rerun`: Visualize streams of multimodal data. Free, fast, easy to use, and simple to integrate. Built in Rust.][c~rerun~github]↗
- [`tock`: A secure embedded operating system for microcontrollers][tock~github]↗
- [`mail-server`: Secure & Modern All-in-One Mail Server (IMAP, JMAP, POP3, SMTP)][mail-server~github]↗
- [`hickory-dns`: A Rust based DNS client, server, and resolver][hickory-dns~github]↗
- [`sniffnet`: Comfortably monitor your Internet traffic][sniffnet~github]↗
- [`ratatui`: App Showcase][ratatui-showcase-apps~website]↗
- [`habitat`: Modern applications with built-in automation][habitat~github]↗
- [`cargo-binstall`: Binary installation for rust projects][c~cargo-binstall~github]↗
- [`cargo-update`][c~cargo-update~crates.io]↗
- [`alacritty`: A cross-platform, OpenGL terminal emulator.][alacritty~github]↗
- [`cargo-hakari`][c~cargo-hakari~crates.io]↗
- [`spacedrive`: Spacedrive is an open source cross-platform file explorer, powered by a virtual distributed filesystem written in Rust.][spacedrive~github]↗
- [`swc`: Rust-based platform for the Web](https://github.com/swc-project/swc)↗.
- [`influxdb`: Scalable datastore for metrics, events, and real-time analytics][influxdb~github]↗
- [`firecracker`: Secure and fast microVMs for serverless computing.][firecracker~github]↗
- [`Warp`: Warp is a modern, Rust-based terminal with AI built in so you and your team can build great software, faster.][warp~github]↗
- [RustPython: A Python Interpreter written in Rust][rustpython~github]↗
- [wezterm: A GPU-accelerated cross-platform terminal emulator and multiplexer written by @wez and implemented in Rust][wezterm~github]↗
- [redox-os: Mirror of https://gitlab.redox-os.org/redox-os/redox][redox~github]↗
- [Neon: Serverless Postgres. We separated storage and compute to offer autoscaling, code-like database branching, and scale to zero.][neon~github]↗
- [navi: An interactive cheatsheet tool for the command-line](https://github.com/denisidoro/navi)↗
- [RustScan: The Modern Port Scanner][rustscan~github]↗
- [workers-rs: Write Cloudflare Workers in 100% Rust via WebAssembly][workers-rs~github]↗
- [Qdrant - High-performance, massive-scale Vector Database and Vector Search Engine for the next generation of AI. Also available in the cloud https://cloud.qdrant.io/][c~qdrant~github]↗
- [Hyperlight is a lightweight Virtual Machine Manager (VMM) designed to be embedded within applications. It enables safe execution of untrusted code within micro virtual machines with very low latency and minimal overhead.][c~hyperlight~github]↗
- [Perseus][perseus~website]↗
- [DORA (Dataflow-Oriented Robotic Architecture) is middleware designed to streamline and simplify the creation of AI-based robotic applications. It offers low latency, composable, and distributed dataflow capabilities. Applications are modeled as directed graphs, also referred to as pipelines.][dora~github]↗
- [television][c~television~crates.io]↗

- [Fish 4.0: The Fish Of Theseus][blog~fishshell-rustport]↗

- [shoes: A multi-protocol proxy server written in Rust (HTTP, SOCKS5, Vmess, Vless, Shadowsocks, Trojan, Snell, Hysteria2, TUIC v5)][c~shoes~github]↗

- [Jujutsu (docs)][doc~jujutsu]↗
- [Steve's Jujutsu Tutorial][blog~steveklabnik-jujutsu-tutorial]↗

- [Codename Goose][doc~block-goose]↗
- [grex: A command-line tool and Rust library with Python bindings for generating regular expressions from user-provided test cases][c~grex~github]↗
- [youki: A container runtime written in Rust][youki~github]↗

</div>
