// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Implements a counter application core that can run behind different platform shells.
//!
//! # Crux Core Example
//!
//! This example keeps a counter's state transitions in a platform-independent
//! core.

use crux_core::App;
use crux_core::Command;
use crux_core::render::render;
use serde::Deserialize;
use serde::Serialize;

/// The counter's internal state.
#[derive(Default)]
pub struct Model {
    /// The current count.
    count: isize,
}

/// The state that a platform shell displays.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ViewModel {
    /// The count formatted for display.
    pub count: String,
}

/// Events that the counter core can process.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Event {
    /// Increments the count by one.
    Increment,

    /// Decrements the count by one.
    Decrement,

    /// Resets the count to zero.
    Reset,
}

/// Effects that the shell must handle for the core.
#[crux_core::macros::effect(typegen)]
pub enum Effect {
    /// Requests that the shell render the updated view model.
    Render(crux_core::render::RenderOperation),
}

/// The Crux counter application.
#[derive(Default)]
pub struct Counter;

impl App for Counter {
    /// Alias for `Effect` used to represent effect.
    type Effect = Effect;
    /// Alias for `Event` used to represent event.
    type Event = Event;
    /// Alias for `Model` used to represent model.
    type Model = Model;
    /// Alias for `ViewModel` used to represent view model.
    type ViewModel = ViewModel;

    /// Handles the update behavior for this component.
    fn update(&self, event: Self::Event, model: &mut Self::Model) -> Command<Effect, Event> {
        match event {
            Event::Increment => model.count += 1,
            Event::Decrement => model.count -= 1,
            Event::Reset => model.count = 0,
        }
        render()
    }

    /// Handles the view behavior for this component.
    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        ViewModel {
            count: format!("Count is: {}", model.count),
        }
    }
}

/// Runs two increments through Crux's in-process core and prints its view.
fn main() {
    let core = crux_core::Core::<Counter>::new();
    core.process_event(Event::Increment);
    core.process_event(Event::Increment);
    println!("{}", core.view().count);
}
// ANCHOR_END: example

/// Exposes this example to the crate's dispatcher.
pub fn run() {
    main();
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use super::App;
    use super::Counter;
    use super::Effect;
    use super::Event;
    use super::Model;
    use super::main;

    /// Runs the crux example through its entry point.
    #[test]
    fn test_crux_example_entry_point() {
        main();
    }

    /// Updates the view and requests a render after an increment.
    #[test]
    fn increments_update_view_and_request_render() {
        let app = Counter;
        let mut model = Model::default();

        for _ in 0..2 {
            let mut command = app.update(Event::Increment, &mut model);
            assert!(matches!(command.expect_one_effect(), Effect::Render(_)));
            command.expect_done();
        }

        assert_eq!(app.view(&model).count, "Count is: 2");
    }
}
