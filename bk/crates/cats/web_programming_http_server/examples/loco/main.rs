// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates a basic HTTP server using the Loco framework.
//! It defines a simple application with a controller and routes.
//!
//! Loco is a web framework for Rust inspired by Ruby on Rails.

#[cfg(feature = "loco-support")]
use loco_rs::app::Hooks;
#[cfg(feature = "loco-support")]
use loco_rs::bgworker::Queue;
#[cfg(feature = "loco-support")]
use loco_rs::boot::BootResult;
#[cfg(feature = "loco-support")]
use loco_rs::boot::StartMode;
#[cfg(feature = "loco-support")]
use loco_rs::boot::create_app;
#[cfg(feature = "loco-support")]
use loco_rs::controller::AppRoutes;
#[cfg(feature = "loco-support")]
use loco_rs::environment::Environment;
#[cfg(feature = "loco-support")]
use loco_rs::prelude::*;
#[cfg(feature = "loco-support")]
use loco_rs::task::Tasks;

/// The application's main structure.
#[cfg(feature = "loco-support")]
pub struct App;

#[cfg(feature = "loco-support")]
#[async_trait]
impl Hooks for App {
    /// Sets the application name.
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    /// Registers the application routes.
    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes().add_route(home::routes())
    }

    /// Creates the application without enabling database support.
    ///
    /// # Arguments
    /// * `mode` - Startup mode selected for the application.
    /// * `environment` - Runtime environment and configuration profile.
    async fn boot(
        mode: StartMode,
        environment: &Environment,
        config: loco_rs::config::Config,
    ) -> Result<BootResult> {
        create_app::<Self>(mode, environment, config).await
    }

    /// Leaves background worker registration empty for this HTTP-only example.
    async fn connect_workers(_ctx: &AppContext, _queue: &Queue) -> Result<()> {
        Ok(())
    }

    /// Registers no scheduled or one-off tasks for this example.
    fn register_tasks(_tasks: &mut Tasks) {}
}

/// A simple controller for the home page.
#[cfg(feature = "loco-support")]
mod home {
    use super::*;

    /// A basic handler that returns a "Hello, World!" message.
    /// The framework supplies the application state, which this handler does
    /// not use.
    ///
    /// # Arguments
    /// * `_ctx` - Application state containing the repositories and services needed by this
    ///   workflow.
    ///
    /// # Errors
    /// Returns an error if a fallible operation cannot complete.
    #[allow(
        clippy::result_large_err,
        reason = "Loco handlers use the framework's Result<Response> error type."
    )]
    async fn hello(State(_ctx): State<AppContext>) -> Result<Response> {
        format::text("Hello, Loco!")
    }
    /// Defines the routes for this controller.
    pub fn routes() -> Routes {
        Routes::new().prefix("home").add("/hello", get(hello))
    }
}
/// Prints the command needed to run this example as a Loco application.
fn main() -> anyhow::Result<()> {
    example()
}

/// Prints how to start the Loco application for the selected feature state.
///
/// # Errors
/// Returns an error if a step of the example fails.
fn example() -> anyhow::Result<()> {
    #[cfg(feature = "loco-support")]
    println!("Run with `cargo loco start` from a Loco application directory.");
    #[cfg(not(feature = "loco-support"))]
    println!("Enable `loco-support` to compile the Loco application hooks.");
    Ok(())
}
// ANCHOR_END: example

/// Verifies the example identifies its Loco application by crate name.
#[cfg(test)]
/// Tests the `main` example and its documented behavior.
#[cfg(feature = "loco-support")]
mod tests {
    use super::*;

    /// Verifies the example identifies its Loco application by crate name.
    #[tokio::test]
    async fn app_name_matches_crate_name() {
        assert_eq!(App::app_name(), env!("CARGO_CRATE_NAME"));
    }
}
