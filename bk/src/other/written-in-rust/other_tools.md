<!-- TODO Reviewed ??-??-2026. -->

# Other Tools Written in Rust

{{#include other_tools.incl.md}}

These applications and services use Rust for tasks such as browsing files, processing payments, and moving telemetry.

## Desktop and Communication {#desktop-and-communication}

| Tool | What it does | Project |
| --- | --- | --- |
| <span id="file-managers"></span>Spacedrive | File explorer with a Rust-based virtual distributed filesystem. | [Repository][spacedrive~repo]↗ |
| <span id="remote-desktops"></span>RustDesk | Remote desktop client with self-hosting options. | [Repository][c~rustdesk~repo]↗ |
| <span id="email-applications"></span>Postsack | Groups email by sender and domain to help identify unwanted mail. | [Repository][postsack~repo]↗ |
| <span id="mail-servers"></span>Stalwart | Mail and collaboration server supporting protocols including SMTP, IMAP, and JMAP. | [Repository][stalwart~project~repo]↗ |
| <span id="news"></span>Headlines | News-reading GUI application; upstream labels it a work in progress. | [Repository][headlines~repo]↗ |
| <span id="social-media"></span>Lemmy | Federated community discussion and link aggregation service. | [Repository][lemmy~repo]↗ |

## Payments and Observability {#services-and-observability}

| Tool | What it does | Project |
| --- | --- | --- |
| <span id="payments"></span>Hyperswitch | Payment orchestration platform that connects applications to payment providers. | [Repository][hyperswitch~repo]↗ |
| <span id="observability"></span>OpenObserve | Observability platform for collecting and querying logs, metrics, and traces. | [Repository][openobserve~repo]↗ |
| Vector | Collects, transforms, and routes logs and metrics. | [Repository][vector-observability~repo]↗ |
| OpenTelemetry Rust | Provides instrumentation APIs, SDKs, and exporters for Rust applications. | [Repository][opentelemetry-rust~repo]↗ |

## Interactive Computing, Automation, and Robotics {#other-applications}

| Tool | What it does | Project |
| --- | --- | --- |
| evcxr Jupyter | Jupyter kernel for executing Rust interactively in notebooks. | [Repository][evcxr-jupyter~repo]↗ |
| Goose | Extensible AI agent with a Rust backend for automating development tasks. | [Repository][goose~project~repo]↗ |
| DORA | Dataflow middleware for composing and running robotics applications. | [Repository][dora~repo]↗ |

The [Ratatui app showcase][ratatui-showcase-apps~website]↗ collects more terminal applications. For shells, editors, file navigation, and terminal search, use the [development tools catalog](development_tools.md#utilities).

## Related Topics {#related-topics .skip}

- [Software Built in Rust](index.md) for the overview and other application categories.
- [Development Tools Written in Rust](development_tools.md) for editors, build systems, and command-line utilities.
- [Python Tools Written in Rust](python_tools.md) for Python interpreters, packaging, linting, and environment management.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
