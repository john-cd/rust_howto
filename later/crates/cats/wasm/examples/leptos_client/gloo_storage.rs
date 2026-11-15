#![expect(
    dead_code,
    reason = "targets the browser (wasm32); native builds cannot call it"
)]
// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Persist typed settings in the browser's local storage with [`gloo_storage`].
//!
//! Local storage is a small key-value store in the browser that survives page
//! reloads, handy for user settings. It holds text only, so values are
//! serialized with Serde. Data is per origin and readable by any script on the
//! page, so never store secrets there.

use serde::Deserialize;
use serde::Serialize;

/// Key under which the settings are stored.
const KEY: &str = "settings";

/// User settings; every field has a default.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// Whether the interface should use its dark color theme.
    pub dark_mode: bool,
    /// Number of seconds to keep a toast notification visible.
    pub toast_seconds: u32,
}
/// Parses stored JSON, falling back to the defaults when the value is missing
///
/// or invalid. Kept free of browser APIs so it can be tested natively.
pub fn parse_settings(raw: Option<&str>) -> Settings {
    raw.and_then(|s| serde_json::from_str(s).ok())
        .unwrap_or_default()
}
/// Loads the settings from `localStorage` (browser only).
#[cfg(target_arch = "wasm32")]
pub fn load() -> Settings {
    use gloo_storage::LocalStorage;
    use gloo_storage::Storage;
    LocalStorage::get(KEY).unwrap_or_default()
}

/// Saves the settings to `localStorage` (browser only).
///
/// # Arguments
/// * `settings` - Settings applied to this operation.
///
/// # Errors
/// Returns an error if the output cannot be written or sent.
#[cfg(target_arch = "wasm32")]
pub fn save(settings: &Settings) -> Result<(), gloo_storage::errors::StorageError> {
    use gloo_storage::LocalStorage;
    use gloo_storage::Storage;
    LocalStorage::set(KEY, settings)
}
// ANCHOR_END: example

/// Parses default settings without accessing browser storage.
fn run() -> Settings {
    parse_settings(None)
}

/// Tests serialization and storage of values in browser local storage.
#[cfg(test)]
mod tests {
    use super::*;

    /// Uses default values when storage is missing or invalid.
    #[test]
    fn missing_or_invalid_gives_defaults() {
        assert_eq!(run(), Settings::default());
        assert_eq!(parse_settings(None), Settings::default());
        assert_eq!(parse_settings(Some("not json")), Settings::default());
    }

    /// Round-trips settings through JSON and fills in the default for an omitted field.
    #[test]
    fn round_trip_and_partial() -> anyhow::Result<()> {
        let s = Settings {
            dark_mode: true,
            toast_seconds: 5,
        };
        let json = serde_json::to_string(&s)?;
        assert_eq!(parse_settings(Some(&json)), s);
        assert_eq!(
            parse_settings(Some(r#"{"dark_mode":true}"#)),
            Settings {
                dark_mode: true,
                toast_seconds: 0
            }
        );
        Ok(())
    }
}
