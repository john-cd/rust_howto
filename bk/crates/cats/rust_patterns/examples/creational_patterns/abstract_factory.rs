#![allow(dead_code)]

// TODO Reviewed ??-??-2026
// ANCHOR: example
//! The Abstract Factory pattern creates families of related objects without
//! tying-in to their concrete implementations.
//!
//! - The client code (in `main()`) doesn't need to know the specific classes of
//!   the objects it creates.
//! - The pattern ensures that the objects created belong to the same family,
//!   enforcing consistency.
//!
//! The following example simulates building a UI library that needs to support
//! different themes, like "Light" and "Dark." Each theme will have its own set
//! of widgets: buttons, text boxes, and so on. The Abstract Factory pattern
//! lets us define an interface for creating these families of widgets,
//!  and then we can have concrete factories for each theme.

// Operations common to all concrete objects.
/// A button, which can be rendered.
trait Button {
    /// Renders the button.
    fn render(&self);
}

/// A text box, which can display text.
trait TextBox {
    /// Displays `text` in the text box.
    fn display_text(&self, text: &str);
}

// Abstract factory interface (trait).
/// A factory of the GUI elements of a theme.
trait GUIFactory {
    /// Creates a button of the theme.
    fn create_button(&self) -> Box<dyn Button>;

    /// Creates a text box of the theme.
    fn create_text_box(&self) -> Box<dyn TextBox>;
}

/// The elements of the light theme.
mod light {
    use super::*;

    // Concrete object implementations for Light Theme.
    /// A button of the light theme.
    struct LightButton;

    impl Button for LightButton {
        fn render(&self) {
            println!("Rendering a light button.");
        }
    }

    /// A text box of the light theme.
    struct LightTextBox;

    impl TextBox for LightTextBox {
        fn display_text(&self, text: &str) {
            println!("Displaying '{text}' in a light text box.");
        }
    }

    // The concrete factory implementation produce concrete objects belonging to
    // a specific family (here, light theme).
    /// The factory of the light theme.
    pub struct LightGUIFactory;

    impl GUIFactory for LightGUIFactory {
        fn create_button(&self) -> Box<dyn Button> {
            Box::new(LightButton)
        }

        fn create_text_box(&self) -> Box<dyn TextBox> {
            Box::new(LightTextBox)
        }
    }
}

/// The elements of the dark theme.
mod dark {

    use super::*;

    // Concrete object implementations for Dark Theme.
    /// A button of the dark theme.
    struct DarkButton;

    impl Button for DarkButton {
        fn render(&self) {
            println!("Rendering a dark button.");
        }
    }

    /// A text box of the dark theme.
    struct DarkTextBox;

    impl TextBox for DarkTextBox {
        fn display_text(&self, text: &str) {
            println!("Displaying '{text}' in a dark text box.");
        }
    }

    // Concrete factory for Dark Theme.
    /// The factory of the dark theme.
    pub struct DarkGUIFactory;

    impl GUIFactory for DarkGUIFactory {
        fn create_button(&self) -> Box<dyn Button> {
            Box::new(DarkButton)
        }

        fn create_text_box(&self) -> Box<dyn TextBox> {
            Box::new(DarkTextBox)
        }
    }
}

/// Renders a button and a text box created by `factory`, displaying `txt`.
fn render(factory: &dyn GUIFactory, txt: &str) {
    let button = factory.create_button();
    let text_box = factory.create_text_box();
    button.render();
    text_box.display_text(txt);
}

/// Renders the GUI elements of the light theme, then of the dark theme.
fn main() {
    // Using the Light theme factory.
    render(&light::LightGUIFactory, "Hello from light theme!");
    println!();

    // Using the Dark theme factory.
    render(&dark::DarkGUIFactory, "Hello from dark theme!");
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
