//! Returns information / list of categories for a given crate, using the
//! crates.io API

mod api_client;
mod cat;
pub use api_client::*;
pub use cat::*;
