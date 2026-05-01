#![allow(dead_code)]
// ANCHOR: example
//! Demonstrates basic internationalization using `sys-locale` and `icu_locid`.
//!
//! This example obtains the system locale, canonicalizes it, extracts the
//! language and region subtags, and prints a locale-aware greeting.

use std::error::Error;

use icu_locid::Locale;
use sys_locale::{get_locale, get_locales};

pub fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    let current_locale = get_locale().unwrap_or_else(|| "und".to_string());
    let mut preferred_locales = get_locales();
    let locale_tag = preferred_locales.next().unwrap_or_else(|| current_locale.clone());

    let locale = Locale::try_from_bytes(locale_tag.as_bytes()).unwrap_or_default();
    let canonical = Locale::canonicalize(locale_tag.as_bytes())?;
    let language = locale.id.language.to_string();
    let region = locale.id.region.map(|r| r.to_string()).unwrap_or_else(|| "unknown".to_string());

    println!("System locale: {}", current_locale);
    println!("Preferred locale: {}", locale_tag);
    println!("Canonical locale: {}", canonical);
    println!("Language: {}", language);
    println!("Region: {}", region);
    println!("Greeting: {}", render_greeting(&language, &region));
    println!("Locale preference order: {:?}", preferred_locales.collect::<Vec<_>>());

    Ok(())
}

fn render_greeting(language: &str, region: &str) -> &'static str {
    match (language, region) {
        ("es", _) => "¡Hola! Bienvenido a la internacionalización en Rust.",
        ("fr", _) => "Bonjour ! Bienvenue dans l'internationalisation en Rust.",
        ("ja", _) => "こんにちは！Rustでの国際化へようこそ。",
        ("de", _) => "Hallo! Willkommen bei der Internationalisierung in Rust.",
        ("pt", "BR") => "Olá! Bem-vindo à internacionalização em Rust.",
        _ => "Hello! Welcome to internationalization in Rust.",
    }
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_greeting_returns_english_for_unknown_locale() {
        assert_eq!(render_greeting("xx", "ZZ"), "Hello! Welcome to internationalization in Rust.");
    }

    #[test]
    fn render_greeting_returns_portuguese_for_brazil() {
        assert_eq!(render_greeting("pt", "BR"), "Olá! Bem-vindo à internacionalização em Rust.");
    }
}
