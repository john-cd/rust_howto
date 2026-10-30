<!-- TODO Reviewed ??-??-2026 -->

Distributed tracing links operations across services into a trace. Rust applications commonly emit spans with [`tracing`][c~tracing~docs] and bridge them to OpenTelemetry with [`tracing-opentelemetry`][tracing-opentelemetry~docs]. The OpenTelemetry SDK batches spans and exports them in the OpenTelemetry Protocol (OTLP) to a collector or compatible backend.

| Recipe                                                                                                                       | Crates                                          | Categories                                             |
| ---------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------- | ------------------------------------------------------ |
| [Export Traces with OpenTelemetry and OTLP][ex~development-tools_debugging~open-telemetry]                                   | [`opentelemetry-otlp`][opentelemetry-otlp~docs] | {{#categories development-tools::debugging=Debugging}} |
| [Propagate Trace Context Across HTTP][ex~development-tools_debugging~propagate-context-across-http]                          | [`opentelemetry-http`][opentelemetry-http~docs] | {{#categories development-tools::debugging=Debugging}} |
| [Send Traces to Jaeger, OpenObserve, or Quickwit][ex~development-tools_debugging~other-common-observability-tools-platforms] | [OpenTelemetry Rust][opentelemetry-rust~repo]   | {{#categories development-tools::debugging=Debugging}} |
