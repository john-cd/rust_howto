// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! The guest side of the `howto:greeter` component.
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

/// Calls the guest greeting implementation with a representative name.
///
/// # Errors
///
/// Returns an error if the example operation fails.
pub fn run() -> Result<String, GreetError> {
    <Component as Guest>::greet("Ada".to_owned())
}

/// Checks successful and rejected guest greetings.
#[cfg(test)]
mod tests {
    use super::Component;
    use super::GreetError;
    use super::Guest;

    /// Greets a non-empty name.
    #[test]
    fn greets_a_name() {
        assert_eq!(
            <Component as Guest>::greet("Ada".to_owned()),
            Ok("Hello, Ada!".to_owned())
        );
    }

    /// Rejects an empty name with the documented error.
    #[test]
    fn rejects_an_empty_name() {
        assert!(matches!(
            <Component as Guest>::greet(String::new()),
            Err(GreetError::EmptyName)
        ));
    }
}
