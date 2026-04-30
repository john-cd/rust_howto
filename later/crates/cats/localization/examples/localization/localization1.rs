#![allow(dead_code)]
// ANCHOR: example
//! Demonstrates locale-aware message selection using the `sys-locale` crate.
//!
//! This example reads the system locale, chooses a translation based on the
//! language code, and prints a localized greeting.

use std::error::Error;

use sys_locale::{get_locale, get_locales};

pub fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    let current_locale = get_locale().unwrap_or_else(|| "en-US".to_string());
    let mut preferred_locales = get_locales();
    let locale = preferred_locales.next().unwrap_or_else(|| current_locale.clone());

    let message = translate_greeting(&locale);

    println!("Detected locale: {}", locale);
    println!("Localized greeting: {}", message);
    println!("Full locale preference list: {:?}", preferred_locales.collect::<Vec<_>>());

    Ok(())
}

fn translate_greeting(locale: &str) -> &'static str {
    if locale.starts_with("es") {
        "¡Hola! Bienvenido al ejemplo de localización."
    } else if locale.starts_with("fr") {
        "Bonjour ! Bienvenue dans l'exemple de localisation."
    } else if locale.starts_with("ja") {
        "こんにちは！ローカリゼーションの例へようこそ。"
    } else if locale.starts_with("de") {
        "Hallo! Willkommen beim Lokalisierungsbeispiel."
    } else {
        "Hello! Welcome to the localization example."
    }
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translate_greeting_selects_spanish() {
        assert_eq!(translate_greeting("es-ES"), "¡Hola! Bienvenido al ejemplo de localización.");
    }

    #[test]
    fn translate_greeting_defaults_to_english() {
        assert_eq!(translate_greeting("unknown"), "Hello! Welcome to the localization example.");
    }
}
