// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A small Rocket server with one route and an in-process test.
//!
//! Run it with `cargo run --example rocket --features rocket-support`.

#[cfg(feature = "rocket-support")]
#[rocket::get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[cfg(feature = "rocket-support")]
fn build_rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build().mount("/", rocket::routes![hello])
}

#[cfg(feature = "rocket-support")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    build_rocket().launch().await?;
    Ok(())
}

#[cfg(not(feature = "rocket-support"))]
fn main() {
    eprintln!("Enable the `rocket-support` feature to run this example.");
}
// ANCHOR_END: example

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
