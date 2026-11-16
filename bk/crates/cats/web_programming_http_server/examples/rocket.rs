// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A small Rocket server with one route and an in-process test.
//!
//! Run it with `cargo run --example rocket --features rocket-support`.

#[cfg(feature = "rocket-support")]
#[rocket::get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

/// Assembles the rocket from its component values so callers receive a ready-to-use result.
#[cfg(feature = "rocket-support")]
fn build_rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build().mount("/", rocket::routes![hello])
}

/// Starts the Rocket server for the HTTP routing example.
///
/// # Errors
///
/// Returns an error if the Rocket server cannot launch.
#[cfg(feature = "rocket-support")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    example_async().await
}

/// Launches the sample Rocket application.
///
/// # Errors
///
/// Returns an error if the server cannot launch.
#[cfg(feature = "rocket-support")]
async fn example_async() -> anyhow::Result<()> {
    build_rocket().launch().await?;
    Ok(())
}

/// Runs the example.
#[cfg(not(feature = "rocket-support"))]
fn main() {
    example_without_feature();
}

/// Prints the feature required to run the Rocket application.
#[cfg(not(feature = "rocket-support"))]
fn example_without_feature() {
    eprintln!("Enable the `rocket-support` feature to run this example.");
}
// ANCHOR_END: example

/// Runs the example through its program entry point.
///
/// # Errors
///
/// Returns an error if the example operation fails.
pub fn run() -> anyhow::Result<()> {
    #[cfg(feature = "rocket-support")]
    {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(example_async())
    }

    #[cfg(not(feature = "rocket-support"))]
    {
        example_without_feature();
        Ok(())
    }
}

/// Tests the route using Rocket's local client without opening a network port.
#[cfg(all(test, feature = "rocket-support"))]
mod tests {
    use rocket::http::Status;
    use rocket::local::asynchronous::Client;

    use super::*;

    /// Returns the greeting and an OK status from the root route.
    #[rocket::async_test]
    async fn root_route_returns_greeting() -> anyhow::Result<()> {
        let client = Client::tracked(build_rocket()).await?;
        let response = client.get("/").dispatch().await;

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().await.as_deref(),
            Some("Hello, world!")
        );
        Ok(())
    }
}
// Implement Rocket Hello World and route mounting

/// Tests the `rocket` example and its documented behavior.
#[cfg(test)]
mod entry_point_tests {

    /// Verifies the no-feature entry point explains how to enable Rocket.
    #[cfg(not(feature = "rocket-support"))]
    #[test]
    fn test_main_entry_point() {
        super::main();
    }
}
