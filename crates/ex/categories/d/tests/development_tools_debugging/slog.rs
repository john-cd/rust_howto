// // ANCHOR: example
// use slog::Drain;
// use slog::Logger;
// use slog::o;
// use slog_async;
// use slog_term;
// // `slog` is a structured, composable logging framework.

// // slog = "2.7"
// // slog-async = "2.7"
// // slog-term = "2.8"
// // slog-scope = "4.4"

// fn main() {
//     // Create a terminal decorator
//     let decorator = slog_term::TermDecorator::new().build();
//     // Create a terminal drain
//     let drain = slog_term::CompactFormat::new(decorator).build().fuse();
//     // Create an async drain
//     let drain = slog_async::Async::new(drain).build().fuse();
//     // Create a root logger
//     let root_logger =
//         Logger::root(drain, o!("version" => env!("CARGO_PKG_VERSION")));

//     // Log some messages
//     slog::info!(root_logger, "Logging with slog"; "example" => "simple");
//     slog::warn!(root_logger, "A warning message");
//     slog::error!(root_logger, "An error occurred"; "code" => 500);

//     // Do some work with logging
//     perform_some_logging(root_logger.clone());
// }

// fn perform_some_logging(logger: Logger) {
//     slog::info!(logger, "Performing some work"; "task" => "example task");
//     // More work...
// }
// // ANCHOR_END: example

// #[test]
// fn test() {
//     main();
// }
// // [P1](https://github.com/john-cd/rust_howto/issues/735)
