// ANCHOR: example
//! Dependency Injection (DI) is a design pattern that allows us to decouple
//! components of our application.
//!
//! Shaku is a Rust dependency injection library.
//!
//! Add to your `Cargo.toml` file:
//! ```toml
//! [dependencies]
//! shaku = "0.7.0" # Or latest
//! ```
use std::sync::Arc;

use shaku::Component;
use shaku::HasComponent;
use shaku::Interface;
use shaku::module;

// Define an interface.
trait Greeter: Interface {
    fn greet(&self, name: &str) -> String;
}

// The `#[derive(Component)]` attribute tells Shaku that these structs are
// components that can be managed by the dependency injection container.
// `Component` represents a single instance of a service, aka a singleton.
// Each time a component is resolved, you will get the same instance.
#[derive(Component)]
// The `#[shaku(interface = Greeter)]` attribute associates each component with
// the Greeter interface.
#[shaku(interface = Greeter)]
struct EnglishGreeter;

impl Greeter for EnglishGreeter {
    fn greet(&self, name: &str) -> String {
        format!("Hello, {}!", name)
    }
}

// A component is a struct that implements an `Interface` trait.
// `Interface` traits require certain bounds, such as 'static and optionally
// `Send
// + Sync` if using the `thread_safe` feature. The `Interface` trait acts as a
// trait alias for these bounds.
trait Conversation: Interface {
    fn start(&self) -> String;
}

#[derive(Component)]
#[shaku(interface = Conversation)]
struct EnglishConversation {
    // Components can depend on other components.
    // Make sure the property is declared as a trait object wrapped in an
    // `Arc`.
    #[shaku(inject)]
    greeter: Arc<dyn Greeter>,
    #[shaku(default)]
    person: String,
}

impl Conversation for EnglishConversation {
    fn start(&self) -> String {
        self.greeter.greet(self.person.as_str())
    }
}

// Define a module.
// `MyModule` is a module that groups related components and providers.
// Modules link together components (and providers), and are core to providing
// shaku's compile time guarentees.
module! {
    MyModule {
        components = [EnglishGreeter, EnglishConversation],
        providers = []
    }
}

fn main() {
    // Create an instance of the module, which manages the dependencies.
    let module = MyModule::builder().build();

    // Resolve the `Greeter` interface.
    let greeter: &dyn Greeter = module.resolve_ref();
    // Use the resolved component.
    let greeting = greeter.greet("World");
    println!("{}", greeting);

    let module = MyModule::builder()
        .with_component_parameters::<EnglishConversation>(
            EnglishConversationParameters {
                person: "John".to_string(),
            },
        )
        .build();
    let convo: Arc<dyn Conversation> = module.resolve();
    println!("{}", convo.start());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
