//! Compares dynamic dispatch with a closed set of enum variants.
//!
//! TODO Reviewed ??-??-2026.

trait Animal {
    /// Returns the noise this animal makes.
    fn make_noise(&self) -> &'static str;
}

/// Data carried by dog operations.
struct Dog;
/// Data carried by cat operations.
struct Cat;

impl Animal for Dog {
    /// Handles the make noise behavior for this component.
    fn make_noise(&self) -> &'static str {
        "woof"
    }
}

impl Animal for Cat {
    /// Handles the make noise behavior for this component.
    fn make_noise(&self) -> &'static str {
        "meow"
    }
}

// ANCHOR: selection
/// Selects a concrete animal behind a common trait object.
fn get_animal(is_dog: bool) -> Box<dyn Animal> {
    if is_dog { Box::new(Dog) } else { Box::new(Cat) }
}
// ANCHOR_END: selection

/// Dispatch resolves to the implementation of each stored animal.
#[test]
fn dispatches_to_each_animal() {
    // ANCHOR: collection
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat), Box::new(Dog)];
    let noises: Vec<_> = animals.iter().map(|animal| animal.make_noise()).collect();
    // ANCHOR_END: collection

    assert_eq!(noises, ["woof", "meow", "woof"]);
}

/// Both branches return a usable trait object.
#[test]
fn selects_an_animal_at_runtime() {
    assert_eq!(get_animal(true).make_noise(), "woof");
    assert_eq!(get_animal(false).make_noise(), "meow");
}

// ANCHOR: shape
enum Shape {
    /// A circle whose radius determines its area.
    Circle {
        /// Distance from the circle's center to its edge.
        radius: f64,
    },
    /// A square whose side length determines its area.
    Square {
        /// Length of each side of the square.
        side: f64,
    },
}

impl Shape {
    /// Describes the variant and its dimensions.
    fn draw(&self) -> String {
        match self {
            Self::Circle { radius } => format!("Draw a circle with radius {radius}"),
            Self::Square { side } => format!("Draw a square with side {side}"),
        }
    }
}
// ANCHOR_END: shape

/// The closed set of shape variants can be handled with a match.
#[test]
fn matches_each_shape_variant() {
    assert_eq!(
        Shape::Circle { radius: 2.0 }.draw(),
        "Draw a circle with radius 2"
    );
    assert_eq!(
        Shape::Square { side: 3.0 }.draw(),
        "Draw a square with side 3"
    );
}

// ANCHOR: bounds
type AnimalOnly = dyn Animal;
/// Alias for `dyn Animal + Send` used to represent send animal.
type SendAnimal = dyn Animal + Send;
/// Alias for `dyn Animal + Send + Sync` used to represent shared animal.
type SharedAnimal = dyn Animal + Send + Sync;
/// Alias for `dyn Animal + 'static` used to represent static animal.
type StaticAnimal = dyn Animal + 'static;
/// Alias for `dyn Animal + Send + 'static` used to represent static send animal.
type StaticSendAnimal = dyn Animal + Send + 'static;
// ANCHOR_END: bounds

/// The example animals meet each declared set of bounds.
#[test]
fn supports_auto_traits_and_lifetime_bounds() {
    let animal: &AnimalOnly = &Cat;
    let send: &SendAnimal = &Dog;
    let shared: &SharedAnimal = &Cat;
    let static_animal: &StaticAnimal = &Dog;
    let static_send: &StaticSendAnimal = &Cat;

    assert_eq!(
        [
            animal.make_noise(),
            send.make_noise(),
            shared.make_noise(),
            static_animal.make_noise(),
            static_send.make_noise()
        ],
        ["meow", "woof", "meow", "woof", "meow"],
    );
}
