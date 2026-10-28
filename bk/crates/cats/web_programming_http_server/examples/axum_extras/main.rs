//! Small axum add-ons: typed headers, an HTTP to HTTPS redirect, TLS, flash
//! messages, cookies and `#[debug_handler]`.
//!
//! TODO Reviewed ??-??-2026.

mod axum_server_tls;
mod cookies;
mod debug_handler;
mod flash_messages;
mod https_redirect;
mod typed_header;

fn main() {
    println!("Run `cargo test --example axum_extras`.");
}
