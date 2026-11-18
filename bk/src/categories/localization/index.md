<!-- Reviewed 2026-11-17. -->

# Localization

[![cat~localization][cat~localization~badge]][cat~localization]{{hi:Localization}}

Localization (l10n) adapts software for a particular language and region, especially by translating user-facing messages. Internationalization prepares software to support different locales; see [Internationalization](../internationalization/index.md) for locale selection, date and number formatting, and Unicode considerations.

## Choose a Message Library

- [Project Fluent][c~fluent~website]↗{{hi:Fluent}} supports message patterns for variables and plural forms. Rust integrations include [`fluent-bundle`][c~fluent-bundle~docs]↗{{hi:fluent-bundle}}, [`fluent-templates`][c~fluent-templates~docs]↗{{hi:fluent-templates}}, and [`i18n-embed`][c~i18n-embed~docs]↗{{hi:i18n-embed}}.
- [`gettext-rs`][c~gettext-rs~docs]↗{{hi:gettext-rs}} provides Rust bindings for projects that use GNU gettext catalogs.
- [`rust-i18n`][c~rust-i18n~docs]↗{{hi:rust-i18n}} offers a macro-based API with translations stored in YAML, JSON, or TOML files.

## Code Examples

{{#include localization.incl.md}}

## Related Topics

- [Command-line Interface](../command-line-interface/index.md)
- [Date and Time](../date-and-time/index.md)
- [Internationalization](../internationalization/index.md)
- [Template Engines](../template-engine/index.md)
- [Value Formatting](../value-formatting/index.md)
- [Web Programming](../web-programming/index.md)

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
