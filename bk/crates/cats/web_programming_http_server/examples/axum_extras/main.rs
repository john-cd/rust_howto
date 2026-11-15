//! Small axum add-ons: typed headers, an HTTP to HTTPS redirect, TLS, flash
//!
//!
//! messages, cookies and `#[debug_handler]`.
//! Some terms used here: a redirect from HTTP to HTTPS sends visitors of the
//! unencrypted address to the encrypted one; TLS is the encryption behind
//! HTTPS; a cookie is a small value the server asks the browser to send back on
//! later requests; a flash message is a one-time notice, such as "saved",
//! stored in a cookie and shown on the next page.
//!
//! TODO Reviewed ??-??-2026.

/// Serve an `axum` router over HTTPS with `axum-server` and `rustls`.
mod axum_server_tls;
/// Set and read cookies with `tower-cookies`.
mod cookies;
/// Get readable compiler errors for handlers with `#[debug_handler]`.
mod debug_handler;
/// One-time "flash" messages with `axum-messages` and `tower-sessions`.
mod flash_messages;
/// Redirect HTTP requests to HTTPS.
mod https_redirect;
/// Extract strongly typed headers with [`axum_extra::TypedHeader`].
mod typed_header;
/// Runs the example.
fn main() {
    // Build the routers; serving them is shown in the tests.
    let routers = [
        cookies::run(),
        debug_handler::run(),
        flash_messages::run(),
        typed_header::run(),
        axum_server_tls::run(),
        https_redirect::run(),
    ];
    println!("Built {} routers.", routers.len());
    println!("Run `cargo test --example axum_extras` to call them.");
}

/// TLS, flash
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_runs() {
        assert!(std::panic::catch_unwind(main).is_ok());
    }
}
