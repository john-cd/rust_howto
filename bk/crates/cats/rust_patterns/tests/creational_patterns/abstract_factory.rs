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
//! lets you define an interface for creating these families of widgets,
//!  and then you can have concrete factories for each theme.

// Operations common to all concrete objects.
trait Button {
    fn render(&self);
}

trait TextBox {
    fn display_text(&self, text: &str);
}

// Abstract factory interface (trait).
trait GUIFactory {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_text_box(&self) -> Box<dyn TextBox>;
}

mod light {
    use super::*;

    // Concrete object implementations for Light Theme.
    struct LightButton;

    impl Button for LightButton {
        fn render(&self) {
            println!("Rendering a light button.");
        }
    }

    struct LightTextBox;

    impl TextBox for LightTextBox {
        fn display_text(&self, text: &str) {
            println!("Displaying '{}' in a light text box.", text);
        }
    }

    // The concrete factory implementation produce concrete objects belonging to
    // a specific family (here, light theme).
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

mod dark {

    use super::*;

    // Concrete object implementations for Dark Theme.
    struct DarkButton;

    impl Button for DarkButton {
        fn render(&self) {
            println!("Rendering a dark button.");
        }
    }

    struct DarkTextBox;

    impl TextBox for DarkTextBox {
        fn display_text(&self, text: &str) {
            println!("Displaying '{}' in a dark text box.", text);
        }
    }

    // Concrete Factory for Dark Theme.
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

fn render(factory: &dyn GUIFactory, txt: &str) {
    let button = factory.create_button();
    let text_box = factory.create_text_box();
    button.render();
    text_box.display_text(txt);
}

fn main() {
    // Using the Light theme factory.
    render(&light::LightGUIFactory, "Hello from light theme!");
    println!();

    // Using the Dark theme factory.
    render(&dark::DarkGUIFactory, "Hello from dark theme!");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
