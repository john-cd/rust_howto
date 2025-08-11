# Value Formatting

[![cat~value-formatting][cat~value-formatting~badge]][cat~value-formatting]{{hi:Value formatting}}

Format values for display to a user, potentially adapting the display to various languages and regions.

If you only need basic number formatting, consider [`num-format`][c~num-format~docs]↗{{hi:num-format}}. Use [`chrono`][c~chrono~docs]↗{{hi:chrono}}, [`time`][c~time~docs]↗{{hi:time}} for simple Date/Time formatting. For complex localization, [`fluent`][c~fluent~docs]↗{{hi:fluent}} or [`icu`][c~icu~docs]↗{{hi:icu}} are better choices.

## Formatting Values for Display

This table covers Rust crates relevant for formatting values for display.

| Topic | Rust Crate(s) | Notes |
|---|---|---|
| [[number_formatting | Number Formatting]] | [`num-format`][c~num-format~docs]↗{{hi:num-format}}, `icu_number` (part of [`icu`][c~icu~docs]↗{{hi:icu}}) | [`num-format`][c~num-format~docs]↗{{hi:num-format}} provides flexible number formatting. `icu_number` (from the [`icu`][c~icu~docs]↗{{hi:icu}} crate family) offers more advanced number formatting with internationalization support. |
| [[date-and-time | Date and Time]] Formatting | [`chrono`][c~chrono~docs]↗{{hi:chrono}}, [`time`][c~time~docs]↗{{hi:time}}, [`icu_datetime`][c~icu_datetime~docs]↗{{hi:icu_datetime}} (part of [`icu`][c~icu~docs]↗{{hi:icu}}) | [`chrono`][c~chrono~docs]↗{{hi:chrono}} and [`time`][c~time~docs]↗{{hi:time}} are popular date and time libraries. [`icu_datetime`][c~icu_datetime~docs]↗{{hi:icu_datetime}} (from [`icu`][c~icu~docs]↗{{hi:icu}}) is part of the International Components for Unicode library and provides advanced date and time formatting with i18n support. |
| Currency Formatting | `icu_number`{{hi:icu_number}} (part of [`icu`][c~icu~docs]↗{{hi:icu}}) | `icu_number`{{hi:icu_number}} handles currency formatting according to locale. |
| Message Formatting (Pluralization, etc.) | [`fluent`][c~fluent~docs]↗{{hi:fluent}} | [`fluent`][c~fluent~docs]↗{{hi:fluent}} is a popular message formatting system that handles pluralization, gender, and other language-specific variations. |

## Code Examples

{{#include number_formatting.incl.md}}

## Related Topics

| Topic | Rust Crate(s) | Notes |
|---|---|---|
| [[localization | Localization]] (l10n) & [[internationalization | Internationalization]] (i18n) | [`fluent`][c~fluent~docs]↗{{hi:fluent}}, [`gettext-rs`][c~gettext~docs]↗{{hi:gettext-rs}} | [`fluent`][c~fluent~docs]↗{{hi:fluent}} is a powerful choice for managing localized messages. [`gettext-rs`][c~gettext~docs]↗{{hi:gettext-rs}} is a Rust implementation of the 'gettext' system, another standard for i18n. |
| Locale Management | [`locale`][c~locale~docs]↗{{hi:locale}} | Helps in determining the user's locale. |
| [[strings | String]] Formatting | [`std::fmt`][c~std::fmt~docs]↗{{hi:std::fmt}} | Rust's standard library provides basic string formatting capabilities. |
| [[command-line-interface | Command Line Interface]] | [`ansi_term`][c~ansi_term~docs]↗{{hi:ansi_term}}, [`termion`][c~termion~docs]↗{{hi:termion}}, [`console`][c~console~docs]↗{{hi:console}} | These crates help with formatting text for terminal output (colors, styles, etc.). |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/967)
need in-depth review
Mention [`itoa`][c~itoa~docs]↗{{hi:itoa}} in this page
cover  `Inflector`

- [ShowOption][c~show-option~lib.rs]

</div>
