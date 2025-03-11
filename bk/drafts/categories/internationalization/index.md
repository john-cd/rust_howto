# Internationalization

[![cat-internationalization][cat-internationalization-badge]][cat-internationalization]{{hi:Internationalization}}

Tools to help develop software capable of adapting to various languages and regions.

For most projects, [`fluent`][c-fluent]⮳{{hi:fluent}} is probably the best starting point due to its modern design and expressiveness. [`gettext`][c-gettext]⮳{{hi:gettext}} is a solid choice if you have existing `gettext` experience or requirements. For basic 'i18n' utilities (language tags, etc.), [`unic-langid`][c-unic_langid]⮳{{hi:unic-langid}} is helpful.

## Definitions

[[localization | Localization]] (l10n) refers to the *adaptation* of an application to meet the language, cultural, and other requirements of a specific target market (a locale) - by translating text, adjusting formats, etc.

Internationalization (i18n) is the design and development of a product, application or document content that *enables* easy localization for target audiences that vary in culture, region, or language - without major changes to the codebase. ([w3.org](https://www.w3.org/International/questions/qa-i18n)).

The following will address both.

## Locale Detection

Locale detection refers to determining the user's language and region settings (e.g., "en-US", "fr-CA").

Locale may be detected using:

- Browser settings for [[web-programming | Web]] applications (via headers).
- [[environment_variables | Environment variables]] like LANG on Unix systems.
- [[command-line-interface | Command-line]] arguments or configuration files for CLI tools.

Useful Rust crates for this purpose include:

- [`unic-langid`][c-unic_langid]⮳{{hi:unic-langid}}: parsing and working with language tags (e.g., "en-US", "fr").
- `unic-locale`: locale parsing and operations.

## Message Formatting, Dynamic Text Interpolation

Internationalization systems typically handle:

- Loading and managing translated resources (strings, images, etc.). That includes fetching and applying the correct language file based on the detected or selected locale.
- Inserting variables and other dynamic content into placeholders within localized strings.
- Handling pluralization and gender-specific terms dynamically.

Recommended crates include the following:

- 'Fluent' is a modern, expressive localization system developed by Mozilla that handles complex localization needs well. The [`fluent-rs`][c-fluent]⮳{{hi:fluent-rs}} crate provides Rust bindings. It is often a preferred option for new projects.
- 'Gettext' is a mature and widely used i18n system. [`gettext-rs`][c-gettext]⮳{{hi:gettext-rs}} or `rust-gettext` provides a Rust implementation. It is a good choice if you're already familiar with 'gettext' or need its specific features.
- [`intl-rs`][c-intl_rs]⮳{{hi:intl-rs}} provides some internationalization utilities, but it's not a full localization solution on its own.

Note that string translation itself is usually done by human translators or using machine translation services.

## Locale-Specific Date and Time Formatting

To generate locale-specific formats for dates and times, use:

- [`chrono`][c-chrono]⮳{{hi:chrono}}.
- [`time`][c-time]⮳{{hi:time}}.

## Formatting of Numbers and Currencies

For number formatting, which also varies by locale, use [`num-format`][c-num_format]⮳{{hi:num-format}} for proper grouping and decimal separators.

The [`icu`][c-icu]⮳{{hi:icu}} crate family (named after 'International Components for Unicode') also provides a robust and comprehensive set of tools for internationalization, including number, date/time, and currency formatting. It's a powerful option but adds some complexity.

## Code Examples

{{#include internationalization.incl.md}}

## See also

- [[accessibility | Accessibility]].
- [[command-line-interface | Command Line Interface]].
- [[date-and-time | Date and Time]].
- [[gui | GUI]].
- [[localization | Localization]] (l10n).
- [[template-engine | Template Engine]].
- [[value-formatting | Value Formatting]].
- [[web-programming | Web Programming]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/402)
Review `whoami`
</div>
