// ANCHOR: example
// COMING SOON
// ANCHOR_END: example
//! # Crux Core Example
//!
//! This example demonstrates the core concepts of a Crux application.
//! It's a simple counter app that can increment and decrement a value.
//!
//! The following only provides an example of the Core of a Crux app.
//! Follow the steps in https://redbadger.github.io/crux/
//! to install the toolchains and structure a complete project.
//!
//! Add the following to your `Cargo.toml`, as needed:
//! `crux_core` - the main Crux crate
//! `crux_http` - HTTP client capability
//! `crux_kv` - Key-value store capability
//! `crux_time` - Time capability

use crux_core::App;
use crux_core::Command;
use crux_core::render::Render;
use crux_core::render::render;
use serde::Deserialize;
use serde::Serialize;

/// Define a `Model` struct to hold the state of the application.
#[derive(Default)]
pub struct Model {
    count: i32,
}

/// Define a `ViewModel` struct to represent the state of the UI.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ViewModel {
    pub count: String,
}

/// Define an `Event` enum for the different messages
/// the application can handle.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Event {
    Increment,
    Decrement,
}

/// Define the `Capabilities` struct to hold the capabilities of the app.
///
/// Capabilities are the means by which the core can request side-effects
/// from the shell. They are defined as a struct containing capability
/// instances.
#[cfg_attr(feature = "typegen", derive(crux_core::macros::Export))]
#[derive(crux_core::macros::Effect)]
#[allow(unused)]
pub struct Capabilities {
    render: Render<Event>,
}

/// Define the `Counter` struct as the main app.
#[derive(Default)]
pub struct Counter;

/// Implement the `App` trait for the `Counter` struct to define how the state
/// should be updated in response to messages.
/// The `App` is the the main module of the core containing the application
/// logic, especially model changes and side-effects triggered by events.
/// Apps can be composed from modules, each resembling a smaller, simpler app.
impl App for Counter {
    // A user-friendly API used to request effects and provide events
    // that should be dispatched when the effect is completed. For example, a
    // HTTP client is a capability.
    type Capabilities = Capabilities;
    // A side-effect the core can request from the shell.
    // This is typically a form of I/O or similar interaction with the host
    // platform. Updating the UI is considered an effect.
    type Effect = Effect;
    // Events are main input for the core, typically triggered by user
    // interaction in the UI
    type Event = Event;
    // Model is a data structure (typically tree-like)
    // holding the entire application state
    type Model = Model;
    // Data structure describing the current state of the user interface
    type ViewModel = ViewModel;

    // The job of the update function is to process an Event, update the
    // model accordingly, and potentially request some side-effects using
    // capabilities.
    fn update(
        &self,
        event: Self::Event,
        model: &mut Self::Model,
        _caps: &Self::Capabilities,
    ) -> Command<Effect, Event> {
        match event {
            Event::Increment => model.count += 1,
            Event::Decrement => model.count -= 1,
        };
        render()
    }

    // The view function transforms the model into a view model, which is a
    // representation of the UI.
    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        ViewModel {
            count: format!("Count is: {}", model.count),
        }
    }
}

// In the real world, the inner "Core" would be compiled and linked to the
// outer "Shell" on each platform as a library:
// - On iOS, as a native static library.
// - On Android, as a dynamic library using Java Native Access.
// - In a browser, as a WebAssembly module.
fn main() {
    let _app = Counter;
    // FIXME
    // Simulate user interactions.
    // app.update(Event::Increment, &mut Model::default(), &Capabilities {
    // render: Render::default() }); app.update(Event::Increment, &mut
    // Model::default(), &Capabilities { render: Render::default() });
    // app.update(Event::Decrement, &mut Model::default(), &Capabilities {
    // render: Render::default() });
}

#[test]
fn test() {
    main();
}
// [finish](https://github.com/john-cd/rust_howto/issues/880)
// https://github.com/redbadger/crux/blob/master/examples/counter/shared/src/app.rs
