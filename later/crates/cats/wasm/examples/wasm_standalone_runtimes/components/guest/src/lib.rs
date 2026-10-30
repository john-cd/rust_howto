// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! The guest side of the `howto:greeter` component.
//!
//! `wit_bindgen::generate!` reads the WIT file and generates the `Guest`
//! trait to implement and the `logger` import to call.

wit_bindgen::generate!({
    path: "../wit",
    world: "greeter",
});

use exports::howto::greeter::greeting::GreetError;
use exports::howto::greeter::greeting::Guest;
use howto::greeter::logger;

/// The longest name, in bytes, that the guest accepts.
const MAX_NAME_LEN: usize = 32;

/// The type that implements the exported interface.
struct Component;

impl Guest for Component {
    /// Greets `name`, rejecting empty or overly long names.
    fn greet(name: String) -> Result<String, GreetError> {
        logger::log(&format!("greet({name:?})"));
        if name.is_empty() {
            return Err(GreetError::EmptyName);
        }
        if name.len() > MAX_NAME_LEN {
            return Err(GreetError::TooLong(MAX_NAME_LEN as u32));
        }
        Ok(format!("Hello, {name}!"))
    }
}

export!(Component);
// ANCHOR_END: example
