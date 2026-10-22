//! Adapt mdBook 0.5's `RenderContext` to mdbook-linkcheck 0.7.7's 0.4 protocol.
//!
//! TODO Reviewed ??-??-2026

use std::io;
use std::process::Command;
use std::process::Stdio;
use std::process::{self};

use mdbook_preprocessor::errors::Error;
use mdbook_preprocessor::errors::Result;
use serde_json::Value;

/// Translates an mdBook 0.5 render context to the 0.4 protocol.
fn legacy_context(mut context: Value) -> Result<Value> {
    let book = context
        .get_mut("book")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| Error::msg("RenderContext is missing its book"))?;
    let items = book
        .remove("items")
        .filter(Value::is_array)
        .ok_or_else(|| Error::msg("expected mdBook 0.5 book.items"))?;
    book.insert("sections".into(), items);
    book.insert("__non_exhaustive".into(), Value::Null);
    // mdBook 0.5 serializes unset configuration defaults as null. The old
    // renderer deserializes configuration through TOML, which has no null.
    if let Some(config) = context.get_mut("config") {
        remove_unset_defaults(config);
    }
    // The checker never compiles Rust, but its old config type cannot decode
    // edition 2024. Change only its metadata copy, not the book's code blocks.
    if let Some(edition) = context.pointer_mut("/config/rust/edition")
        && edition.as_str() == Some("2024")
    {
        *edition = Value::String("2021".into());
    }
    // Advertise the translated protocol to the legacy renderer's version gate.
    context["version"] = Value::String("0.4.43".into());
    Ok(context)
}

/// Removes the null values, which TOML cannot represent.
fn remove_unset_defaults(value: &mut Value) {
    match value {
        Value::Object(table) => {
            table.retain(|_, value| !value.is_null());
            for value in table.values_mut() {
                remove_unset_defaults(value);
            }
        }
        Value::Array(values) => {
            for value in values {
                remove_unset_defaults(value);
            }
        }
        _ => {}
    }
}

/// Passes the translated context to `mdbook-linkcheck` and returns its exit
/// code.
fn run() -> Result<i32> {
    let context = legacy_context(serde_json::from_reader(io::stdin().lock())?)?;
    let mut child = Command::new("mdbook-linkcheck")
        .stdin(Stdio::piped())
        .spawn()?;
    // Closing stdin signals EOF; always reap the checker, even after a write error.
    let write_result = serde_json::to_writer(child.stdin.take().expect("piped stdin"), &context);
    let status = child.wait()?;
    write_result?;
    Ok(status.code().unwrap_or(1))
}

/// Runs the wrapper and exits with the checker's status code.
fn main() {
    let code = match run() {
        Ok(code) => code,
        Err(error) => {
            eprintln!("mdbook-linkcheck-compat: {error:#}");
            1
        }
    };
    process::exit(code);
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn preserves_content_and_settings_while_translating_the_book_container() {
        let items = json!([{"Chapter":{"content":"[link](chapter.md)","sub_items":[]}}]);
        let config = json!({"output":{"linkcheck":{"warning-policy":"error"}}});
        let adapted =
            legacy_context(json!({"book":{"items":items},"config":config,"version":"0.5.4"}))
                .expect("input should parse");
        assert_eq!(adapted["book"]["sections"], items);
        assert_eq!(adapted["config"], config);
        assert_eq!(adapted["version"], "0.4.43");
        assert!(adapted["book"].get("items").is_none());
        assert_eq!(adapted["book"]["__non_exhaustive"], Value::Null);
    }

    #[test]
    fn rejects_invalid_book_protocol() {
        legacy_context(json!({})).expect_err("input should be rejected");
        legacy_context(json!({"book":{"items":"wrong type"}}))
            .expect_err("input should be rejected");
    }

    #[test]
    fn omits_unset_config_defaults_without_changing_nullable_book_fields() {
        let adapted = legacy_context(json!({
            "book":{"items":[{"Chapter":{"path":null}}]},
            "config":{"book":{"title":null,"language":"en"}}
        }))
        .expect("input should parse");
        assert!(adapted["config"]["book"].get("title").is_none());
        assert_eq!(adapted["config"]["book"]["language"], "en");
        assert!(adapted["book"]["sections"][0]["Chapter"]["path"].is_null());
    }

    #[test]
    fn adapts_edition_metadata_without_modifying_rust_examples() {
        let items = json!([{"Chapter":{"content":"```rust,edition2024\nfn main() {}\n```"}}]);
        let adapted =
            legacy_context(json!({"book":{"items":items},"config":{"rust":{"edition":"2024"}}}))
                .expect("input should parse");
        assert_eq!(adapted["config"]["rust"]["edition"], "2021");
        assert_eq!(adapted["book"]["sections"], items);
    }
}
