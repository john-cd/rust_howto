// ANCHOR: example
//! This example demonstrates how to use the `monostate` crate with `serde` to
//! deserialize untagged enums where certain fields must have specific values.
//!
//! The default representation for enums in Serde is called the externally
//! tagged enum representation: {"Success": {"success": "..."}}
//! In JSON and other self-describing formats, this externally tagged
//! representation is often not ideal for readability. That can be changed with
//! e.g. the `#[serde(untagged)]` attribute: the untagged JSON representation is
//! {"success": "..."}. See https://serde.rs/enum-representations.html
//!
//! The `monostate` crate implements the `MustBe` macro, which makes fields
//! `serde` deserializable only if they have one specific value. This is
//! particularly helpful when deserializing untagged enums, as it allows you to
//! differentiate between variants based on the values of their fields.
use monostate::MustBe;
use serde::Deserialize;

/// Represents a response that can be either a success or an error.
#[derive(Deserialize)]
#[serde(untagged)]
pub enum ApiResponse {
    Success {
        success: MustBe!(true),
    },
    Error {
        success: MustBe!(false),
        _message: String,
    },
}

fn main() {
    let success = "{\"success\":true}";
    let response: ApiResponse = serde_json::from_str(success).unwrap();

    match response {
        ApiResponse::Success {
            success: MustBe!(true),
        } => {}
        ApiResponse::Error { .. } => panic!(),
    }

    let error = "{\"success\":false,\"_message\":\"...\"}";
    let response: ApiResponse = serde_json::from_str(error).unwrap();
    match response {
        ApiResponse::Error {
            success: MustBe!(false),
            ..
        } => {}
        ApiResponse::Success { .. } => panic!(),
    }
}
// Example adapted from https://github.com/dtolnay/monostate/blob/master/tests/test.rs
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
