// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Propagate a W3C trace context from an HTTP client to a handler, so both
//! sides report spans of the same trace.
//!
//! The client injects a `traceparent` header from its current span; the
//! handler extracts it and parents its own span on it. Spans are collected by
//! an in-memory exporter, so no collector or network is needed.

#[cfg(feature = "opentelemetry")]
use http::HeaderMap;
#[cfg(feature = "opentelemetry")]
use opentelemetry::KeyValue;
#[cfg(feature = "opentelemetry")]
use opentelemetry::propagation::TextMapPropagator;
#[cfg(feature = "opentelemetry")]
use opentelemetry::trace::TracerProvider as _;
#[cfg(feature = "opentelemetry")]
use opentelemetry_http::HeaderExtractor;
#[cfg(feature = "opentelemetry")]
use opentelemetry_http::HeaderInjector;
#[cfg(feature = "opentelemetry")]
use opentelemetry_sdk::propagation::TraceContextPropagator;
#[cfg(feature = "opentelemetry")]
use opentelemetry_sdk::trace::InMemorySpanExporter;
#[cfg(feature = "opentelemetry")]
use opentelemetry_sdk::trace::SdkTracerProvider;
#[cfg(feature = "opentelemetry")]
use tracing_opentelemetry::OpenTelemetrySpanExt;
#[cfg(feature = "opentelemetry")]
use tracing_subscriber::layer::SubscriberExt;

/// Starts a client span and copies its trace context into `headers`.
///
/// Only `traceparent` and `tracestate` are written: no baggage propagator is
/// used, so application data never travels in headers by accident.
#[cfg(feature = "opentelemetry")]
fn client_request(headers: &mut HeaderMap) {
    let span = tracing::info_span!("client_request");
    let propagator = TraceContextPropagator::new();
    propagator.inject_context(&span.context(), &mut HeaderInjector(headers));
}

/// Handles a request: extracts the caller's context and parents a new span.
///
/// Headers come from an untrusted peer. A missing or malformed `traceparent`
/// is ignored, and the handler then starts a new, independent trace.
#[cfg(feature = "opentelemetry")]
fn handle_request(headers: &HeaderMap, route: &str) {
    let propagator = TraceContextPropagator::new();
    let parent = propagator.extract(&HeaderExtractor(headers));
    let span = tracing::info_span!("handle_request", http.route = route);
    // The extracted context is attached to this span only, never to the
    // thread, so concurrent requests cannot leak context into each other.
    if let Err(error) = span.set_parent(parent) {
        tracing::warn!(%error, "ignoring the incoming trace context");
    }
    let _entered = span.enter();
    tracing::info!("handling request");
}

/// Builds a tracer provider that keeps finished spans in memory.
#[cfg(feature = "opentelemetry")]
fn in_memory_tracing() -> (SdkTracerProvider, InMemorySpanExporter) {
    let exporter = InMemorySpanExporter::default();
    let provider = SdkTracerProvider::builder()
        .with_resource(
            opentelemetry_sdk::Resource::builder()
                .with_service_name("rust_howto")
                .with_attribute(KeyValue::new("deployment.environment", "doc"))
                .build(),
        )
        .with_simple_exporter(exporter.clone())
        .build();
    (provider, exporter)
}

/// Sends one request through the client and the handler and prints the spans.
#[cfg(feature = "opentelemetry")]
fn main() -> anyhow::Result<()> {
    let (provider, exporter) = in_memory_tracing();
    let subscriber = tracing_subscriber::registry().with(
        tracing_opentelemetry::layer()
            .with_tracer(provider.tracer("rust_howto")),
    );

    tracing::subscriber::with_default(subscriber, || {
        let mut headers = HeaderMap::new();
        {
            let _client = tracing::info_span!("client").entered();
            client_request(&mut headers);
        }
        println!("traceparent: {:?}", headers.get("traceparent"));
        handle_request(&headers, "/health");
    });

    provider.force_flush()?;
    for span in exporter.get_finished_spans()? {
        println!(
            "{}: trace {} span {} parent {}",
            span.name,
            span.span_context.trace_id(),
            span.span_context.span_id(),
            span.parent_span_id
        );
    }
    provider.shutdown()?;
    Ok(())
}

/// Prints a message when the `opentelemetry` feature is not enabled.
#[cfg(not(feature = "opentelemetry"))]
fn main() -> anyhow::Result<()> {
    println!("Enable the `opentelemetry` feature to run this example.");
    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod entry_tests {
    use super::*;

    /// The entry point runs without a collector.
    #[test]
    fn main_runs_offline() {
        main().expect("the in-memory exporter needs no network");
    }
}

#[cfg(all(test, feature = "opentelemetry"))]
mod tests {
    use http::HeaderValue;
    use opentelemetry::trace::SpanId;
    use opentelemetry_sdk::trace::SpanData;

    use super::*;

    /// Runs `body` with a subscriber that exports to memory, then returns the
    /// finished spans.
    fn collect_spans(body: impl FnOnce()) -> Vec<SpanData> {
        let (provider, exporter) = in_memory_tracing();
        let subscriber = tracing_subscriber::registry().with(
            tracing_opentelemetry::layer()
                .with_tracer(provider.tracer("trace_context_test")),
        );
        tracing::subscriber::with_default(subscriber, body);
        provider.force_flush().expect("flush should succeed");
        let spans = exporter
            .get_finished_spans()
            .expect("the in-memory exporter should be readable");
        provider.shutdown().expect("shutdown should succeed");
        spans
    }

    /// Finds a finished span by name.
    fn span_named<'a>(spans: &'a [SpanData], name: &str) -> &'a SpanData {
        spans
            .iter()
            .find(|span| span.name == name)
            .unwrap_or_else(|| panic!("span `{name}` should be exported"))
    }

    /// The handler span joins the client's trace and is its child.
    #[test]
    fn handler_continues_the_client_trace() {
        let spans = collect_spans(|| {
            let mut headers = HeaderMap::new();
            {
                let _client = tracing::info_span!("client").entered();
                client_request(&mut headers);
            }
            handle_request(&headers, "/health");
        });
        let client = span_named(&spans, "client_request");
        let handler = span_named(&spans, "handle_request");
        assert_eq!(
            client.span_context.trace_id(),
            handler.span_context.trace_id()
        );
        assert_eq!(handler.parent_span_id, client.span_context.span_id());
        assert!(
            handler
                .attributes
                .iter()
                .any(|kv| kv.key.as_str() == "http.route"
                    && kv.value.as_str() == "/health"),
            "the route attribute should be exported"
        );
    }

    /// The injected header follows the W3C `traceparent` format.
    #[test]
    fn traceparent_header_is_well_formed() {
        let mut headers = HeaderMap::new();
        collect_spans(|| client_request(&mut headers));
        let value = headers
            .get("traceparent")
            .expect("a sampled span should inject `traceparent`")
            .to_str()
            .expect("the header is ASCII");
        let parts: Vec<&str> = value.split('-').collect();
        assert_eq!(parts.len(), 4);
        assert_eq!(parts[0], "00");
        assert_eq!(parts[1].len(), 32);
        assert_eq!(parts[2].len(), 16);
    }

    /// Without a header, the handler starts a new root trace.
    #[test]
    fn missing_header_starts_a_new_trace() {
        let spans =
            collect_spans(|| handle_request(&HeaderMap::new(), "/health"));
        let handler = span_named(&spans, "handle_request");
        assert_eq!(handler.parent_span_id, SpanId::INVALID);
    }

    /// A malformed header is ignored instead of being trusted or panicking.
    #[test]
    fn malformed_header_is_ignored() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "traceparent",
            HeaderValue::from_static("not-a-trace-context"),
        );
        headers.insert("baggage", HeaderValue::from_static("user=secret"));
        let spans = collect_spans(|| handle_request(&headers, "/health"));
        let handler = span_named(&spans, "handle_request");
        assert_eq!(handler.parent_span_id, SpanId::INVALID);
        assert!(
            handler
                .attributes
                .iter()
                .all(|kv| !kv.value.as_str().contains("secret")),
            "baggage from untrusted peers must not be copied into spans"
        );
    }

    /// Two requests with different callers produce two independent traces.
    #[test]
    fn concurrent_requests_stay_independent() {
        let spans = collect_spans(|| {
            let mut first = HeaderMap::new();
            let mut second = HeaderMap::new();
            {
                let _a = tracing::info_span!("client_a").entered();
                client_request(&mut first);
            }
            {
                let _b = tracing::info_span!("client_b").entered();
                client_request(&mut second);
            }
            // Interleave: both handler spans are open at the same time.
            let span_a =
                tracing::info_span!("handle_request", http.route = "a");
            let span_b =
                tracing::info_span!("handle_request", http.route = "b");
            let parent_a =
                TraceContextPropagator::new().extract(&HeaderExtractor(&first));
            let parent_b = TraceContextPropagator::new()
                .extract(&HeaderExtractor(&second));
            span_a
                .set_parent(parent_a)
                .expect("the span is registered with the subscriber");
            span_b
                .set_parent(parent_b)
                .expect("the span is registered with the subscriber");
            drop(span_a);
            drop(span_b);
        });
        let handlers: Vec<_> = spans
            .iter()
            .filter(|span| span.name == "handle_request")
            .collect();
        assert_eq!(handlers.len(), 2);
        assert_ne!(
            handlers[0].span_context.trace_id(),
            handlers[1].span_context.trace_id()
        );
    }
}
