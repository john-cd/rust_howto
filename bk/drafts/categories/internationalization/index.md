# Internationalization

[![cat-internationalization][cat-internationalization-badge]][cat-internationalization]{{hi:Internationalization}}

Tools to help develop software capable of adapting to various languages and regions.

For most projects, [`fluent`][c-fluent]⮳{{hi:fluent}} is probably the best starting point due to its modern design and expressiveness. [`gettext`][c-gettext]⮳{{hi:gettext}} is a solid choice if you have existing `gettext` experience or requirements. For basic 'i18n' utilities (language tags, etc.), [`unic-langid`][c-unic_langid]⮳{{hi:unic-langid}} is helpful.

For most new projects, [`fluent`][c-fluent]⮳{{hi:fluent}} is a good starting point due to its modern design and expressiveness. [`gettext`][c-gettext]⮳{{hi:gettext}} is a robust choice if you have existing gettext experience or project requirements.

## Internationalization vs Localization

- Internationalization (i18n): Make the application flexible to support multiple languages and regions without changing the codebase.
- [[localization | Localization]] (l10n): Adapt the application to a specific locale by translating text, adjusting formats, etc.

Localization refers to the *adaptation* of a product, application or document content to meet the language, cultural and other requirements of a specific target market (a locale).

Internationalization is the design and development of a product, application or document content that *enables* easy localization for target audiences that vary in culture, region, or language ([w3.org](https://www.w3.org/International/questions/qa-i18n)).

## Locale Detection

- Locale information:
  - [`unic-langid`][c-unic_langid]⮳{{hi:unic-langid}}: parsing and working with language tags (e.g., "en-US", "fr").
  - `unic-locale`: locale parsing and operations.

Determine the user's language and region settings (e.g., "en-US", "fr-CA") using:

- Browser settings for [[web-programming | Web]] applications (via headers).
- [[environment_variables | Environment variables]] like LANG on Unix systems.
- [[command-line-interface | Command-line]] arguments or configuration files for CLI tools.

## Localization systems

- Text Formatting: Handling pluralization, gender-specific terms, and placeholders dynamically.
  - `fluent` (Mozilla's library for complex localization features like placeholders, gender, and pluralization).
  - `gettext` or `rust-gettext` (for working with .po and .mo files).

Localization systems typically handle

- loading and managing translated resources (strings, images, etc.).
- Pluralization: handling plural forms correctly in different languages is a key part of localization.

### Dynamically Load Translations

Use a loader to fetch and apply the correct language file based on the detected or selected locale.
Example: With `fluent`, load `.ftl` files corresponding to the locale.

### Message Formatting / Dynamic Text Interpolation

Message formatting involves inserting variables and other dynamic content into localized strings. It is often handled by the localization system you choose (e.g., `gettext`, `fluent`).

### Pluralization and Gender Handling

- Pluralization: Handling different plural forms. Use libraries like `fluent` to manage.

- Singular/plural forms.
- Gender-specific terms dynamically.

## Locale-Specific Date and Time Formatting

- Date and Number Formatting: Use locale-specific formats for dates, times, numbers, and currencies.
  - [`chrono`][c-chrono]⮳{{hi:chrono}}.
  - `time`.

Use `chrono` or `time` for formatting e.g. 2025-03-03 as 03/03/2025 (US) or 03-03-2025 (EU).

## Numbers and Currencies

- Number formatting, which varies by locale: use [`num-format`][c-num_format]⮳{{hi:num-format}} for proper grouping and decimal separators.

### Message Formatting

Often handled by the localization system you choose (e.g., [`gettext`][c-gettext]⮳{{hi:gettext}}, [`Fluent`][c-fluent]⮳{{hi:Fluent}}). Message formatting involves inserting variables and other dynamic content into localized strings.

### Resource Management

Localization systems typically handle loading and managing translated resources (strings, images, etc.).

### Pluralization

Handling plural forms correctly in different languages is a key part of localization. Localization systems often provide features for this.

## Related Topics

## Internationalization (i18n) Utilities

- [`unic-langid`][c-unic_langid]⮳{{hi:unic-langid}}: For parsing and working with language tags (e.g., "en-US", "fr"). Essential for identifying locales.
- [`intl-rs`][c-intl_rs]⮳{{hi:intl-rs}}: Provides some internationalization utilities, but it's not a full localization solution on its own. It can be helpful for specific formatting tasks.
- [`chrono`][c-chrono]⮳{{hi:chrono}}: (Not specifically for i18n, but essential) For date and time formatting, which needs to be localized.
- [`num-format`][c-num_format]⮳{{hi:num-format}}: (Also not specifically for i18n, but important) For number formatting, which varies by locale.

## Translation

Translation itself is usually done by human translators or using machine translation services.

- ICU (International Components for Unicode): The [`icu`][c-icu]⮳{{hi:icu}} crate family provides a robust and comprehensive set of tools for internationalization, including number, date/time, and currency formatting. It's a powerful option but adds some complexity.
- [`Fluent`][c-fluent]⮳{{hi:Fluent}} is a modern and expressive message formatting system that handles complex localization needs well.
- [`Gettext`][c-gettext]⮳{{hi:Gettext}} is a mature and widely used i18n system. [`gettext-rs`][c-gettext]⮳{{hi:gettext-rs}} provides a Rust implementation.

## Localization (l10n) Systems

- [`gettext`][c-gettext]⮳{{hi:gettext}}: A widely used localization system. The [`gettext`][c-gettext]⮳{{hi:gettext}} crate provides bindings for Rust. A good choice if you're already familiar with gettext or need its specific features.
- [`fluent`][c-fluent]⮳{{hi:fluent}}: A modern localization system developed by Mozilla. The [`fluent-rs`][c-fluent]⮳{{hi:fluent-rs}} crate provides Rust bindings. Fluent is designed to be expressive and efficient. Often a preferred option for new projects.

- [`gettext`][c-gettext]⮳{{hi:gettext}}: A popular crate for using gettext, a widely used localization system. It's a good choice if you're already familiar with gettext or need its specific features.
- [`fluent`][c-fluent]⮳{{hi:fluent}}: A localization system developed by Mozilla. The [`fluent-rs`][c-fluent]⮳{{hi:fluent-rs}} crate provides bindings to Fluent. Fluent is designed to be expressive and efficient.
- [`intl-rs`][c-intl_rs]⮳{{hi:intl-rs}}: Provides some internationalization utilities, but it's not a full localization solution on its own.

## Code Examples

{{#include internationalization.incl.md}}

## Related Topics

- [[command-line-interface | Command Line Interface]].
- [[date-and-time | Date and Time]].
- [[gui | GUI]].
- [[internationalization | Internationalization]].
- [[template-engine | Template Engine]].
- [[value-formatting | Value Formatting]].
- [[web-programming | Web Programming]].

- [[accessibility | Accessibility]]
- [[localization | Localization]] (l10n)

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/402)
</div>

## Key Concepts

- Locales: Identifying specific languages and regions (e.g., "en-US", "fr-CA").
- Message catalogs: Storing translated strings.
- Message formatting: Inserting dynamic content into translations.
- Pluralization: Handling different plural forms.
- Internationalization: Designing software to be adaptable to different languages and cultures.

## Choosing the right crate

- If you're already using gettext or need its features, [`gettext`][c-gettext]⮳{{hi:gettext}} is a good choice.
- If you want a modern and expressive localization system, [`fluent`][c-fluent]⮳{{hi:fluent}} is often the preferred option.
- For basic i18n utilities (language tags, etc.), [`unic-langid`][c-unic_langid]⮳{{hi:unic-langid}} is helpful.
