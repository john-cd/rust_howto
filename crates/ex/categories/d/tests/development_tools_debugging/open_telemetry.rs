// // ANCHOR: example
// // ANCHOR_END: example

// use std::net::SocketAddr;

// use axum::Router;
// use axum::handler::get;
// use opentelemetry::global;
// use opentelemetry::sdk::propagation::TraceContextPropagator;
// use opentelemetry::sdk::trace as sdktrace;
// use opentelemetry::trace::Tracer;
// use tracing::info;
// use tracing_subscriber::layer::SubscriberExt;

// // This setup traces a simple HTTP request and sends the trace data to a
// // Jaeger instance running locally. To see the traces, you'll need to have
// // Jaeger running on localhost:6831.

// // The init_tracer function sets up OpenTelemetry with Jaeger as the tracing
// // backend.
// fn init_tracer() {
//     global::set_text_map_propagator(TraceContextPropagator::new());
//     let tracer = sdktrace::TracerProvider::builder()
//         .with_simple_exporter(
//             opentelemetry_jaeger::Exporter::builder()
//                 .with_agent_endpoint("localhost:6831".parse().unwrap())
//                 .with_service_name("rust-axum")
//                 .init(),
//         )
//         .build()
//         .tracer("rust-axum");
//     global::set_tracer_provider(tracer);
// }

// // The handler function creates a span, indicating a trace point in the
// // request handling process.
// async fn handler() -> &'static str {
//     let tracer = global::tracer("rust-axum");
//     let span = tracer.start("handling request");
//     span.end();

//     "Hello, world!"
// }

// #[tokio::main]
// async fn main() {
//     init_tracer();
//     tracing_subscriber::registry()
//         .with(tracing_subscriber::fmt::layer())
//         .with(tracing_opentelemetry::layer())
//         .init();

//     let app = Router::new().route("/", get(handler));

//     let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
//     info!("Listening on {}", addr);
//     axum::Server::bind(&addr)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }

// #[test]
// fn test() {
//     main();
// }
// // [P1](https://github.com/john-cd/rust_howto/issues/734)
