//! Small axum add-ons: typed headers, an HTTP to HTTPS redirect, TLS, flash
//! messages, cookies and `#[debug_handler]`.
//!
//! TODO Reviewed ??-??-2026.
//!
//! Some terms used here: a redirect from HTTP to HTTPS sends visitors of the
//! unencrypted address to the encrypted one; TLS is the encryption behind
//! HTTPS; a cookie is a small value the server asks the browser to send back on
//! later requests; a flash message is a one-time notice, such as "saved",
//! stored in a cookie and shown on the next page.

mod axum_server_tls;
mod cookies;
mod debug_handler;
mod flash_messages;
mod https_redirect;
mod typed_header;

fn main() {
    // Build the routers; serving them is shown in the tests.
    let routers = [
        cookies::app(),
        debug_handler::app(),
        flash_messages::app(),
        typed_header::app(),
        axum_server_tls::app(),
        https_redirect::app(https_redirect::Ports {
            http: 8080,
            https: 8443,
        }),
    ];
    println!("Built {} routers.", routers.len());
    println!("Run `cargo test --example axum_extras` to call them.");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point.
    #[test]
    fn main_runs() {
        assert!(std::panic::catch_unwind(main).is_ok());
    }
}
