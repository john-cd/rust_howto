//! Returns information / list of categories for a given crate, using the
//! crates.io API
//!
//! TODO Reviewed ??-??-2026.

/// Looks up crate categories through the crates.io API.
mod categories_for_crate;
/// Retrieves crate metadata and retains contextual lookup failures.
mod info;

use anyhow::Result;
pub use categories_for_crate::*;
use crates_io_api::SyncClient;
pub use info::*;

/// Instantiate the crates.io API client.
fn get_client() -> Result<SyncClient> {
    let client = SyncClient::new(
        "rust_howto_agent (agent@example.com)",
        std::time::Duration::from_millis(1000), // rate limit interval
    )?;
    Ok(client)
}

// unit tests; cleanup

/// the crates.io API
#[cfg(test)]
mod tests {
    use super::get_client;

    /// Initializes the client without starting a network request.
    #[test]
    fn client_initializes_without_making_a_request() {
        get_client().expect("input should parse");
    }
}
