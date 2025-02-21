# Value formatting

[![cat-value-formatting][cat-value-formatting-badge]][cat-value-formatting]{{hi:Value formatting}}

Format values for display to a user, potentially adapting the display to various languages and regions.

{{#include value-formatting.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[P2 review](https://github.com/john-cd/rust_howto/issues/967)

## Rust Crates for Formatting Values for Display

This table covers Rust crates relevant for formatting values for display, including internationalization (i18n) and localization (l10n) aspects.

| Topic | Rust Crates (Examples) | Notes |
|---|---|---|
| Number Formatting | `num-format`, `icu_number` (part of `icu`) | `num-format` provides flexible number formatting. `icu_number` (from the `icu` crate family) offers more advanced number formatting with internationalization support. |
| Date & Time Formatting | `chrono`, `time`, `icu_datetime` (part of `icu`) | `chrono` and `time` are popular date and time libraries. `icu_datetime` (from `icu`) is part of the International Components for Unicode library and provides advanced date and time formatting with i18n support. |
| Currency Formatting | `icu_number` (part of `icu`) | `icu_number` handles currency formatting according to locale. |
| Message Formatting (Pluralization, etc.) | `fluent`, `intl-memo` | `fluent` is a popular message formatting system that handles pluralization, gender, and other language-specific variations. `intl-memo` is a Rust implementation of the MessageFormat spec. |
| Localization (l10n) & Internationalization (i18n) | `fluent`, `gettext-rs` | `fluent` is a powerful choice for managing localized messages. `gettext-rs` is a Rust implementation of the gettext system, another standard for i18n. |
| Locale Management | `locale` | Helps in determining the user's locale. |
| Text Formatting (General) | `ansi_term`, `termion`, `console` | While not strictly i18n-focused, these crates help with formatting text for terminal output (colors, styles, etc.). |
| String Formatting | `std::fmt` | Rust's standard library provides basic string formatting capabilities. |

## Key Considerations

- ICU (International Components for Unicode): The `icu` crate family provides a robust and comprehensive set of tools for internationalization, including number, date/time, and currency formatting. It's a powerful option but adds some complexity.
- `Fluent`: Fluent is a modern and expressive message formatting system that handles complex localization needs well.
- `Gettext`: Gettext is a mature and widely used i18n system. `gettext-rs` provides a Rust implementation.
- Performance: Consider the performance implications of different formatting and localization libraries, especially in performance-critical applications.
- Complexity: Choose the library that best balances features and complexity for your project. If you only need basic number formatting, `num-format` might be sufficient. For complex localization, `fluent` or `icu` are better choices.

## Choosing the Right Crate

- Simple Number/Date/Time: `num-format`, `chrono`, `time` might be sufficient.
- Advanced Number/Date/Time/Currency (i18n): `icu` (specifically `icu_number`, `icu_datetime`) is a good choice.
- Message Formatting (Pluralization, etc.): `fluent` is recommended.
- General Localization (l10n/i18n): `fluent` or `gettext-rs` can be used.

</div>
