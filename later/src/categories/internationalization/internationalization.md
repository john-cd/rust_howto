<!-- TODO Reviewed ??-??-2026 -->

# Internationalization

{{#include internationalization.incl.md}}

Internationalization (i18n) prepares software for different languages and regions. Select a locale from user preferences with a documented fallback, and keep it overridable. Localization (l10n) adapts translated messages and presentation for a particular audience; see [[localization | Localization]] and [issue #404](https://github.com/john-cd/rust_howto/issues/404).

## Format Numbers and Dates by Locale {#internationalization}

ICU4X's `DecimalFormatter` formats basic decimal numbers using locale-specific digits and grouping separators. The example formats the same value and date for US English and German. ICU4X 2.x does not yet format currencies or measurement units with this formatter; see [[value-formatting | Value Formatting]] and the open [number-formatting work in issue #490](https://github.com/john-cd/rust_howto/issues/490) for currency display.

```rust,editable
{{#include ../../../crates/cats/internationalization/examples/internationalization/icu_formatting.rs:example}}
```

Enable ICU4X's `compiled_data` feature to use the locale data bundled with the crate. This is the simplest setup for common locales and services. If binary size or supported locales need tighter control, use `icu4x-datagen` to produce application-specific data and load it through an ICU4X data provider.

## Dates, Sorting, and Text Boundaries

Use `icu::datetime` to format dates and times with locale-aware field order, calendar, and presentation. Keep the underlying date/time value independent of its formatted display.

Use `icu::collator` for locale-sensitive sorting, `icu::plurals` to select grammatical plural categories, and `icu::segmenter` to find word, line, sentence, or grapheme boundaries. These operations depend on locale data and should not be approximated with byte or character comparisons.

## Translate Messages and Test Localized Behavior

Keep user-facing messages in translation resources instead of concatenating translated fragments. Libraries such as [`Fluent`][c~fluent~docs]↗{{hi:Fluent}} and [`gettext`][c~gettext~docs]↗{{hi:gettext}} provide established message workflows; pass typed values into complete messages so translators can reorder words and apply language-specific grammar.

Test fallback locales, plural categories, long translated strings, dates around calendar boundaries, and right-to-left layouts where supported. Automated checks cannot replace review by native speakers or testing with the target operating system and assistive technologies.

## Related Topics {#related-topics .skip}

- [[localization | Localization]].
- [[value-formatting | Value Formatting]].
- [[date-and-time | Date and Time]].
- [[accessibility | Accessibility]].
- [[gui | GUI]].
- [[web-programming | Web Programming]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
