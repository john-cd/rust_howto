# Localization

[![cat-localization][cat-localization-badge]][cat-localization]{{hi:Localization}}

Tooling to help adapting internationalized software to specific languages and regions.

For most new projects, [`fluent`][c-fluent]⮳{{hi:fluent}} is a good starting point due to its modern design and expressiveness. [`gettext`][c-gettext]⮳{{hi:gettext}} is a robust choice if you have existing gettext experience or project requirements.

## Localization (l10n) Systems

- [`gettext`][c-gettext]⮳{{hi:gettext}}: A widely used localization system. The [`gettext`][c-gettext]⮳{{hi:gettext}} crate provides bindings for Rust. A good choice if you're already familiar with gettext or need its specific features.
- [`fluent`][c-fluent]⮳{{hi:fluent}}: A modern localization system developed by Mozilla. The [`fluent-rs`][c-fluent]⮳{{hi:fluent-rs}} crate provides Rust bindings. Fluent is designed to be expressive and efficient. Often a preferred option for new projects.

{{#include localization.incl.md}}

### Message Formatting

Often handled by the localization system you choose (e.g., [`gettext`][c-gettext]⮳{{hi:gettext}}, [`Fluent`][c-fluent]⮳{{hi:Fluent}}). Message formatting involves inserting variables and other dynamic content into localized strings.

### Resource Management

Localization systems typically handle loading and managing translated resources (strings, images, etc.).

### Pluralization

Handling plural forms correctly in different languages is a key part of localization. Localization systems often provide features for this.

## Related Topics

## Internationalization (i18n) Utilities (Supporting Localization)

- [`unic-langid`][c-unic_langid]⮳{{hi:unic-langid}}: For parsing and working with language tags (e.g., "en-US", "fr"). Essential for identifying locales.
- [`intl-rs`][c-intl_rs]⮳{{hi:intl-rs}}: Provides some internationalization utilities, but it's not a full localization solution on its own. It can be helpful for specific formatting tasks.
- [`chrono`][c-chrono]⮳{{hi:chrono}}: (Not specifically for i18n, but essential) For date and time formatting, which needs to be localized.
- [`num-format`][c-num_format]⮳{{hi:num-format}}: (Also not specifically for i18n, but important) For number formatting, which varies by locale.

## Translation

Translation itself is usually done by human translators or using machine translation services.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[localization/_index: write (P2)](https://github.com/john-cd/rust_howto/issues/405)
TODO review below

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

</div>
