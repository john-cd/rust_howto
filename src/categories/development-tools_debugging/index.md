# Debugging, Logging

Help you figure out what is going on with your code such as logging{{hi:Logging}}, tracing{{hi:Tracing}}, or assertions{{hi:Assertions}}.

{{#include config_log.incl.md}}

{{#include log.incl.md}}

{{#include tracing.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO:

## Observability

petabyte scale Elasticsearch/Splunk/Datadog alternative for logs, metrics, traces, RUM, Error tracking, Session replay: [openobserve][c-openobserve-github]⮳

## Tracing

Support for logging [`tracing`][c-tracing-crates.io]⮳ events natively to [journald][journald]⮳, preserving structured information: [tracing_journald][c-tracing_journald]⮳

[c-openobserve-github]: https://github.com/openobserve/openobserve
[c-tracing_journald]: https://docs.rs/tracing-journald
[journald]: https://www.freedesktop.org/software/systemd/man/systemd-journald.service.html
[c-tracing-events-crates.io]: https://crates.io/crates/tracing
</div>
