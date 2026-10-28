// TODO Reviewed ??-??-2026

//! # Crux Core Example
//!
//! This example keeps a counter's state transitions in a platform-independent
//! core.

// ANCHOR: example
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
    type Effect = Effect;
    type Event = Event;
    type Model = Model;
    type ViewModel = ViewModel;

    fn update(
        &self,
        event: Self::Event,
        model: &mut Self::Model,
    ) -> Command<Effect, Event> {
        match event {
            Event::Increment => model.count += 1,
            Event::Decrement => model.count -= 1,
            Event::Reset => model.count = 0,
        }
        render()
    }

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

    /// Verifies two increments update the view and request a render each time.
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
