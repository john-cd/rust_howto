#![cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "targets the browser (wasm32); native builds cannot call it"
    )
)]
// ANCHOR: example
//! Converts structured values between JavaScript and Rust with Serde.
//!
//! JavaScript values and Rust structs have different representations. Serde
//! converts between them, so Rust code compiled to WebAssembly can receive a
//! JavaScript object as a typed struct and return one back.

use serde::Deserialize;
use serde::Serialize;

/// A small profile value exchanged with JavaScript.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct Profile {
    /// The profile's display name.
    name: String,
    /// The number of visits recorded for this profile.
    visits: u32,
}

/// Increments the visit count, reporting overflow instead of wrapping.
fn increment_profile(profile: &mut Profile) -> Result<(), &'static str> {
    profile.visits = profile
        .visits
        .checked_add(1)
        .ok_or("profile visit count overflowed")?;
    Ok(())
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use wasm_bindgen::prelude::*;

/// Deserializes a JavaScript profile, increments its visit count, and returns
/// it.
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[wasm_bindgen]
pub fn increment_visits(value: JsValue) -> Result<JsValue, JsValue> {
    let mut profile: Profile = serde_wasm_bindgen::from_value(value)
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    increment_profile(&mut profile).map_err(JsValue::from_str)?;
    serde_wasm_bindgen::to_value(&profile)
        .map_err(|error| JsValue::from_str(&error.to_string()))
}
// ANCHOR_END: example

/// Explains how to run the browser-only conversion on a native host.
fn main() {
    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    println!("Build for wasm32-unknown-unknown to convert JavaScript values.");
}

/// Runs the example's host-side runner.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::Profile;
    use super::increment_profile;

    /// Verifies that a profile's visit count can be incremented.
    #[test]
    fn increments_profile_visits() {
        let mut profile = Profile {
            name: String::from("Ada"),
            visits: 3,
        };

        assert_eq!(increment_profile(&mut profile), Ok(()));
        assert_eq!(profile.visits, 4);
    }

    /// Verifies that visit-count overflow is reported.
    #[test]
    fn reports_profile_visit_overflow() {
        let mut profile = Profile {
            name: String::from("Grace"),
            visits: u32::MAX,
        };

        assert_eq!(
            increment_profile(&mut profile),
            Err("profile visit count overflowed")
        );
    }

    /// Verifies that the host-side example runner succeeds.
    #[test]
    fn host_runner_succeeds() {
        super::main();
    }
}
