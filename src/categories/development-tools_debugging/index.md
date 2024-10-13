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

petabyte scale Elasticsearch/Splunk/Datadog alternative for logs, metrics, traces, RUM, Error tracking, Session replay: [openobserve](https://github.com/openobserve/openobserve)

## Tracing

Support for logging [`tracing`](https://crates.io/crates/tracing) events natively to [journald](https://www.freedesktop.org/software/systemd/man/systemd-journald.service.html), preserving structured information: [tracing_journald](https://docs.rs/tracing-journald/latest/tracing_journald/)

</div>
