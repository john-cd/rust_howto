<!-- TODO Reviewed ??-??-2026 -->

# Software Built in Rust

This overview highlights a small set of software projects that demonstrate where Rust is used. It is a curated introduction, not a complete catalog.

## Systems Programming

| Area                            | Examples and details                                                                                                                                                                                                                                                                                                                   |
| ------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Operating systems and isolation | [Redox OS][redox~repo]↗, [Stratis][stratis~repo]↗, [youki][youki~repo]↗, [Kata Containers][kata-containers~repo]↗, [Firecracker][firecracker~repo]↗, [Bottlerocket][c~bottlerocket~repo]↗, [Hyperlight][hyperlight~repo]↗, and [Rust for Linux][rust-for-linux~repo]↗; see [Virtualization](../../categories/virtualization/index.md). |
| Embedded systems                | [Tock OS][tock~repo]↗, [`esp-rs`][espressif~repo]↗, and [OxidOS Automotive][oxidos~website]↗; see [Embedded Systems](../../categories/embedded/index.md).                                                                                                                                                                              |
| Command-line utilities          | See the [[command-line-utilities                                                                                                                                                                                                                                                                                                       | Command-line Utilities Written in Rust]] index for command-line tools; see the [additional command-line examples](development_tools.md#utilities) and Filesystem CLI chapter. |

## Web Development and Networking

| Area                           | Examples and details                                                                                                                                                                                                      |
| ------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Search                         | [Tantivy][c~tantivy~docs]↗ and [Meilisearch][c~meilisearch~docs]↗.                                                                                                                                                        |
| Web and front-end applications | [Yew][c~yew~docs]↗, [Leptos][c~leptos~docs]↗, [Perseus][perseus~website]↗, and [Cloudflare `workers-rs`][workers-rs~repo]↗, which supports writing Workers in Rust. Cloudflare also runs Rust services.                   |
| WebAssembly and runtimes       | [Deno][c~deno~docs]↗ and [Wasmer][c~wasmer~docs]↗; see [WebAssembly](../../categories/wasm/index.md).                                                                                                                     |
| Networking                     | [Hickory DNS][hickory-dns~repo]↗, [BoringTun][boringtun~repo]↗, [Pingora][c~pingora~docs]↗, [Linkerd][linkerd2~repo]↗, [shoes][c~shoes~repo]↗, [RustScan][rustscan~repo]↗, and [Stalwart Mail Server][mail-server~repo]↗. |

## Application Software

| Area                    | Examples and details                                                                                                                                                                                                       |
| ----------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Browsers                | [Firefox][mozilla-firefox~website]↗ and the [Servo engine][servo~repo]↗.                                                                                                                                                   |
| Databases and analytics | [TiKV][tikv~repo]↗, [Neon][neon~repo]↗, [ArangoDB][arangodb.com~website]↗, [Databend][databend~repo]↗, [Qdrant][qdrant~repo]↗, [InfluxDB][influxdb~repo]↗, [DataFusion][c~datafusion~docs]↗, and [Polars][c~polars~docs]↗. |
| Distributed systems     | [Substrate][substrate~repo]↗ and [Solana][c~solana~repo]↗.                                                                                                                                                                 |
| Desktop applications    | [Tauri][c~tauri~repo]↗, [Spacedrive][spacedrive~website]↗, [RustDesk][c~rustdesk~repo]↗, and [Sniffnet][c~sniffnet~repo]↗.                                                                                                 |
| Games and visualization | [Bevy][c~bevy~repo]↗, [Veloren][veloren~repo]↗, and [Rerun][c~rerun~docs]↗.                                                                                                                                                |

## Development Tools

{{#include development_tools.incl.md}}

See the [development tools detail page](development_tools.md) for version control, build systems, editors, and JavaScript tooling.

## Python Tools Written in Rust

{{#include python_tools.incl.md}}

See the [Python tooling detail page](python_tools.md) for interpreters, package tools, and Python bindings.

## Other Tools Written in Rust

{{#include other_tools.incl.md}}

The [other tools detail page](other_tools.md) groups file managers, remote desktops, email, news, payments, observability, and other applications.

## Related Topics

| Topic                                                                                                                 | Why it is related                                   |
| --------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------- |
| [Awesome Rust][awesome-rust~repo]↗                                                                                    | Broader catalog of projects, libraries, and tools.  |
| [Awesome Rust Tools][awesome-rust-tools~repo]↗ and [Awesome Alternatives in Rust][awesome-alternatives-in-rust~repo]↗ | Curated lists of Rust-built tools and alternatives. |
| [Development Tools](development_tools.md)                                                                             | Tools built with or used alongside Rust.            |
| [Scripting and Rust](../scripting/index.md)                                                                           | Scripting engines and language integration.         |
| [Embedded Systems](../../categories/embedded/index.md)                                                                | Firmware and constrained applications.              |
| [WebAssembly](../../categories/wasm/index.md)                                                                         | Servers, clients, and web tooling.                  |
| [Virtualization](../../categories/virtualization/index.md)                                                            | Runtime and isolation projects.                     |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [content audit: Written-in-Rust tool lists trimmed by recent commits; review and restore removed entries](https://github.com/john-cd/rust_howto/issues/1902)
</div>
