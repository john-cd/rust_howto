# Internationalization

[![cat-internationalization][cat-internationalization-badge]][cat-internationalization]{{hi:Internationalization}}

Tools to help develop software capable of adapting to various languages and regions.

{{#include internationalization.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write (P2)](https://github.com/john-cd/rust_howto/issues/402)
TODO review below

Link to localization, accessibility

## Key Concepts

- Locales: Identifying specific languages and regions (e.g., "en-US", "fr-CA").
- Message catalogs: Storing translated strings.
- Message formatting: Inserting dynamic content into translations.
- Pluralization: Handling different plural forms.
- Internationalization: Designing software to be adaptable to different languages and cultures.

## Choosing the right crate

- If you're already using `gettext` or need its features, [`gettext`][c-gettext]⮳{{hi:gettext}} is a good choice.
- If you want a modern and expressive localization system, [`fluent`][c-fluent]⮳{{hi:fluent}} is a strong option.
- For basic 'i18n' utilities (language tags, etc.), [`unic-langid`][c-unic_langid]⮳{{hi:unic-langid}} is helpful.

For most projects, [`fluent`][c-fluent]⮳{{hi:fluent}} is probably the best starting point due to its modern design and expressiveness. [`gettext`][c-gettext]⮳{{hi:gettext}} is a solid choice if you have existing gettext experience or requirements. Remember that translation itself is an external process; the Rust crates help you manage and use the translated resources.

## Localization (l10n)

- [`gettext`][c-gettext]⮳{{hi:gettext}}: A popular crate for using gettext, a widely used localization system. It's a good choice if you're already familiar with gettext or need its specific features.
- [`fluent`][c-fluent]⮳{{hi:fluent}}: A localization system developed by Mozilla. The [`fluent-rs`][c-fluent]⮳{{hi:fluent-rs}} crate provides bindings to Fluent. Fluent is designed to be expressive and efficient.
- [`intl-rs`][c-intl_rs]⮳{{hi:intl-rs}}: Provides some internationalization utilities, but it's not a full localization solution on its own.

## Internationalization (i18n) Utilities

- [`unic-langid`][c-unic_langid]⮳{{hi:unic-langid}}: For parsing and working with language tags (e.g., "en-US", "fr"). Essential for handling locale information.
- [`chrono`][c-chrono]⮳{{hi:chrono}}: (Not specifically for i18n, but essential) For date and time formatting, which needs to be localized.
- [`num-format`][c-num_format]⮳{{hi:num-format}}: (Also not specifically for i18n, but important) For number formatting, which varies by locale.

## Message Formatting

Often handled by the localization system you choose (e.g., gettext, Fluent). Message formatting involves inserting variables and other dynamic content into localized strings.

## Resource Management

Localization systems typically handle loading and managing translated resources (strings, images, etc.).

## Pluralization

Handling plural forms correctly in different languages is a key part of localization. Localization systems often provide features for this.

## Translation

Translation itself is usually done by human translators or using machine translation services. Rust helps you use the translations, but they don't perform the translation.

</div>
