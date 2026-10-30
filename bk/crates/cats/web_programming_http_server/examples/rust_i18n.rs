// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Localizes an API error using the request's `Accept-Language` header.

// This macro must be declared at the crate root. Keeping this example in its
// own binary target gives it that root and loads `locales/*.yml`.
rust_i18n::i18n!("locales", fallback = "en");

/// Chooses a supported locale from an HTTP `Accept-Language` header.
fn locale_for(header: &str) -> &'static str {
    let supported = ["en", "fr"];
    match accept_language::intersection(header, &supported)
        .first()
        .map(String::as_str)
    {
        Some("fr") => "fr",
        _ => "en",
    }
}

/// Returns a localized API error for a missing user.
fn missing_user(header: &str, user_id: u64) -> String {
    let locale = locale_for(header);
    rust_i18n::t!("errors.user_not_found", locale = locale, id = user_id)
        .to_string()
}

/// Prints one localized API error using the client's preferred language.
fn main() {
    println!("{}", missing_user("fr-CA,fr;q=0.9,en;q=0.8", 42));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Selects the best supported language and falls back for unknown ones.
    #[test]
    fn selects_supported_locale() {
        assert_eq!(locale_for("fr-CA,fr;q=0.9,en;q=0.8"), "fr");
        assert_eq!(locale_for("de,de-AT;q=0.9"), "en");
    }

    /// Interpolates the identifier into each translated error message.
    #[test]
    fn translates_missing_user_error() {
        assert_eq!(missing_user("fr", 42), "Utilisateur 42 introuvable.");
        assert_eq!(missing_user("en", 42), "User 42 was not found.");
    }

    /// Runs the example entry point.
    #[test]
    fn test_main_entry_point() {
        main();
    }
}
// ANCHOR_END: example
