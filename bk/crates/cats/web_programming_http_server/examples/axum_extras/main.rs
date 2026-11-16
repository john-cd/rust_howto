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
    let mut command = clap::Command::new("examples");
    command = command.subcommand(clap::Command::new("cookies"));
    command = command.subcommand(clap::Command::new("debug-handler"));
    command = command.subcommand(clap::Command::new("flash-messages"));
    command = command.subcommand(clap::Command::new("typed-header"));
    command = command.subcommand(clap::Command::new("axum-server-tls"));
    command = command.subcommand(clap::Command::new("https-redirect"));
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        Some(("cookies", _)) => {
            let _ = cookies::run();
        }
        Some(("debug-handler", _)) => {
            let _ = debug_handler::run();
        }
        Some(("flash-messages", _)) => {
            let _ = flash_messages::run();
        }
        Some(("typed-header", _)) => {
            let _ = typed_header::run();
        }
        Some(("axum-server-tls", _)) => {
            let _ = axum_server_tls::run();
        }
        Some(("https-redirect", _)) => {
            let _ = https_redirect::run();
        }
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}
