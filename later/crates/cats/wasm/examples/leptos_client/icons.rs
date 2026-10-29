#![allow(clippy::same_name_method)]
// TODO Reviewed ??-??-2026.
#![cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "the snippet declares items to illustrate them, not to call them"
    )
)]
// ANCHOR: example
//! Use [`icondata`] icon data as inline SVG in a Leptos view.
//!
//! `icondata` ships the paths of many icon sets as constants; enable only the
//! set you need through its Cargo features (here `lucide`) to keep builds
//! small.

use icondata::Icon as IconData;
use leptos::prelude::*;

/// Renders `icon` as an inline `<svg>` that follows the surrounding text color.
#[component]
pub fn Icon(icon: IconData, #[prop(into)] label: String) -> impl IntoView {
    view! {
        <svg
            viewBox=icon.view_box
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            role="img"
            aria-label=label
            inner_html=icon.data
        ></svg>
    }
}

/// Renders a small toolbar to a string, as a server would.
pub fn toolbar_html() -> String {
    view! {
        <nav>
            <Icon icon=icondata::LuHouse label="Home"/>
            <Icon icon=icondata::LuSettings label="Settings"/>
        </nav>
    }
    .to_html()
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that both icons are rendered with their accessible labels.
    #[test]
    fn toolbar_renders_labeled_svg_icons() {
        let html = toolbar_html();
        assert_eq!(html.matches("<svg").count(), 2);
        assert!(html.contains("aria-label=\"Home\""));
        assert!(html.contains("aria-label=\"Settings\""));
    }

    /// Verifies that the icon constants carry drawable data.
    #[test]
    fn icon_data_is_not_empty() {
        assert!(!icondata::LuHouse.data.is_empty());
        assert!(icondata::LuHouse.view_box.is_some());
    }
}
