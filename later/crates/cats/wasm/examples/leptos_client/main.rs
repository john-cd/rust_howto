//! Client-side Leptos building blocks: routing, HTTP calls, local storage and
//! a gRPC-Web client.
//!
//! TODO Reviewed ??-??-2026.

mod gloo_fetch;
mod gloo_storage;
mod grpc_web_client;
mod icons;
mod leptos_router;

fn main() {
    println!("Run `cargo test --example leptos_client`.");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point (output-only: the modules
    /// test their own results).
    // output-only: the entry point only prints a hint; the sibling modules hold
    // the tests.
    #[test]
    fn test_main_entry_point() {
        main();
    }
}
