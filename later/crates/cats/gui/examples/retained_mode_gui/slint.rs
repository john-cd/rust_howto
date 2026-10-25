#![allow(dead_code)]
// Slint's macro emits accessibility methods whose names match framework trait
// methods.
#![allow(clippy::same_name_method)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A counter window defined with Slint.

slint::slint! {
    import { Button, VerticalBox } from "std-widgets.slint";

    export component App inherits Window {
        in-out property <int> counter: 0;
        callback increment_counter();

        VerticalBox {
            Button {
                text: "Increment";
                clicked => { root.increment_counter(); }
            }
            Text {
                text: "Count: {root.counter}";
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

/// Runs the example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the counter callback updates state without a display server.
    #[test]
    fn increments_counter_without_a_display() -> anyhow::Result<()> {
        i_slint_backend_testing::init_no_event_loop();
        let app = App::new()?;
        connect_counter_increment(&app);

        assert_eq!(app.get_counter(), 0);
        app.invoke_increment_counter();
        assert_eq!(app.get_counter(), 1);

        Ok(())
    }
}
