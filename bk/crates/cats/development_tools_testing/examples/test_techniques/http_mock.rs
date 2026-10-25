#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Tests an HTTP client against a local mock server with `httpmock`.

/// Fetches the plain-text greeting for `name` from `base_url`.
fn fetch_greeting(base_url: &str, name: &str) -> anyhow::Result<String> {
    let mut response = ureq::get(&format!("{base_url}/greet/{name}"))
        .config()
        .http_status_as_error(false)
        .build()
        .call()?;
    let status = response.status().as_u16();
    let body = response.body_mut().read_to_string()?;
    anyhow::ensure!(status == 200, "unexpected status {status}: {body}");
    Ok(body)
}

/// Calls the greeting service at the local development address.
fn main() -> anyhow::Result<()> {
    println!("{}", fetch_greeting("http://localhost:8080", "Ferris")?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use httpmock::prelude::*;

    use super::*;

    /// Returns the mocked greeting and verifies the request was received.
    #[test]
    fn returns_the_greeting() -> anyhow::Result<()> {
        // Starts a server on a free local port, shut down on drop.
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET).path("/greet/Ferris");
            then.status(200).body("Hello, Ferris!");
        });

        let greeting = fetch_greeting(&server.base_url(), "Ferris")?;

        assert_eq!(greeting, "Hello, Ferris!");
        mock.assert(); // The request was received exactly once.
        Ok(())
    }

    /// Converts a non-success HTTP status into an error.
    #[test]
    fn reports_a_server_error() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET).path("/greet/Ferris");
            then.status(503).body("try later");
        });

        let error = fetch_greeting(&server.base_url(), "Ferris").unwrap_err();

        assert!(error.to_string().contains("503"));
        mock.assert_calls(1);
    }
}
// ANCHOR_END: example

/// Runs the example.
#[allow(clippy::main_recursion)]
pub fn run() -> anyhow::Result<()> {
    main()
}
