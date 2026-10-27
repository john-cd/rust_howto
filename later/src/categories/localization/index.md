<!-- TODO Reviewed ??-??-2026 -->

# Localization

[![cat~localization][cat~localization~badge]][cat~localization]{{hi:Localization}}

Tools for localization help translate user-facing messages and adapt software to a particular locale.

Internationalization (i18n) prepares software to work across languages and regional conventions. Localization (l10n) adapts it for a particular locale, including translating user-facing messages.

## Choose a Message Library

- [Project Fluent][c~fluent~website]↗{{hi:Fluent}} uses message patterns for variables and plural forms; Rust integrations include [`fluent`][c~fluent~docs]↗{{hi:fluent}}, [`fluent-templates`][c~fluent-templates~docs]↗{{hi:fluent-templates}}, and [`i18n-embed`][c~i18n-embed~docs]↗{{hi:i18n-embed}}.
- [`gettext-rs`][c~gettext-rs~docs]↗{{hi:gettext-rs}} is a Rust binding for projects that use GNU gettext tools and catalogs.
- [`rust-i18n`][c~rust-i18n~docs]↗{{hi:rust-i18n}} provides a macro-based API with translations stored in YAML, JSON, or TOML files.

## Code Examples

{{#include localization.incl.md}}

## Related Topics

- [Localization guide][ex~localization~localization]↗{{hi:Localization}}.
- [[accessibility | Accessibility]].
- [[command-line-interface | Command Line Interface]].
- [[date-and-time | Date and Time]].
- [[gui | GUI]].
- [[internationalization | Internationalization]].
- [[template-engine | Template Engine]].
- [[value-formatting | Value Formatting]].
- [[web-programming | Web Programming]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
