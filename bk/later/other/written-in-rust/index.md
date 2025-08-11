# Software Built in Rust

This section provides a few examples of software written in Rust, organized by topic, to give an idea of what has been and can be built with Rust. The [`awesome-rust`][awesome-rust~github] repository contains a far more complete list of Rust applications, dev tools, and libraries.

## Systems Programming

- Operating System Components:
  - Kernel modules (Parts of the Linux kernel are now being written in Rust.)
    - [Rust-for-Linux: Adding support for the Rust language to the Linux kernel.](https://github.com/Rust-for-Linux/linux)↗.
  - Bootloaders.
  - Virtual machine monitors. See [[virtualization | Virtualization]].
  - Examples:
    - `Redox OS`: A microkernel operating system written in Rust, aiming for safety and security.
    - `Stratis`: A local storage management system for Linux.
    - `youki` [(GitHub)](https://github.com/youki-dev/youki)↗: a container runtime.
    - `Kata Containers` [(GitHub)](https://github.com/kata-containers/kata-containers)↗: lightweight Virtual Machines (VMs) that feel and perform like containers, but provide the workload isolation and security advantages of VMs.
    - `firecracker` [(GitHub)](https://github.com/firecracker-microvm/firecracker): microVMs for serverless computing.
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
    - [`ripgrep`][c~ripgrep~docs] [(crates.io)](https://crates.io/crates/ripgrep)↗: a line-oriented search tool that recursively searches the current directory for a regex pattern.
    - [`exa`][c~exa~docs]↗{{hi:exa}}: a modern replacement for `ls`.
    - [`eza`](https://github.com/eza-community/eza): another alternative to `ls`.
    - [`lsd`][c~lsd~docs] [(GitHub)](https://github.com/lsd-rs/lsd)↗{{hi:lsd}}: next-gen `ls` command.
    - [`bat`][c~bat~docs] [(GitHub)][c~bat~github]↗{{hi:bat}}: a `cat` clone with syntax highlighting and Git integration.
    - [`fd`][c~fd~docs] [(GitHub)][c~fd-find~github]↗: an alternative to 'find'.
    - [`fish`](https://github.com/fish-shell/fish-shell): a command-line shell.
    - [`nushell`][nushell~website]↗ [(GitHub)](https://github.com/nushell/nushell): a new type of shell.
    - [`zoxide`][c~zoxide~docs]↗{{hi:zoxide}}: a smarter `cd` command.
    - [`dust`][c~dust~docs] [(GitHub)](https://github.com/bootandy/dust)↗: a more intuitive version of `du`.
    - [`bottom`][c~bottom~docs] [(GitHub)][c~bottom~github]↗: a cross-platform graphical process/system monitor.
    - [`sd`][c~sd~docs]↗{{hi:sd}}: an intuitive find & replace CLI.
    - [`starship`][c~starship~docs] [(GitHub)][c~starship~github]↗{{hi:starship}} - a customizable prompt for any shell. See [[shells | Shells]].
    - [`broot`][c~broot~docs] [(GitHub)][c~broot~github]↗: a tool to navigate directories.
    - [`television`][c~television~docs] [(GitHub)](https://github.com/alexpasmantier/television){{hi:television}}↗: a cross-platform and extensible general purpose fuzzy finder TUI.
    - [`sd`][c~sd~docs] [(crates.io)](https://crates.io/crates/sd)↗: an intuitive find & replace CLI.
    - `uutils` [(GitHub)](https://github.com/uutils/coreutils)↗: cross-platform Rust rewrite of the GNU coreutils.

## Web Development and Networking

- Web Servers:
  - High-performance web servers.
  - API servers.
  - Examples: Cloudflare uses Rust for performance critical services.
- Search Engines:
  - [`tantivy`][c~tantivy~docs] [(GitHub)](https://github.com/quickwit-oss/tantivy)↗: a full-text search engine library inspired by Apache Lucene.
  - [`meilisearch`][c~meilisearch~docs] [(GitHub)](https://github.com/meilisearch/MeiliSearch)↗: a fast search engine API bringing AI-powered hybrid search to sites and applications.
- WebAssembly ([[wasm | WASM]]):
  - Front-end web applications.
  - Portable code for web browsers.
    - Libraries like [`yew`][c~yew~docs]↗{{hi:yew}} and [`leptos`][c~leptos~docs]↗{{hi:leptos}} enable building complex front-end web applications with Rust.
    - [`wasmer`][c~wasmer~docs] [(GitHub)][c~wasmer~github]↗: lightweight containers based on WebAssembly.
- Networking:
  - Network protocols.
  - VPN software.
  - Proxy servers.
  - Examples:
    - [`boringtun`](https://github.com/cloudflare/boringtun): a `Wireguard` VPN implementations.
    - [`Hickory DNS`](https://github.com/hickory-dns/hickory-dns): a DNS client, server, and resolver.
    - [`pingora`][c~pingora~docs] [(GitHub)](https://github.com/cloudflare/pingora)↗: a library for building fast, reliable and evolvable network services.
- Tools:
  - [`imager`](https://github.com/imager-io/imager): automated image compression for efficiently distributing images on the web.

## Application Software

- Browsers:
  - Components of web browsers.
  - Example: [`Mozilla Firefox`]( ){{hi: }} uses Rust in its `Servo` rendering engine.
- Databases:
  - Database engines.
  - Database drivers.
  - Examples:
    - [`TiKV`](https://github.com/tikv/tikv): an open-source, distributed, and transactional key-value database.
    - [`neon`][c~neon~docs] [(GitHub)](https://github.com/neondatabase/neon)↗: Serverless Postgres. Separated storage and compute to offer autoscaling, code-like database branching, and scale to zero.
    - [`ArangoDB`](https://arangodb.com): a graph database.
- Analytics:
  - [`databend`](https://github.com/databendlabs/databend) - alternative to Snowflake.
  - [`rerun`][c~rerun~docs][(GitHub)](https://github.com/rerun-io/rerun)↗{{hi:rerun}} visualizes streams of multimodal data.
  - [`polars`][c~polars~docs] [(GitHub)](https://github.com/pola-rs/polars)↗: dataframes powered by a multithreaded, vectorized query engine.
  - [`datafusion`][c~datafusion~docs] [(GitHub)](https://github.com/apache/datafusion)↗: Apache DataFusion SQL Query Engine.
- Distributed Systems:
  - Blockchain technology.
  - Cloud computing tools.
  - Example: `Substrate`, a framework for building blockchains.
- Games:
  - [`Veloren`](https://gitlab.com/veloren/veloren): a multiplayer voxel RPG written in Rust.
- Other:
  - [`hyperswitch`](https://github.com/juspay/hyperswitch): an open-source payments switch to make payments fast, reliable and affordable.
  - [`rustdesk`][c~rustdesk~docs] [(GitHub)](https://github.com/rustdesk/rustdesk)↗{{hi:rustdesk}}: an open-source remote desktop application designed for self-hosting, as an alternative to TeamViewer.

## Development Tools

- Terminal Emulators:
  - [`alacritty`](https://github.com/alacritty/alacritty)↗: a fast, cross-platform, OpenGL terminal emulator.
- Editors:
  - `lapce` [(GitHub)](https://github.com/lapce/lapce)↗: a code editor.
  - `Zed` [(GitHub)](https://github.com/zed-industries/zed)↗: a multiplayer code editor from the creators of Atom and Tree-sitter.
  - `helix` [(GitHub)](https://github.com/helix-editor/helix)↗: a post-modern modal text editor.
- Compilers, Interpreters, Runtimes:
  - Language compilers.
  - Language interpreters.
  - Examples:
    - [`SWC`][c~swc~docs]↗{{hi:SWC}}: a very fast Typescript/Javascript compiler.
    - [`Deno`][c~deno~docs] [(GitHub)][c~deno~github]↗{{hi:Deno}}: a JavaScript, TypeScript, and WebAssembly runtime.
    - `starlark-rust` [(GitHub)](https://github.com/facebook/starlark-rust)↗: a Rust implementation of the Starlark language. Starlark is a deterministic language inspired by Python3, used for configuration in the build systems `Bazel`, `Buck` and `Buck2`.
- Package Managers:
  - Software distribution tools.
  - Example: [`habitat`](https://github.com/habitat-sh/habitat): open-source software that creates platform-independent build artifacts and provides built-in deployment and management capabilities.
- Linters and Formatters:
  - [`ruff`][c~ruff~docs] [(GitHub)](https://github.com/astral-sh/ruff)↗: an extremely fast Python linter and code formatter.
- Cache Cleaning Utilities:
  - [`kondo`](https://github.com/tbillington/kondo)↗ cleans dependencies and build artifacts from your projects.
- Desktop Frameworks
  - [`tauri`][c~tauri~docs][(GitHub)][c~tauri~github]↗: build desktop and mobile applications with a web frontend.
- Game Development:
  - Game engines.
  - Game tools.
  - Example: [`Bevy`][c~bevy~docs]↗{{hi:Bevy}} game engine.
- Observability:
  - [`openobserve`](https://github.com/openobserve/openobserve): petabyte scale Elasticsearch/Splunk/Datadog alternative for logs, metrics, traces, RUM, Error tracking, Session replay.
  - [`opentelemetry-rust`](https://github.com/open-telemetry/opentelemetry-rust): the Rust OpenTelemetry implementation.
  - [`influxdb`]( ){{hi: }} [(GitHub)](https://github.com/influxdata/influxdb): a scalable datastore for metrics, events, and real-time analytics.

{{#include development_tools.incl.md}}

## Python Tools Written in Rust

{{#include python_tools.incl.md}}

## Other Tools Written in Rust

{{#include other_tools.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[organize; decide if we need other pages or if we consolidate here; table?](https://github.com/john-cd/rust_howto/issues/993)

- [awesome-rust-tools: Harness the power of Rust. Those fast productivity tools based on Rust.](https://github.com/unpluggedcoder/awesome-rust-tools)
- [awesome-alternatives-in-rust: A curated list of replacements for existing software written in Rust](https://github.com/TaKO8Ki/awesome-alternatives-in-rust)

- [RustDesk](https://rustdesk.com/docs/en/)
- [OxidOS Automotive](https://oxidos.io/)
- [ripgrep: `ripgrep` recursively searches directories for a regex pattern while respecting your gitignore](https://github.com/BurntSushi/ripgrep)
- [zola: A fast static site generator in a single binary with everything built-in. https://www.getzola.org](https://github.com/getzola/zola)
- [solana: Web-Scale Blockchain for fast, secure, scalable, decentralized apps and marketplaces.][c~solana~github]
- [linkerd2: Ultralight, security-first service mesh for Kubernetes. Main repo for Linkerd 2.x.](https://github.com/linkerd/linkerd2)
- [bottlerocket: An operating system designed for hosting containers][c~bottlerocket~github]
- [`databend`: 𝗗𝗮𝘁𝗮, 𝗔𝗻𝗮𝗹𝘆𝘁𝗶𝗰𝘀 & 𝗔𝗜. Modern alternative to Snowflake. Cost-effective and simple for massive-scale analytics. https://databend.com](https://github.com/databendlabs/databend)
- [`rerun`: Visualize streams of multimodal data. Free, fast, easy to use, and simple to integrate. Built in Rust.](https://github.com/rerun-io/rerun)
- [`tock`: A secure embedded operating system for microcontrollers](https://github.com/tock/tock)
- [`mail-server`: Secure & Modern All-in-One Mail Server (IMAP, JMAP, POP3, SMTP)](https://github.com/stalwartlabs/mail-server)
- [`hickory-dns`: A Rust based DNS client, server, and resolver](https://github.com/hickory-dns/hickory-dns)
- [`sniffnet`: Comfortably monitor your Internet traffic](https://github.com/GyulyVGC/sniffnet)
- [`ratatui`: App Showcase](https://ratatui.rs/showcase/apps/)
- [`habitat`: Modern applications with built-in automation](https://github.com/habitat-sh/habitat)
- [`cargo-binstall`: Binary installation for rust projects](https://github.com/cargo-bins/cargo-binstall)
- [`cargo-update`](https://crates.io/crates/cargo-update)
- [`alacritty`: A cross-platform, OpenGL terminal emulator.](https://github.com/alacritty/alacritty)
- [`cargo-hakari`](https://crates.io/crates/cargo-hakari)
- [`spacedrive`: Spacedrive is an open source cross-platform file explorer, powered by a virtual distributed filesystem written in Rust.](https://github.com/spacedriveapp/spacedrive)
- [`swc`: Rust-based platform for the Web](https://github.com/swc-project/swc)
- [`influxdb`: Scalable datastore for metrics, events, and real-time analytics](https://github.com/influxdata/influxdb)
- [`firecracker`: Secure and fast microVMs for serverless computing.](https://github.com/firecracker-microvm/firecracker)
- [`Warp`: Warp is a modern, Rust-based terminal with AI built in so you and your team can build great software, faster.](https://github.com/warpdotdev/Warp)
- [RustPython: A Python Interpreter written in Rust][rustpython~github]
- [wezterm: A GPU-accelerated cross-platform terminal emulator and multiplexer written by @wez and implemented in Rust](https://github.com/wez/wezterm)
- [redox-os: Mirror of https://gitlab.redox-os.org/redox-os/redox][redox~github]
- [Neon: Serverless Postgres. We separated storage and compute to offer autoscaling, code-like database branching, and scale to zero.](https://github.com/neondatabase/neon)
- [navi: An interactive cheatsheet tool for the command-line](https://github.com/denisidoro/navi)
- [RustScan: The Modern Port Scanner](https://github.com/RustScan/RustScan)
- [workers-rs: Write Cloudflare Workers in 100% Rust via WebAssembly](https://github.com/cloudflare/workers-rs)
- [Qdrant - High-performance, massive-scale Vector Database and Vector Search Engine for the next generation of AI. Also available in the cloud https://cloud.qdrant.io/][c~qdrant~github]
- [Hyperlight is a lightweight Virtual Machine Manager (VMM) designed to be embedded within applications. It enables safe execution of untrusted code within micro virtual machines with very low latency and minimal overhead.][c~hyperlight~github]
- [Perseus](https://framesurge.sh/perseus/en-US/)
- [DORA (Dataflow-Oriented Robotic Architecture) is middleware designed to streamline and simplify the creation of AI-based robotic applications. It offers low latency, composable, and distributed dataflow capabilities. Applications are modeled as directed graphs, also referred to as pipelines.](https://github.com/dora-rs/dora)
- [television][c~television~crates.io]

- [Fish 4.0: The Fish Of Theseus](https://fishshell.com/blog/rustport/)

- [shoes: A multi-protocol proxy server written in Rust (HTTP, SOCKS5, Vmess, Vless, Shadowsocks, Trojan, Snell, Hysteria2, TUIC v5)][c~shoes~github]

- [Jujutsu (docs)](https://jj-vcs.github.io/jj/latest/)
- [Steve's Jujutsu Tutorial](https://steveklabnik.github.io/jujutsu-tutorial/)

- [Codename Goose](https://block.github.io/goose/docs/quickstart)
- [grex: A command-line tool and Rust library with Python bindings for generating regular expressions from user-provided test cases][c~grex~github]
- [youki: A container runtime written in Rust](https://github.com/youki-dev/youki)

</div>
