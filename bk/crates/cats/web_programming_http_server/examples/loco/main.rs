#![allow(dead_code)]
#![allow(unused_variables)]
// ANCHOR: example
//! This example demonstrates a basic HTTP server using the Loco framework.
//! It defines a simple application with a controller and routes.
//!
//! Loco is a web framework for Rust inspired by Ruby on Rails.

#[cfg(feature = "loco-support")]
use std::path::Path;

#[cfg(feature = "loco-support")]
use loco_rs::app::Hooks;
#[cfg(feature = "loco-support")]
use loco_rs::bgworker::Queue;
#[cfg(feature = "loco-support")]
use loco_rs::boot::BootResult;
#[cfg(feature = "loco-support")]
use loco_rs::boot::StartMode;
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

    async fn boot(
        _mode: StartMode,
        _environment: &Environment,
        _config: loco_rs::config::Config,
    ) -> Result<BootResult> {
        // In a real app, this would initialize the application context.
        todo!()
    }

    async fn connect_workers(_ctx: &AppContext, _queue: &Queue) -> Result<()> {
        Ok(())
    }

    fn register_tasks(_tasks: &mut Tasks) {}

    async fn truncate(_ctx: &AppContext) -> Result<()> {
        Ok(())
    }

    async fn seed(_ctx: &AppContext, _path: &Path) -> Result<()> {
        Ok(())
    }
}

/// A simple controller for the home page.
#[cfg(feature = "loco-support")]
mod home {
    use super::*;

    /// A basic handler that returns a "Hello, World!" message.
    async fn hello(State(_ctx): State<AppContext>) -> Result<Response> {
        format::text("Hello, Loco!")
    }

    /// Defines the routes for this controller.
    pub fn routes() -> Routes {
        Routes::new().prefix("home").add("/hello", get(hello))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // In a real application, you would use:
    // use loco_rs::boot::{create_app, StartMode};
    // let boot = create_app::<App>(StartMode::Server,
    // &Environment::Development).await?; boot.start().await?;

    println!("Loco application initialized (simulated).");
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "loco-support")]
    #[tokio::test]
    async fn test_app_init() {
        // Verification that the main logic is accessible
        assert_eq!(App::app_name(), env!("CARGO_CRATE_NAME"));
    }
}
