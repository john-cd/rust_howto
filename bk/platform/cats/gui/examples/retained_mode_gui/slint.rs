#![allow(clippy::same_name_method)]
// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A counter window defined with Slint.
//!
//! Slint describes the interface in its own markup language (`.slint`), which
//! is compiled with the program. Properties declared there are bound to Rust
//! code through callbacks and setters. It targets desktops and embedded
//! devices.

slint::slint! {
    import { Button, VerticalBox } from "std-widgets.slint";

    export component App inherits Window {
        title: "Slint Counter";
        in-out property <int> counter: 0;
        out property <string> counter_label: "Count: " + root.counter;
        callback increment_counter();

        VerticalBox {
            Button {
                text: "Increment";
                clicked => { root.increment_counter(); }
            }
            Text {
                text: root.counter_label;
            }
        }
    }
}

/// Shows the counter window and runs Slint's event loop.
fn main() -> anyhow::Result<()> {
    let app = App::new()?;
    connect_counter_increment(&app);
    app.run()?;
    Ok(())
}

/// Connects the counter callback to its Rust-side state update.
fn connect_counter_increment(app: &App) {
    let weak_app = app.as_weak();
    app.on_increment_counter(move || {
        if let Some(app) = weak_app.upgrade() {
            app.set_counter(app.get_counter() + 1);
        }
    });
}
// ANCHOR_END: example

/// Opens the counter window and processes GUI events.
/// A counter window defined with Slint.
///
/// # Errors
///
/// Returns an error if a step of the example fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

/// Verifies that the counter callback updates state without a display server.
#[cfg(test)]
mod tests {
    use super::*;

    /// Updates the counter and its label through the callback without a display server.
    #[test]
    fn increments_counter_without_a_display() -> anyhow::Result<()> {
        i_slint_backend_testing::init_no_event_loop();
        let app = App::new()?;
        connect_counter_increment(&app);

        assert_eq!(app.get_counter(), 0);
        assert_eq!(app.get_counter_label(), "Count: 0");
        app.invoke_increment_counter();
        assert_eq!(app.get_counter(), 1);
        assert_eq!(app.get_counter_label(), "Count: 1");

        Ok(())
    }

    /// Opens the application only when an interactive display is available.
    #[test]
    #[ignore = "requires an interactive Slint session"]
    fn test_main_entry_point() -> anyhow::Result<()> {
        main()
    }
}
