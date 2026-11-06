//! Integration tests of the public badge templates; no network access.
//!
//! TODO Reviewed ??-??-2026.

use tool_lib::create_category_badge;
use tool_lib::create_rbe_badge;

/// The category badge mentions the (trimmed) category and slug.
#[test]
fn category_badge_contains_category_and_slug() -> anyhow::Result<()> {
    let badge = create_category_badge(" Encoding ", " encoding ")?;
    assert!(badge.contains("Encoding"));
    assert!(badge.contains("encoding"));
    assert!(!badge.contains(" Encoding "));
    Ok(())
}

/// The Rust By Example badge mentions the concept.
#[test]
fn rbe_badge_contains_concept() -> anyhow::Result<()> {
    let badge = create_rbe_badge("attributes")?;
    assert!(badge.contains("attributes"));
    Ok(())
}
