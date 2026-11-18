<!-- TODO Reviewed ??-??-2026. -->

# Internationalization

{{#include internationalization.incl.md}}

Internationalization (i18n) prepares software for different languages and regions. Select a locale from user preferences with a documented fallback, and keep it overridable. [Localization](../localization/index.md) adapts translated messages and presentation for a particular audience.

## Format Numbers and Dates by Locale {#internationalization}

ICU4X's `DecimalFormatter` formats basic decimal numbers using locale-specific digits and grouping separators. The example formats the same value and date for US English and German. It formats a plain decimal, not a currency or measurement amount; those need domain-specific formatting rules. See [Value Formatting](../value-formatting/number_formatting.md) for related numeric output options.

```rust,editable
{{#include ../../../crates/cats/internationalization/examples/internationalization/icu_formatting.rs:example}}
```

Enable ICU4X's `compiled_data` feature to use the locale data bundled with the crate. This is the simplest setup for common locales and services. If binary size or supported locales need tighter control, use `icu4x-datagen` to produce application-specific data and load it through an ICU4X data provider.

## Dates, Sorting, and Text Boundaries

{{#badges icu_datetime}}

The `icu` crate re-exports the ICU4X component crates; [`icu_datetime`][c~icu_datetime~docs]↗ is the one behind the date formatting in the example above. Depend on it directly when you only need date and time formatting and want to avoid compiling the other components.

Use `icu::datetime` to format dates and times with locale-aware field order, calendar, and presentation. Keep the underlying date/time value independent of its formatted display.

Use `icu::collator` for locale-sensitive sorting, `icu::plurals` to select grammatical plural categories, and `icu::segmenter` to find word, line, sentence, or grapheme boundaries. These operations depend on locale data and should not be approximated with byte or character comparisons.

## Translate Messages and Test Localized Behavior

Keep user-facing messages in translation resources instead of concatenating translated fragments. Libraries such as [`Fluent`][c~fluent~docs]↗{{hi:Fluent}} and [`gettext`][c~gettext~docs]↗{{hi:gettext}} provide established message workflows; pass typed values into complete messages so translators can reorder words and apply language-specific grammar.

Test fallback locales, plural categories, long translated strings, dates around calendar boundaries, and right-to-left layouts where supported. Automated checks cannot replace review by native speakers or testing with the target operating system and assistive technologies.

## Related Topics {#related-topics .skip}

- [Localization](../localization/index.md).
- [Value Formatting](../value-formatting/number_formatting.md).
- [Date and Time](../date-and-time/index.md).
- [Unicode Text](../text-processing/unicode.md).
- [Localize API Errors](../web-programming_http-server/i18n.md).
- [Web Programming](../web-programming/index.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
