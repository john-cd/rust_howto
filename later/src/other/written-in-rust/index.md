<!-- TODO Reviewed ??-??-2026. -->

# Software Built in Rust

Rust powers projects ranging from command-line utilities to databases and virtual machine monitors. This guide highlights standalone Rust projects and identifies the Rust components in software that uses several languages.

## Systems Programming

| Area | Examples and details |
| --- | --- |
| Operating systems | [Redox OS][redox~repo]↗ is a Rust operating system. [Bottlerocket][c~bottlerocket~repo]↗ is a Linux distribution for containers, and [Rust for Linux][kernel-rust~docs]↗ brings Rust support to the Linux kernel. |
| Containers and virtualization | [youki][youki~repo]↗ is an OCI container runtime. [Kata Containers][kata-containers~repo]↗ includes a Rust agent. [Firecracker][firecracker~repo]↗ and [Cloud Hypervisor][cloud-hypervisor~repo]↗ are virtual machine monitors, and [Hyperlight][hyperlight~repo]↗ isolates workloads in micro-VMs. See [Virtualization](../../categories/virtualization/index.md). |
| Storage | [Stratis][stratis~repo]↗ provides a storage management daemon for Linux. |
| Embedded systems | [Tock OS][tock~repo]↗, [`esp-rs`][espressif~repo]↗, and [OxidOS Automotive][oxidos~website]↗; see [Embedded Systems](../../categories/embedded/index.md). |
| Command-line utilities | See the [command-line tools](development_tools.md#utilities) for examples such as `ripgrep`, `fd`, and `bat`. |

## Web Development and Networking

| Area | Examples and details |
| --- | --- |
| Search | [Tantivy][c~tantivy~docs]↗ and [Meilisearch][c~meilisearch~docs]↗. |
| Web applications | [Yew][c~yew~docs]↗, [Leptos][c~leptos~docs]↗, and [Perseus][c~perseus~repo]↗ are web frameworks. Cloudflare's [`workers-rs`][workers-rs~repo]↗ SDK lets developers write Workers in Rust. |
| WebAssembly runtimes | [Wasmtime][wasmtime~repo]↗ and [Wasmer][c~wasmer~docs]↗ run WebAssembly outside the browser. See [WebAssembly](../../categories/wasm/index.md). |
| Networking | [Hickory DNS][hickory-dns~repo]↗ implements DNS, [BoringTun][boringtun~repo]↗ implements WireGuard, and [Rustls][rustls~repo]↗ implements TLS. [Pingora][c~pingora~docs]↗ is Cloudflare's proxy framework, and [Linkerd][linkerd-proxy~repo]↗ uses a Rust data-plane proxy. Other examples include the [shoes proxy][c~shoes~repo]↗, the [RustScan port scanner][rustscan~repo]↗, and [mail servers](other_tools.md#mail-servers). |

## Application Software

| Area | Examples and details |
| --- | --- |
| Browsers | [Servo][servo~repo]↗ is a browser engine written in Rust. [Firefox][firefox-rust~docs]↗ uses Rust for components including Stylo, its CSS engine, and WebRender, its GPU rendering engine. |
| Databases and analytics | [TiKV][tikv~repo]↗ is a distributed key-value store. [Neon][neon~repo]↗ uses Rust for its PostgreSQL storage layer. [Databend][databend~repo]↗, [Qdrant][qdrant~repo]↗, [InfluxDB 3][influxdb3~repo]↗, [Apache DataFusion][c~datafusion~docs]↗, and [Polars][c~polars~docs]↗ cover cloud analytics, vector search, time-series data, SQL execution, and DataFrames. |
| Blockchains | [Polkadot SDK][polkadot-sdk~repo]↗ includes Substrate, a framework for building blockchains. [Agave][agave~repo]↗ is a Rust validator client for Solana. |
| Desktop software | [COSMIC][cosmic-desktop~website]↗ is a Wayland desktop environment. [Tauri][c~tauri~repo]↗ is a framework for building apps with a Rust backend and a web-based interface. [Sniffnet][c~sniffnet~repo]↗ monitors network traffic. See also [file managers](other_tools.md#file-managers) and [remote desktop tools](other_tools.md#remote-desktops). |
| Games and visualization | [Bevy][c~bevy~repo]↗, [Veloren][veloren~repo]↗, and [Rerun][c~rerun~docs]↗ are a game engine, a multiplayer voxel RPG, and a visualization toolkit for time-series data, respectively. |

## Development Tools

{{#include development_tools.incl.md}}

## Python Tools Written in Rust

{{#include python_tools.incl.md}}

## Other Tools Written in Rust

{{#include other_tools.incl.md}}

## Related Topics

| Topic | Why it is related |
| --- | --- |
| [Awesome Rust][awesome-rust~repo]↗ | Broader catalog of projects, libraries, and tools. |
| [Awesome Rust Tools][awesome-rust-tools~repo]↗ and [Awesome Alternatives in Rust][awesome-alternatives-in-rust~repo]↗ | Curated lists of Rust-built tools and alternatives. |
| [Scripting and Rust](../scripting/index.md) | Scripting engines and language integration. |
| [Embedded Systems](../../categories/embedded/index.md) | Firmware and constrained applications. |
| [WebAssembly](../../categories/wasm/index.md) | Servers, clients, and web tooling. |
| [Virtualization](../../categories/virtualization/index.md) | Runtime and isolation projects. |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
