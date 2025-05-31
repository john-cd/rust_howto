// ANCHOR: example
//! Trait object example: a highly simplified GUI.

/// A trait for types that can be drawn.
trait Draw {
    /// Draws the object.
    fn draw(&self);
}

/// A struct representing a button.
struct Button;

/// Dummy implementation of the trait for `Button`.
impl Draw for Button {
    fn draw(&self) {
        println!("Button");
    }
}

/// A struct representing text to be drawn on a screen.
struct Text;

/// Dummy implementation.
impl Draw for Text {
    fn draw(&self) {
        println!("Text");
    }
}

/// A struct representing a screen that can display multiple components.
struct Screen {
    /// A vector of trait objects that can be drawn.
    /// Note the `dyn` keyword. Trait objects are dynamically sized types.
    /// Like all DSTs, trait objects must be used behind some type of pointer -
    /// here, `Box`.
    ///
    /// We use a trait object here, because the `Screen` may hold a mix of
    /// `Button` and `Text` objects, which may be unknown until run-time.
    /// A generic type would not work here - it would allow only one type or
    /// the other, not a mix of both.
    components: Vec<Box<dyn Draw>>, // Heterogenous collection.
}

impl Screen {
    /// Creates a new screen with some components. The list below
    /// could be dynamically generated at run time.
    fn new() -> Self {
        Screen {
            components: vec![Box::new(Button), Box::new(Text), Box::new(Text)],
        }
    }

    /// Runs the screen, drawing each component.
    fn run(&self) {
        for component in self.components.iter() {
            // The purpose of trait objects is to permit "late binding" of
            // methods. Calling a method on a trait object results
            // in dynamic dispatch at runtime.
            component.draw();
        }
    }
}

fn main() {
    let s = Screen::new();
    s.run();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
