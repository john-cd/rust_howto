//! Formats a greeting through the public module API.

use crate::module1::submodule2::Greeting;
use crate::module2;

/// Sends the input through the library's greeting API.
pub fn format_welcome(name: &str) -> String {
    module2::message(Greeting(name))
}
