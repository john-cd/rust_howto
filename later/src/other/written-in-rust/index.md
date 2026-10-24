<!-- TODO Reviewed ??-??-2026 -->

# Software Built in Rust

This overview highlights a small set of software projects that demonstrate where Rust is used. It is a curated introduction, not a complete catalog.

## Systems Programming

| Area | Examples and details |
|---|---|
| Operating systems and isolation | [youki][youki~repo]↗, [Firecracker][firecracker~repo]↗, [Rust for Linux][rust-for-linux~repo]↗; see [[virtualization | Virtualization]]. |
| Embedded systems | [Tock OS][tock~repo]↗ and [Espressif Rust][espressif~repo]↗; see [[embedded | Embedded Systems]]. |
| Command-line utilities | [ripgrep][c~ripgrep~docs]↗, [bat][c~bat~docs]↗, [fd][c~fd~docs]↗, [eza][eza~repo]↗, and [uutils][coreutils~repo]↗; see [[filesystem_cli | Filesystem CLI]]. |

## Web Development and Networking

| Area | Examples and details |
|---|---|
| Search | [Tantivy][c~tantivy~docs]↗ and [Meilisearch][c~meilisearch~docs]↗. |
| WebAssembly and runtimes | [Deno][c~deno~docs]↗ and [Wasmer][c~wasmer~docs]↗; see [[wasm | WebAssembly]]. |
| Networking | [Hickory DNS][hickory-dns~repo]↗, [BoringTun][boringtun~repo]↗, and [Pingora][c~pingora~docs]↗. |

## Application Software

| Area | Examples and details |
|---|---|
| Databases and analytics | [TiKV][tikv~repo]↗, [DataFusion][c~datafusion~docs]↗, and [Polars][c~polars~docs]↗. |
| Desktop applications | [RustDesk][c~rustdesk~repo]↗ and [Sniffnet][c~sniffnet~repo]↗. |
| Games and visualization | [Veloren][veloren~repo]↗ and [Rerun][c~rerun~docs]↗. |

## Development Tools

{{#include development_tools.incl.md}}

See the [development tools detail page](development_tools.md) for version control, build systems, editors, and JavaScript tooling.

## Python Tools Written in Rust

{{#include python_tools.incl.md}}

See the [Python tooling detail page](python_tools.md) for interpreters, package tools, and Python bindings.

## Other Tools Written in Rust

{{#include other_tools.incl.md}}

The [other tools detail page](other_tools.md) groups file managers, remote desktops, email, news, payments, and social media applications.

## Related Topics

| Topic | Why it is related |
|---|---|
| [Awesome Rust][awesome-rust~repo]↗ | Broader catalog of projects, libraries, and tools. |
| [[development-tools | Development Tools]] | Tools built with or used alongside Rust. |
| [[scripting | Scripting and Rust]] | Scripting engines and language integration. |
| [[embedded | Embedded Systems]] | Firmware and constrained applications. |
| [[web-programming | Web Programming]] | Servers, clients, and web tooling. |
| [[virtualization | Virtualization]] | Runtime and isolation projects. |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[content audit: Written-in-Rust tool lists trimmed by recent commits; review and restore removed entries](https://github.com/john-cd/rust_howto/issues/1902)
</div>
