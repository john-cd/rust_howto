
// TODO Reviewed ??-??-2026.

// Preserved for reference; its Cargo example is disabled while tao/wry require
// the affected GTK 3/glib 0.18 dependency chain.

// ANCHOR: example
//! Shows a web page in a native window with `tao` and `wry`.
//!
//! `tao` creates the window and runs the event loop, `wry` puts the
//! system web view inside.
//!
//! Only local `file://` pages are accepted from the command line.

use std::error::Error;
use std::fmt;

use tao::event::Event;
use tao::event::WindowEvent;
use tao::event_loop::ControlFlow;
use tao::event_loop::EventLoop;
use tao::window::WindowBuilder;
use url::Url;
use wry::WebViewBuilder;

/// A page with a button that sends a message to Rust.
const HTML: &str = r#"<!doctype html>
<html>
  <body>
    <h1>Hello from wry</h1>
    <button onclick="window.ipc.postMessage('clicked')">Say hello</button>
  </body>
</html>"#;

/// The reasons for rejecting a URI.
#[derive(Debug, PartialEq, Eq)]
enum UriError {
    /// The text is not a valid URI.
    Invalid,
    /// The URI does not point to a file on this computer.
    NotLocal,
}

impl fmt::Display for UriError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid => write!(f, "not a valid URI"),
            Self::NotLocal => write!(f, "only local file:// URIs are allowed"),
        }
    }
}

impl Error for UriError {}

/// Accepts `file:///...` and `file://localhost/...` URIs only.
fn validate_local_uri(uri: &str) -> Result<Url, UriError> {
    let url = Url::parse(uri).map_err(|_| UriError::Invalid)?;
    if url.scheme() != "file" {
        return Err(UriError::NotLocal);
    }
    match url.host_str() {
        None | Some("" | "localhost") => Ok(url),
        Some(_) => Err(UriError::NotLocal),
    }
}

/// Validates `uri`, then hands it to `open`.
///
/// Taking the opener as a closure allows testing the validation without a
/// window.
fn open_validated_uri(
    uri: &str,
    open: impl FnOnce(&Url) -> Result<(), Box<dyn Error>>,
) -> Result<(), Box<dyn Error>> {
    let url = validate_local_uri(uri)?;
    open(&url)
}

/// Opens a window that shows `url`, or `HTML` when there is no URL.
///
/// The function returns only when the process exits.
fn show_window(url: Option<&Url>) -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("wry").build(&event_loop)?;

    let builder = match url {
        Some(url) => WebViewBuilder::new().with_url(url.as_str()),
        None => WebViewBuilder::new().with_html(HTML),
    }
    // Receives `window.ipc.postMessage(...)` calls from JavaScript.
    .with_ipc_handler(|request| {
        println!("From JavaScript: {}", request.body());
    });

    // On Linux, `wry` needs the GTK container of the `tao` window.
    #[cfg(target_os = "linux")]
    let _webview = {
        use tao::platform::unix::WindowExtUnix as _;
        use wry::WebViewBuilderExtUnix as _;
        builder.build_gtk(window.gtk_window())?
    };
    #[cfg(not(target_os = "linux"))]
    let _webview = builder.build(&window)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    })
}

/// Shows the file given on the command line, or an inline page.
///
/// Run with `cargo run --example wry_window --features wry -- file:///tmp/report.html`.
fn main() -> Result<(), Box<dyn Error>> {
    match std::env::args().nth(1) {
        Some(uri) => open_validated_uri(&uri, |url| show_window(Some(url))),
        None => show_window(None),
    }
}
// ANCHOR_END: example

pub fn run() -> Result<(), Box<dyn Error>> {
    // Call through a binding: calling `main` directly trips `clippy::main_recursion`.
    let entry = main;
    entry()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_local_files() {
        assert!(validate_local_uri("file:///tmp/report.html").is_ok());
        assert!(validate_local_uri("file://localhost/tmp/report.html").is_ok());
    }

    #[test]
    fn rejects_remote_content() {
        for uri in [
            "https://example.com/",
            "http://localhost/",
            "javascript:alert(1)",
            "file://example.com/share/report.html",
        ] {
            assert_eq!(
                validate_local_uri(uri),
                Err(UriError::NotLocal),
                "{uri}"
            );
        }
        assert_eq!(validate_local_uri("not a uri"), Err(UriError::Invalid));
    }

    #[test]
    fn opener_is_not_called_for_rejected_uri() {
        let mut called = false;
        let result = open_validated_uri("https://example.com/", |_| {
            called = true;
            Ok(())
        });
        assert!(result.is_err());
        assert!(!called);
    }

    #[test]
    fn opener_receives_the_validated_url() -> Result<(), Box<dyn Error>> {
        let mut opened = None;
        open_validated_uri("file:///tmp/a.html", |url| {
            opened = Some(url.clone());
            Ok(())
        })?;
        assert_eq!(
            opened.map(|url| url.path().to_owned()).as_deref(),
            Some("/tmp/a.html")
        );
        Ok(())
    }

    #[test]
    #[ignore = "requires a display"]
    fn shows_a_window() -> Result<(), Box<dyn Error>> {
        main()
    }
}
