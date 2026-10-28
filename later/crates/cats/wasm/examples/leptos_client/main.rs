//! Client-side Leptos building blocks: routing, HTTP calls, local storage and
//! a gRPC-Web client.
//!
//! TODO Reviewed ??-??-2026.

mod gloo_fetch;
mod gloo_storage;
mod grpc_web_client;
mod leptos_router;

fn main() {
    println!("Run `cargo test --example leptos_client`.");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
