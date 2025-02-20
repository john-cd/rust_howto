| Recipe | Crates | Categories |
|--------|--------|------------|
| [Initialize the logger][ex-development-tools_debugging-initialization] | [![tracing][c-tracing-badge]][c-tracing] | [![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging] |
| [Enable basic tracing][ex-development-tools_debugging-basic-tracing] | [![tracing][c-tracing-badge]][c-tracing] | [![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging] |
| [Combine layers][ex-development-tools_debugging-combine-layers] | [![tracing][c-tracing-badge]][c-tracing] | [![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging] |
| [Configure a custom event formatter][ex-development-tools_debugging-custom-event-formatter] | [![tracing][c-tracing-badge]][c-tracing] | [![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging] |
| [Events][ex-development-tools_debugging-events] | [![tracing][c-tracing-badge]][c-tracing] | [![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging] |
| [Spans][ex-development-tools_debugging-spans] | [![tracing][c-tracing-badge]][c-tracing] | [![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging] |
| [Add tracing spans to functions][ex-development-tools_debugging-add-tracing-spans-to-fn] | [![tracing][c-tracing-badge]][c-tracing] | [![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging] |
| [See also][ex-development-tools_debugging-related-crates] | [![tracing][c-tracing-badge]][c-tracing] | [![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging] |

<div class="hidden">
[tracing.incl: fix (P1)](https://github.com/john-cd/rust_howto/issues/321)

Tracing Framework: tracing (provides the core tracing functionality, spans, events, etc.)
Span Management: (Handled by tracing through its span API)
Event Logging: (Also handled by tracing using macros like event! and debug!, info!, warn!, error!)
Context Propagation: (Built into tracing's span system)
Instrumentation: (Often done with macros provided by tracing or other instrumentation libraries)
Output and Formatting: tracing-subscriber (formats and outputs traces to various destinations)
Filtering: tracing-subscriber (allows filtering of traces based on level, target, etc.)
Asynchronous Tracing: (Supported by tracing through its asynchronous span management)
Integration with other tools: (Often tracing is used with other tools like Jaeger or Zipkin for distributed tracing.  There might be specific crates for those integrations, but tracing is the core.)
</div>
