//! - Recognize an absolute HTTP / HTTPS URL.
//! - Recognize specific URLs (e.g., specific domains and path patterns).

mod url_kind;
// TODO mod url_valid_chars;

pub use url_kind::*;
// pub use url_valid_chars::*;
