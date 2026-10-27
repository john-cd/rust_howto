<!-- TODO Reviewed ??-??-2026 -->

# Other Tools Written in Rust

{{#include other_tools.incl.md}}

This catalog groups applications and services that use Rust for all or part of their implementation. It complements the [development tools](development_tools.md) and [Python tools](python_tools.md) pages. Upstream links describe each project's architecture, installation, license, and supported platforms; inclusion here is not a maintenance guarantee.

## Desktop and Communication {#desktop-and-communication}

Some applications pair a Rust core with a user interface written in another language. Check the linked project before choosing a deployment or installation method.

| Tool                                          | What it does                                                                       | Project                              |
| --------------------------------------------- | ---------------------------------------------------------------------------------- | ------------------------------------ |
| <span id="file-managers"></span>Spacedrive    | File explorer with a Rust-based virtual distributed filesystem.                    | [Repository][spacedrive~repo]↗       |
| <span id="remote-desktops"></span>RustDesk    | Remote desktop client with self-hosting options.                                   | [Repository][c~rustdesk~repo]↗       |
| <span id="email-applications"></span>Postsack | Groups email by sender and domain to help identify unwanted mail.                  | [Repository][postsack~repo]↗         |
| <span id="mail-servers"></span>Stalwart       | Mail and collaboration server supporting protocols including SMTP, IMAP, and JMAP. | [Repository][stalwart~project~repo]↗ |
| <span id="news"></span>Headlines              | News-reading GUI application; upstream labels it a work in progress.               | [Repository][headlines~repo]↗        |
| <span id="social-media"></span>Lemmy          | Federated community discussion and link aggregation service.                       | [Repository][lemmy~repo]↗            |

## Payments and Observability {#services-and-observability}

These projects run as services or integrate into an application. An observability platform stores and queries telemetry; an instrumentation library produces and exports it.

| Tool                                        | What it does                                                                                                                 | Project                                |
| ------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- | -------------------------------------- |
| <span id="payments"></span>Hyperswitch      | Payment orchestration platform that connects applications to payment providers.                                              | [Repository][hyperswitch~repo]↗        |
| <span id="observability"></span>OpenObserve | Observability platform for collecting and querying logs, metrics, and traces.                                                | [Repository][openobserve~repo]↗        |
| OpenTelemetry Rust                          | Instrumentation APIs, SDKs, and exporters for Rust applications; a library suite rather than a standalone monitoring server. | [Repository][opentelemetry-rust~repo]↗ |

## Interactive Computing, Automation, and Robotics {#other-applications}

| Tool          | What it does                                                              | Project                           |
| ------------- | ------------------------------------------------------------------------- | --------------------------------- |
| evcxr Jupyter | Jupyter kernel for executing Rust interactively in notebooks.             | [Repository][evcxr-jupyter~repo]↗ |
| Goose         | Extensible AI agent with a Rust backend for automating development tasks. | [Repository][goose~project~repo]↗ |
| DORA          | Dataflow middleware for composing and running robotics applications.      | [Repository][dora~repo]↗          |

The [Ratatui app showcase][ratatui-showcase-apps~website]↗ collects more terminal applications. For shells, editors, file navigation, and terminal search, use the [development tools catalog](development_tools.md#utilities).

## Related Topics {#related-topics .skip}

- [Software Built in Rust](index.md) for the overview and other application categories.
- [Development Tools Written in Rust](development_tools.md) for editors, build systems, and command-line utilities.
- [Python Tools Written in Rust](python_tools.md) for Python interpreters, packaging, linting, and environment management.

{{#include refs.incl.md}}
{{#include ../../../../bk/src/refs/link-refs.md}}
